//! `odm-actions` — resolve cwd + shell-out Action dispatch.

use std::path::{Path, PathBuf};
use std::process::Command;

use odm_core::{abs_checkout, ActionDef, ActionTask, OdmError, Workspace};

#[derive(Debug, Clone, Copy)]
pub struct RunOptions<'a> {
    pub project: Option<&'a str>,
    pub wt: Option<&'a str>,
    pub extra_args: &'a [String],
}

pub fn list_actions(ws: &Workspace) -> Vec<(&str, &ActionDef)> {
    ws.actions
        .iter()
        .map(|(n, d)| (n.as_str(), d))
        .collect()
}

pub fn resolve_cwd(
    ws_root: &Path,
    project_path: Option<&str>,
    wt_slot: Option<&str>,
    task_dir: Option<&str>,
) -> Result<PathBuf, OdmError> {
    if let Some(dir) = task_dir {
        let cwd = ws_root.join(dir);
        if !cwd.is_dir() {
            return Err(OdmError::usage(format!(
                "action task dir does not exist: {dir}"
            )));
        }
        return Ok(cwd);
    }
    if let Some(slot) = wt_slot {
        let project = project_path.ok_or_else(|| {
            OdmError::usage("--wt requires --project")
        })?;
        let cwd = ws_root
            .join("worktrees")
            .join(project)
            .join(slot);
        if !cwd.is_dir() {
            return Err(OdmError::usage(format!(
                "worktree path does not exist: worktrees/{project}/{slot}"
            )));
        }
        return Ok(cwd);
    }
    if let Some(rel) = project_path {
        let cwd = abs_checkout(ws_root, rel);
        if !cwd.is_dir() {
            return Err(OdmError::usage(format!(
                "project path does not exist: {rel}"
            )));
        }
        return Ok(cwd);
    }
    Ok(ws_root.to_path_buf())
}

pub fn run_action(
    ws: &Workspace,
    action_name: &str,
    opts: RunOptions<'_>,
) -> Result<i32, OdmError> {
    let def = ws.actions.get(action_name).ok_or_else(|| {
        OdmError::usage(format!("unknown action '{action_name}'"))
    })?;

    if opts.wt.is_some() && opts.project.is_none() {
        return Err(OdmError::usage("--wt requires --project"));
    }

    let project_rel = if let Some(name) = opts.project {
        let entry = ws.config.projects.get(name).ok_or_else(|| {
            OdmError::usage(format!("unknown project '{name}'"))
        })?;
        Some(entry.path.as_str())
    } else {
        None
    };

    // resolve_cwd: with --wt, project_path arg is the project *name* (worktrees/<name>/<slot>).
    // without --wt, it is the project primary relative path.
    let cwd_project = if opts.wt.is_some() {
        opts.project
    } else {
        project_rel
    };

    let n = def.tasks.len();
    for (i, task) in def.tasks.iter().enumerate() {
        let cwd = resolve_cwd(
            ws.root.as_path(),
            cwd_project,
            opts.wt,
            task.dir.as_deref(),
        )?;
        let is_last = i + 1 == n;
        let extra = if is_last { opts.extra_args } else { &[] };
        let code = run_task(task, &cwd, extra)?;
        if code != 0 {
            return Ok(code);
        }
    }
    Ok(0)
}

fn run_task(task: &ActionTask, cwd: &Path, extra_args: &[String]) -> Result<i32, OdmError> {
    let mut cmd = Command::new("sh");
    cmd.current_dir(cwd);
    if extra_args.is_empty() {
        cmd.arg("-c").arg(&task.run);
    } else {
        let script = format!("{} \"$@\"", task.run);
        cmd.arg("-c").arg(script).arg("_");
        for a in extra_args {
            cmd.arg(a);
        }
    }
    let status = cmd.status().map_err(|e| {
        OdmError::operation(format!("failed to spawn shell for action: {e}"))
    })?;
    Ok(status.code().unwrap_or(1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::fs;

    use odm_core::{ActionDef, ActionTask, ProjectEntry, Workspace, WorkspaceConfig};
    use tempfile::tempdir;

    fn ws_with_actions(root: PathBuf, actions: BTreeMap<String, ActionDef>) -> Workspace {
        let mut config = WorkspaceConfig::default();
        config.projects.insert(
            "alpha".into(),
            ProjectEntry {
                path: "projects/alpha".into(),
                url: None,
                branch: None,
                type_: None,
            },
        );
        Workspace {
            root,
            config,
            actions,
            generators: BTreeMap::new(),
        }
    }

    fn single(run: &str, dir: Option<&str>) -> ActionDef {
        ActionDef {
            tasks: vec![ActionTask {
                run: run.into(),
                dir: dir.map(str::to_string),
            }],
        }
    }

    #[test]
    fn echo_success() {
        let dir = tempdir().unwrap();
        let mut actions = BTreeMap::new();
        actions.insert("hello".into(), single("echo hello-desk", None));
        let ws = ws_with_actions(dir.path().to_path_buf(), actions);
        let code = run_action(
            &ws,
            "hello",
            RunOptions {
                project: None,
                wt: None,
                extra_args: &[],
            },
        )
        .unwrap();
        assert_eq!(code, 0);
    }

    #[test]
    fn fail_exit_code() {
        let dir = tempdir().unwrap();
        let mut actions = BTreeMap::new();
        actions.insert("fail".into(), single("exit 7", None));
        let ws = ws_with_actions(dir.path().to_path_buf(), actions);
        let code = run_action(
            &ws,
            "fail",
            RunOptions {
                project: None,
                wt: None,
                extra_args: &[],
            },
        )
        .unwrap();
        assert_eq!(code, 7);
    }

    #[test]
    fn multi_task_stop_on_fail() {
        let dir = tempdir().unwrap();
        let marker = dir.path().join("should-not-exist");
        let mut actions = BTreeMap::new();
        actions.insert(
            "chain".into(),
            ActionDef {
                tasks: vec![
                    ActionTask {
                        run: "exit 3".into(),
                        dir: None,
                    },
                    ActionTask {
                        run: format!("touch {}", marker.display()),
                        dir: None,
                    },
                ],
            },
        );
        let ws = ws_with_actions(dir.path().to_path_buf(), actions);
        let code = run_action(
            &ws,
            "chain",
            RunOptions {
                project: None,
                wt: None,
                extra_args: &[],
            },
        )
        .unwrap();
        assert_eq!(code, 3);
        assert!(!marker.exists());
    }

    #[test]
    fn cwd_dir() {
        let dir = tempdir().unwrap();
        let sub = dir.path().join("sub");
        fs::create_dir_all(&sub).unwrap();
        let out = dir.path().join("out.txt");
        let mut actions = BTreeMap::new();
        actions.insert(
            "pwd".into(),
            single(&format!("pwd > {}", out.display()), Some("sub")),
        );
        let ws = ws_with_actions(dir.path().to_path_buf(), actions);
        let code = run_action(
            &ws,
            "pwd",
            RunOptions {
                project: None,
                wt: None,
                extra_args: &[],
            },
        )
        .unwrap();
        assert_eq!(code, 0);
        let text = fs::read_to_string(&out).unwrap();
        assert!(text.contains("sub"), "got: {text}");
    }

    #[test]
    fn extra_args_on_last_task() {
        let dir = tempdir().unwrap();
        let out = dir.path().join("args.txt");
        let mut actions = BTreeMap::new();
        actions.insert(
            "args".into(),
            ActionDef {
                tasks: vec![
                    ActionTask {
                        run: "true".into(),
                        dir: None,
                    },
                    ActionTask {
                        run: format!("printf '%s\\n' > {}", out.display()),
                        dir: None,
                    },
                ],
            },
        );
        let ws = ws_with_actions(dir.path().to_path_buf(), actions);
        let extras = vec!["one".into(), "two".into()];
        let code = run_action(
            &ws,
            "args",
            RunOptions {
                project: None,
                wt: None,
                extra_args: &extras,
            },
        )
        .unwrap();
        assert_eq!(code, 0);
        let text = fs::read_to_string(&out).unwrap();
        assert_eq!(text, "one\ntwo\n");
    }

    #[test]
    fn unknown_action() {
        let dir = tempdir().unwrap();
        let ws = ws_with_actions(dir.path().to_path_buf(), BTreeMap::new());
        let err = run_action(
            &ws,
            "nope",
            RunOptions {
                project: None,
                wt: None,
                extra_args: &[],
            },
        )
        .unwrap_err();
        assert!(err.to_string().contains("unknown action"));
    }

    #[test]
    fn missing_task_dir() {
        let dir = tempdir().unwrap();
        let mut actions = BTreeMap::new();
        actions.insert("x".into(), single("true", Some("missing")));
        let ws = ws_with_actions(dir.path().to_path_buf(), actions);
        let err = run_action(
            &ws,
            "x",
            RunOptions {
                project: None,
                wt: None,
                extra_args: &[],
            },
        )
        .unwrap_err();
        assert!(err.to_string().contains("does not exist"));
    }

    #[test]
    fn resolve_cwd_priority() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("projects/alpha")).unwrap();
        fs::create_dir_all(root.join("worktrees/alpha/slot1")).unwrap();
        fs::create_dir_all(root.join("taskdir")).unwrap();

        let task = resolve_cwd(root, Some("projects/alpha"), Some("slot1"), Some("taskdir")).unwrap();
        assert_eq!(task, root.join("taskdir"));

        let wt = resolve_cwd(root, Some("alpha"), Some("slot1"), None).unwrap();
        assert_eq!(wt, root.join("worktrees/alpha/slot1"));

        let proj = resolve_cwd(root, Some("projects/alpha"), None, None).unwrap();
        assert_eq!(proj, root.join("projects/alpha"));

        let base = resolve_cwd(root, None, None, None).unwrap();
        assert_eq!(base, root);
    }

    #[test]
    fn run_action_project_cwd() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("projects/alpha")).unwrap();
        fs::write(root.join("projects/alpha/marker"), "proj\n").unwrap();
        let out = root.join("out.txt");
        let mut actions = BTreeMap::new();
        actions.insert(
            "cat".into(),
            single(&format!("cat marker > {}", out.display()), None),
        );
        let ws = ws_with_actions(root.to_path_buf(), actions);
        let code = run_action(
            &ws,
            "cat",
            RunOptions {
                project: Some("alpha"),
                wt: None,
                extra_args: &[],
            },
        )
        .unwrap();
        assert_eq!(code, 0);
        assert_eq!(fs::read_to_string(&out).unwrap(), "proj\n");
    }

    #[test]
    fn run_action_wt_cwd() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("projects/alpha")).unwrap();
        fs::create_dir_all(root.join("worktrees/alpha/slot1")).unwrap();
        fs::write(root.join("worktrees/alpha/slot1/marker"), "wt\n").unwrap();
        let out = root.join("out.txt");
        let mut actions = BTreeMap::new();
        actions.insert(
            "cat".into(),
            single(&format!("cat marker > {}", out.display()), None),
        );
        let ws = ws_with_actions(root.to_path_buf(), actions);
        let code = run_action(
            &ws,
            "cat",
            RunOptions {
                project: Some("alpha"),
                wt: Some("slot1"),
                extra_args: &[],
            },
        )
        .unwrap();
        assert_eq!(code, 0);
        assert_eq!(fs::read_to_string(&out).unwrap(), "wt\n");
    }

    #[test]
    fn run_action_wt_requires_project() {
        let dir = tempdir().unwrap();
        let mut actions = BTreeMap::new();
        actions.insert("x".into(), single("true", None));
        let ws = ws_with_actions(dir.path().to_path_buf(), actions);
        let err = run_action(
            &ws,
            "x",
            RunOptions {
                project: None,
                wt: Some("slot1"),
                extra_args: &[],
            },
        )
        .unwrap_err();
        assert!(err.to_string().contains("--wt requires --project"));
    }
}

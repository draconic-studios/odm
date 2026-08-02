//! One-shot agent start: resolve Project/wt cwd and direct-exec a program.

use std::path::PathBuf;
use std::process::Command;

use odm_core::{OdmError, Workspace};

use crate::{resolve_cwd, CwdTarget, StdioMode};

/// Options for [`start_agent`].
#[derive(Debug, Clone, Copy)]
pub struct StartOptions<'a> {
    /// Required project name (start is project-scoped; no workspace-root cwd).
    pub project: &'a str,
    /// Optional worktree slot under `worktrees/<project>/`.
    pub wt: Option<&'a str>,
    /// Program to exec (caller-supplied; no default agent binary).
    pub program: &'a str,
    /// Arguments passed to `program`.
    pub args: &'a [String],
    pub stdio: StdioMode,
}

/// Outcome of a one-shot agent start.
///
/// Streams are `Some` only under [`StdioMode::Capture`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartResult {
    pub exit_code: i32,
    pub cwd: PathBuf,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

/// Resolve cwd for a Project Primary or worktree slot, then direct-exec `program`.
///
/// Not a shell (`sh -c`), session, or pack/prompt composition — pure argv exec.
pub fn start_agent(ws: &Workspace, opts: StartOptions<'_>) -> Result<StartResult, OdmError> {
    if opts.project.is_empty() {
        return Err(OdmError::usage("project is required"));
    }
    if opts.program.is_empty() {
        return Err(OdmError::usage("program is required"));
    }
    let target = match opts.wt {
        None => CwdTarget::Project {
            name: opts.project,
        },
        Some(slot) => {
            if slot.is_empty() {
                return Err(OdmError::usage("worktree slot must not be empty"));
            }
            CwdTarget::Worktree {
                project: opts.project,
                slot,
            }
        }
    };
    let cwd = resolve_cwd(ws, target, None)?;

    let mut cmd = Command::new(opts.program);
    cmd.args(opts.args).current_dir(&cwd);

    match opts.stdio {
        StdioMode::Inherit => {
            let status = cmd.status().map_err(|e| {
                OdmError::operation(format!("failed to spawn agent: {e}"))
            })?;
            Ok(StartResult {
                exit_code: status.code().unwrap_or(1),
                cwd,
                stdout: None,
                stderr: None,
            })
        }
        StdioMode::Capture => {
            let output = cmd.output().map_err(|e| {
                OdmError::operation(format!("failed to spawn agent: {e}"))
            })?;
            Ok(StartResult {
                exit_code: output.status.code().unwrap_or(1),
                cwd,
                stdout: Some(String::from_utf8_lossy(&output.stdout).into_owned()),
                stderr: Some(String::from_utf8_lossy(&output.stderr).into_owned()),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::fs;

    use odm_core::{ProjectEntry, Workspace, WorkspaceConfig};
    use tempfile::tempdir;

    fn ws_with_alpha(root: PathBuf) -> Workspace {
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
            actions: BTreeMap::new(),
            generators: BTreeMap::new(),
        }
    }

    fn opts<'a>(
        project: &'a str,
        wt: Option<&'a str>,
        program: &'a str,
        args: &'a [String],
        stdio: StdioMode,
    ) -> StartOptions<'a> {
        StartOptions {
            project,
            wt,
            program,
            args,
            stdio,
        }
    }

    #[test]
    fn true_exits_zero_on_project_primary() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("projects/alpha")).unwrap();
        let ws = ws_with_alpha(root.to_path_buf());
        let result = start_agent(
            &ws,
            opts("alpha", None, "true", &[], StdioMode::Inherit),
        )
        .unwrap();
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.cwd, root.join("projects/alpha"));
        assert!(result.stdout.is_none());
        assert!(result.stderr.is_none());
    }

    #[test]
    fn false_passthrough_nonzero() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("projects/alpha")).unwrap();
        let ws = ws_with_alpha(root.to_path_buf());
        let result = start_agent(
            &ws,
            opts("alpha", None, "false", &[], StdioMode::Inherit),
        )
        .unwrap();
        assert_ne!(result.exit_code, 0);
        assert!(result.stdout.is_none());
        assert!(result.stderr.is_none());
    }

    #[test]
    fn capture_echoes_stdout() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("projects/alpha")).unwrap();
        let ws = ws_with_alpha(root.to_path_buf());
        let args = vec!["start-lib-marker".into()];
        let result = start_agent(
            &ws,
            opts("alpha", None, "echo", &args, StdioMode::Capture),
        )
        .unwrap();
        assert_eq!(result.exit_code, 0);
        let stdout = result.stdout.as_deref().unwrap();
        assert!(stdout.contains("start-lib-marker"), "got: {stdout}");
        assert!(result.stderr.is_some());
    }

    #[test]
    fn wt_slot_cwd() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("projects/alpha")).unwrap();
        fs::create_dir_all(root.join("worktrees/alpha/slot1")).unwrap();
        let ws = ws_with_alpha(root.to_path_buf());
        let args = vec!["-P".into()];
        let result = start_agent(
            &ws,
            opts("alpha", Some("slot1"), "pwd", &args, StdioMode::Capture),
        )
        .unwrap();
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.cwd, root.join("worktrees/alpha/slot1"));
        let stdout = result.stdout.as_deref().unwrap();
        let want = root.join("worktrees/alpha/slot1");
        assert!(
            stdout.contains(want.to_str().unwrap()),
            "cwd pwd got: {stdout}"
        );
    }

    #[test]
    fn unknown_project_is_usage() {
        let dir = tempdir().unwrap();
        let ws = ws_with_alpha(dir.path().to_path_buf());
        let err = start_agent(
            &ws,
            opts("nope", None, "true", &[], StdioMode::Inherit),
        )
        .unwrap_err();
        assert!(matches!(err, OdmError::Usage(_)), "{err:?}");
        assert!(err.to_string().contains("unknown project"), "{err}");
    }

    #[test]
    fn missing_project_path_is_not_found() {
        let dir = tempdir().unwrap();
        let ws = ws_with_alpha(dir.path().to_path_buf());
        let err = start_agent(
            &ws,
            opts("alpha", None, "true", &[], StdioMode::Inherit),
        )
        .unwrap_err();
        assert!(matches!(err, OdmError::NotFound(_)), "{err:?}");
        assert!(err.to_string().contains("project path missing"), "{err}");
    }

    #[test]
    fn missing_wt_slot_is_not_found() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("projects/alpha")).unwrap();
        let ws = ws_with_alpha(root.to_path_buf());
        let err = start_agent(
            &ws,
            opts("alpha", Some("missing"), "true", &[], StdioMode::Inherit),
        )
        .unwrap_err();
        assert!(matches!(err, OdmError::NotFound(_)), "{err:?}");
        assert!(err.to_string().contains("worktree slot not found"), "{err}");
    }

    #[test]
    fn empty_project_is_usage() {
        let dir = tempdir().unwrap();
        let ws = ws_with_alpha(dir.path().to_path_buf());
        let err = start_agent(
            &ws,
            opts("", None, "true", &[], StdioMode::Inherit),
        )
        .unwrap_err();
        assert!(matches!(err, OdmError::Usage(_)), "{err:?}");
        assert!(err.to_string().contains("project is required"), "{err}");
    }

    #[test]
    fn empty_program_is_usage() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("projects/alpha")).unwrap();
        let ws = ws_with_alpha(root.to_path_buf());
        let err = start_agent(
            &ws,
            opts("alpha", None, "", &[], StdioMode::Inherit),
        )
        .unwrap_err();
        assert!(matches!(err, OdmError::Usage(_)), "{err:?}");
        assert!(err.to_string().contains("program is required"), "{err}");
    }

    #[test]
    fn spawn_fail_is_operation() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("projects/alpha")).unwrap();
        let ws = ws_with_alpha(root.to_path_buf());
        let err = start_agent(
            &ws,
            opts(
                "alpha",
                None,
                "/nonexistent/odm-agent-start-bin",
                &[],
                StdioMode::Inherit,
            ),
        )
        .unwrap_err();
        assert!(matches!(err, OdmError::Operation(_)), "{err:?}");
        assert!(err.to_string().contains("failed to spawn agent"), "{err}");
    }
}

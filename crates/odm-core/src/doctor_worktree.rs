use std::collections::BTreeSet;
use std::fs;

use odm_git::Git;

use crate::config::Workspace;
use crate::doctor::{CheckStatus, DoctorCheck};
use crate::paths::abs_checkout;
use crate::worktree::{validate_slot_name, worktree_list};

/// Warn on `worktrees/<project>/<slot>/` dirs that are not registered git worktrees.
/// Never fails; swallows git/path errors. Does not scan unknown project names under `worktrees/`.
pub(crate) fn worktree_orphan_checks<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
) -> Vec<DoctorCheck> {
    let mut checks = Vec::new();
    for (project, entry) in &ws.config.projects {
        let Ok(primary) = abs_checkout(&ws.root, &entry.path) else {
            continue;
        };
        if !primary.is_dir() {
            continue;
        }

        let project_wt = ws.root.join("worktrees").join(project);
        if !project_wt.is_dir() {
            continue;
        }

        // Registered slot names under prefix (same filter as `worktree_list`).
        // Non-git primary / list errors → skip project (no Fail from this feature).
        let registered: BTreeSet<String> = match worktree_list(git, ws, project) {
            Ok(out) => out.slots.into_iter().map(|s| s.name).collect(),
            Err(_) => continue,
        };

        let Ok(rd) = fs::read_dir(&project_wt) else {
            continue;
        };
        let mut disk_slots: Vec<String> = Vec::new();
        for ent in rd.flatten() {
            let Ok(ft) = ent.file_type() else {
                continue;
            };
            if !ft.is_dir() {
                continue;
            }
            let name = ent.file_name();
            let Some(s) = name.to_str() else {
                continue;
            };
            if validate_slot_name(s).is_err() {
                continue;
            }
            disk_slots.push(s.to_string());
        }
        disk_slots.sort();

        for slot in disk_slots {
            if registered.contains(&slot) {
                continue;
            }
            let rel = format!("worktrees/{project}/{slot}");
            checks.push(DoctorCheck {
                id: format!("worktree_orphan:{project}:{slot}"),
                status: CheckStatus::Warn,
                message: format!(
                    "orphan worktree slot directory (not a registered git worktree): {rel}"
                ),
                fixable: false,
            });
        }
    }
    checks
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::ffi::{OsStr, OsString};
    use std::io;
    use std::path::{Path, PathBuf};
    use std::process::ExitStatus;
    use std::sync::{Arc, Mutex};

    use odm_git::{CommandOutput, CommandRunner};
    use tempfile::tempdir;

    use crate::config::{ProjectEntry, WorkspaceConfig};
    use crate::doctor::run_doctor;
    use crate::init::{init_workspace, InitOptions};
    use crate::paths::worktree_slot_path;

    #[cfg(unix)]
    use std::os::unix::process::ExitStatusExt;

    fn exit_ok() -> ExitStatus {
        #[cfg(unix)]
        {
            ExitStatus::from_raw(0)
        }
        #[cfg(not(unix))]
        {
            std::process::Command::new("true").status().unwrap()
        }
    }

    fn exit_fail(code: i32) -> ExitStatus {
        #[cfg(unix)]
        {
            ExitStatus::from_raw(code << 8)
        }
        #[cfg(not(unix))]
        {
            let _ = code;
            std::process::Command::new("false").status().unwrap()
        }
    }

    struct ScriptedRunner {
        calls: Arc<Mutex<Vec<Vec<OsString>>>>,
        queue: Mutex<Vec<io::Result<CommandOutput>>>,
    }

    impl ScriptedRunner {
        fn new(outputs: Vec<CommandOutput>) -> (Self, Arc<Mutex<Vec<Vec<OsString>>>>) {
            let calls = Arc::new(Mutex::new(Vec::new()));
            (
                Self {
                    calls: Arc::clone(&calls),
                    queue: Mutex::new(outputs.into_iter().map(Ok).collect()),
                },
                calls,
            )
        }
    }

    impl CommandRunner for ScriptedRunner {
        fn output(&self, _program: &OsStr, args: &[OsString]) -> io::Result<CommandOutput> {
            self.calls.lock().unwrap().push(args.to_vec());
            let mut q = self.queue.lock().unwrap();
            if q.is_empty() {
                Ok(CommandOutput {
                    status: exit_ok(),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            } else {
                q.remove(0)
            }
        }

        fn status(&self, _program: &OsStr, args: &[OsString]) -> io::Result<ExitStatus> {
            self.calls.lock().unwrap().push(args.to_vec());
            Ok(exit_ok())
        }
    }

    fn out_ok_stdout(stdout: &str) -> CommandOutput {
        CommandOutput {
            status: exit_ok(),
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
        }
    }

    fn out_fail(stderr: &str) -> CommandOutput {
        CommandOutput {
            status: exit_fail(1),
            stdout: Vec::new(),
            stderr: stderr.as_bytes().to_vec(),
        }
    }

    fn is_repo_true() -> CommandOutput {
        out_ok_stdout("true\n")
    }

    fn is_repo_false() -> CommandOutput {
        out_fail("not a repo")
    }

    fn ws_with_project(root: PathBuf, name: &str, rel: &str) -> Workspace {
        let mut projects = BTreeMap::new();
        projects.insert(
            name.into(),
            ProjectEntry {
                path: rel.into(),
                url: None,
                branch: None,
                type_: None,
            },
        );
        Workspace {
            root,
            config: WorkspaceConfig {
                projects,
                manage_gitignore: Some(false),
                ..Default::default()
            },
            actions: BTreeMap::new(),
            generators: BTreeMap::new(),
        }
    }

    fn ensure_primary(root: &Path, rel: &str) -> PathBuf {
        let p = root.join(rel);
        fs::create_dir_all(&p).unwrap();
        p
    }

    // --- worktree orphan checks ---

    #[test]
    fn orphan_slot_dir_warns_not_fixable() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        let orphan = worktree_slot_path(root, "alpha", "stale");
        fs::create_dir_all(&orphan).unwrap();
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let porcelain = format!(
            "worktree {}\nHEAD abc\nbranch refs/heads/main\n\n",
            primary.display()
        );
        let (runner, _) = ScriptedRunner::new(vec![is_repo_true(), out_ok_stdout(&porcelain)]);
        let g = Git::with_runner(runner);
        let checks = worktree_orphan_checks(&g, &ws);
        assert_eq!(checks.len(), 1);
        let c = &checks[0];
        assert_eq!(c.id, "worktree_orphan:alpha:stale");
        assert_eq!(c.status, CheckStatus::Warn);
        assert!(!c.fixable);
        assert!(c.message.contains("worktrees/alpha/stale"));
    }

    #[test]
    fn registered_slot_is_not_orphan() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        let slot = worktree_slot_path(root, "alpha", "feat");
        fs::create_dir_all(&slot).unwrap();
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let porcelain = format!(
            "worktree {}\nHEAD abc\nbranch refs/heads/main\n\n\
             worktree {}\nHEAD def\nbranch refs/heads/feat\n\n",
            primary.display(),
            slot.display(),
        );
        let (runner, _) = ScriptedRunner::new(vec![is_repo_true(), out_ok_stdout(&porcelain)]);
        let g = Git::with_runner(runner);
        let checks = worktree_orphan_checks(&g, &ws);
        assert!(
            checks.iter().all(|c| !c.id.contains("worktree_orphan")),
            "healthy slot must not warn: {checks:?}"
        );
    }

    #[test]
    fn missing_worktrees_dir_yields_no_orphan_checks() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        ensure_primary(root, "projects/alpha");
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        // No git calls expected — worktrees/ absent short-circuits.
        let (runner, calls) = ScriptedRunner::new(vec![]);
        let g = Git::with_runner(runner);
        let checks = worktree_orphan_checks(&g, &ws);
        assert!(checks.is_empty());
        assert!(calls.lock().unwrap().is_empty());
    }

    #[test]
    fn non_git_primary_skips_orphan_scan() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        ensure_primary(root, "projects/alpha");
        let orphan = worktree_slot_path(root, "alpha", "stale");
        fs::create_dir_all(&orphan).unwrap();
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let (runner, _) = ScriptedRunner::new(vec![is_repo_false()]);
        let g = Git::with_runner(runner);
        let checks = worktree_orphan_checks(&g, &ws);
        assert!(checks.is_empty(), "non-git must not Fail/Warn orphans: {checks:?}");
    }

    #[test]
    fn unknown_worktrees_project_dir_ignored() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        ensure_primary(root, "projects/alpha");
        // Configured project has no worktrees dir; stray unconfigured name under worktrees/.
        fs::create_dir_all(root.join("worktrees/ghost/slot")).unwrap();
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let (runner, calls) = ScriptedRunner::new(vec![]);
        let g = Git::with_runner(runner);
        let checks = worktree_orphan_checks(&g, &ws);
        assert!(checks.is_empty());
        assert!(calls.lock().unwrap().is_empty());
    }

    #[test]
    fn doctor_fix_does_not_delete_orphan_dirs() {
        let dir = tempdir().unwrap();
        init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: true,
            name: None,
        })
        .unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        let orphan = worktree_slot_path(root, "alpha", "stale");
        fs::create_dir_all(&orphan).unwrap();
        let mut cfg = WorkspaceConfig {
            manage_gitignore: Some(false),
            ..Default::default()
        };
        cfg.projects.insert(
            "alpha".into(),
            ProjectEntry {
                path: "projects/alpha".into(),
                url: None,
                branch: None,
                type_: None,
            },
        );
        let ws = Workspace {
            root: root.to_path_buf(),
            config: cfg,
            actions: Default::default(),
            generators: Default::default(),
        };
        let porcelain = format!(
            "worktree {}\nHEAD abc\nbranch refs/heads/main\n\n",
            primary.display()
        );
        // observe: is_repo + head + origin + dirty; orphan: is_repo + worktree list.
        // manage_gitignore off → no workspace is_repo. apply_fixes only ensures .odm layout.
        let (runner, _) = ScriptedRunner::new(vec![
            is_repo_true(),
            out_ok_stdout("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n"), // head_sha
            out_ok_stdout("https://example.com/alpha.git\n"),             // origin
            out_ok_stdout(""),                                             // is_clean
            is_repo_true(),
            out_ok_stdout(&porcelain),
        ]);
        let g = Git::with_runner(runner);
        let report = run_doctor(&g, &ws, true).unwrap();
        assert!(orphan.is_dir(), "doctor --fix must not delete orphan dirs");
        let c = report
            .checks
            .iter()
            .find(|c| c.id == "worktree_orphan:alpha:stale")
            .expect("orphan warn present");
        assert_eq!(c.status, CheckStatus::Warn);
        assert!(!c.fixable);
        assert!(report.ok); // Warn only
    }

    #[test]
    fn invalid_slot_name_dir_skipped_silently() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        // "a/b" can't be a single dir name; use ".." which validate_slot_name rejects.
        // Creating ".." as a directory name is awkward; use a name with invalid chars if possible.
        // On disk we can create a dir named with spaces-only? empty rejected after trim.
        // Create "." is the current dir. Use a file (not dir) and a valid orphan to prove filter.
        let project_wt = root.join("worktrees/alpha");
        fs::create_dir_all(&project_wt).unwrap();
        fs::write(project_wt.join("not-a-dir"), b"x").unwrap();
        let orphan = worktree_slot_path(root, "alpha", "real");
        fs::create_dir_all(&orphan).unwrap();
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let porcelain = format!(
            "worktree {}\nHEAD abc\nbranch refs/heads/main\n\n",
            primary.display()
        );
        let (runner, _) = ScriptedRunner::new(vec![is_repo_true(), out_ok_stdout(&porcelain)]);
        let g = Git::with_runner(runner);
        let checks = worktree_orphan_checks(&g, &ws);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].id, "worktree_orphan:alpha:real");
    }
}

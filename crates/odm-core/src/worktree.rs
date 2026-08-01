//! Worktree slot lifecycle — list/add/rm under `worktrees/<project>/<slot>/`.

use std::fs;
use std::path::{Component, Path, PathBuf};

use odm_git::Git;

use crate::config::Workspace;
use crate::error::OdmError;
use crate::paths::{abs_checkout, worktree_slot_path};

/// One slot after add or rm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorktreeSlotOutcome {
    pub project: String,
    pub slot: String,
    /// Relative to workspace root: `worktrees/<project>/<slot>`.
    pub path: String,
}

/// One slot row for list.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct WorktreeSlotInfo {
    pub name: String,
    /// Relative to workspace root: `worktrees/<project>/<slot>`.
    pub path: String,
}

/// List outcome for a project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorktreeListOutcome {
    pub project: String,
    pub slots: Vec<WorktreeSlotInfo>,
}

/// Validate a worktree slot name; return trimmed name.
///
/// Rejects empty, `.` / `..`, path separators, NUL, and absolute-looking names.
pub fn validate_slot_name(name: &str) -> Result<String, OdmError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(OdmError::usage("worktree slot name must not be empty"));
    }
    if trimmed == "." || trimmed == ".." {
        return Err(OdmError::usage(format!(
            "invalid worktree slot name '{trimmed}'"
        )));
    }
    if trimmed.contains('/') || trimmed.contains('\\') || trimmed.contains('\0') {
        return Err(OdmError::usage(format!(
            "invalid worktree slot name '{trimmed}': path separators not allowed"
        )));
    }
    // Belt-and-suspenders: absolute / Windows drive even without separators.
    if trimmed.starts_with('/') {
        return Err(OdmError::usage(format!(
            "invalid worktree slot name '{trimmed}': must be a simple name"
        )));
    }
    let bytes = trimmed.as_bytes();
    if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
        return Err(OdmError::usage(format!(
            "invalid worktree slot name '{trimmed}': must be a simple name"
        )));
    }
    Ok(trimmed.to_string())
}

/// List git worktree slots under `worktrees/<project>/` (sorted by name).
pub fn worktree_list<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
    project: &str,
) -> Result<WorktreeListOutcome, OdmError> {
    let primary = resolve_primary_git(git, ws, project)?;
    let prefix = ws.root.join("worktrees").join(project);
    let entries = git.worktree_list(&primary)?;
    let mut slots: Vec<WorktreeSlotInfo> = entries
        .into_iter()
        .filter_map(|e| slot_info_under_prefix(&prefix, project, &e.path))
        .collect();
    slots.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(WorktreeListOutcome {
        project: project.to_string(),
        slots,
    })
}

/// Add a worktree slot at `worktrees/<project>/<slot>/`.
pub fn worktree_add<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
    project: &str,
    slot: &str,
    branch: Option<&str>,
) -> Result<WorktreeSlotOutcome, OdmError> {
    let slot = validate_slot_name(slot)?;
    let primary = resolve_primary_git(git, ws, project)?;
    let slot_path = worktree_slot_path(&ws.root, project, &slot);
    if slot_path.exists() {
        return Err(OdmError::operation(format!(
            "worktree slot path already exists: {}",
            rel_slot_path(project, &slot)
        )));
    }
    if let Some(parent) = slot_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            OdmError::operation(format!(
                "failed to create worktrees parent {}: {e}",
                parent.display()
            ))
        })?;
    }
    git.worktree_add(&primary, &slot_path, branch)?;
    Ok(WorktreeSlotOutcome {
        project: project.to_string(),
        slot: slot.clone(),
        path: rel_slot_path(project, &slot),
    })
}

/// Remove a worktree slot.
pub fn worktree_rm<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
    project: &str,
    slot: &str,
    force: bool,
) -> Result<WorktreeSlotOutcome, OdmError> {
    let slot = validate_slot_name(slot)?;
    let primary = resolve_primary_git(git, ws, project)?;
    let slot_path = worktree_slot_path(&ws.root, project, &slot);
    if !slot_path.exists() {
        return Err(OdmError::not_found(format!(
            "worktree slot not found: {}",
            rel_slot_path(project, &slot)
        )));
    }
    let entries = git.worktree_list(&primary)?;
    let registered = entries.iter().any(|e| paths_equal(&e.path, &slot_path));
    if !registered {
        return Err(OdmError::operation(format!(
            "path exists but is not a registered git worktree: {}",
            rel_slot_path(project, &slot)
        )));
    }
    git.worktree_remove(&primary, &slot_path, force)?;
    // Best-effort: remove empty worktrees/<project>/ directory.
    let project_wt = ws.root.join("worktrees").join(project);
    let _ = fs::remove_dir(&project_wt);
    Ok(WorktreeSlotOutcome {
        project: project.to_string(),
        slot: slot.clone(),
        path: rel_slot_path(project, &slot),
    })
}

fn rel_slot_path(project: &str, slot: &str) -> String {
    format!("worktrees/{project}/{slot}")
}

fn resolve_primary_git<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
    project: &str,
) -> Result<PathBuf, OdmError> {
    let entry = ws.config.projects.get(project).ok_or_else(|| {
        OdmError::usage(format!("unknown project '{project}'"))
    })?;
    let path = abs_checkout(&ws.root, &entry.path)?;
    if !path.exists() {
        return Err(OdmError::not_found(format!(
            "project path missing: {}",
            entry.path
        )));
    }
    if !git.is_repo(&path)? {
        return Err(OdmError::operation(format!(
            "project path is not a git repo: {}",
            entry.path
        )));
    }
    Ok(path)
}

fn slot_info_under_prefix(
    prefix: &Path,
    project: &str,
    abs_path: &Path,
) -> Option<WorktreeSlotInfo> {
    let rel = abs_path.strip_prefix(prefix).ok()?;
    let mut comps = rel.components();
    match (comps.next(), comps.next()) {
        (Some(Component::Normal(name)), None) => {
            let name = name.to_string_lossy().into_owned();
            if name.is_empty() || name == "." || name == ".." {
                return None;
            }
            Some(WorktreeSlotInfo {
                name: name.clone(),
                path: rel_slot_path(project, &name),
            })
        }
        _ => None,
    }
}

fn paths_equal(a: &Path, b: &Path) -> bool {
    if a == b {
        return true;
    }
    // Compare component-wise after normalizing `.` only (no symlink resolve).
    let na: Vec<_> = a.components().filter(|c| *c != Component::CurDir).collect();
    let nb: Vec<_> = b.components().filter(|c| *c != Component::CurDir).collect();
    na == nb
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::ffi::{OsStr, OsString};
    use std::io;
    use std::process::ExitStatus;
    use std::sync::{Arc, Mutex};

    use odm_git::{CommandOutput, CommandRunner, Git};
    use tempfile::tempdir;

    use crate::config::{ProjectEntry, WorkspaceConfig};

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

    /// Scripted runner: queue of capture results; records argv.
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

    fn out_ok() -> CommandOutput {
        out_ok_stdout("")
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
        // failed status → is_repo returns false
        out_fail("not a repo")
    }

    fn args_as_strings(args: &[OsString]) -> Vec<String> {
        args.iter()
            .map(|a| a.to_string_lossy().into_owned())
            .collect()
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

    // --- validate_slot_name ---

    #[test]
    fn validate_slot_name_accepts_simple() {
        assert_eq!(validate_slot_name("slot1").unwrap(), "slot1");
        assert_eq!(validate_slot_name("  feat-x  ").unwrap(), "feat-x");
    }

    #[test]
    fn validate_slot_name_rejects_empty() {
        assert!(validate_slot_name("").is_err());
        assert!(validate_slot_name("   ").is_err());
    }

    #[test]
    fn validate_slot_name_rejects_dot_and_dotdot() {
        assert!(validate_slot_name(".").is_err());
        assert!(validate_slot_name("..").is_err());
    }

    #[test]
    fn validate_slot_name_rejects_separators_and_nul() {
        assert!(validate_slot_name("a/b").is_err());
        assert!(validate_slot_name("a\\b").is_err());
        assert!(validate_slot_name("a\0b").is_err());
    }

    #[test]
    fn validate_slot_name_rejects_absolute_looking() {
        assert!(validate_slot_name("/abs").is_err());
        assert!(validate_slot_name("C:").is_err());
        assert!(validate_slot_name("C:foo").is_err());
    }

    // --- resolve / unknown / non-git ---

    #[test]
    fn unknown_project_is_usage() {
        let dir = tempdir().unwrap();
        let ws = ws_with_project(dir.path().to_path_buf(), "alpha", "projects/alpha");
        let (runner, calls) = ScriptedRunner::new(vec![]);
        let g = Git::with_runner(runner);
        let err = worktree_list(&g, &ws, "missing").unwrap_err();
        assert!(matches!(err, OdmError::Usage(_)));
        assert!(err.to_string().contains("unknown project"));
        assert!(calls.lock().unwrap().is_empty());
    }

    #[test]
    fn non_git_primary_is_operation() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        ensure_primary(root, "projects/alpha");
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let (runner, _) = ScriptedRunner::new(vec![is_repo_false()]);
        let g = Git::with_runner(runner);
        let err = worktree_list(&g, &ws, "alpha").unwrap_err();
        assert!(matches!(err, OdmError::Operation(_)));
        assert!(err.to_string().contains("not a git repo"));
    }

    #[test]
    fn missing_primary_is_not_found() {
        let dir = tempdir().unwrap();
        let ws = ws_with_project(dir.path().to_path_buf(), "alpha", "projects/alpha");
        let (runner, calls) = ScriptedRunner::new(vec![]);
        let g = Git::with_runner(runner);
        let err = worktree_add(&g, &ws, "alpha", "s1", None).unwrap_err();
        assert!(matches!(err, OdmError::NotFound(_)));
        assert!(calls.lock().unwrap().is_empty());
    }

    // --- add ---

    #[test]
    fn add_refuses_existing_path_without_worktree_add() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        ensure_primary(root, "projects/alpha");
        let slot = worktree_slot_path(root, "alpha", "s1");
        fs::create_dir_all(&slot).unwrap();
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let (runner, calls) = ScriptedRunner::new(vec![is_repo_true()]);
        let g = Git::with_runner(runner);
        let err = worktree_add(&g, &ws, "alpha", "s1", None).unwrap_err();
        assert!(matches!(err, OdmError::Operation(_)));
        assert!(err.to_string().contains("already exists"));
        let calls = calls.lock().unwrap();
        assert_eq!(calls.len(), 1, "only is_repo");
        let a = args_as_strings(&calls[0]);
        assert!(a.iter().any(|s| s == "rev-parse"));
    }

    #[test]
    fn add_happy_path_argv() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let slot_path = worktree_slot_path(root, "alpha", "feat");
        let (runner, calls) = ScriptedRunner::new(vec![is_repo_true(), out_ok()]);
        let g = Git::with_runner(runner);
        let out = worktree_add(&g, &ws, "alpha", "feat", Some("topic")).unwrap();
        assert_eq!(out.project, "alpha");
        assert_eq!(out.slot, "feat");
        assert_eq!(out.path, "worktrees/alpha/feat");
        assert!(slot_path.parent().unwrap().is_dir());
        let calls = calls.lock().unwrap();
        assert_eq!(calls.len(), 2);
        let add_args = args_as_strings(&calls[1]);
        assert_eq!(
            add_args,
            vec![
                "-C".into(),
                primary.to_string_lossy().into_owned(),
                "worktree".into(),
                "add".into(),
                "-b".into(),
                "topic".into(),
                "--".into(),
                slot_path.to_string_lossy().into_owned(),
            ]
        );
    }

    #[test]
    fn add_rejects_invalid_slot_before_git() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        ensure_primary(root, "projects/alpha");
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let (runner, calls) = ScriptedRunner::new(vec![]);
        let g = Git::with_runner(runner);
        let err = worktree_add(&g, &ws, "alpha", "../x", None).unwrap_err();
        assert!(matches!(err, OdmError::Usage(_)));
        assert!(calls.lock().unwrap().is_empty());
    }

    // --- list ---

    #[test]
    fn list_filters_and_sorts_slots() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let s_b = worktree_slot_path(root, "alpha", "b-slot");
        let s_a = worktree_slot_path(root, "alpha", "a-slot");
        let other = root.join("worktrees/other/x");
        let porcelain = format!(
            "worktree {}\nHEAD abc\nbranch refs/heads/main\n\n\
             worktree {}\nHEAD def\nbranch refs/heads/b\n\n\
             worktree {}\nHEAD ghi\nbranch refs/heads/a\n\n\
             worktree {}\nHEAD jkl\nbranch refs/heads/o\n\n",
            primary.display(),
            s_b.display(),
            s_a.display(),
            other.display(),
        );
        let (runner, _) = ScriptedRunner::new(vec![is_repo_true(), out_ok_stdout(&porcelain)]);
        let g = Git::with_runner(runner);
        let out = worktree_list(&g, &ws, "alpha").unwrap();
        assert_eq!(out.project, "alpha");
        assert_eq!(out.slots.len(), 2);
        assert_eq!(out.slots[0].name, "a-slot");
        assert_eq!(out.slots[0].path, "worktrees/alpha/a-slot");
        assert_eq!(out.slots[1].name, "b-slot");
        assert_eq!(out.slots[1].path, "worktrees/alpha/b-slot");
    }

    #[test]
    fn list_empty_is_ok() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let porcelain = format!(
            "worktree {}\nHEAD abc\nbranch refs/heads/main\n\n",
            primary.display()
        );
        let (runner, _) = ScriptedRunner::new(vec![is_repo_true(), out_ok_stdout(&porcelain)]);
        let g = Git::with_runner(runner);
        let out = worktree_list(&g, &ws, "alpha").unwrap();
        assert!(out.slots.is_empty());
    }

    // --- rm ---

    #[test]
    fn rm_missing_slot_is_not_found() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        ensure_primary(root, "projects/alpha");
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let (runner, calls) = ScriptedRunner::new(vec![is_repo_true()]);
        let g = Git::with_runner(runner);
        let err = worktree_rm(&g, &ws, "alpha", "gone", false).unwrap_err();
        assert!(matches!(err, OdmError::NotFound(_)));
        let calls = calls.lock().unwrap();
        assert_eq!(calls.len(), 1, "only is_repo; no list/remove");
    }

    #[test]
    fn rm_force_passthrough_argv() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        let slot_path = worktree_slot_path(root, "alpha", "s1");
        fs::create_dir_all(&slot_path).unwrap();
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let porcelain = format!(
            "worktree {}\nHEAD abc\nbranch refs/heads/main\n\n\
             worktree {}\nHEAD def\nbranch refs/heads/s\n\n",
            primary.display(),
            slot_path.display(),
        );
        let (runner, calls) = ScriptedRunner::new(vec![
            is_repo_true(),
            out_ok_stdout(&porcelain),
            out_ok(),
        ]);
        let g = Git::with_runner(runner);
        let out = worktree_rm(&g, &ws, "alpha", "s1", true).unwrap();
        assert_eq!(out.path, "worktrees/alpha/s1");
        let calls = calls.lock().unwrap();
        assert_eq!(calls.len(), 3);
        let rm_args = args_as_strings(&calls[2]);
        assert_eq!(
            rm_args,
            vec![
                "-C".into(),
                primary.to_string_lossy().into_owned(),
                "worktree".into(),
                "remove".into(),
                "--force".into(),
                "--".into(),
                slot_path.to_string_lossy().into_owned(),
            ]
        );
    }
}

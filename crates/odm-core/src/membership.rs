//! Workspace membership — add/rm Project or Progen with kind-specific hooks.

use std::fs;
use std::path::Path;

use odm_git::Git;

use crate::checkout::{materialize, ManagedEntity, MaterializeOutcome};
use crate::config::{
    save_config, ProjectEntry, ProgenEntry, Workspace, WorkspaceConfig,
};
use crate::error::OdmError;
use crate::gitignore::apply_managed_gitignore;
use crate::paths::{abs_checkout, progen_index_dir, worktree_slot_path};
use crate::pin_maintain::{maintain_pins_after, prune_pin_file_if_present};
use crate::worktree::validate_slot_name;

/// Kind of Workspace membership entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MembershipKind {
    Project,
    Progen,
}

impl MembershipKind {
    fn label(self) -> &'static str {
        match self {
            MembershipKind::Project => "project",
            MembershipKind::Progen => "progen",
        }
    }
}

/// Entry payload for membership add.
#[derive(Debug, Clone)]
pub enum MembershipEntry {
    Project(ProjectEntry),
    Progen(ProgenEntry),
}

impl MembershipEntry {
    fn kind(&self) -> MembershipKind {
        match self {
            MembershipEntry::Project(_) => MembershipKind::Project,
            MembershipEntry::Progen(_) => MembershipKind::Progen,
        }
    }

    fn path(&self) -> &str {
        match self {
            MembershipEntry::Project(e) => &e.path,
            MembershipEntry::Progen(e) => &e.path,
        }
    }

    fn url(&self) -> Option<&str> {
        match self {
            MembershipEntry::Project(e) => e.url.as_deref(),
            MembershipEntry::Progen(e) => e.url.as_deref(),
        }
    }

    fn branch(&self) -> Option<&str> {
        match self {
            MembershipEntry::Project(e) => e.branch.as_deref(),
            MembershipEntry::Progen(e) => e.branch.as_deref(),
        }
    }
}

/// Add a Project or Progen entry; optional materialize; gitignore + pin maintain.
/// Does not scaffold Progen vaults — that composition lives in `odm-progen`.
pub fn membership_add<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &mut WorkspaceConfig,
    name: &str,
    entry: MembershipEntry,
    no_clone: bool,
) -> Result<Option<MaterializeOutcome>, OdmError> {
    let kind = entry.kind();
    let label = kind.label();
    if name.trim().is_empty() {
        return Err(OdmError::usage(format!("{label} name must not be empty")));
    }
    let already = match kind {
        MembershipKind::Project => config.projects.contains_key(name),
        MembershipKind::Progen => config.progens.contains_key(name),
    };
    if already {
        return Err(OdmError::usage(format!("{label} '{name}' already exists")));
    }
    if entry.path().trim().is_empty() {
        return Err(OdmError::usage(format!("{label} path must not be empty")));
    }
    if Path::new(entry.path()).is_absolute() {
        return Err(OdmError::usage(format!(
            "{label} path must be relative, got '{}'",
            entry.path()
        )));
    }

    let managed = entry.url().map(|url| ManagedEntity {
        name: name.to_string(),
        path: entry.path().to_string(),
        url: url.to_string(),
        branch: entry.branch().map(|s| s.to_string()),
    });

    match entry {
        MembershipEntry::Project(e) => {
            config.projects.insert(name.to_string(), e);
        }
        MembershipEntry::Progen(e) => {
            config.progens.insert(name.to_string(), e);
        }
    }
    save_config(root, config)?;

    if config.manage_gitignore() {
        apply_managed_gitignore(root, config)?;
    }

    let mut outcome = None;
    if let Some(entity) = &managed {
        if !no_clone {
            outcome = Some(materialize(git, root, entity)?);
            maintain_pins_after(git, root, config, &[entity])?;
        }
    }
    Ok(outcome)
}

/// Remove a Project or Progen from config; optional tree delete.
/// Progen hooks: drop ODM index dir and strip progen_groups members.
pub fn membership_rm<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &mut WorkspaceConfig,
    kind: MembershipKind,
    name: &str,
    delete: bool,
    force: bool,
) -> Result<(), OdmError> {
    match kind {
        MembershipKind::Project => {
            let entry = config.projects.remove(name).ok_or_else(|| {
                OdmError::usage(format!("unknown project '{name}'"))
            })?;
            if delete {
                maybe_delete_checkout(git, root, name, &entry.path, force, || {
                    config.projects.insert(name.to_string(), entry.clone());
                })?;
            }
        }
        MembershipKind::Progen => {
            let entry = config.progens.remove(name).ok_or_else(|| {
                OdmError::usage(format!("unknown progen '{name}'"))
            })?;
            if delete {
                maybe_delete_checkout(git, root, name, &entry.path, force, || {
                    config.progens.insert(name.to_string(), entry.clone());
                })?;
            }
        }
    }

    if kind == MembershipKind::Progen {
        let idx = progen_index_dir(root, name);
        if idx.exists() {
            let _ = remove_path(&idx);
        }
        for members in config.progen_groups.values_mut() {
            members.retain(|m| m != name);
        }
    }

    save_config(root, config)?;

    if config.manage_gitignore() {
        apply_managed_gitignore(root, config)?;
    }

    prune_pin_file_if_present(root, config)?;
    Ok(())
}

fn maybe_delete_checkout<R, F>(
    git: &Git<R>,
    root: &Path,
    name: &str,
    rel: &str,
    force: bool,
    restore: F,
) -> Result<(), OdmError>
where
    R: odm_git::CommandRunner,
    F: FnOnce(),
{
    let path = abs_checkout(root, rel)?;
    if path.exists() {
        if git.is_repo(&path)? && !force && !git.is_clean(&path)? {
            restore();
            return Err(OdmError::operation(format!(
                "working tree dirty for '{name}' (use --force with --delete)"
            )));
        }
        remove_path(&path)?;
    }
    Ok(())
}

fn remove_path(path: &Path) -> Result<(), OdmError> {
    if path.is_dir() {
        fs::remove_dir_all(path).map_err(|e| {
            OdmError::operation(format!("failed to delete {}: {e}", path.display()))
        })?;
    } else {
        fs::remove_file(path).map_err(|e| {
            OdmError::operation(format!("failed to delete {}: {e}", path.display()))
        })?;
    }
    Ok(())
}

/// Add a project entry; optional materialize; gitignore + pin maintain.
pub fn project_add<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &mut WorkspaceConfig,
    name: &str,
    entry: ProjectEntry,
    no_clone: bool,
) -> Result<Option<MaterializeOutcome>, OdmError> {
    membership_add(
        git,
        root,
        config,
        name,
        MembershipEntry::Project(entry),
        no_clone,
    )
}

/// Remove project from config; optional tree delete.
pub fn project_rm<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &mut WorkspaceConfig,
    name: &str,
    delete: bool,
    force: bool,
) -> Result<(), OdmError> {
    membership_rm(git, root, config, MembershipKind::Project, name, delete, force)
}

/// Add a progen entry; optional materialize; gitignore + pin maintain.
/// Vault scaffold is composed by `odm-progen`, not here.
pub fn progen_add<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &mut WorkspaceConfig,
    name: &str,
    entry: ProgenEntry,
    no_clone: bool,
) -> Result<Option<MaterializeOutcome>, OdmError> {
    membership_add(
        git,
        root,
        config,
        name,
        MembershipEntry::Progen(entry),
        no_clone,
    )
}

/// Remove progen from config; optional tree delete; drop ODM index + group members.
pub fn progen_rm<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &mut WorkspaceConfig,
    name: &str,
    delete: bool,
    force: bool,
) -> Result<(), OdmError> {
    membership_rm(git, root, config, MembershipKind::Progen, name, delete, force)
}

/// Run git passthrough in project checkout (or worktree slot when `wt` is set).
/// Auto-maintain pin if HEAD changed on Primary only (`wt` is `None`).
pub fn project_git<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
    name: &str,
    git_args: &[String],
    wt: Option<&str>,
) -> Result<std::process::ExitStatus, OdmError> {
    if git_args.is_empty() {
        return Err(OdmError::usage(
            "project git requires arguments after --",
        ));
    }
    let entry = ws.config.projects.get(name).ok_or_else(|| {
        OdmError::usage(format!("unknown project '{name}'"))
    })?;

    if let Some(slot) = wt {
        let slot = validate_slot_name(slot)?;
        let path = worktree_slot_path(&ws.root, name, &slot);
        let rel = format!("worktrees/{name}/{slot}");
        if !path.exists() {
            return Err(OdmError::not_found(format!(
                "worktree slot not found: {rel}"
            )));
        }
        if !git.is_repo(&path)? {
            return Err(OdmError::operation(format!(
                "worktree slot path is not a git repo: {rel}"
            )));
        }
        // Pin auto-maintain is Primary-only.
        return Ok(git.run(&path, git_args)?);
    }

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

    let before = git.head_sha(&path).ok();
    let status = git.run(&path, git_args)?;
    if status.success() {
        if let Some(url) = &entry.url {
            let after = git.head_sha(&path).ok();
            if after.is_some() && after != before {
                let entity = ManagedEntity {
                    name: name.to_string(),
                    path: entry.path.clone(),
                    url: url.clone(),
                    branch: entry.branch.clone(),
                };
                maintain_pins_after(git, &ws.root, &ws.config, &[&entity])?;
            }
        }
    }
    Ok(status)
}

/// Relative path string helper for CLI.
pub fn path_buf_to_rel(path: &Path) -> Result<String, OdmError> {
    let s = path.to_string_lossy();
    if path.is_absolute() {
        return Err(OdmError::usage(format!(
            "path must be relative, got '{s}'"
        )));
    }
    Ok(s.replace('\\', "/"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        save_config, ProjectEntry, ProgenEntry, Workspace, WorkspaceConfig,
    };
    use crate::init::{init_workspace, InitOptions};
    use crate::paths::{pin_path, progen_index_dir, worktree_slot_path};
    use crate::pin::{load_pin, PinEntry, PinFile};
    use std::collections::BTreeMap;
    use std::ffi::{OsStr, OsString};
    use std::io;
    use std::path::PathBuf;
    use std::process::{Command, ExitStatus};
    use std::sync::{Arc, Mutex};
    use tempfile::tempdir;

    use odm_git::{CommandOutput, CommandRunner};

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

    fn is_repo_true() -> CommandOutput {
        out_ok_stdout("true\n")
    }

    fn args_as_strings(args: &[OsString]) -> Vec<String> {
        args.iter()
            .map(|a| a.to_string_lossy().into_owned())
            .collect()
    }

    fn ws_with_project(root: PathBuf, name: &str, rel: &str, url: Option<&str>) -> Workspace {
        let mut projects = BTreeMap::new();
        projects.insert(
            name.into(),
            ProjectEntry {
                path: rel.into(),
                url: url.map(|s| s.into()),
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

    fn git_args(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| (*s).to_string()).collect()
    }

    fn git_user(repo: &Path) {
        Command::new("git")
            .args(["-C", repo.to_str().unwrap(), "config", "user.email", "t@est"])
            .status()
            .unwrap();
        Command::new("git")
            .args(["-C", repo.to_str().unwrap(), "config", "user.name", "t"])
            .status()
            .unwrap();
    }

    fn bare_fixture(root: &Path, name: &str) -> PathBuf {
        let bare = root.join(format!("{name}.git"));
        assert!(Command::new("git")
            .args(["init", "--bare", bare.to_str().unwrap()])
            .status()
            .unwrap()
            .success());
        let seed = root.join(format!("{name}-seed"));
        assert!(Command::new("git")
            .args(["clone", bare.to_str().unwrap(), seed.to_str().unwrap()])
            .status()
            .unwrap()
            .success());
        git_user(&seed);
        fs::write(seed.join("README"), name).unwrap();
        assert!(Command::new("git")
            .args(["-C", seed.to_str().unwrap(), "add", "README"])
            .status()
            .unwrap()
            .success());
        assert!(Command::new("git")
            .args(["-C", seed.to_str().unwrap(), "commit", "-m", "init"])
            .status()
            .unwrap()
            .success());
        assert!(Command::new("git")
            .args(["-C", seed.to_str().unwrap(), "branch", "-M", "main"])
            .status()
            .unwrap()
            .success());
        assert!(Command::new("git")
            .args(["-C", seed.to_str().unwrap(), "push", "-u", "origin", "main"])
            .status()
            .unwrap()
            .success());
        bare
    }

    #[test]
    fn project_add_rm_cycle() {
        let dir = tempdir().unwrap();
        let res = init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: false,
            name: None,
        })
        .unwrap();
        let root = res.root;
        let bare = bare_fixture(&root, "alpha");
        let g = Git::new();
        let mut cfg = WorkspaceConfig::default();
        project_add(
            &g,
            &root,
            &mut cfg,
            "alpha",
            ProjectEntry {
                path: "projects/alpha".into(),
                url: Some(bare.to_string_lossy().into()),
                branch: Some("main".into()),
                type_: None,
            },
            false,
        )
        .unwrap();
        assert!(
            g.is_repo(&root.join("projects/alpha")).unwrap()
        );
        assert!(load_pin(&root).unwrap().unwrap().pins.contains_key("alpha"));

        project_rm(&g, &root, &mut cfg, "alpha", true, false).unwrap();
        assert!(!cfg.projects.contains_key("alpha"));
        assert!(!root.join("projects/alpha").exists());
    }

    #[test]
    fn progen_rm_strips_group_and_index_dir() {
        let dir = tempdir().unwrap();
        let res = init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: true,
            name: None,
        })
        .unwrap();
        let root = res.root;
        let g = Git::new();
        let mut cfg = WorkspaceConfig::default();
        progen_add(
            &g,
            &root,
            &mut cfg,
            "desk",
            ProgenEntry {
                path: "vaults/desk".into(),
                url: None,
                branch: None,
            },
            false,
        )
        .unwrap();
        cfg.progen_groups
            .insert("all".into(), vec!["desk".into(), "other".into()]);
        save_config(&root, &cfg).unwrap();

        let idx = progen_index_dir(&root, "desk");
        fs::create_dir_all(&idx).unwrap();
        fs::write(idx.join("index.db"), b"x").unwrap();
        assert!(idx.exists());

        progen_rm(&g, &root, &mut cfg, "desk", false, false).unwrap();
        assert!(!cfg.progens.contains_key("desk"));
        assert_eq!(cfg.progen_groups.get("all").unwrap(), &vec!["other".to_string()]);
        assert!(!idx.exists());
    }

    #[test]
    fn path_buf_to_rel_rejects_absolute() {
        let err = path_buf_to_rel(Path::new("/abs")).unwrap_err();
        assert!(err.to_string().contains("relative"));
        assert_eq!(path_buf_to_rel(Path::new("vaults/desk")).unwrap(), "vaults/desk");
    }

    // --- project_git --wt ---

    #[test]
    fn project_git_wt_runs_in_slot_path() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let ws = ws_with_project(root.clone(), "alpha", "projects/alpha", None);
        let slot = worktree_slot_path(&root, "alpha", "feat");
        fs::create_dir_all(&slot).unwrap();

        let (runner, calls) = ScriptedRunner::new(vec![is_repo_true()]);
        let git = Git::with_runner(runner);
        let status = project_git(
            &git,
            &ws,
            "alpha",
            &git_args(&["status"]),
            Some("feat"),
        )
        .unwrap();
        assert!(status.success());

        let recorded = calls.lock().unwrap();
        // is_repo (output) then run (status)
        assert_eq!(recorded.len(), 2);
        let run_args = args_as_strings(&recorded[1]);
        assert_eq!(run_args[0], "-C");
        assert_eq!(PathBuf::from(&run_args[1]), slot);
        assert_eq!(run_args[2], "status");
    }

    #[test]
    fn project_git_wt_missing_slot_is_not_found() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let ws = ws_with_project(root, "alpha", "projects/alpha", None);
        let (runner, _) = ScriptedRunner::new(vec![]);
        let git = Git::with_runner(runner);
        let err = project_git(
            &git,
            &ws,
            "alpha",
            &git_args(&["status"]),
            Some("missing"),
        )
        .unwrap_err();
        assert!(matches!(err, OdmError::NotFound(_)));
        assert!(!err.to_string().contains("not implemented"));
        assert!(err.to_string().contains("worktree slot not found"));
    }

    #[test]
    fn project_git_wt_invalid_slot_name_is_usage() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let ws = ws_with_project(root, "alpha", "projects/alpha", None);
        let (runner, _) = ScriptedRunner::new(vec![]);
        let git = Git::with_runner(runner);
        let err = project_git(
            &git,
            &ws,
            "alpha",
            &git_args(&["status"]),
            Some("a/b"),
        )
        .unwrap_err();
        assert!(matches!(err, OdmError::Usage(_)));
    }

    #[test]
    fn project_git_wt_unknown_project_is_usage() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let ws = ws_with_project(root, "alpha", "projects/alpha", None);
        let (runner, _) = ScriptedRunner::new(vec![]);
        let git = Git::with_runner(runner);
        let err = project_git(
            &git,
            &ws,
            "nope",
            &git_args(&["status"]),
            Some("feat"),
        )
        .unwrap_err();
        assert!(matches!(err, OdmError::Usage(_)));
        assert!(err.to_string().contains("unknown project"));
    }

    #[test]
    fn project_git_none_wt_uses_primary_path() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let primary = root.join("projects/alpha");
        fs::create_dir_all(&primary).unwrap();
        let ws = ws_with_project(root, "alpha", "projects/alpha", None);

        let (runner, calls) = ScriptedRunner::new(vec![
            is_repo_true(),
            // head_sha before (ok → ignored if fail; provide valid-looking)
            out_ok_stdout("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n"),
            // head_sha after (same → no pin maintain)
            out_ok_stdout("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n"),
        ]);
        let git = Git::with_runner(runner);
        let status = project_git(&git, &ws, "alpha", &git_args(&["status"]), None).unwrap();
        assert!(status.success());

        let recorded = calls.lock().unwrap();
        // is_repo, head_sha before, run, head_sha after
        let run = recorded
            .iter()
            .map(|a| args_as_strings(a))
            .find(|a| a.get(2).map(|s| s.as_str()) == Some("status"))
            .expect("run status call");
        assert_eq!(run[0], "-C");
        assert_eq!(PathBuf::from(&run[1]), primary);
    }

    #[test]
    fn project_git_wt_does_not_update_pin() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let primary = root.join("projects/alpha");
        fs::create_dir_all(&primary).unwrap();
        let slot = worktree_slot_path(&root, "alpha", "feat");
        fs::create_dir_all(&slot).unwrap();

        let old_sha = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let mut pin = PinFile::new_v1();
        pin.pins.insert(
            "alpha".into(),
            PinEntry {
                rev: old_sha.into(),
                url: "https://example.com/alpha.git".into(),
                branch: Some("main".into()),
            },
        );
        // Ensure pin parent exists.
        fs::create_dir_all(root.join(".odm")).unwrap();
        crate::pin::save_pin(&root, &pin).unwrap();
        let before_bytes = fs::read(pin_path(&root)).unwrap();

        let ws = ws_with_project(
            root.clone(),
            "alpha",
            "projects/alpha",
            Some("https://example.com/alpha.git"),
        );
        let (runner, calls) = ScriptedRunner::new(vec![is_repo_true()]);
        let git = Git::with_runner(runner);
        project_git(
            &git,
            &ws,
            "alpha",
            &git_args(&["checkout", "other"]),
            Some("feat"),
        )
        .unwrap();

        let after_bytes = fs::read(pin_path(&root)).unwrap();
        assert_eq!(before_bytes, after_bytes, "pin file must not change with --wt");

        // Only is_repo + run — no head_sha / pin maintain git calls.
        let recorded = calls.lock().unwrap();
        assert_eq!(recorded.len(), 2);
        let run_args = args_as_strings(&recorded[1]);
        assert_eq!(PathBuf::from(&run_args[1]), slot);
    }
}

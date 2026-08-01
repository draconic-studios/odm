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
use crate::paths::{abs_checkout, progen_index_dir};
use crate::pin_maintain::{maintain_pins_after, prune_pin_file_if_present};

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

/// Run git passthrough in project checkout; auto-maintain pin if HEAD changed.
pub fn project_git<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
    name: &str,
    git_args: &[String],
) -> Result<std::process::ExitStatus, OdmError> {
    if git_args.is_empty() {
        return Err(OdmError::usage(
            "project git requires arguments after --",
        ));
    }
    let entry = ws.config.projects.get(name).ok_or_else(|| {
        OdmError::usage(format!("unknown project '{name}'"))
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
    use crate::config::{save_config, ProjectEntry, ProgenEntry, WorkspaceConfig};
    use crate::init::{init_workspace, InitOptions};
    use crate::paths::progen_index_dir;
    use crate::pin::load_pin;
    use std::path::PathBuf;
    use std::process::Command;
    use tempfile::tempdir;

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
}

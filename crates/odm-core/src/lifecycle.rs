use std::fs;
use std::path::{Path, PathBuf};

use odm_git::Git;

use crate::config::{save_config, ProjectEntry, Workspace, WorkspaceConfig};
use crate::error::OdmError;
use crate::gitignore::apply_managed_gitignore;
use crate::pin::{load_pin, prune_pins, save_pin, PinEntry, PinFile};
use crate::url_match::urls_match_with_root;

/// A managed (url-bearing) Project or Progen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedEntity {
    pub name: String,
    pub path: String,
    pub url: String,
    pub branch: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterializeOutcome {
    Cloned,
    AlreadyPresent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncResult {
    pub name: String,
    pub materialized: MaterializeOutcome,
    pub fetched: bool,
    pub head: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinStatusReport {
    pub pin_file: String,
    pub present: bool,
    pub entries: Vec<PinStatusEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinStatusEntry {
    pub name: String,
    pub pin_rev: Option<String>,
    pub head: Option<String>,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinApplyResult {
    pub name: String,
    pub status: String,
    pub rev: Option<String>,
}

/// Collect managed entities from config (projects + progens with url).
pub fn all_managed(config: &WorkspaceConfig) -> Vec<ManagedEntity> {
    let mut out = Vec::new();
    for (name, e) in &config.projects {
        if let Some(url) = &e.url {
            out.push(ManagedEntity {
                name: name.clone(),
                path: e.path.clone(),
                url: url.clone(),
                branch: e.branch.clone(),
            });
        }
    }
    for (name, e) in &config.progens {
        if let Some(url) = &e.url {
            out.push(ManagedEntity {
                name: name.clone(),
                path: e.path.clone(),
                url: url.clone(),
                branch: e.branch.clone(),
            });
        }
    }
    out
}

/// Resolve named entities; unknown name → Usage.
pub fn resolve_managed(
    config: &WorkspaceConfig,
    names: &[String],
) -> Result<Vec<ManagedEntity>, OdmError> {
    if names.is_empty() {
        return Ok(all_managed(config));
    }
    let mut out = Vec::new();
    for name in names {
        if let Some(e) = config.projects.get(name) {
            let url = e.url.as_ref().ok_or_else(|| {
                OdmError::usage(format!("project '{name}' is path-only (not managed)"))
            })?;
            out.push(ManagedEntity {
                name: name.clone(),
                path: e.path.clone(),
                url: url.clone(),
                branch: e.branch.clone(),
            });
            continue;
        }
        if let Some(e) = config.progens.get(name) {
            let url = e.url.as_ref().ok_or_else(|| {
                OdmError::usage(format!("progen '{name}' is path-only (not managed)"))
            })?;
            out.push(ManagedEntity {
                name: name.clone(),
                path: e.path.clone(),
                url: url.clone(),
                branch: e.branch.clone(),
            });
            continue;
        }
        return Err(OdmError::usage(format!("unknown entity '{name}'")));
    }
    Ok(out)
}

/// Sort managed entries by increasing path depth (parents before children).
pub fn sort_by_depth(entities: &mut [ManagedEntity]) {
    entities.sort_by(|a, b| {
        let da = path_depth(&a.path);
        let db = path_depth(&b.path);
        da.cmp(&db)
            .then_with(|| a.path.cmp(&b.path))
            .then_with(|| a.name.cmp(&b.name))
    });
}

fn path_depth(rel: &str) -> usize {
    Path::new(rel)
        .components()
        .filter(|c| !matches!(c, std::path::Component::CurDir))
        .count()
}

/// Absolute checkout path for a relative config path.
pub fn abs_checkout(root: &Path, rel: &str) -> PathBuf {
    root.join(rel)
}

/// Resolve config url for `git clone` (relative → absolute under workspace root).
pub fn resolve_clone_url(root: &Path, url: &str) -> String {
    let t = url.trim();
    if t.contains("://") {
        return t.to_string();
    }
    // SCP-like user@host:path
    if let Some(colon) = t.find(':') {
        let left = &t[..colon];
        if left.contains('@') && !Path::new(t).is_absolute() {
            return t.to_string();
        }
    }
    if Path::new(t).is_absolute() {
        return t.to_string();
    }
    root.join(t.trim_start_matches("./"))
        .to_string_lossy()
        .into_owned()
}

/// Ensure managed entry exists as a plain clone with matching origin.
pub fn materialize<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    entity: &ManagedEntity,
) -> Result<MaterializeOutcome, OdmError> {
    let path = abs_checkout(root, &entity.path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            OdmError::operation(format!(
                "failed to create parent for {}: {e}",
                path.display()
            ))
        })?;
    }

    if !path.exists() {
        let url = resolve_clone_url(root, &entity.url);
        git.clone(&url, &path, entity.branch.as_deref())?;
        return Ok(MaterializeOutcome::Cloned);
    }

    if path.is_file() {
        return Err(OdmError::operation(format!(
            "path exists and is not a directory: {}",
            path.display()
        )));
    }

    // directory
    if is_empty_dir(&path)? {
        let url = resolve_clone_url(root, &entity.url);
        git.clone(&url, &path, entity.branch.as_deref())?;
        return Ok(MaterializeOutcome::Cloned);
    }

    if !git.is_repo(&path)? {
        return Err(OdmError::operation(format!(
            "path exists and is not a git repository: {}",
            path.display()
        )));
    }

    let origin = match git.origin_url(&path) {
        Ok(u) => u,
        Err(odm_git::GitError::OriginMissing { .. }) => {
            return Err(OdmError::operation(format!(
                "origin mismatch for '{}': remote 'origin' is not configured at {}",
                entity.name,
                path.display()
            )));
        }
        Err(e) => return Err(e.into()),
    };

    if !urls_match_with_root(&entity.url, &origin, Some(root)) {
        return Err(OdmError::operation(format!(
            "origin mismatch for '{}': config url '{}' does not match origin '{}'",
            entity.name, entity.url, origin
        )));
    }

    Ok(MaterializeOutcome::AlreadyPresent)
}

fn is_empty_dir(path: &Path) -> Result<bool, OdmError> {
    let mut rd = fs::read_dir(path).map_err(|e| {
        OdmError::operation(format!("failed to read {}: {e}", path.display()))
    })?;
    Ok(rd.next().is_none())
}

/// Materialize if needed, then fetch. Depth-ordered, fail-fast.
/// Pin auto-maintain runs for successful entries when workspace is a git repo.
pub fn sync_managed<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &WorkspaceConfig,
    names: &[String],
) -> Result<Vec<SyncResult>, OdmError> {
    let mut entities = resolve_managed(config, names)?;
    sort_by_depth(&mut entities);

    let mut results = Vec::new();
    let mut succeeded: Vec<&ManagedEntity> = Vec::new();

    for entity in &entities {
        let outcome = materialize(git, root, entity)?;
        let path = abs_checkout(root, &entity.path);
        git.fetch(&path)?;
        let head = git.head_sha(&path).ok();
        results.push(SyncResult {
            name: entity.name.clone(),
            materialized: outcome,
            fetched: true,
            head: head.clone(),
        });
        if head.is_some() {
            succeeded.push(entity);
        }
    }

    maintain_pins_after(git, root, config, &succeeded)?;
    Ok(results)
}

/// Create/update pin file for managed entities that have a defined HEAD.
/// No-op when workspace root is not a git repo.
pub fn maintain_pins_after<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &WorkspaceConfig,
    entities: &[&ManagedEntity],
) -> Result<(), OdmError> {
    if !git.is_repo(root)? {
        return Ok(());
    }

    let managed = all_managed(config);
    let managed_names: Vec<&str> = managed.iter().map(|e| e.name.as_str()).collect();

    let mut pin = match load_pin(root)? {
        Some(p) => p,
        None => {
            if entities.is_empty() {
                return Ok(());
            }
            PinFile::new_v1()
        }
    };

    prune_pins(&mut pin, &managed_names);

    for entity in entities {
        let path = abs_checkout(root, &entity.path);
        if !path.exists() || !git.is_repo(&path)? {
            continue;
        }
        let rev = match git.head_sha(&path) {
            Ok(r) => r,
            Err(_) => continue,
        };
        pin.pins.insert(
            entity.name.clone(),
            PinEntry {
                rev,
                url: entity.url.clone(),
                branch: entity.branch.clone(),
            },
        );
    }

    if pin.pins.is_empty() && !pin_path_exists(root) {
        return Ok(());
    }

    save_pin(root, &pin)
}

fn pin_path_exists(root: &Path) -> bool {
    crate::config::pin_path(root).is_file()
}

/// Pin status for named entities (or all managed when empty).
pub fn pin_status<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &WorkspaceConfig,
    names: &[String],
) -> Result<PinStatusReport, OdmError> {
    let pin = load_pin(root)?;
    let present = pin.is_some();
    let pin_file = ".odm/odm.lock.yaml".to_string();

    let entities = if names.is_empty() {
        // all managed, plus any pin-only names already covered by managed
        all_managed(config)
    } else {
        resolve_status_names(config, names)?
    };

    let mut entries = Vec::new();
    for entity in entities {
        let path = abs_checkout(root, &entity.path);
        let pin_rev = pin
            .as_ref()
            .and_then(|p| p.pins.get(&entity.name))
            .map(|e| e.rev.clone());

        let (head, on_disk_git) = if !path.exists() {
            (None, false)
        } else if git.is_repo(&path)? {
            (git.head_sha(&path).ok(), true)
        } else {
            (None, false)
        };

        let state = if !present {
            "missing_pin_file".to_string()
        } else if !path.exists() {
            "missing_path".to_string()
        } else if pin_rev.is_none() {
            "unpinned".to_string()
        } else if !on_disk_git {
            "missing_path".to_string()
        } else if head.as_ref() == pin_rev.as_ref() {
            "in_sync".to_string()
        } else {
            "drift".to_string()
        };

        entries.push(PinStatusEntry {
            name: entity.name,
            pin_rev,
            head,
            state,
        });
    }

    Ok(PinStatusReport {
        pin_file,
        present,
        entries,
    })
}

fn resolve_status_names(
    config: &WorkspaceConfig,
    names: &[String],
) -> Result<Vec<ManagedEntity>, OdmError> {
    // For pin status, named entities must exist; path-only → usage
    resolve_managed(config, names)
}

/// Apply pins (detached HEAD). Dirty → fail unless force. Missing path → NotFound.
pub fn pin_apply<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &WorkspaceConfig,
    names: &[String],
    force: bool,
) -> Result<Vec<PinApplyResult>, OdmError> {
    let pin = load_pin(root)?.ok_or_else(|| {
        OdmError::not_found("pin file not found: .odm/odm.lock.yaml")
    })?;

    let targets: Vec<(String, PinEntry, String)> = if names.is_empty() {
        let mut v = Vec::new();
        for (name, entry) in &pin.pins {
            let path = find_managed_path(config, name).ok_or_else(|| {
                OdmError::usage(format!(
                    "pin '{name}' has no managed config entry"
                ))
            })?;
            v.push((name.clone(), entry.clone(), path));
        }
        v
    } else {
        let mut v = Vec::new();
        for name in names {
            let entry = pin.pins.get(name).ok_or_else(|| {
                OdmError::not_found(format!("no pin for '{name}'"))
            })?;
            let path = find_managed_path(config, name).ok_or_else(|| {
                OdmError::usage(format!("unknown or unmanaged entity '{name}'"))
            })?;
            v.push((name.clone(), entry.clone(), path));
        }
        v
    };

    let mut results = Vec::new();
    for (name, entry, rel) in targets {
        let path = abs_checkout(root, &rel);
        if !path.exists() {
            return Err(OdmError::not_found(format!(
                "path missing for '{name}': {rel}"
            )));
        }
        if !git.is_repo(&path)? {
            return Err(OdmError::not_found(format!(
                "path is not a git repo for '{name}': {rel}"
            )));
        }
        if !force && !git.is_clean(&path)? {
            return Err(OdmError::operation(format!(
                "working tree dirty for '{name}' (use --force)"
            )));
        }
        git.checkout_detached(&path, &entry.rev)?;
        results.push(PinApplyResult {
            name,
            status: "applied".into(),
            rev: Some(entry.rev.clone()),
        });
    }
    Ok(results)
}

fn find_managed_path(config: &WorkspaceConfig, name: &str) -> Option<String> {
    if let Some(e) = config.projects.get(name) {
        if e.url.is_some() {
            return Some(e.path.clone());
        }
    }
    if let Some(e) = config.progens.get(name) {
        if e.url.is_some() {
            return Some(e.path.clone());
        }
    }
    None
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
    if name.trim().is_empty() {
        return Err(OdmError::usage("project name must not be empty"));
    }
    if config.projects.contains_key(name) {
        return Err(OdmError::usage(format!("project '{name}' already exists")));
    }
    if entry.path.trim().is_empty() {
        return Err(OdmError::usage("project path must not be empty"));
    }
    if Path::new(&entry.path).is_absolute() {
        return Err(OdmError::usage(format!(
            "project path must be relative, got '{}'",
            entry.path
        )));
    }

    let managed = entry.url.as_ref().map(|url| ManagedEntity {
        name: name.to_string(),
        path: entry.path.clone(),
        url: url.clone(),
        branch: entry.branch.clone(),
    });

    config.projects.insert(name.to_string(), entry);
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

/// Remove project from config; optional tree delete.
pub fn project_rm<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &mut WorkspaceConfig,
    name: &str,
    delete: bool,
    force: bool,
) -> Result<(), OdmError> {
    let entry = config.projects.remove(name).ok_or_else(|| {
        OdmError::usage(format!("unknown project '{name}'"))
    })?;

    if delete {
        let path = abs_checkout(root, &entry.path);
        if path.exists() {
            if git.is_repo(&path)? && !force && !git.is_clean(&path)? {
                config.projects.insert(name.to_string(), entry);
                return Err(OdmError::operation(format!(
                    "working tree dirty for '{name}' (use --force with --delete)"
                )));
            }
            remove_path(&path)?;
        }
    }

    save_config(root, config)?;

    if config.manage_gitignore() {
        apply_managed_gitignore(root, config)?;
    }

    // prune pin if present
    if let Some(mut pin) = load_pin(root)? {
        let managed = all_managed(config);
        let names: Vec<&str> = managed.iter().map(|e| e.name.as_str()).collect();
        prune_pins(&mut pin, &names);
        save_pin(root, &pin)?;
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
    let path = abs_checkout(&ws.root, &entry.path);
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

/// Disk/git summary helpers for project info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityDiskInfo {
    pub on_disk: bool,
    pub is_git: bool,
    pub head: Option<String>,
    pub origin: Option<String>,
    pub dirty: Option<bool>,
    pub pin_rev: Option<String>,
    pub pin_state: String,
}

pub fn entity_disk_info<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    name: &str,
    rel_path: &str,
    managed_url: Option<&str>,
) -> Result<EntityDiskInfo, OdmError> {
    let path = abs_checkout(root, rel_path);
    let pin = load_pin(root)?;
    let pin_rev = pin
        .as_ref()
        .and_then(|p| p.pins.get(name))
        .map(|e| e.rev.clone());

    if !path.exists() {
        let pin_state = if pin.is_none() && managed_url.is_some() {
            "missing_pin_file"
        } else if pin_rev.is_some() {
            "missing_path"
        } else if managed_url.is_none() {
            "none"
        } else {
            "missing_path"
        };
        return Ok(EntityDiskInfo {
            on_disk: false,
            is_git: false,
            head: None,
            origin: None,
            dirty: None,
            pin_rev,
            pin_state: pin_state.into(),
        });
    }

    let is_git = git.is_repo(&path)?;
    let (head, origin, dirty) = if is_git {
        (
            git.head_sha(&path).ok(),
            git.origin_url(&path).ok(),
            git.is_clean(&path).ok().map(|c| !c),
        )
    } else {
        (None, None, None)
    };

    let pin_state = if managed_url.is_none() {
        "none".to_string()
    } else if pin.is_none() {
        "missing_pin_file".to_string()
    } else if pin_rev.is_none() {
        "unpinned".to_string()
    } else if !is_git {
        "missing_path".to_string()
    } else if head.as_ref() == pin_rev.as_ref() {
        "in_sync".to_string()
    } else {
        "drift".to_string()
    };

    Ok(EntityDiskInfo {
        on_disk: true,
        is_git,
        head,
        origin,
        dirty,
        pin_rev,
        pin_state,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{save_config, ProjectEntry, WorkspaceConfig};
    use crate::init::{init_workspace, InitOptions};
    use crate::pin::load_pin;
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
    fn depth_order_shallow_first() {
        let mut ents = vec![
            ManagedEntity {
                name: "deep".into(),
                path: "a/b/c".into(),
                url: "u".into(),
                branch: None,
            },
            ManagedEntity {
                name: "rootish".into(),
                path: "a".into(),
                url: "u".into(),
                branch: None,
            },
            ManagedEntity {
                name: "mid".into(),
                path: "a/b".into(),
                url: "u".into(),
                branch: None,
            },
        ];
        sort_by_depth(&mut ents);
        assert_eq!(
            ents.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(),
            vec!["rootish", "mid", "deep"]
        );
    }

    #[test]
    fn materialize_clone_and_already_present() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let bare = bare_fixture(root, "alpha");
        let g = Git::new();
        let entity = ManagedEntity {
            name: "alpha".into(),
            path: "projects/alpha".into(),
            url: bare.to_string_lossy().into(),
            branch: Some("main".into()),
        };
        assert_eq!(
            materialize(&g, root, &entity).unwrap(),
            MaterializeOutcome::Cloned
        );
        assert_eq!(
            materialize(&g, root, &entity).unwrap(),
            MaterializeOutcome::AlreadyPresent
        );
    }

    #[test]
    fn materialize_origin_mismatch() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let bare_a = bare_fixture(root, "a");
        let bare_b = bare_fixture(root, "b");
        let g = Git::new();
        let path = root.join("proj");
        g.clone(bare_a.to_str().unwrap(), &path, Some("main"))
            .unwrap();
        let entity = ManagedEntity {
            name: "x".into(),
            path: "proj".into(),
            url: bare_b.to_string_lossy().into(),
            branch: None,
        };
        let err = materialize(&g, root, &entity).unwrap_err();
        assert!(err.to_string().contains("origin mismatch"));
        assert!(matches!(err, OdmError::Operation(_)));
    }

    #[test]
    fn materialize_not_git_fails() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let p = root.join("proj");
        fs::create_dir_all(&p).unwrap();
        fs::write(p.join("file"), "x").unwrap();
        let g = Git::new();
        let entity = ManagedEntity {
            name: "x".into(),
            path: "proj".into(),
            url: "https://example.com/x.git".into(),
            branch: None,
        };
        let err = materialize(&g, root, &entity).unwrap_err();
        assert!(err.to_string().contains("not a git repository"));
    }

    #[test]
    fn pin_maintain_on_sync_when_ws_git() {
        let dir = tempdir().unwrap();
        let res = init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: false,
            name: Some("t".into()),
        })
        .unwrap();
        let root = res.root;
        let bare = bare_fixture(&root, "alpha");
        let mut cfg = WorkspaceConfig::default();
        cfg.projects.insert(
            "alpha".into(),
            ProjectEntry {
                path: "projects/alpha".into(),
                url: Some(bare.to_string_lossy().into()),
                branch: Some("main".into()),
                type_: None,
            },
        );
        save_config(&root, &cfg).unwrap();
        let g = Git::new();
        sync_managed(&g, &root, &cfg, &[]).unwrap();
        let pin = load_pin(&root).unwrap().expect("pin file");
        assert!(pin.pins.contains_key("alpha"));
        assert_eq!(pin.pins["alpha"].rev.len(), 40);
    }

    #[test]
    fn pin_maintain_skipped_when_ws_not_git() {
        let dir = tempdir().unwrap();
        let res = init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: true,
            name: None,
        })
        .unwrap();
        let root = res.root;
        let bare = bare_fixture(&root, "alpha");
        let mut cfg = WorkspaceConfig::default();
        cfg.projects.insert(
            "alpha".into(),
            ProjectEntry {
                path: "projects/alpha".into(),
                url: Some(bare.to_string_lossy().into()),
                branch: Some("main".into()),
                type_: None,
            },
        );
        save_config(&root, &cfg).unwrap();
        let g = Git::new();
        sync_managed(&g, &root, &cfg, &[]).unwrap();
        assert!(load_pin(&root).unwrap().is_none());
    }

    #[test]
    fn pin_apply_and_status() {
        let dir = tempdir().unwrap();
        let res = init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: false,
            name: None,
        })
        .unwrap();
        let root = res.root;
        let bare = bare_fixture(&root, "alpha");
        let mut cfg = WorkspaceConfig::default();
        cfg.projects.insert(
            "alpha".into(),
            ProjectEntry {
                path: "projects/alpha".into(),
                url: Some(bare.to_string_lossy().into()),
                branch: Some("main".into()),
                type_: None,
            },
        );
        save_config(&root, &cfg).unwrap();
        let g = Git::new();
        sync_managed(&g, &root, &cfg, &[]).unwrap();
        let pin = load_pin(&root).unwrap().unwrap();
        let rev = pin.pins["alpha"].rev.clone();

        let st = pin_status(&g, &root, &cfg, &[]).unwrap();
        assert!(st.present);
        assert_eq!(st.entries[0].state, "in_sync");

        let applied = pin_apply(&g, &root, &cfg, &[], false).unwrap();
        assert_eq!(applied[0].status, "applied");
        assert_eq!(applied[0].rev.as_deref(), Some(rev.as_str()));
    }

    #[test]
    fn pin_apply_dirty_fails_without_force() {
        let dir = tempdir().unwrap();
        let res = init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: false,
            name: None,
        })
        .unwrap();
        let root = res.root;
        let bare = bare_fixture(&root, "alpha");
        let mut cfg = WorkspaceConfig::default();
        cfg.projects.insert(
            "alpha".into(),
            ProjectEntry {
                path: "projects/alpha".into(),
                url: Some(bare.to_string_lossy().into()),
                branch: Some("main".into()),
                type_: None,
            },
        );
        save_config(&root, &cfg).unwrap();
        let g = Git::new();
        sync_managed(&g, &root, &cfg, &[]).unwrap();
        fs::write(root.join("projects/alpha/dirty"), "x").unwrap();
        let err = pin_apply(&g, &root, &cfg, &[], false).unwrap_err();
        assert!(err.to_string().contains("dirty"));
        pin_apply(&g, &root, &cfg, &[], true).unwrap();
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
        assert!(root.join("projects/alpha").join(".git").exists() || root.join("projects/alpha").join(".git").is_file() || g.is_repo(&root.join("projects/alpha")).unwrap());
        assert!(load_pin(&root).unwrap().unwrap().pins.contains_key("alpha"));

        project_rm(&g, &root, &mut cfg, "alpha", true, false).unwrap();
        assert!(!cfg.projects.contains_key("alpha"));
        assert!(!root.join("projects/alpha").exists());
    }
}

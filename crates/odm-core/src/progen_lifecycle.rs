//! Progen lifecycle (list is config-only; add/rm mirror project).

use std::path::Path;

use odm_git::Git;

use crate::config::{save_config, ProgenEntry, WorkspaceConfig};
use crate::error::OdmError;
use crate::gitignore::apply_managed_gitignore;
use crate::lifecycle::{
    abs_checkout, all_managed, materialize, maintain_pins_after, ManagedEntity, MaterializeOutcome,
};
use crate::pin::{load_pin, prune_pins, save_pin};

/// Add a progen entry; optional materialize; scaffold vault when path-only / after clone.
pub fn progen_add<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &mut WorkspaceConfig,
    name: &str,
    entry: ProgenEntry,
    no_clone: bool,
    ensure_vault: impl FnOnce(&Path) -> Result<(), OdmError>,
) -> Result<Option<MaterializeOutcome>, OdmError> {
    if name.trim().is_empty() {
        return Err(OdmError::usage("progen name must not be empty"));
    }
    if config.progens.contains_key(name) {
        return Err(OdmError::usage(format!("progen '{name}' already exists")));
    }
    if entry.path.trim().is_empty() {
        return Err(OdmError::usage("progen path must not be empty"));
    }
    if Path::new(&entry.path).is_absolute() {
        return Err(OdmError::usage(format!(
            "progen path must be relative, got '{}'",
            entry.path
        )));
    }

    let managed = entry.url.as_ref().map(|url| ManagedEntity {
        name: name.to_string(),
        path: entry.path.clone(),
        url: url.clone(),
        branch: entry.branch.clone(),
    });

    let rel = entry.path.clone();
    config.progens.insert(name.to_string(), entry);
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

    let abs = abs_checkout(root, &rel);
    if managed.is_none() || outcome.is_some() {
        // Path-only always scaffold; managed after successful materialize.
        if managed.is_none() || abs.exists() {
            ensure_vault(&abs)?;
        }
    }

    Ok(outcome)
}

/// Remove progen from config; optional tree delete; drop ODM index dir.
pub fn progen_rm<R: odm_git::CommandRunner>(
    git: &Git<R>,
    root: &Path,
    config: &mut WorkspaceConfig,
    name: &str,
    delete: bool,
    force: bool,
) -> Result<(), OdmError> {
    let entry = config.progens.remove(name).ok_or_else(|| {
        OdmError::usage(format!("unknown progen '{name}'"))
    })?;

    if delete {
        let path = abs_checkout(root, &entry.path);
        if path.exists() {
            if git.is_repo(&path)? && !force && !git.is_clean(&path)? {
                config.progens.insert(name.to_string(), entry);
                return Err(OdmError::operation(format!(
                    "working tree dirty for '{name}' (use --force with --delete)"
                )));
            }
            remove_path(&path)?;
        }
    }

    // Drop ODM-side index cache
    let idx = crate::config::odm_dir(root).join("progen").join(name);
    if idx.exists() {
        let _ = remove_path(&idx);
    }

    // Drop from progen_groups membership
    for members in config.progen_groups.values_mut() {
        members.retain(|m| m != name);
    }

    save_config(root, config)?;

    if config.manage_gitignore() {
        apply_managed_gitignore(root, config)?;
    }

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
        std::fs::remove_dir_all(path).map_err(|e| {
            OdmError::operation(format!("failed to delete {}: {e}", path.display()))
        })?;
    } else {
        std::fs::remove_file(path).map_err(|e| {
            OdmError::operation(format!("failed to delete {}: {e}", path.display()))
        })?;
    }
    Ok(())
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

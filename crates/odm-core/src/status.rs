use std::path::Path;

use odm_git::Git;
use serde::Serialize;

use crate::config::Workspace;
use crate::error::OdmError;
use crate::gitignore::resolve_under_root;
use crate::pin::{load_pin, PinFile};

/// `odm status --json` snapshot.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct StatusSnapshot {
    pub root: String,
    pub projects: Vec<EntityStatus>,
    pub progens: Vec<EntityStatus>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EntityStatus {
    pub name: String,
    pub path: String,
    pub url: Option<String>,
    pub managed: bool,
    pub on_disk: bool,
    pub is_git: bool,
    pub head: Option<String>,
    pub pin_rev: Option<String>,
    pub pin_state: PinState,
    pub dirty: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PinState {
    None,
    MissingPath,
    Unpinned,
    InSync,
    Drift,
    MissingPinFile,
}

/// Build Workspace status snapshot. Does not fetch.
pub fn build_status(ws: &Workspace) -> Result<StatusSnapshot, OdmError> {
    let git = Git::new();
    let pin = load_pin(&ws.root)?;
    let pin_present = pin.is_some();

    let mut projects = Vec::new();
    for (name, entry) in &ws.config.projects {
        projects.push(entity_status(
            &ws.root,
            name,
            &entry.path,
            entry.url.as_deref(),
            entry.is_managed(),
            pin.as_ref(),
            pin_present,
            &git,
        )?);
    }

    let mut progens = Vec::new();
    for (name, entry) in &ws.config.progens {
        progens.push(entity_status(
            &ws.root,
            name,
            &entry.path,
            entry.url.as_deref(),
            entry.is_managed(),
            pin.as_ref(),
            pin_present,
            &git,
        )?);
    }

    Ok(StatusSnapshot {
        root: ws.root.display().to_string(),
        projects,
        progens,
    })
}

fn entity_status(
    root: &Path,
    name: &str,
    rel_path: &str,
    url: Option<&str>,
    managed: bool,
    pin: Option<&PinFile>,
    pin_present: bool,
    git: &Git,
) -> Result<EntityStatus, OdmError> {
    let abs = match resolve_under_root(root, rel_path) {
        Ok(p) => p,
        Err(_) => {
            return Ok(EntityStatus {
                name: name.into(),
                path: rel_path.into(),
                url: url.map(str::to_string),
                managed,
                on_disk: false,
                is_git: false,
                head: None,
                pin_rev: pin.and_then(|p| p.pins.get(name).map(|e| e.rev.clone())),
                pin_state: if !managed {
                    PinState::None
                } else if !pin_present {
                    PinState::MissingPinFile
                } else {
                    PinState::MissingPath
                },
                dirty: None,
            });
        }
    };

    let on_disk = abs.exists();
    let is_git = if on_disk {
        git.is_repo(&abs).unwrap_or(false)
    } else {
        false
    };

    let head = if is_git {
        git.head_sha(&abs).ok()
    } else {
        None
    };

    let dirty = if is_git {
        git.is_clean(&abs).ok().map(|clean| !clean)
    } else {
        None
    };

    let pin_entry = pin.and_then(|p| p.pins.get(name));
    let pin_rev = pin_entry.map(|e| e.rev.clone());

    let pin_state = compute_pin_state(
        managed,
        pin_present,
        on_disk,
        pin_entry.map(|e| e.rev.as_str()),
        head.as_deref(),
    );

    Ok(EntityStatus {
        name: name.into(),
        path: rel_path.into(),
        url: url.map(str::to_string),
        managed,
        on_disk,
        is_git,
        head,
        pin_rev,
        pin_state,
        dirty,
    })
}

pub fn compute_pin_state(
    managed: bool,
    pin_present: bool,
    on_disk: bool,
    pin_rev: Option<&str>,
    head: Option<&str>,
) -> PinState {
    if !managed {
        return PinState::None;
    }
    if !pin_present {
        return PinState::MissingPinFile;
    }
    if pin_rev.is_none() {
        return PinState::Unpinned;
    }
    if !on_disk {
        return PinState::MissingPath;
    }
    match (pin_rev, head) {
        (Some(p), Some(h)) if p == h => PinState::InSync,
        (Some(_), Some(_)) => PinState::Drift,
        (Some(_), None) => PinState::Drift,
        _ => PinState::Unpinned,
    }
}

/// Human-readable multi-line summary.
pub fn format_status_human(snap: &StatusSnapshot) -> String {
    let mut out = String::new();
    out.push_str(&format!("Workspace: {}\n", snap.root));
    if snap.projects.is_empty() && snap.progens.is_empty() {
        out.push_str("(no projects or progens)\n");
        return out;
    }
    if !snap.projects.is_empty() {
        out.push_str("\nProjects:\n");
        for e in &snap.projects {
            out.push_str(&format_entity_line(e));
        }
    }
    if !snap.progens.is_empty() {
        out.push_str("\nProgens:\n");
        for e in &snap.progens {
            out.push_str(&format_entity_line(e));
        }
    }
    out
}

fn format_entity_line(e: &EntityStatus) -> String {
    let kind = if e.managed { "managed" } else { "path" };
    let disk = if e.on_disk {
        if e.is_git {
            "git"
        } else {
            "disk"
        }
    } else {
        "missing"
    };
    let pin: &str = match e.pin_state {
        PinState::None => "-",
        PinState::MissingPath => "missing_path",
        PinState::Unpinned => "unpinned",
        PinState::InSync => "in_sync",
        PinState::Drift => "drift",
        PinState::MissingPinFile => "missing_pin_file",
    };
    let dirty = match e.dirty {
        Some(true) => " dirty",
        Some(false) => " clean",
        None => "",
    };
    format!(
        "  {}\t{}\t{}\t{}\tpin={}{}\n",
        e.name, e.path, kind, disk, pin, dirty
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pin_state_matrix() {
        assert_eq!(
            compute_pin_state(false, false, true, None, None),
            PinState::None
        );
        assert_eq!(
            compute_pin_state(true, false, true, None, Some("a")),
            PinState::MissingPinFile
        );
        assert_eq!(
            compute_pin_state(true, true, true, None, Some("a")),
            PinState::Unpinned
        );
        assert_eq!(
            compute_pin_state(true, true, false, Some("a"), None),
            PinState::MissingPath
        );
        let sha = "a".repeat(40);
        assert_eq!(
            compute_pin_state(true, true, true, Some(&sha), Some(&sha)),
            PinState::InSync
        );
        assert_eq!(
            compute_pin_state(true, true, true, Some(&sha), Some("b")),
            PinState::Drift
        );
    }
}

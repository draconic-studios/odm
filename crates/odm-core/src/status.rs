use odm_git::Git;
use serde::Serialize;

use crate::config::Workspace;
use crate::error::OdmError;
use crate::pin::load_pin;

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

impl PinState {
    /// Locked snake_case label for JSON `pin_state` / pin status `state`.
    pub fn as_str(self) -> &'static str {
        match self {
            PinState::None => "none",
            PinState::MissingPath => "missing_path",
            PinState::Unpinned => "unpinned",
            PinState::InSync => "in_sync",
            PinState::Drift => "drift",
            PinState::MissingPinFile => "missing_pin_file",
        }
    }
}

/// Build Workspace status snapshot. Does not fetch.
pub fn build_status<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
) -> Result<StatusSnapshot, OdmError> {
    let pin = load_pin(&ws.root)?;
    let obs = crate::observation::observe_workspace(git, &ws.root, &ws.config, pin.as_ref())?;
    Ok(status_from_observation(&obs))
}

/// Pure projection: observation → status snapshot.
pub fn status_from_observation(obs: &crate::observation::WorkspaceObservation) -> StatusSnapshot {
    StatusSnapshot {
        root: obs.root.clone(),
        projects: obs.projects.iter().map(entity_status_from_obs).collect(),
        progens: obs.progens.iter().map(entity_status_from_obs).collect(),
    }
}

fn entity_status_from_obs(e: &crate::observation::EntityObservation) -> EntityStatus {
    EntityStatus {
        name: e.name.clone(),
        path: e.path.clone(),
        url: e.url.clone(),
        managed: e.managed,
        on_disk: e.on_disk,
        is_git: e.is_git,
        head: e.head.clone(),
        pin_rev: e.pin_rev.clone(),
        pin_state: e.pin_state,
        dirty: e.dirty,
    }
}

/// Single source of truth for pin drift labels.
///
/// Order: `!managed` → None; `!pin_present` → MissingPinFile; `pin_rev` none → Unpinned;
/// `!on_disk` → MissingPath; head==pin_rev → InSync; else Drift.
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
        (Some(_), _) => PinState::Drift,
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
    let pin = match e.pin_state {
        PinState::None => "-",
        other => other.as_str(),
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
        // unmanaged
        assert_eq!(
            compute_pin_state(false, false, true, None, None),
            PinState::None
        );
        assert_eq!(
            compute_pin_state(false, true, false, Some("a"), None),
            PinState::None
        );
        // missing pin file
        assert_eq!(
            compute_pin_state(true, false, true, None, Some("a")),
            PinState::MissingPinFile
        );
        assert_eq!(
            compute_pin_state(true, false, false, None, None),
            PinState::MissingPinFile
        );
        // unpinned (pin present, no entry) — including path missing
        assert_eq!(
            compute_pin_state(true, true, true, None, Some("a")),
            PinState::Unpinned
        );
        assert_eq!(
            compute_pin_state(true, true, false, None, None),
            PinState::Unpinned
        );
        // missing path (pin entry, not on disk)
        assert_eq!(
            compute_pin_state(true, true, false, Some("a"), None),
            PinState::MissingPath
        );
        let sha = "a".repeat(40);
        // in_sync
        assert_eq!(
            compute_pin_state(true, true, true, Some(&sha), Some(&sha)),
            PinState::InSync
        );
        // drift: head differs
        assert_eq!(
            compute_pin_state(true, true, true, Some(&sha), Some("b")),
            PinState::Drift
        );
        // drift: on disk but not git / no head (former lifecycle "missing_path")
        assert_eq!(
            compute_pin_state(true, true, true, Some(&sha), None),
            PinState::Drift
        );
        assert_eq!(PinState::InSync.as_str(), "in_sync");
        assert_eq!(PinState::MissingPinFile.as_str(), "missing_pin_file");
    }
}

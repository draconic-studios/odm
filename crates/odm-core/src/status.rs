use odm_git::Git;
use serde::Serialize;

use crate::agent_pack::{pack_list, PackMode};
use crate::config::Workspace;
use crate::error::OdmError;
use crate::pin::load_pin;
use crate::worktree::{worktree_list, WorktreeSlotInfo};

/// `odm status --json` snapshot.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct StatusSnapshot {
    pub root: String,
    pub projects: Vec<EntityStatus>,
    pub progens: Vec<EntityStatus>,
    /// Registered agent packs from `.odm/agent-packs.json` (empty when absent/unloadable).
    pub agent_packs: Vec<StatusPackInfo>,
}

/// One registered agent pack row on status.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct StatusPackInfo {
    pub name: String,
    pub source: String,
    pub path: String,
    pub mode: PackMode,
    /// `true` when destination has no path/symlink entry (same rule as doctor `pack_missing`).
    pub missing: bool,
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
    /// Registered worktree slots for Projects only (`None` on Progens → omitted from JSON).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worktree_slots: Option<Vec<WorktreeSlotInfo>>,
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
    let mut snap = status_from_observation(&obs);
    for p in &mut snap.projects {
        p.worktree_slots = Some(match worktree_list(git, ws, &p.name) {
            Ok(out) => out.slots,
            Err(_) => vec![],
        });
    }
    snap.agent_packs = match pack_list(ws) {
        Ok(entries) => entries
            .into_iter()
            .map(|e| StatusPackInfo {
                name: e.name,
                source: e.source,
                path: e.path.display().to_string(),
                mode: e.mode,
                missing: e.path.symlink_metadata().is_err(),
            })
            .collect(),
        Err(_) => vec![],
    };
    Ok(snap)
}

/// Pure projection: observation → status snapshot.
///
/// `worktree_slots` stays `None` and `agent_packs` empty here; [`build_status`] fills both.
pub fn status_from_observation(obs: &crate::observation::WorkspaceObservation) -> StatusSnapshot {
    StatusSnapshot {
        root: obs.root.clone(),
        projects: obs.projects.iter().map(entity_status_from_obs).collect(),
        progens: obs.progens.iter().map(entity_status_from_obs).collect(),
        agent_packs: vec![],
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
        worktree_slots: None,
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
    if snap.projects.is_empty() && snap.progens.is_empty() && snap.agent_packs.is_empty() {
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
    if !snap.agent_packs.is_empty() {
        out.push_str("\nAgent packs:\n");
        for p in &snap.agent_packs {
            out.push_str(&format_pack_line(p));
        }
    }
    out
}

fn format_pack_line(p: &StatusPackInfo) -> String {
    let mode = match p.mode {
        PackMode::Install => "install",
        PackMode::Link => "link",
    };
    let missing = if p.missing { " missing" } else { "" };
    format!("  {}\t{}{}\n", p.name, mode, missing)
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
    let mut line = format!(
        "  {}\t{}\t{}\t{}\tpin={}{}\n",
        e.name, e.path, kind, disk, pin, dirty
    );
    if let Some(slots) = &e.worktree_slots {
        if !slots.is_empty() {
            let names: Vec<String> = slots
                .iter()
                .map(|s| {
                    if s.dirty == Some(true) {
                        format!("{} dirty", s.name)
                    } else {
                        s.name.clone()
                    }
                })
                .collect();
            line.push_str(&format!("    worktrees: {}\n", names.join(", ")));
        }
    }
    line
}

#[cfg(test)]
#[path = "status_tests.rs"]
mod tests;

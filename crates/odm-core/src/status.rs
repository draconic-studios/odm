use odm_git::Git;
use serde::Serialize;

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
    Ok(snap)
}

/// Pure projection: observation → status snapshot.
///
/// `worktree_slots` stays `None` here; [`build_status`] fills project slots.
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
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::ffi::{OsStr, OsString};
    use std::fs;
    use std::io;
    use std::path::{Path, PathBuf};
    use std::process::ExitStatus;
    use std::sync::{Arc, Mutex};

    use odm_git::{CommandOutput, CommandRunner, Git};
    use tempfile::tempdir;

    use crate::config::{ProjectEntry, ProgenEntry, WorkspaceConfig};
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
        out_fail("not a repo")
    }

    fn sha40() -> String {
        "a".repeat(40)
    }

    /// Observation git sample for one on-disk git entity: is_repo, head, origin, clean.
    fn observe_git_ok() -> Vec<CommandOutput> {
        vec![
            is_repo_true(),
            out_ok_stdout(&format!("{}\n", sha40())),
            out_ok_stdout("https://example.com/a.git\n"),
            out_ok(), // clean
        ]
    }

    fn ensure_primary(root: &Path, rel: &str) -> PathBuf {
        let p = root.join(rel);
        fs::create_dir_all(&p).unwrap();
        p
    }

    fn ws_project_and_progen(root: PathBuf) -> Workspace {
        let mut projects = BTreeMap::new();
        projects.insert(
            "alpha".into(),
            ProjectEntry {
                path: "projects/alpha".into(),
                url: None,
                branch: None,
                type_: None,
            },
        );
        let mut progens = BTreeMap::new();
        progens.insert(
            "notes".into(),
            ProgenEntry {
                path: "progens/notes".into(),
                url: None,
                branch: None,
            },
        );
        Workspace {
            root,
            config: WorkspaceConfig {
                projects,
                progens,
                ..Default::default()
            },
            actions: BTreeMap::new(),
            generators: BTreeMap::new(),
        }
    }

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

    #[test]
    fn build_status_includes_registered_worktree_slots() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        fs::create_dir_all(root.join("progens/notes")).unwrap();
        let ws = ws_project_and_progen(root.to_path_buf());
        let s_b = worktree_slot_path(root, "alpha", "b-slot");
        let s_a = worktree_slot_path(root, "alpha", "a-slot");
        let porcelain = format!(
            "worktree {}\nHEAD abc\nbranch refs/heads/main\n\n\
             worktree {}\nHEAD def\nbranch refs/heads/b\n\n\
             worktree {}\nHEAD ghi\nbranch refs/heads/a\n\n",
            primary.display(),
            s_b.display(),
            s_a.display(),
        );
        let mut outs = observe_git_ok();
        // progen on disk, not git
        outs.push(is_repo_false());
        // worktree_list for project
        outs.push(is_repo_true());
        outs.push(out_ok_stdout(&porcelain));
        let (runner, _) = ScriptedRunner::new(outs);
        let g = Git::with_runner(runner);
        let snap = build_status(&g, &ws).unwrap();
        let p = &snap.projects[0];
        let slots = p.worktree_slots.as_ref().expect("project slots");
        assert_eq!(slots.len(), 2);
        assert_eq!(slots[0].name, "a-slot");
        assert_eq!(slots[0].path, "worktrees/alpha/a-slot");
        // ScriptedRunner empty queue → is_clean ok/empty → dirty false
        assert_eq!(slots[0].dirty, Some(false));
        assert_eq!(slots[1].name, "b-slot");
        assert_eq!(slots[1].path, "worktrees/alpha/b-slot");
        assert_eq!(slots[1].dirty, Some(false));
        let v = serde_json::to_value(&snap).unwrap();
        assert!(v["projects"][0]["worktree_slots"].is_array());
        assert_eq!(v["projects"][0]["worktree_slots"][0]["name"], "a-slot");
        assert_eq!(v["projects"][0]["worktree_slots"][0]["dirty"], false);
        assert!(v["progens"][0].get("worktree_slots").is_none());
    }

    #[test]
    fn build_status_empty_slots_when_list_errors_or_non_git() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        ensure_primary(root, "projects/alpha");
        let mut projects = BTreeMap::new();
        projects.insert(
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
            config: WorkspaceConfig {
                projects,
                ..Default::default()
            },
            actions: BTreeMap::new(),
            generators: BTreeMap::new(),
        };
        // non-git primary: observe is_repo false; list is_repo false → Err → []
        let (runner, _) = ScriptedRunner::new(vec![is_repo_false(), is_repo_false()]);
        let g = Git::with_runner(runner);
        let snap = build_status(&g, &ws).unwrap();
        assert_eq!(snap.projects[0].worktree_slots.as_ref().unwrap().len(), 0);
        let v = serde_json::to_value(&snap).unwrap();
        assert_eq!(v["projects"][0]["worktree_slots"], serde_json::json!([]));
    }

    #[test]
    fn build_status_soft_fails_worktree_list_error() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        ensure_primary(root, "projects/alpha");
        let mut projects = BTreeMap::new();
        projects.insert(
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
            config: WorkspaceConfig {
                projects,
                ..Default::default()
            },
            actions: BTreeMap::new(),
            generators: BTreeMap::new(),
        };
        let mut outs = observe_git_ok();
        outs.push(is_repo_true());
        outs.push(out_fail("worktree list boom"));
        let (runner, _) = ScriptedRunner::new(outs);
        let g = Git::with_runner(runner);
        let snap = build_status(&g, &ws).unwrap();
        assert_eq!(
            snap.projects[0].worktree_slots.as_ref().unwrap().as_slice(),
            &[]
        );
    }

    #[test]
    fn format_status_human_shows_slots_when_non_empty() {
        let snap = StatusSnapshot {
            root: "/ws".into(),
            projects: vec![EntityStatus {
                name: "alpha".into(),
                path: "projects/alpha".into(),
                url: None,
                managed: false,
                on_disk: true,
                is_git: true,
                head: None,
                pin_rev: None,
                pin_state: PinState::None,
                dirty: Some(false),
                worktree_slots: Some(vec![
                    WorktreeSlotInfo {
                        name: "a".into(),
                        path: "worktrees/alpha/a".into(),
                        dirty: Some(false),
                    },
                    WorktreeSlotInfo {
                        name: "b".into(),
                        path: "worktrees/alpha/b".into(),
                        dirty: Some(true),
                    },
                ]),
            }],
            progens: vec![],
        };
        let human = format_status_human(&snap);
        assert!(human.contains("worktrees: a, b dirty"), "{human}");
    }

    #[test]
    fn format_status_human_silent_when_slots_empty() {
        let snap = StatusSnapshot {
            root: "/ws".into(),
            projects: vec![EntityStatus {
                name: "alpha".into(),
                path: "projects/alpha".into(),
                url: None,
                managed: false,
                on_disk: true,
                is_git: true,
                head: None,
                pin_rev: None,
                pin_state: PinState::None,
                dirty: None,
                worktree_slots: Some(vec![]),
            }],
            progens: vec![EntityStatus {
                name: "notes".into(),
                path: "progens/notes".into(),
                url: None,
                managed: false,
                on_disk: true,
                is_git: false,
                head: None,
                pin_rev: None,
                pin_state: PinState::None,
                dirty: None,
                worktree_slots: None,
            }],
        };
        let human = format_status_human(&snap);
        assert!(!human.contains("worktrees"), "{human}");
    }
}

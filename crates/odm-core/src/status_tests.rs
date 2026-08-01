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

use crate::agent_pack::PackMode;
use crate::config::{ProjectEntry, ProgenEntry, WorkspaceConfig};
use crate::paths::{agent_packs_path, worktree_slot_path};

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

fn empty_ws(root: PathBuf) -> Workspace {
    Workspace {
        root,
        config: WorkspaceConfig::default(),
        actions: BTreeMap::new(),
        generators: BTreeMap::new(),
    }
}

fn write_registry(root: &Path, json: &str) {
    let odm = root.join(".odm");
    fs::create_dir_all(&odm).unwrap();
    fs::write(agent_packs_path(root), json).unwrap();
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
    assert!(v["agent_packs"].is_array());
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
        agent_packs: vec![],
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
        agent_packs: vec![],
    };
    let human = format_status_human(&snap);
    assert!(!human.contains("worktrees"), "{human}");
}

#[test]
fn build_status_present_pack_missing_false() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let dest = root.join("agent-home/live-pack");
    fs::create_dir_all(&dest).unwrap();
    let json = format!(
        r#"[{{"name":"live-pack","source":"src/live","path":"{}","mode":"install"}}]"#,
        dest.display()
    );
    write_registry(root, &json);
    let ws = empty_ws(root.to_path_buf());
    let (runner, _) = ScriptedRunner::new(vec![]);
    let g = Git::with_runner(runner);
    let snap = build_status(&g, &ws).unwrap();
    assert_eq!(snap.agent_packs.len(), 1);
    let p = &snap.agent_packs[0];
    assert_eq!(p.name, "live-pack");
    assert_eq!(p.source, "src/live");
    assert_eq!(p.path, dest.display().to_string());
    assert_eq!(p.mode, PackMode::Install);
    assert!(!p.missing);
}

#[test]
fn build_status_absent_pack_missing_true() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let missing = root.join("agent-home/gone-pack");
    let json = format!(
        r#"[{{"name":"gone-pack","source":"src/gone","path":"{}","mode":"install"}}]"#,
        missing.display()
    );
    write_registry(root, &json);
    let ws = empty_ws(root.to_path_buf());
    let (runner, _) = ScriptedRunner::new(vec![]);
    let g = Git::with_runner(runner);
    let snap = build_status(&g, &ws).unwrap();
    assert_eq!(snap.agent_packs.len(), 1);
    assert_eq!(snap.agent_packs[0].name, "gone-pack");
    assert!(snap.agent_packs[0].missing);
}

#[test]
fn build_status_dangling_symlink_not_missing() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let home = root.join("agent-home");
    fs::create_dir_all(&home).unwrap();
    let dest = home.join("link-pack");
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(root.join("no-such-target"), &dest).unwrap();
    }
    #[cfg(not(unix))]
    {
        if std::os::windows::fs::symlink_dir(root.join("no-such-target"), &dest).is_err() {
            return;
        }
    }
    assert!(dest.symlink_metadata().is_ok());
    assert!(!dest.exists());
    let json = format!(
        r#"[{{"name":"link-pack","source":"src/link","path":"{}","mode":"link"}}]"#,
        dest.display()
    );
    write_registry(root, &json);
    let ws = empty_ws(root.to_path_buf());
    let (runner, _) = ScriptedRunner::new(vec![]);
    let g = Git::with_runner(runner);
    let snap = build_status(&g, &ws).unwrap();
    assert_eq!(snap.agent_packs.len(), 1);
    assert_eq!(snap.agent_packs[0].mode, PackMode::Link);
    assert!(
        !snap.agent_packs[0].missing,
        "dangling symlink must not be missing"
    );
}

#[test]
fn build_status_empty_or_missing_registry_agent_packs_empty() {
    let dir = tempdir().unwrap();
    let ws = empty_ws(dir.path().to_path_buf());
    let (runner, _) = ScriptedRunner::new(vec![]);
    let g = Git::with_runner(runner);
    let snap = build_status(&g, &ws).unwrap();
    assert!(snap.agent_packs.is_empty());

    write_registry(dir.path(), "[]\n");
    let (runner, _) = ScriptedRunner::new(vec![]);
    let g = Git::with_runner(runner);
    let snap = build_status(&g, &ws).unwrap();
    assert!(snap.agent_packs.is_empty());
}

#[test]
fn build_status_corrupt_registry_soft_fails_to_empty() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    write_registry(root, "not-json{{{");
    let ws = empty_ws(root.to_path_buf());
    let (runner, _) = ScriptedRunner::new(vec![]);
    let g = Git::with_runner(runner);
    let snap = build_status(&g, &ws).unwrap();
    assert!(
        snap.agent_packs.is_empty(),
        "corrupt registry must soft-fail to []"
    );
}

#[test]
fn build_status_agent_packs_sorted_by_name() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let zeta = root.join("agent-home/zeta");
    let alpha = root.join("agent-home/alpha");
    fs::create_dir_all(&zeta).unwrap();
    fs::create_dir_all(&alpha).unwrap();
    let json = format!(
        r#"[
            {{"name":"zeta","source":"s/z","path":"{}","mode":"install"}},
            {{"name":"alpha","source":"s/a","path":"{}","mode":"link"}}
        ]"#,
        zeta.display(),
        alpha.display()
    );
    write_registry(root, &json);
    let ws = empty_ws(root.to_path_buf());
    let (runner, _) = ScriptedRunner::new(vec![]);
    let g = Git::with_runner(runner);
    let snap = build_status(&g, &ws).unwrap();
    assert_eq!(snap.agent_packs.len(), 2);
    assert_eq!(snap.agent_packs[0].name, "alpha");
    assert_eq!(snap.agent_packs[1].name, "zeta");
}

#[test]
fn build_status_json_agent_packs_shape() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    let dest = root.join("agent-home/shape-pack");
    fs::create_dir_all(&dest).unwrap();
    let json = format!(
        r#"[{{"name":"shape-pack","source":"src/shape","path":"{}","mode":"install"}}]"#,
        dest.display()
    );
    write_registry(root, &json);
    let ws = empty_ws(root.to_path_buf());
    let (runner, _) = ScriptedRunner::new(vec![]);
    let g = Git::with_runner(runner);
    let snap = build_status(&g, &ws).unwrap();
    let v = serde_json::to_value(&snap).unwrap();
    assert!(v["agent_packs"].is_array());
    let p = &v["agent_packs"][0];
    assert_eq!(p["name"], "shape-pack");
    assert_eq!(p["source"], "src/shape");
    assert_eq!(p["path"], dest.display().to_string());
    assert_eq!(p["mode"], "install");
    assert_eq!(p["missing"], false);
    // project/progen keys still present
    assert!(v["projects"].is_array());
    assert!(v["progens"].is_array());
    assert!(v["root"].is_string());
}

#[test]
fn format_status_human_shows_agent_packs_and_missing_suffix() {
    let snap = StatusSnapshot {
        root: "/ws".into(),
        projects: vec![],
        progens: vec![],
        agent_packs: vec![
            StatusPackInfo {
                name: "live".into(),
                source: "s/live".into(),
                path: "/ws/home/live".into(),
                mode: PackMode::Install,
                missing: false,
            },
            StatusPackInfo {
                name: "gone".into(),
                source: "s/gone".into(),
                path: "/ws/home/gone".into(),
                mode: PackMode::Link,
                missing: true,
            },
        ],
    };
    let human = format_status_human(&snap);
    assert!(human.contains("Agent packs:"), "{human}");
    assert!(human.contains("  live\tinstall\n"), "{human}");
    assert!(human.contains("  gone\tlink missing\n"), "{human}");
    assert!(
        !human.contains("(no projects or progens)"),
        "packs-only must not be swallowed: {human}"
    );
    assert!(human.starts_with("Workspace: /ws\n"), "{human}");
}

#[test]
fn format_status_human_fully_empty_still_one_empty_message() {
    let snap = StatusSnapshot {
        root: "/ws".into(),
        projects: vec![],
        progens: vec![],
        agent_packs: vec![],
    };
    let human = format_status_human(&snap);
    assert_eq!(human, "Workspace: /ws\n(no projects or progens)\n");
}

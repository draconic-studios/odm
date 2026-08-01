//! Integration harness: drive `odm` against a temp copy of examples/core-desk.
//! Offline only; requires `git` on PATH (skip otherwise).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use assert_cmd::cargo::cargo_bin;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::tempdir;

fn git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn skip_without_git() -> bool {
    if git_available() {
        return false;
    }
    eprintln!("skipping: git not found on PATH");
    true
}

fn odm() -> assert_cmd::Command {
    assert_cmd::Command::new(cargo_bin("odm"))
}

fn git_user(repo: &Path) {
    assert!(Command::new("git")
        .args(["-C", repo.to_str().unwrap(), "config", "user.email", "t@est"])
        .status()
        .unwrap()
        .success());
    assert!(Command::new("git")
        .args(["-C", repo.to_str().unwrap(), "config", "user.name", "t"])
        .status()
        .unwrap()
        .success());
}

fn copy_dir(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let ty = entry.file_type().unwrap();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir(&entry.path(), &to);
        } else {
            fs::copy(entry.path(), to).unwrap();
        }
    }
}

fn core_desk_example() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/core-desk")
        .canonicalize()
        .expect("examples/core-desk")
}

fn setup_temp_core_desk() -> (tempfile::TempDir, PathBuf) {
    let dir = tempdir().unwrap();
    let root = dir.path().join("core-desk");
    copy_dir(&core_desk_example(), &root);
    assert!(Command::new("git")
        .args(["init", root.to_str().unwrap()])
        .status()
        .unwrap()
        .success());
    git_user(&root);
    (dir, root)
}

fn json_stdout(cmd: &mut assert_cmd::Command) -> Value {
    let out = cmd.assert().success().get_output().stdout.clone();
    serde_json::from_slice(&out).expect("stdout JSON")
}

#[test]
fn init_empty_dir_json() {
    if skip_without_git() {
        return;
    }
    let dir = tempdir().unwrap();
    let root = dir.path().join("fresh");

    let v = json_stdout(odm().args(["--json", "init", root.to_str().unwrap()]));
    assert_eq!(v["root"].as_str().unwrap(), root.to_str().unwrap());
    assert_eq!(v["git"].as_bool(), Some(true));
    assert!(root.join(".odm/odm.config.yaml").is_file());
}

#[test]
fn init_empty_dir_no_git() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("fresh-nogit");

    let v = json_stdout(odm().args([
        "--json",
        "init",
        root.to_str().unwrap(),
        "--no-git",
    ]));
    assert!(v.get("root").and_then(|r| r.as_str()).is_some());
    assert_eq!(v["git"].as_bool(), Some(false));
    assert!(root.join(".odm/odm.config.yaml").is_file());
}

#[test]
fn core_desk_full_gate() {
    if skip_without_git() {
        return;
    }
    let (_dir, root) = setup_temp_core_desk();
    let root_s = root.to_str().unwrap();

    // sync → clones alpha/beta from bare fixtures
    odm()
        .args(["--root", root_s, "sync"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha"))
        .stdout(predicate::str::contains("beta"));
    assert!(root.join("projects/alpha").is_dir());
    assert!(root.join("projects/beta").is_dir());
    assert!(root.join(".odm/odm.lock.yaml").is_file());

    // pin status
    let pin = json_stdout(odm().args(["--root", root_s, "--json", "pin", "status"]));
    assert_eq!(pin["present"].as_bool(), Some(true));
    let entries = pin["entries"].as_array().expect("entries");
    assert!(entries.len() >= 2);
    assert!(entries.iter().any(|e| e["state"] == "in_sync"));

    // pin apply
    odm()
        .args(["--root", root_s, "pin", "apply"])
        .assert()
        .success()
        .stdout(predicate::str::contains("applied"));

    // status --json → on_disk / pin_state populated
    let st = json_stdout(odm().args(["--root", root_s, "--json", "status"]));
    let projects = st["projects"].as_array().expect("projects");
    assert_eq!(projects.len(), 2);
    for p in projects {
        assert_eq!(p["on_disk"].as_bool(), Some(true));
        assert!(p.get("pin_state").is_some());
        assert!(p["is_git"].as_bool().unwrap_or(false));
    }

    // doctor → exit 0 (or only warns; ok flag true means no fails)
    let doc = json_stdout(odm().args(["--root", root_s, "--json", "doctor"]));
    assert_eq!(doc["ok"].as_bool(), Some(true));

    // doctor --fix repairs layout/gitignore if needed
    odm()
        .args(["--root", root_s, "doctor", "--fix"])
        .assert()
        .success()
        .stdout(predicate::str::contains("doctor: ok"));
    assert!(root.join(".odm/cache").is_dir());
}

#[test]
fn core_desk_unknown_project_exit_1() {
    if skip_without_git() {
        return;
    }
    let (_dir, root) = setup_temp_core_desk();
    odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "project",
            "info",
            "nope",
        ])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("unknown project"));
}

//! CLI integration: pin apply --force, named sync, project/progen rm --delete/--force.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use assert_cmd::cargo::cargo_bin;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::tempdir;

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

fn bare_with_main(root: &Path, name: &str) -> PathBuf {
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

/// Init workspace + managed project `alpha` cloned from bare fixture.
fn ws_with_project() -> (tempfile::TempDir, PathBuf) {
    let dir = tempdir().unwrap();
    let root = dir.path().join("ws");
    odm()
        .args(["init", root.to_str().unwrap()])
        .assert()
        .success();
    let bare = bare_with_main(dir.path(), "alpha");
    odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "project",
            "add",
            "alpha",
            "--path",
            "projects/alpha",
            "--url",
            bare.to_str().unwrap(),
            "--branch",
            "main",
        ])
        .assert()
        .success();
    (dir, root)
}

fn json_stdout(cmd: &mut assert_cmd::Command) -> Value {
    let out = cmd.assert().success().get_output().stdout.clone();
    serde_json::from_slice(&out).expect("stdout JSON")
}

// ── pin ──────────────────────────────────────────────────────────────────────

#[test]
fn pin_apply_dirty_exits_3_force_ok() {
    let (_dir, root) = ws_with_project();
    let root_s = root.to_str().unwrap();
    let alpha = root.join("projects/alpha");

    fs::write(alpha.join("dirty"), "x").unwrap();

    odm()
        .args(["--root", root_s, "pin", "apply"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("dirty"));

    odm()
        .args(["--root", root_s, "pin", "apply", "--force"])
        .assert()
        .success()
        .stdout(predicate::str::contains("applied"));
}

#[test]
fn pin_status_json_stable_fields() {
    let (_dir, root) = ws_with_project();
    let root_s = root.to_str().unwrap();

    let pin = json_stdout(odm().args(["--root", root_s, "--json", "pin", "status"]));
    assert_eq!(pin["present"].as_bool(), Some(true));
    assert!(pin.get("pin_file").and_then(|v| v.as_str()).is_some());
    let entries = pin["entries"].as_array().expect("entries");
    assert!(!entries.is_empty());
    let e = &entries[0];
    assert!(e.get("name").and_then(|v| v.as_str()).is_some());
    assert!(e.get("pin_rev").is_some());
    assert!(e.get("head").is_some());
    assert!(e.get("state").and_then(|v| v.as_str()).is_some());
}

#[test]
fn pin_status_named_subset() {
    let (_dir, root) = ws_with_project();
    let root_s = root.to_str().unwrap();

    let pin = json_stdout(
        odm().args(["--root", root_s, "--json", "pin", "status", "alpha"]),
    );
    let entries = pin["entries"].as_array().expect("entries");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["name"], "alpha");
}

// ── sync ─────────────────────────────────────────────────────────────────────

#[test]
fn sync_named_ok_and_json_shape() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("ws");
    odm()
        .args(["init", root.to_str().unwrap()])
        .assert()
        .success();
    let bare = bare_with_main(dir.path(), "alpha");
    // Declare without clone so named sync materializes.
    odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "project",
            "add",
            "alpha",
            "--path",
            "projects/alpha",
            "--url",
            bare.to_str().unwrap(),
            "--branch",
            "main",
            "--no-clone",
        ])
        .assert()
        .success();
    let root_s = root.to_str().unwrap();

    let v = json_stdout(odm().args(["--root", root_s, "--json", "sync", "alpha"]));
    let results = v["results"].as_array().expect("results");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["name"], "alpha");
    assert_eq!(results[0]["fetched"].as_bool(), Some(true));
    assert!(results[0].get("materialized").is_some());
    assert!(results[0].get("head").is_some());
    assert!(root.join("projects/alpha").is_dir());
}

#[test]
fn sync_unknown_name_exits_1() {
    let (_dir, root) = ws_with_project();
    let root_s = root.to_str().unwrap();

    odm()
        .args(["--root", root_s, "sync", "no-such-entity"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("unknown entity"));
}

// ── project rm ───────────────────────────────────────────────────────────────

#[test]
fn project_rm_keeps_tree_by_default() {
    let (_dir, root) = ws_with_project();
    let root_s = root.to_str().unwrap();
    let alpha = root.join("projects/alpha");
    assert!(alpha.is_dir());

    odm()
        .args(["--root", root_s, "project", "rm", "alpha"])
        .assert()
        .success()
        .stdout(predicate::str::contains("removed project alpha"));

    assert!(alpha.is_dir());
    odm()
        .args(["--root", root_s, "project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha").not());
}

#[test]
fn project_rm_delete_clean_removes_tree() {
    let (_dir, root) = ws_with_project();
    let root_s = root.to_str().unwrap();
    let alpha = root.join("projects/alpha");

    odm()
        .args(["--root", root_s, "project", "rm", "alpha", "--delete"])
        .assert()
        .success();

    assert!(!alpha.exists());
}

#[test]
fn project_rm_delete_dirty_needs_force() {
    let (_dir, root) = ws_with_project();
    let root_s = root.to_str().unwrap();
    let alpha = root.join("projects/alpha");

    fs::write(alpha.join("dirty"), "x").unwrap();

    odm()
        .args(["--root", root_s, "project", "rm", "alpha", "--delete"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("dirty"));

    assert!(alpha.is_dir());

    odm()
        .args([
            "--root",
            root_s,
            "project",
            "rm",
            "alpha",
            "--delete",
            "--force",
        ])
        .assert()
        .success();

    assert!(!alpha.exists());
}

// ── progen rm ────────────────────────────────────────────────────────────────

#[test]
fn progen_rm_undeclares_without_delete() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("ws");
    odm()
        .args(["init", root.to_str().unwrap(), "--no-git"])
        .assert()
        .success();
    let root_s = root.to_str().unwrap();

    odm()
        .args(["--root", root_s, "progen", "add", "desk", "--path", "vaults/desk"])
        .assert()
        .success();
    let vault = root.join("vaults/desk");
    assert!(vault.is_dir());

    odm()
        .args(["--root", root_s, "progen", "rm", "desk"])
        .assert()
        .success()
        .stdout(predicate::str::contains("removed progen desk"));

    assert!(vault.is_dir());
    odm()
        .args(["--root", root_s, "progen", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("desk").not());
}

#[test]
fn progen_rm_delete_removes_clean_path() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("ws");
    odm()
        .args(["init", root.to_str().unwrap(), "--no-git"])
        .assert()
        .success();
    let root_s = root.to_str().unwrap();

    odm()
        .args(["--root", root_s, "progen", "add", "desk", "--path", "vaults/desk"])
        .assert()
        .success();
    let vault = root.join("vaults/desk");
    assert!(vault.is_dir());

    odm()
        .args(["--root", root_s, "progen", "rm", "desk", "--delete"])
        .assert()
        .success();

    assert!(!vault.exists());
}

#[test]
fn progen_rm_unknown_exits_1() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("ws");
    odm()
        .args(["init", root.to_str().unwrap(), "--no-git"])
        .assert()
        .success();

    odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "progen",
            "rm",
            "no-such-progen",
        ])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("unknown progen"));
}

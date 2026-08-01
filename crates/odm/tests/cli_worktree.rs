//! Integration tests for `odm project worktree` and `project git --wt`.
//! Requires real `git` on PATH (same as `cli_init` clone/sync flow).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use assert_cmd::cargo::cargo_bin;
use predicates::prelude::*;
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

/// init workspace + project add with bare remote so primary is a real git checkout.
fn workspace_with_git_project(root: &Path) {
    odm()
        .args(["init", root.to_str().unwrap()])
        .assert()
        .success();

    let bare = bare_with_main(root, "alpha");

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
}

#[test]
fn worktree_add_list_git_wt_rm_roundtrip() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("ws");
    workspace_with_git_project(&root);

    let slot_path = root.join("worktrees/alpha/slot1");

    // add with --branch so primary's main is not double-checked-out
    odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "project",
            "worktree",
            "add",
            "alpha",
            "slot1",
            "--branch",
            "slot1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("slot1"));

    assert!(slot_path.is_dir());
    assert!(Command::new("git")
        .args([
            "-C",
            slot_path.to_str().unwrap(),
            "rev-parse",
            "--is-inside-work-tree",
        ])
        .status()
        .unwrap()
        .success());

    // list human
    odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "project",
            "worktree",
            "list",
            "alpha",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("slot1"));

    // list json
    let list_out = odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "--json",
            "project",
            "worktree",
            "list",
            "alpha",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let list_json: serde_json::Value = serde_json::from_slice(&list_out).unwrap();
    assert_eq!(list_json["project"], "alpha");
    assert_eq!(list_json["slots"][0]["name"], "slot1");
    assert_eq!(list_json["slots"][0]["path"], "worktrees/alpha/slot1");

    // project git --wt slot1 resolves to slot path
    let toplevel = odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "project",
            "git",
            "alpha",
            "--wt",
            "slot1",
            "--",
            "rev-parse",
            "--show-toplevel",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let toplevel = String::from_utf8_lossy(&toplevel).trim().to_string();
    let expected = slot_path
        .canonicalize()
        .unwrap_or_else(|_| slot_path.clone());
    let got = PathBuf::from(&toplevel)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(&toplevel));
    assert_eq!(got, expected, "git --wt toplevel should be slot path");

    // rm
    odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "project",
            "worktree",
            "rm",
            "alpha",
            "slot1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("slot1"));

    assert!(!slot_path.exists());

    let list_after = odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "--json",
            "project",
            "worktree",
            "list",
            "alpha",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let list_after: serde_json::Value = serde_json::from_slice(&list_after).unwrap();
    assert_eq!(list_after["project"], "alpha");
    let slots = list_after["slots"].as_array().unwrap();
    assert!(slots.is_empty(), "list empty after rm: {list_after}");
}

#[test]
fn project_git_wt_missing_slot_fails_without_creating_path() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("ws");
    workspace_with_git_project(&root);

    let missing = root.join("worktrees/alpha/missing");

    odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "project",
            "git",
            "alpha",
            "--wt",
            "missing",
            "--",
            "rev-parse",
            "HEAD",
        ])
        .assert()
        .failure()
        .code(4)
        .stderr(predicate::str::contains("worktree slot not found"));

    assert!(
        !missing.exists(),
        "missing --wt must not create worktrees/.../missing"
    );
}

#[test]
fn worktree_add_non_git_project_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("ws");

    odm()
        .args(["init", root.to_str().unwrap(), "--no-git"])
        .assert()
        .success();

    // path-only project: mkdir checkout, no git init
    odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "project",
            "add",
            "alpha",
            "--path",
            "projects/alpha",
        ])
        .assert()
        .success();

    fs::create_dir_all(root.join("projects/alpha")).unwrap();
    fs::write(root.join("projects/alpha/README"), "not a git repo").unwrap();

    odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "project",
            "worktree",
            "add",
            "alpha",
            "slot1",
            "--branch",
            "slot1",
        ])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("not a git repo"));
}

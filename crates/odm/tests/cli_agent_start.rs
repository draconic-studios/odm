//! Integration tests for `odm agent start`.

use std::fs;
use std::path::PathBuf;

use assert_cmd::cargo::cargo_bin;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::tempdir;

fn odm() -> assert_cmd::Command {
    assert_cmd::Command::new(cargo_bin("odm"))
}

fn setup_ws_with_alpha() -> (tempfile::TempDir, PathBuf) {
    let dir = tempdir().unwrap();
    let root = dir.path().join("ws");
    fs::create_dir_all(root.join(".odm")).unwrap();
    fs::write(
        root.join(".odm/odm.config.yaml"),
        "name: start-ws\nprojects:\n  alpha:\n    path: projects/alpha\n",
    )
    .unwrap();
    fs::create_dir_all(root.join("projects/alpha")).unwrap();
    fs::write(root.join("projects/alpha/marker"), "x\n").unwrap();
    (dir, root)
}

fn json_from(cmd: &mut assert_cmd::Command) -> (i32, Value) {
    let assert = cmd.assert();
    let out = assert.get_output();
    let code = out.status.code().unwrap_or(1);
    let v: Value = serde_json::from_slice(&out.stdout).expect("stdout JSON");
    (code, v)
}

#[test]
fn start_true_exit_0() {
    let (_dir, root) = setup_ws_with_alpha();
    let root_s = root.to_str().unwrap();
    odm()
        .args([
            "--root",
            root_s,
            "--project",
            "alpha",
            "agent",
            "start",
            "--",
            "true",
        ])
        .assert()
        .success()
        .code(0);
}

#[test]
fn start_false_passthrough() {
    let (_dir, root) = setup_ws_with_alpha();
    let root_s = root.to_str().unwrap();
    odm()
        .args([
            "--root",
            root_s,
            "--project",
            "alpha",
            "agent",
            "start",
            "false",
        ])
        .assert()
        .failure()
        .code(predicate::ne(0));
}

#[test]
fn start_missing_project_exit_1() {
    let (_dir, root) = setup_ws_with_alpha();
    let root_s = root.to_str().unwrap();
    odm()
        .args(["--root", root_s, "agent", "start", "true"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("project is required"));
}

#[test]
fn start_empty_program_exit_1() {
    let (_dir, root) = setup_ws_with_alpha();
    let root_s = root.to_str().unwrap();
    odm()
        .args(["--root", root_s, "--project", "alpha", "agent", "start"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("program is required"));
}

#[test]
fn start_missing_wt_exit_4() {
    let (_dir, root) = setup_ws_with_alpha();
    let root_s = root.to_str().unwrap();
    odm()
        .args([
            "--root",
            root_s,
            "--project",
            "alpha",
            "--wt",
            "missing",
            "agent",
            "start",
            "true",
        ])
        .assert()
        .failure()
        .code(4)
        .stderr(predicate::str::contains("worktree slot not found"));
}

#[test]
fn start_json_shape() {
    let (_dir, root) = setup_ws_with_alpha();
    let root_s = root.to_str().unwrap();
    let (code, v) = json_from(odm().args([
        "--root",
        root_s,
        "--project",
        "alpha",
        "--json",
        "agent",
        "start",
        "--",
        "echo",
        "start-cli-marker",
    ]));
    assert_eq!(code, 0);
    assert!(
        v["cwd"]
            .as_str()
            .unwrap()
            .ends_with("projects/alpha"),
        "cwd: {}",
        v["cwd"]
    );
    assert_eq!(v["program"], "echo");
    assert_eq!(v["args"], serde_json::json!(["start-cli-marker"]));
    assert_eq!(v["exitCode"], 0);
    assert!(
        v["stdout"]
            .as_str()
            .unwrap()
            .contains("start-cli-marker"),
        "stdout: {}",
        v["stdout"]
    );
    assert!(v.get("stderr").is_some());
}

#[test]
fn start_json_false_passthrough() {
    let (_dir, root) = setup_ws_with_alpha();
    let root_s = root.to_str().unwrap();
    let (code, v) = json_from(odm().args([
        "--root",
        root_s,
        "--project",
        "alpha",
        "--json",
        "agent",
        "start",
        "false",
    ]));
    assert_ne!(code, 0);
    assert_eq!(v["program"], "false");
    assert_ne!(v["exitCode"], 0);
    assert_eq!(v["exitCode"], code);
}

#[test]
fn start_wt_slot_cwd() {
    let (_dir, root) = setup_ws_with_alpha();
    let root_s = root.to_str().unwrap();
    fs::create_dir_all(root.join("worktrees/alpha/slot1")).unwrap();
    let (code, v) = json_from(odm().args([
        "--root",
        root_s,
        "--project",
        "alpha",
        "--wt",
        "slot1",
        "--json",
        "agent",
        "start",
        "--",
        "pwd",
        "-P",
    ]));
    assert_eq!(code, 0);
    let cwd = v["cwd"].as_str().unwrap();
    assert!(
        cwd.ends_with("worktrees/alpha/slot1"),
        "cwd: {cwd}"
    );
    let stdout = v["stdout"].as_str().unwrap();
    assert!(
        stdout.contains("worktrees/alpha/slot1"),
        "pwd: {stdout}"
    );
}

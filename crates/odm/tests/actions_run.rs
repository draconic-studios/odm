//! Integration harness: `odm run` against a temp copy of examples/core-desk.

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
    (dir, root)
}

fn json_stdout(cmd: &mut assert_cmd::Command) -> Value {
    let out = cmd.assert().success().get_output().stdout.clone();
    serde_json::from_slice(&out).expect("stdout JSON")
}

/// Action stdio inherits, so run --json may prefix action output; parse trailing JSON object.
fn json_from_stdout_mixed(cmd: &mut assert_cmd::Command) -> Value {
    let out = cmd.assert().success().get_output().stdout.clone();
    let text = String::from_utf8_lossy(&out);
    let start = text.rfind('{').expect("JSON object in stdout");
    serde_json::from_str(&text[start..]).expect("stdout JSON")
}

#[test]
fn run_lists_hello() {
    let (_dir, root) = setup_temp_core_desk();
    odm()
        .args(["--root", root.to_str().unwrap(), "run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"))
        .stdout(predicate::str::contains("fail"))
        .stdout(predicate::str::contains("chain"));
}

#[test]
fn run_hello_success() {
    let (_dir, root) = setup_temp_core_desk();
    odm()
        .args(["--root", root.to_str().unwrap(), "run", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello-desk"));
}

#[test]
fn run_fail_exit_7() {
    let (_dir, root) = setup_temp_core_desk();
    odm()
        .args(["--root", root.to_str().unwrap(), "run", "fail"])
        .assert()
        .failure()
        .code(7);
}

#[test]
fn run_unknown_exit_1() {
    let (_dir, root) = setup_temp_core_desk();
    odm()
        .args(["--root", root.to_str().unwrap(), "run", "nope"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("unknown action"));
}

#[test]
fn run_json_hello() {
    let (_dir, root) = setup_temp_core_desk();
    let v = json_from_stdout_mixed(odm().args([
        "--root",
        root.to_str().unwrap(),
        "--json",
        "run",
        "hello",
    ]));
    assert_eq!(v["action"].as_str(), Some("hello"));
    assert_eq!(v["exitCode"].as_i64(), Some(0));
}

#[test]
fn run_json_list() {
    let (_dir, root) = setup_temp_core_desk();
    let v = json_stdout(odm().args(["--root", root.to_str().unwrap(), "--json", "run"]));
    let actions = v["actions"].as_array().expect("actions");
    assert!(actions.len() >= 3);
    let names: Vec<_> = actions
        .iter()
        .map(|a| a["name"].as_str().unwrap())
        .collect();
    assert!(names.contains(&"hello"));
    let hello = actions.iter().find(|a| a["name"] == "hello").unwrap();
    let tasks = hello["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 1);
    assert!(tasks[0]["run"].as_str().unwrap().contains("hello-desk"));
}

#[test]
fn run_chain_success() {
    let (_dir, root) = setup_temp_core_desk();
    odm()
        .args(["--root", root.to_str().unwrap(), "run", "chain"])
        .assert()
        .success()
        .stdout(predicate::str::contains("step1"))
        .stdout(predicate::str::contains("step2"));
}

#[test]
fn run_no_actions_message() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("empty-ws");
    assert!(Command::new(cargo_bin("odm"))
        .args(["init", root.to_str().unwrap(), "--no-git"])
        .status()
        .unwrap()
        .success());
    odm()
        .args(["--root", root.to_str().unwrap(), "run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("(no actions)"));
}

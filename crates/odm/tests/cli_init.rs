use std::fs;

use assert_cmd::cargo::cargo_bin;
use predicates::prelude::*;
use tempfile::tempdir;

fn odm() -> assert_cmd::Command {
    assert_cmd::Command::new(cargo_bin("odm"))
}

#[test]
fn help_works() {
    odm()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Orchestrated Development Management"));
}

#[test]
fn init_and_project_list() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("ws");

    odm()
        .args(["init", root.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized Workspace"));

    assert!(root.join(".odm/odm.config.yaml").is_file());

    // inject a project into config
    fs::write(
        root.join(".odm/odm.config.yaml"),
        "projects:\n  alpha:\n    path: projects/alpha\n    url: ./fixtures/alpha.git\n",
    )
    .unwrap();

    odm()
        .args(["--root", root.to_str().unwrap(), "project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha"));

    odm()
        .args([
            "--root",
            root.to_str().unwrap(),
            "--json",
            "project",
            "info",
            "alpha",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"alpha\""));
}

#[test]
fn init_json_and_refuse_second() {
    let dir = tempdir().unwrap();
    let root = dir.path().join("ws2");

    odm()
        .args(["--json", "init", root.to_str().unwrap(), "--no-git"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"git\": false"));

    odm()
        .args(["init", root.to_str().unwrap(), "--no-git"])
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("already a Workspace"));
}

#[test]
fn stub_sync_exit_1() {
    let dir = tempdir().unwrap();
    odm()
        .current_dir(dir.path())
        .args(["init", ".", "--no-git"])
        .assert()
        .success();

    odm()
        .current_dir(dir.path())
        .arg("sync")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("not implemented: sync"));
}

#[test]
fn discover_walk_up() {
    let dir = tempdir().unwrap();
    odm()
        .args(["init", dir.path().to_str().unwrap(), "--no-git"])
        .assert()
        .success();
    let nested = dir.path().join("a/b");
    fs::create_dir_all(&nested).unwrap();
    odm()
        .current_dir(&nested)
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("(no projects)"));
}

#[test]
fn unknown_project_usage() {
    let dir = tempdir().unwrap();
    odm()
        .args(["init", dir.path().to_str().unwrap(), "--no-git"])
        .assert()
        .success();
    odm()
        .args([
            "--root",
            dir.path().to_str().unwrap(),
            "project",
            "info",
            "nope",
        ])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("unknown project"));
}



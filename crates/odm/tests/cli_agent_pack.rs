//! Integration tests for `odm agent pack` list|install|link|rm.

use std::fs;
use std::path::Path;

use assert_cmd::cargo::cargo_bin;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::tempdir;

fn odm() -> assert_cmd::Command {
    assert_cmd::Command::new(cargo_bin("odm"))
}

fn setup_pack_ws() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempdir().unwrap();
    let root = dir.path().join("pack-ws");
    fs::create_dir_all(root.join(".odm")).unwrap();
    fs::write(root.join(".odm/odm.config.yaml"), "name: pack-ws\n").unwrap();
    let pack = root.join("packs/core-desk");
    fs::create_dir_all(pack.join("skills")).unwrap();
    fs::write(pack.join("skills/hello.md"), "# hello\n").unwrap();
    (dir, root)
}

fn json_stdout(cmd: &mut assert_cmd::Command) -> Value {
    let out = cmd.assert().success().get_output().stdout.clone();
    serde_json::from_slice(&out).expect("stdout JSON")
}

fn assert_file_eq(path: &Path, want: &str) {
    let got = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    assert_eq!(got, want, "contents of {}", path.display());
}

#[test]
fn pack_list_empty() {
    let (_dir, root) = setup_pack_ws();
    let root_s = root.to_str().unwrap();

    odm()
        .args(["--root", root_s, "agent", "pack", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("(no agent packs)"));

    let v = json_stdout(odm().args(["--root", root_s, "--json", "agent", "pack", "list"]));
    assert_eq!(v["packs"].as_array().unwrap().len(), 0);
}

#[test]
fn pack_install_and_list() {
    let (_dir, root) = setup_pack_ws();
    let root_s = root.to_str().unwrap();
    let home = root.join("agent-home");
    let home_s = home.to_str().unwrap();

    odm()
        .args([
            "--root",
            root_s,
            "agent",
            "pack",
            "install",
            "packs/core-desk",
            "--home",
            home_s,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("installed core-desk ->"));

    assert_file_eq(
        &home.join("core-desk/skills/hello.md"),
        "# hello\n",
    );

    odm()
        .args(["--root", root_s, "agent", "pack", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("core-desk"));

    let v = json_stdout(odm().args(["--root", root_s, "--json", "agent", "pack", "list"]));
    let packs = v["packs"].as_array().unwrap();
    assert_eq!(packs.len(), 1);
    assert_eq!(packs[0]["name"], "core-desk");
    assert_eq!(packs[0]["source"], "packs/core-desk");
    assert_eq!(packs[0]["mode"], "install");
    assert!(packs[0]["path"].as_str().unwrap().contains("core-desk"));
    assert_eq!(packs[0]["missing"].as_bool(), Some(false));
}

#[test]
fn pack_install_json_and_force() {
    let (_dir, root) = setup_pack_ws();
    let root_s = root.to_str().unwrap();
    let home = root.join("home");
    let home_s = home.to_str().unwrap();

    let v = json_stdout(odm().args([
        "--root",
        root_s,
        "--json",
        "agent",
        "pack",
        "install",
        "packs/core-desk",
        "--home",
        home_s,
    ]));
    assert_eq!(v["name"], "core-desk");
    assert_eq!(v["mode"], "install");
    assert_eq!(v["source"], "packs/core-desk");
    assert_eq!(v["missing"].as_bool(), Some(false));

    // second install without force → operation error (exit 3)
    odm()
        .args([
            "--root",
            root_s,
            "agent",
            "pack",
            "install",
            "packs/core-desk",
            "--home",
            home_s,
        ])
        .assert()
        .failure()
        .code(3);

    fs::write(home.join("core-desk/skills/hello.md"), "# old\n").unwrap();
    odm()
        .args([
            "--root",
            root_s,
            "agent",
            "pack",
            "install",
            "packs/core-desk",
            "--home",
            home_s,
            "--force",
        ])
        .assert()
        .success();
    assert_file_eq(&home.join("core-desk/skills/hello.md"), "# hello\n");
}

#[test]
#[cfg(unix)]
fn pack_link() {
    let (_dir, root) = setup_pack_ws();
    let root_s = root.to_str().unwrap();
    let home = root.join("link-home");
    let home_s = home.to_str().unwrap();

    odm()
        .args([
            "--root",
            root_s,
            "agent",
            "pack",
            "link",
            "packs/core-desk",
            "--home",
            home_s,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("linked core-desk ->"));

    let dest = home.join("core-desk");
    assert!(dest
        .symlink_metadata()
        .unwrap()
        .file_type()
        .is_symlink());
    assert_file_eq(&dest.join("skills/hello.md"), "# hello\n");

    let v = json_stdout(odm().args([
        "--root",
        root_s,
        "--json",
        "agent",
        "pack",
        "link",
        "packs/core-desk",
        "--home",
        home_s,
        "--force",
    ]));
    assert_eq!(v["name"], "core-desk");
    assert_eq!(v["mode"], "link");
}

#[test]
fn pack_missing_source_not_found() {
    let (_dir, root) = setup_pack_ws();
    let root_s = root.to_str().unwrap();
    let home = root.join("h");
    fs::create_dir_all(&home).unwrap();
    let home_s = home.to_str().unwrap();

    odm()
        .args([
            "--root",
            root_s,
            "agent",
            "pack",
            "install",
            "packs/missing",
            "--home",
            home_s,
        ])
        .assert()
        .failure()
        .code(4)
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn pack_install_then_rm() {
    let (_dir, root) = setup_pack_ws();
    let root_s = root.to_str().unwrap();
    let home = root.join("rm-home");
    let home_s = home.to_str().unwrap();
    let dest = home.join("core-desk");

    odm()
        .args([
            "--root",
            root_s,
            "agent",
            "pack",
            "install",
            "packs/core-desk",
            "--home",
            home_s,
        ])
        .assert()
        .success();
    assert!(dest.is_dir());

    odm()
        .args(["--root", root_s, "agent", "pack", "rm", "core-desk"])
        .assert()
        .success()
        .stdout(predicate::str::contains("removed core-desk ->"));

    assert!(!dest.exists());

    odm()
        .args(["--root", root_s, "agent", "pack", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("(no agent packs)"));
}

#[test]
fn pack_rm_json() {
    let (_dir, root) = setup_pack_ws();
    let root_s = root.to_str().unwrap();
    let home = root.join("rm-json-home");
    let home_s = home.to_str().unwrap();

    odm()
        .args([
            "--root",
            root_s,
            "agent",
            "pack",
            "install",
            "packs/core-desk",
            "--home",
            home_s,
        ])
        .assert()
        .success();

    let v = json_stdout(odm().args([
        "--root",
        root_s,
        "--json",
        "agent",
        "pack",
        "rm",
        "core-desk",
    ]));
    assert_eq!(v["name"], "core-desk");
    assert_eq!(v["source"], "packs/core-desk");
    assert_eq!(v["mode"], "install");
    assert!(v["path"].as_str().unwrap().contains("core-desk"));
}

#[test]
fn pack_rm_unknown_not_found() {
    let (_dir, root) = setup_pack_ws();
    let root_s = root.to_str().unwrap();

    odm()
        .args(["--root", root_s, "agent", "pack", "rm", "nope"])
        .assert()
        .failure()
        .code(4)
        .stderr(predicate::str::contains("not found"));
}

#[test]
#[cfg(unix)]
fn pack_link_then_rm() {
    let (_dir, root) = setup_pack_ws();
    let root_s = root.to_str().unwrap();
    let home = root.join("link-rm-home");
    let home_s = home.to_str().unwrap();
    let dest = home.join("core-desk");

    odm()
        .args([
            "--root",
            root_s,
            "agent",
            "pack",
            "link",
            "packs/core-desk",
            "--home",
            home_s,
        ])
        .assert()
        .success();
    assert!(dest
        .symlink_metadata()
        .unwrap()
        .file_type()
        .is_symlink());

    odm()
        .args(["--root", root_s, "agent", "pack", "rm", "core-desk"])
        .assert()
        .success()
        .stdout(predicate::str::contains("removed core-desk ->"));

    assert!(dest.symlink_metadata().is_err());

    odm()
        .args(["--root", root_s, "agent", "pack", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("(no agent packs)"));
}

#[test]
fn agent_start_requires_project() {
    let (_dir, root) = setup_pack_ws();
    let root_s = root.to_str().unwrap();

    odm()
        .args(["--root", root_s, "agent", "start", "true"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("project is required"));
}

#[test]
fn pack_list_missing_after_deleted_dest() {
    let (_dir, root) = setup_pack_ws();
    let root_s = root.to_str().unwrap();
    let home = root.join("agent-home");
    let home_s = home.to_str().unwrap();
    let dest = home.join("core-desk");

    odm()
        .args([
            "--root",
            root_s,
            "agent",
            "pack",
            "install",
            "packs/core-desk",
            "--home",
            home_s,
        ])
        .assert()
        .success();

    odm()
        .args(["--root", root_s, "agent", "pack", "list"])
        .assert()
        .success()
        .stdout(predicate::eq("core-desk\n"));

    let v = json_stdout(odm().args(["--root", root_s, "--json", "agent", "pack", "list"]));
    assert_eq!(v["packs"][0]["missing"].as_bool(), Some(false));

    fs::remove_dir_all(&dest).unwrap();
    assert!(!dest.exists());

    odm()
        .args(["--root", root_s, "agent", "pack", "list"])
        .assert()
        .success()
        .stdout(predicate::eq("core-desk missing\n"));

    let v = json_stdout(odm().args(["--root", root_s, "--json", "agent", "pack", "list"]));
    let packs = v["packs"].as_array().unwrap();
    assert_eq!(packs.len(), 1);
    assert_eq!(packs[0]["name"], "core-desk");
    assert_eq!(packs[0]["missing"].as_bool(), Some(true));

    odm()
        .args(["--root", root_s, "agent", "pack", "rm", "core-desk"])
        .assert()
        .success();

    let v = json_stdout(odm().args(["--root", root_s, "--json", "agent", "pack", "list"]));
    assert_eq!(v["packs"].as_array().unwrap().len(), 0);
}

#[test]
fn doctor_pack_missing_after_deleted_dest() {
    let (_dir, root) = setup_pack_ws();
    let root_s = root.to_str().unwrap();
    let home = root.join("agent-home");
    let home_s = home.to_str().unwrap();
    let dest = home.join("core-desk");
    let registry = root.join(".odm/agent-packs.json");

    odm()
        .args([
            "--root",
            root_s,
            "agent",
            "pack",
            "install",
            "packs/core-desk",
            "--home",
            home_s,
        ])
        .assert()
        .success();
    assert!(dest.is_dir());

    let doc = json_stdout(odm().args(["--root", root_s, "--json", "doctor"]));
    assert_eq!(doc["ok"].as_bool(), Some(true));
    let checks = doc["checks"].as_array().expect("checks");
    assert!(
        checks
            .iter()
            .all(|c| c["id"] != "pack_missing:core-desk"),
        "present pack must not pack_missing: {:?}",
        checks
            .iter()
            .map(|c| c["id"].as_str())
            .collect::<Vec<_>>()
    );

    fs::remove_dir_all(&dest).unwrap();
    assert!(!dest.exists());
    let registry_before = fs::read(&registry).expect("registry after install");

    let doc = json_stdout(odm().args(["--root", root_s, "--json", "doctor"]));
    assert_eq!(doc["ok"].as_bool(), Some(true));
    let checks = doc["checks"].as_array().expect("checks");
    let missing = checks
        .iter()
        .find(|c| c["id"] == "pack_missing:core-desk")
        .expect("pack_missing:core-desk after deleted dest");
    assert_eq!(missing["status"], "warn");
    assert_eq!(missing["fixable"].as_bool(), Some(false));

    odm()
        .args(["--root", root_s, "doctor", "--fix"])
        .assert()
        .success();

    let doc = json_stdout(odm().args(["--root", root_s, "--json", "doctor"]));
    assert_eq!(doc["ok"].as_bool(), Some(true));
    let checks = doc["checks"].as_array().expect("checks");
    let missing = checks
        .iter()
        .find(|c| c["id"] == "pack_missing:core-desk")
        .expect("pack_missing still present after --fix");
    assert_eq!(missing["status"], "warn");
    assert_eq!(missing["fixable"].as_bool(), Some(false));

    let registry_after = fs::read(&registry).expect("registry after --fix");
    assert_eq!(
        registry_before, registry_after,
        "doctor --fix must not rewrite agent-packs registry"
    );

    odm()
        .args(["--root", root_s, "agent", "pack", "rm", "core-desk"])
        .assert()
        .success();

    let doc = json_stdout(odm().args(["--root", root_s, "--json", "doctor"]));
    let checks = doc["checks"].as_array().expect("checks");
    assert!(
        checks
            .iter()
            .all(|c| c["id"] != "pack_missing:core-desk"),
        "after pack rm, no pack_missing:core-desk: {:?}",
        checks
            .iter()
            .map(|c| c["id"].as_str())
            .collect::<Vec<_>>()
    );
}

use crate::agent_pack::pack_list;
use crate::config::Workspace;
use crate::doctor::{CheckStatus, DoctorCheck};

/// Warn when a registered agent pack path is absent on disk (no path or symlink entry).
/// Soft-skips on registry load errors so other doctor checks still run.
pub(crate) fn pack_missing_checks(ws: &Workspace) -> Vec<DoctorCheck> {
    let packs = match pack_list(ws) {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };
    let mut checks = Vec::new();
    for entry in packs {
        if !entry.is_missing() {
            continue;
        }
        checks.push(DoctorCheck {
            id: format!("pack_missing:{}", entry.name),
            status: CheckStatus::Warn,
            message: format!(
                "agent pack path missing: {} ({})",
                entry.name,
                entry.path.display()
            ),
            fixable: false,
        });
    }
    checks
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};

    use odm_git::Git;
    use tempfile::tempdir;

    use crate::config::WorkspaceConfig;
    use crate::doctor::run_doctor;
    use crate::init::{init_workspace, InitOptions};
    use crate::paths::agent_packs_path;

    fn empty_ws(root: PathBuf) -> Workspace {
        Workspace {
            root,
            config: WorkspaceConfig {
                manage_gitignore: Some(false),
                ..Default::default()
            },
            actions: BTreeMap::new(),
            generators: BTreeMap::new(),
        }
    }

    fn write_registry(root: &Path, json: &str) {
        let odm = root.join(".odm");
        fs::create_dir_all(&odm).unwrap();
        fs::write(agent_packs_path(root), json).unwrap();
    }

    #[test]
    fn missing_pack_path_warns_not_fixable() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let missing = root.join("agent-home/gone-pack");
        let json = format!(
            r#"[{{"name":"gone-pack","source":"src/gone","path":"{}","mode":"install"}}]"#,
            missing.display()
        );
        write_registry(root, &json);
        let ws = empty_ws(root.to_path_buf());

        let checks = pack_missing_checks(&ws);
        assert_eq!(checks.len(), 1);
        let c = &checks[0];
        assert_eq!(c.id, "pack_missing:gone-pack");
        assert_eq!(c.status, CheckStatus::Warn);
        assert!(!c.fixable);
        assert!(c.message.contains("gone-pack"), "message: {}", c.message);
        assert!(
            c.message.contains(&missing.display().to_string()),
            "message: {}",
            c.message
        );
    }

    #[test]
    fn present_pack_path_no_missing_check() {
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

        let checks = pack_missing_checks(&ws);
        assert!(
            checks.iter().all(|c| !c.id.starts_with("pack_missing:")),
            "present path must not warn: {checks:?}"
        );
    }

    #[test]
    fn empty_or_missing_registry_no_pack_checks() {
        let dir = tempdir().unwrap();
        let ws = empty_ws(dir.path().to_path_buf());
        assert!(pack_missing_checks(&ws).is_empty());

        write_registry(dir.path(), "[]\n");
        assert!(pack_missing_checks(&ws).is_empty());
    }

    #[test]
    fn dangling_symlink_is_not_pack_missing() {
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
            // Windows: skip if symlink creation fails; still assert via metadata path.
            if std::os::windows::fs::symlink_dir(root.join("no-such-target"), &dest).is_err() {
                return;
            }
        }
        assert!(dest.symlink_metadata().is_ok());
        assert!(!dest.exists()); // target missing

        let json = format!(
            r#"[{{"name":"link-pack","source":"src/link","path":"{}","mode":"link"}}]"#,
            dest.display()
        );
        write_registry(root, &json);
        let ws = empty_ws(root.to_path_buf());

        let checks = pack_missing_checks(&ws);
        assert!(
            checks.iter().all(|c| c.id != "pack_missing:link-pack"),
            "dangling symlink must not pack_missing: {checks:?}"
        );
    }

    #[test]
    fn doctor_fix_does_not_alter_packs_or_registry() {
        let dir = tempdir().unwrap();
        init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: true,
            name: None,
        })
        .unwrap();
        let root = dir.path();
        let missing = root.join("agent-home/stale-pack");
        let registry_body = format!(
            r#"[{{"name":"stale-pack","source":"src/stale","path":"{}","mode":"install"}}]
"#,
            missing.display()
        );
        let reg_path = agent_packs_path(root);
        fs::write(&reg_path, &registry_body).unwrap();
        let before = fs::read_to_string(&reg_path).unwrap();

        let ws = empty_ws(root.to_path_buf());
        let git = Git::new();
        let report = run_doctor(&git, &ws, true).unwrap();

        let c = report
            .checks
            .iter()
            .find(|c| c.id == "pack_missing:stale-pack")
            .expect("pack_missing warn present");
        assert_eq!(c.status, CheckStatus::Warn);
        assert!(!c.fixable);
        assert!(report.ok); // Warn only

        let after = fs::read_to_string(&reg_path).unwrap();
        assert_eq!(before, after, "doctor --fix must not rewrite registry");
        assert!(
            !missing.exists() && missing.symlink_metadata().is_err(),
            "doctor --fix must not create pack path"
        );
    }

    #[test]
    fn corrupt_registry_soft_skips() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        write_registry(root, "not-json{{{");
        let ws = empty_ws(root.to_path_buf());
        assert!(
            pack_missing_checks(&ws).is_empty(),
            "corrupt registry must soft-skip"
        );
    }
}

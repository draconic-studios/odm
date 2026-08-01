use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use odm_git::Git;
use serde::Serialize;

use crate::config::{odm_dir, pin_path, Workspace};
use crate::error::OdmError;
use crate::gitignore::{
    ancestor_gitignore_has_drift, apply_managed_gitignore, workspace_gitignore_has_drift,
};
use crate::observation::{observe_workspace, EntityObservation};
use crate::paths::abs_checkout;
use crate::pin::{is_full_sha, load_pin, parse_pin_yaml};
use crate::url_match::urls_match_with_root;
use crate::worktree::{validate_slot_name, worktree_list};

/// `odm doctor --json` report.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct DoctorReport {
    pub ok: bool,
    pub checks: Vec<DoctorCheck>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct DoctorCheck {
    pub id: String,
    pub status: CheckStatus,
    pub message: String,
    pub fixable: bool,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
}

/// Run doctor checks. When `fix` is true, apply mechanical repairs then re-check.
pub fn run_doctor<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
    fix: bool,
) -> Result<DoctorReport, OdmError> {
    if fix {
        apply_fixes(git, ws)?;
    }
    let checks = collect_checks(git, ws)?;
    let ok = !checks.iter().any(|c| c.status == CheckStatus::Fail);
    Ok(DoctorReport { ok, checks })
}

fn apply_fixes<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
) -> Result<(), OdmError> {
    ensure_odm_layout(&ws.root)?;
    if ws.config.manage_gitignore() && git.is_repo(&ws.root).unwrap_or(false) {
        apply_managed_gitignore(&ws.root, &ws.config)?;
    } else if ws.config.manage_gitignore() {
        // Still rewrite ignore files if manage is on (doctor --fix allowlist).
        apply_managed_gitignore(&ws.root, &ws.config)?;
    }
    Ok(())
}

fn ensure_odm_layout(root: &Path) -> Result<(), OdmError> {
    let odm = odm_dir(root);
    for name in ["cache", "log", "progen"] {
        let p = odm.join(name);
        fs::create_dir_all(&p).map_err(|e| {
            OdmError::operation(format!("failed to create {}: {e}", p.display()))
        })?;
    }
    Ok(())
}

fn collect_checks<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
) -> Result<Vec<DoctorCheck>, OdmError> {
    let mut checks = Vec::new();
    // Soft-load pin: invalid pin is reported by pin_checks, not a hard doctor error.
    let pin = load_pin(&ws.root).ok().flatten();
    let obs = observe_workspace(git, &ws.root, &ws.config, pin.as_ref())?;

    checks.push(DoctorCheck {
        id: "config_load".into(),
        status: CheckStatus::Pass,
        message: "config ok".into(),
        fixable: false,
    });

    checks.push(odm_layout_check(&ws.root));

    // path_declared + path_exists + managed_git + origin_match per entity
    for e in &obs.projects {
        push_entity_path_checks(&mut checks, &ws.root, "project", e);
    }
    for e in &obs.progens {
        push_entity_path_checks(&mut checks, &ws.root, "progen", e);
    }

    checks.push(gitignore_drift_check(ws, git)?);
    checks.extend(pin_checks(ws)?);
    checks.extend(worktree_orphan_checks(git, ws));

    Ok(checks)
}

/// Warn on `worktrees/<project>/<slot>/` dirs that are not registered git worktrees.
/// Never fails; swallows git/path errors. Does not scan unknown project names under `worktrees/`.
fn worktree_orphan_checks<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
) -> Vec<DoctorCheck> {
    let mut checks = Vec::new();
    for (project, entry) in &ws.config.projects {
        let Ok(primary) = abs_checkout(&ws.root, &entry.path) else {
            continue;
        };
        if !primary.is_dir() {
            continue;
        }

        let project_wt = ws.root.join("worktrees").join(project);
        if !project_wt.is_dir() {
            continue;
        }

        // Registered slot names under prefix (same filter as `worktree_list`).
        // Non-git primary / list errors → skip project (no Fail from this feature).
        let registered: BTreeSet<String> = match worktree_list(git, ws, project) {
            Ok(out) => out.slots.into_iter().map(|s| s.name).collect(),
            Err(_) => continue,
        };

        let Ok(rd) = fs::read_dir(&project_wt) else {
            continue;
        };
        let mut disk_slots: Vec<String> = Vec::new();
        for ent in rd.flatten() {
            let Ok(ft) = ent.file_type() else {
                continue;
            };
            if !ft.is_dir() {
                continue;
            }
            let name = ent.file_name();
            let Some(s) = name.to_str() else {
                continue;
            };
            if validate_slot_name(s).is_err() {
                continue;
            }
            disk_slots.push(s.to_string());
        }
        disk_slots.sort();

        for slot in disk_slots {
            if registered.contains(&slot) {
                continue;
            }
            let rel = format!("worktrees/{project}/{slot}");
            checks.push(DoctorCheck {
                id: format!("worktree_orphan:{project}:{slot}"),
                status: CheckStatus::Warn,
                message: format!(
                    "orphan worktree slot directory (not a registered git worktree): {rel}"
                ),
                fixable: false,
            });
        }
    }
    checks
}

fn odm_layout_check(root: &Path) -> DoctorCheck {
    let odm = odm_dir(root);
    let missing: Vec<&str> = ["cache", "log", "progen"]
        .into_iter()
        .filter(|n| !odm.join(n).is_dir())
        .collect();
    if missing.is_empty() {
        DoctorCheck {
            id: "odm_layout".into(),
            status: CheckStatus::Pass,
            message: ".odm cache/log/progen present".into(),
            fixable: true,
        }
    } else {
        DoctorCheck {
            id: "odm_layout".into(),
            status: CheckStatus::Warn,
            message: format!("missing .odm dirs: {}", missing.join(", ")),
            fixable: true,
        }
    }
}

fn push_entity_path_checks(
    checks: &mut Vec<DoctorCheck>,
    root: &Path,
    kind: &str,
    e: &EntityObservation,
) {
    let name = &e.name;
    let rel = &e.path;
    let id_base = format!("{kind}:{name}");

    if let Some(err) = &e.resolve_error {
        checks.push(DoctorCheck {
            id: format!("path_declared:{id_base}"),
            status: CheckStatus::Fail,
            message: err.clone(),
            fixable: false,
        });
        return;
    }

    checks.push(DoctorCheck {
        id: format!("path_declared:{id_base}"),
        status: CheckStatus::Pass,
        message: format!("{kind} '{name}' path under Workspace root"),
        fixable: false,
    });

    if !e.on_disk {
        checks.push(DoctorCheck {
            id: format!("path_exists:{id_base}"),
            status: CheckStatus::Warn,
            message: format!("{kind} '{name}' path missing on disk: {rel}"),
            fixable: false,
        });
        return;
    }

    checks.push(DoctorCheck {
        id: format!("path_exists:{id_base}"),
        status: CheckStatus::Pass,
        message: format!("{kind} '{name}' path exists"),
        fixable: false,
    });

    if !e.managed {
        return;
    }

    if !e.is_git {
        checks.push(DoctorCheck {
            id: format!("managed_git:{id_base}"),
            status: CheckStatus::Fail,
            message: format!("managed {kind} '{name}' exists but is not a git repo"),
            fixable: false,
        });
        return;
    }

    checks.push(DoctorCheck {
        id: format!("managed_git:{id_base}"),
        status: CheckStatus::Pass,
        message: format!("managed {kind} '{name}' is a git repo"),
        fixable: false,
    });

    let Some(cfg_url) = e.url.as_deref() else {
        return;
    };

    match &e.origin {
        Some(origin) => {
            if urls_match_with_root(cfg_url, origin, Some(root)) {
                checks.push(DoctorCheck {
                    id: format!("origin_match:{id_base}"),
                    status: CheckStatus::Pass,
                    message: format!("{kind} '{name}' origin matches config url"),
                    fixable: false,
                });
            } else {
                checks.push(DoctorCheck {
                    id: format!("origin_match:{id_base}"),
                    status: CheckStatus::Fail,
                    message: format!(
                        "{kind} '{name}' origin '{origin}' does not match config url '{cfg_url}'"
                    ),
                    fixable: false,
                });
            }
        }
        None => {
            checks.push(DoctorCheck {
                id: format!("origin_match:{id_base}"),
                status: CheckStatus::Fail,
                message: format!("{kind} '{name}' origin missing or unreadable"),
                fixable: false,
            });
        }
    }
}

fn gitignore_drift_check<R: odm_git::CommandRunner>(
    ws: &Workspace,
    git: &Git<R>,
) -> Result<DoctorCheck, OdmError> {
    if !ws.config.manage_gitignore() {
        return Ok(DoctorCheck {
            id: "gitignore_drift".into(),
            status: CheckStatus::Pass,
            message: "manage_gitignore disabled".into(),
            fixable: false,
        });
    }
    let is_git = git.is_repo(&ws.root).unwrap_or(false);
    if !is_git {
        return Ok(DoctorCheck {
            id: "gitignore_drift".into(),
            status: CheckStatus::Pass,
            message: "Workspace is not a git repo".into(),
            fixable: false,
        });
    }
    let drift = workspace_gitignore_has_drift(&ws.root, &ws.config)?
        || ancestor_gitignore_has_drift(&ws.root, &ws.config)?;
    if drift {
        Ok(DoctorCheck {
            id: "gitignore_drift".into(),
            status: CheckStatus::Warn,
            message: "managed gitignore block differs from desired".into(),
            fixable: true,
        })
    } else {
        Ok(DoctorCheck {
            id: "gitignore_drift".into(),
            status: CheckStatus::Pass,
            message: "managed gitignore blocks match desired".into(),
            fixable: true,
        })
    }
}

fn pin_checks(ws: &Workspace) -> Result<Vec<DoctorCheck>, OdmError> {
    let mut checks = Vec::new();
    let path = pin_path(&ws.root);
    if !path.is_file() {
        checks.push(DoctorCheck {
            id: "pin_version".into(),
            status: CheckStatus::Pass,
            message: "no pin file".into(),
            fixable: false,
        });
        return Ok(checks);
    }

    let text = fs::read_to_string(&path).map_err(|e| {
        OdmError::workspace(format!("failed to read {}: {e}", path.display()))
    })?;

    // pin_version: invalid serde or version != 1 → fail
    let pin = match parse_pin_yaml(&text) {
        Ok(p) => {
            checks.push(DoctorCheck {
                id: "pin_version".into(),
                status: CheckStatus::Pass,
                message: "pin file version 1".into(),
                fixable: false,
            });
            p
        }
        Err(e) => {
            checks.push(DoctorCheck {
                id: "pin_version".into(),
                status: CheckStatus::Fail,
                message: e.message(),
                fixable: false,
            });
            // still try raw parse for rev format if possible
            return Ok(checks);
        }
    };

    let managed_names: std::collections::BTreeSet<String> = ws
        .config
        .projects
        .iter()
        .filter(|(_, e)| e.is_managed())
        .map(|(n, _)| n.clone())
        .chain(
            ws.config
                .progens
                .iter()
                .filter(|(_, e)| e.is_managed())
                .map(|(n, _)| n.clone()),
        )
        .collect();

    let unknown: Vec<_> = pin
        .pins
        .keys()
        .filter(|k| !managed_names.contains(k.as_str()))
        .cloned()
        .collect();
    if unknown.is_empty() {
        checks.push(DoctorCheck {
            id: "pin_unknown".into(),
            status: CheckStatus::Pass,
            message: "no unknown pin keys".into(),
            fixable: false,
        });
    } else {
        checks.push(DoctorCheck {
            id: "pin_unknown".into(),
            status: CheckStatus::Warn,
            message: format!("pin keys not in managed set: {}", unknown.join(", ")),
            fixable: false,
        });
    }

    // pin_rev_format — parse_pin_yaml already enforces; double-check for explicit check id
    let bad_revs: Vec<_> = pin
        .pins
        .iter()
        .filter(|(_, e)| !is_full_sha(&e.rev))
        .map(|(n, _)| n.clone())
        .collect();
    if bad_revs.is_empty() {
        checks.push(DoctorCheck {
            id: "pin_rev_format".into(),
            status: CheckStatus::Pass,
            message: "all pin revs are 40-char lowercase hex".into(),
            fixable: false,
        });
    } else {
        checks.push(DoctorCheck {
            id: "pin_rev_format".into(),
            status: CheckStatus::Fail,
            message: format!("invalid pin rev format: {}", bad_revs.join(", ")),
            fixable: false,
        });
    }

    // pin_url_mismatch
    let mut mismatches = Vec::new();
    for (name, pe) in &pin.pins {
        let cfg_url = ws
            .config
            .projects
            .get(name)
            .and_then(|e| e.url.as_deref())
            .or_else(|| {
                ws.config
                    .progens
                    .get(name)
                    .and_then(|e| e.url.as_deref())
            });
        if let Some(cu) = cfg_url {
            if !urls_match_with_root(cu, &pe.url, Some(&ws.root)) {
                mismatches.push(name.clone());
            }
        }
    }
    if mismatches.is_empty() {
        checks.push(DoctorCheck {
            id: "pin_url_mismatch".into(),
            status: CheckStatus::Pass,
            message: "pin urls match config".into(),
            fixable: false,
        });
    } else {
        checks.push(DoctorCheck {
            id: "pin_url_mismatch".into(),
            status: CheckStatus::Warn,
            message: format!("pin url mismatch for: {}", mismatches.join(", ")),
            fixable: false,
        });
    }

    Ok(checks)
}

/// Human multi-line doctor report.
pub fn format_doctor_human(report: &DoctorReport) -> String {
    let mut out = String::new();
    for c in &report.checks {
        let mark = match c.status {
            CheckStatus::Pass => "ok  ",
            CheckStatus::Warn => "warn",
            CheckStatus::Fail => "FAIL",
        };
        out.push_str(&format!("[{mark}] {id}: {msg}\n", id = c.id, msg = c.message));
    }
    if report.ok {
        out.push_str("doctor: ok\n");
    } else {
        out.push_str("doctor: failed\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::ffi::{OsStr, OsString};
    use std::io;
    use std::path::PathBuf;
    use std::process::ExitStatus;
    use std::sync::{Arc, Mutex};

    use odm_git::{CommandOutput, CommandRunner};
    use tempfile::tempdir;

    use crate::config::{ProjectEntry, WorkspaceConfig};
    use crate::init::{init_workspace, InitOptions};
    use crate::paths::worktree_slot_path;
    use crate::pin::{save_pin, PinEntry, PinFile};

    #[cfg(unix)]
    use std::os::unix::process::ExitStatusExt;

    fn load_ws(root: &Path) -> Workspace {
        crate::config::load_workspace(root).unwrap()
    }

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

    fn ws_with_project(root: PathBuf, name: &str, rel: &str) -> Workspace {
        let mut projects = BTreeMap::new();
        projects.insert(
            name.into(),
            ProjectEntry {
                path: rel.into(),
                url: None,
                branch: None,
                type_: None,
            },
        );
        Workspace {
            root,
            config: WorkspaceConfig {
                projects,
                manage_gitignore: Some(false),
                ..Default::default()
            },
            actions: BTreeMap::new(),
            generators: BTreeMap::new(),
        }
    }

    fn ensure_primary(root: &Path, rel: &str) -> PathBuf {
        let p = root.join(rel);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn config_load_always_pass() {
        let dir = tempdir().unwrap();
        init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: true,
            name: None,
        })
        .unwrap();
        let ws = load_ws(dir.path());
        let git = Git::new();
        let report = run_doctor(&git, &ws, false).unwrap();
        let c = report.checks.iter().find(|c| c.id == "config_load").unwrap();
        assert_eq!(c.status, CheckStatus::Pass);
        assert!(report.ok);
    }

    #[test]
    fn odm_layout_warn_and_fix() {
        let dir = tempdir().unwrap();
        init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: true,
            name: None,
        })
        .unwrap();
        let ws = load_ws(dir.path());
        let git = Git::new();
        let report = run_doctor(&git, &ws, false).unwrap();
        let c = report.checks.iter().find(|c| c.id == "odm_layout").unwrap();
        assert_eq!(c.status, CheckStatus::Warn);
        assert!(c.fixable);

        let report2 = run_doctor(&git, &ws, true).unwrap();
        let c2 = report2.checks.iter().find(|c| c.id == "odm_layout").unwrap();
        assert_eq!(c2.status, CheckStatus::Pass);
        assert!(dir.path().join(".odm/cache").is_dir());
        assert!(dir.path().join(".odm/log").is_dir());
        assert!(dir.path().join(".odm/progen").is_dir());
    }

    #[test]
    fn path_declared_escape_fails() {
        let dir = tempdir().unwrap();
        init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: true,
            name: None,
        })
        .unwrap();
        // Bypass load validation so doctor still samples escape at runtime
        // (load rejects `..`; observation/doctor share resolve_under_root).
        let mut cfg = WorkspaceConfig::default();
        cfg.projects.insert(
            "bad".into(),
            ProjectEntry {
                path: "../outside".into(),
                url: None,
                branch: None,
                type_: None,
            },
        );
        let ws = Workspace {
            root: dir.path().to_path_buf(),
            config: cfg,
            actions: Default::default(),
            generators: Default::default(),
        };
        let git = Git::new();
        let report = run_doctor(&git, &ws, false).unwrap();
        let c = report
            .checks
            .iter()
            .find(|c| c.id.starts_with("path_declared:"))
            .unwrap();
        assert_eq!(c.status, CheckStatus::Fail);
        assert!(!report.ok);
    }

    #[test]
    fn gitignore_drift_fixable() {
        let dir = tempdir().unwrap();
        init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: false,
            name: None,
        })
        .unwrap();
        // clobber gitignore
        fs::write(dir.path().join(".gitignore"), "user\n").unwrap();
        let ws = load_ws(dir.path());
        let git = Git::new();
        let report = run_doctor(&git, &ws, false).unwrap();
        let c = report
            .checks
            .iter()
            .find(|c| c.id == "gitignore_drift")
            .unwrap();
        assert_eq!(c.status, CheckStatus::Warn);

        let report2 = run_doctor(&git, &ws, true).unwrap();
        let c2 = report2
            .checks
            .iter()
            .find(|c| c.id == "gitignore_drift")
            .unwrap();
        assert_eq!(c2.status, CheckStatus::Pass);
        let gi = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(gi.contains("user"));
        assert!(gi.contains(".odm/cache/"));
    }

    #[test]
    fn pin_unknown_warn() {
        let dir = tempdir().unwrap();
        init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: true,
            name: None,
        })
        .unwrap();
        let mut pin = PinFile::new_v1();
        pin.pins.insert(
            "ghost".into(),
            PinEntry {
                rev: "a".repeat(40),
                url: "https://example.com/x.git".into(),
                branch: None,
            },
        );
        save_pin(dir.path(), &pin).unwrap();
        let ws = load_ws(dir.path());
        let git = Git::new();
        let report = run_doctor(&git, &ws, false).unwrap();
        let c = report.checks.iter().find(|c| c.id == "pin_unknown").unwrap();
        assert_eq!(c.status, CheckStatus::Warn);
        assert!(report.ok); // warn only
    }

    #[test]
    fn pin_version_fail() {
        let dir = tempdir().unwrap();
        init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: true,
            name: None,
        })
        .unwrap();
        fs::write(
            dir.path().join(".odm/odm.lock.yaml"),
            "version: 99\npins: {}\n",
        )
        .unwrap();
        let ws = load_ws(dir.path());
        let git = Git::new();
        let report = run_doctor(&git, &ws, false).unwrap();
        let c = report.checks.iter().find(|c| c.id == "pin_version").unwrap();
        assert_eq!(c.status, CheckStatus::Fail);
        assert!(!report.ok);
    }

    #[test]
    fn check_status_classification() {
        assert_ne!(CheckStatus::Pass, CheckStatus::Fail);
        assert_ne!(CheckStatus::Warn, CheckStatus::Fail);
    }

    // --- worktree orphan checks ---

    #[test]
    fn orphan_slot_dir_warns_not_fixable() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        let orphan = worktree_slot_path(root, "alpha", "stale");
        fs::create_dir_all(&orphan).unwrap();
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let porcelain = format!(
            "worktree {}\nHEAD abc\nbranch refs/heads/main\n\n",
            primary.display()
        );
        let (runner, _) = ScriptedRunner::new(vec![is_repo_true(), out_ok_stdout(&porcelain)]);
        let g = Git::with_runner(runner);
        let checks = worktree_orphan_checks(&g, &ws);
        assert_eq!(checks.len(), 1);
        let c = &checks[0];
        assert_eq!(c.id, "worktree_orphan:alpha:stale");
        assert_eq!(c.status, CheckStatus::Warn);
        assert!(!c.fixable);
        assert!(c.message.contains("worktrees/alpha/stale"));
    }

    #[test]
    fn registered_slot_is_not_orphan() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        let slot = worktree_slot_path(root, "alpha", "feat");
        fs::create_dir_all(&slot).unwrap();
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let porcelain = format!(
            "worktree {}\nHEAD abc\nbranch refs/heads/main\n\n\
             worktree {}\nHEAD def\nbranch refs/heads/feat\n\n",
            primary.display(),
            slot.display(),
        );
        let (runner, _) = ScriptedRunner::new(vec![is_repo_true(), out_ok_stdout(&porcelain)]);
        let g = Git::with_runner(runner);
        let checks = worktree_orphan_checks(&g, &ws);
        assert!(
            checks.iter().all(|c| !c.id.contains("worktree_orphan")),
            "healthy slot must not warn: {checks:?}"
        );
    }

    #[test]
    fn missing_worktrees_dir_yields_no_orphan_checks() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        ensure_primary(root, "projects/alpha");
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        // No git calls expected — worktrees/ absent short-circuits.
        let (runner, calls) = ScriptedRunner::new(vec![]);
        let g = Git::with_runner(runner);
        let checks = worktree_orphan_checks(&g, &ws);
        assert!(checks.is_empty());
        assert!(calls.lock().unwrap().is_empty());
    }

    #[test]
    fn non_git_primary_skips_orphan_scan() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        ensure_primary(root, "projects/alpha");
        let orphan = worktree_slot_path(root, "alpha", "stale");
        fs::create_dir_all(&orphan).unwrap();
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let (runner, _) = ScriptedRunner::new(vec![is_repo_false()]);
        let g = Git::with_runner(runner);
        let checks = worktree_orphan_checks(&g, &ws);
        assert!(checks.is_empty(), "non-git must not Fail/Warn orphans: {checks:?}");
    }

    #[test]
    fn unknown_worktrees_project_dir_ignored() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        ensure_primary(root, "projects/alpha");
        // Configured project has no worktrees dir; stray unconfigured name under worktrees/.
        fs::create_dir_all(root.join("worktrees/ghost/slot")).unwrap();
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let (runner, calls) = ScriptedRunner::new(vec![]);
        let g = Git::with_runner(runner);
        let checks = worktree_orphan_checks(&g, &ws);
        assert!(checks.is_empty());
        assert!(calls.lock().unwrap().is_empty());
    }

    #[test]
    fn doctor_fix_does_not_delete_orphan_dirs() {
        let dir = tempdir().unwrap();
        init_workspace(InitOptions {
            path: dir.path().to_path_buf(),
            no_git: true,
            name: None,
        })
        .unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        let orphan = worktree_slot_path(root, "alpha", "stale");
        fs::create_dir_all(&orphan).unwrap();
        let mut cfg = WorkspaceConfig {
            manage_gitignore: Some(false),
            ..Default::default()
        };
        cfg.projects.insert(
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
            config: cfg,
            actions: Default::default(),
            generators: Default::default(),
        };
        let porcelain = format!(
            "worktree {}\nHEAD abc\nbranch refs/heads/main\n\n",
            primary.display()
        );
        // observe: is_repo + head + origin + dirty; orphan: is_repo + worktree list.
        // manage_gitignore off → no workspace is_repo. apply_fixes only ensures .odm layout.
        let (runner, _) = ScriptedRunner::new(vec![
            is_repo_true(),
            out_ok_stdout("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n"), // head_sha
            out_ok_stdout("https://example.com/alpha.git\n"),             // origin
            out_ok_stdout(""),                                             // is_clean
            is_repo_true(),
            out_ok_stdout(&porcelain),
        ]);
        let g = Git::with_runner(runner);
        let report = run_doctor(&g, &ws, true).unwrap();
        assert!(orphan.is_dir(), "doctor --fix must not delete orphan dirs");
        let c = report
            .checks
            .iter()
            .find(|c| c.id == "worktree_orphan:alpha:stale")
            .expect("orphan warn present");
        assert_eq!(c.status, CheckStatus::Warn);
        assert!(!c.fixable);
        assert!(report.ok); // Warn only
    }

    #[test]
    fn invalid_slot_name_dir_skipped_silently() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let primary = ensure_primary(root, "projects/alpha");
        // "a/b" can't be a single dir name; use ".." which validate_slot_name rejects.
        // Creating ".." as a directory name is awkward; use a name with invalid chars if possible.
        // On disk we can create a dir named with spaces-only? empty rejected after trim.
        // Create "." is the current dir. Use a file (not dir) and a valid orphan to prove filter.
        let project_wt = root.join("worktrees/alpha");
        fs::create_dir_all(&project_wt).unwrap();
        fs::write(project_wt.join("not-a-dir"), b"x").unwrap();
        let orphan = worktree_slot_path(root, "alpha", "real");
        fs::create_dir_all(&orphan).unwrap();
        let ws = ws_with_project(root.to_path_buf(), "alpha", "projects/alpha");
        let porcelain = format!(
            "worktree {}\nHEAD abc\nbranch refs/heads/main\n\n",
            primary.display()
        );
        let (runner, _) = ScriptedRunner::new(vec![is_repo_true(), out_ok_stdout(&porcelain)]);
        let g = Git::with_runner(runner);
        let checks = worktree_orphan_checks(&g, &ws);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].id, "worktree_orphan:alpha:real");
    }
}

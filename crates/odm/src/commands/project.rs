//! `odm project list` / `info` — config ⨝ observation → DTO.

use odm_core::{
    build_status, load_pin, observe_workspace, EntityObservation, OdmError, PinState,
    StatusSnapshot, Workspace,
};
use odm_git::Git;
use serde::Serialize;

/// `odm project list --json` envelope.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ProjectListDto {
    pub projects: Vec<ProjectListItem>,
}

/// One Project row for list JSON (locked field names).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ProjectListItem {
    pub name: String,
    pub path: String,
    pub url: Option<String>,
    pub branch: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub on_disk: bool,
    pub is_git: bool,
    pub pin_state: Option<PinState>,
}

/// `odm project info --json` (locked field names).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ProjectInfoDto {
    pub name: String,
    pub path: String,
    pub url: Option<String>,
    pub branch: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub on_disk: bool,
    pub is_git: bool,
    pub head: Option<String>,
    pub origin: Option<String>,
    pub dirty: Option<bool>,
    pub pin_rev: Option<String>,
    pub pin_state: PinState,
}

/// Pure projection: Workspace config ⨝ status snapshot → list DTO.
pub fn project_list_from(ws: &Workspace, snap: &StatusSnapshot) -> ProjectListDto {
    let projects = ws
        .config
        .projects
        .iter()
        .map(|(name, e)| {
            let st = snap.projects.iter().find(|p| p.name == *name);
            ProjectListItem {
                name: name.clone(),
                path: e.path.clone(),
                url: e.url.clone(),
                branch: e.branch.clone(),
                type_: e.type_.clone(),
                on_disk: st.map(|s| s.on_disk).unwrap_or(false),
                is_git: st.map(|s| s.is_git).unwrap_or(false),
                pin_state: st.map(|s| s.pin_state),
            }
        })
        .collect();
    ProjectListDto { projects }
}

/// Pure projection: config entry ⨝ observation row → info DTO.
/// Origin comes from observation (no second git query).
pub fn project_info_from(
    name: &str,
    path: &str,
    url: Option<&str>,
    branch: Option<&str>,
    type_: Option<&str>,
    obs: &EntityObservation,
) -> ProjectInfoDto {
    ProjectInfoDto {
        name: name.into(),
        path: path.into(),
        url: url.map(str::to_string),
        branch: branch.map(str::to_string),
        type_: type_.map(str::to_string),
        on_disk: obs.on_disk,
        is_git: obs.is_git,
        head: obs.head.clone(),
        origin: if obs.is_git {
            obs.origin.clone()
        } else {
            None
        },
        dirty: obs.dirty,
        pin_rev: obs.pin_rev.clone(),
        pin_state: obs.pin_state,
    }
}

/// Library entrypoint: observe + project list DTO.
pub fn list_projects<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
) -> Result<ProjectListDto, OdmError> {
    let snap = build_status(git, ws)?;
    Ok(project_list_from(ws, &snap))
}

/// Library entrypoint: observe once + project info DTO (origin from observation).
pub fn project_info<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
    name: &str,
) -> Result<ProjectInfoDto, OdmError> {
    let entry = ws
        .config
        .projects
        .get(name)
        .ok_or_else(|| OdmError::usage(format!("unknown project '{name}'")))?;
    let pin = load_pin(&ws.root)?;
    let obs = observe_workspace(git, &ws.root, &ws.config, pin.as_ref())?;
    let st = obs
        .projects
        .iter()
        .find(|p| p.name == name)
        .ok_or_else(|| OdmError::usage(format!("unknown project '{name}'")))?;
    Ok(project_info_from(
        name,
        &entry.path,
        entry.url.as_deref(),
        entry.branch.as_deref(),
        entry.type_.as_deref(),
        st,
    ))
}

/// Human multi-line list (beside DTO; not a third presentation home).
pub fn format_project_list_human(ws: &Workspace, snap: &StatusSnapshot) -> String {
    if ws.config.projects.is_empty() {
        return "(no projects)\n".into();
    }
    let mut out = String::new();
    for (name, e) in &ws.config.projects {
        let managed = if e.url.is_some() { "managed" } else { "path" };
        let st = snap.projects.iter().find(|p| p.name == *name);
        let on_disk = st.map(|s| s.on_disk).unwrap_or(false);
        let is_git = st.map(|s| s.is_git).unwrap_or(false);
        let pin = st.map(|s| s.pin_state.as_str()).unwrap_or("-");
        out.push_str(&format!(
            "{name}\t{}\t{managed}\ton_disk={on_disk}\tis_git={is_git}\tpin={pin}\n",
            e.path
        ));
    }
    out
}

/// Human multi-line info (beside DTO).
pub fn format_project_info_human(dto: &ProjectInfoDto) -> String {
    let mut out = String::new();
    out.push_str(&format!("name: {}\n", dto.name));
    out.push_str(&format!("path: {}\n", dto.path));
    if let Some(u) = &dto.url {
        out.push_str(&format!("url: {u}\n"));
    }
    if let Some(b) = &dto.branch {
        out.push_str(&format!("branch: {b}\n"));
    }
    if let Some(t) = &dto.type_ {
        out.push_str(&format!("type: {t}\n"));
    }
    out.push_str(&format!("on_disk: {}\n", dto.on_disk));
    out.push_str(&format!("is_git: {}\n", dto.is_git));
    if let Some(h) = &dto.head {
        out.push_str(&format!("head: {h}\n"));
    }
    if let Some(o) = &dto.origin {
        out.push_str(&format!("origin: {o}\n"));
    }
    out.push_str(&format!("pin_state: {:?}\n", dto.pin_state));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use odm_core::{EntityStatus, PinState, ProjectEntry, WorkspaceConfig};

    fn ws_one(name: &str, path: &str) -> Workspace {
        let mut projects = BTreeMap::new();
        projects.insert(
            name.into(),
            ProjectEntry {
                path: path.into(),
                url: Some("https://example.com/a.git".into()),
                branch: Some("main".into()),
                type_: Some("app".into()),
            },
        );
        Workspace {
            root: PathBuf::from("/tmp/ws"),
            config: WorkspaceConfig {
                projects,
                ..Default::default()
            },
            actions: BTreeMap::new(),
            generators: BTreeMap::new(),
        }
    }

    fn snap_one(name: &str, path: &str) -> StatusSnapshot {
        StatusSnapshot {
            root: "/tmp/ws".into(),
            projects: vec![EntityStatus {
                name: name.into(),
                path: path.into(),
                url: Some("https://example.com/a.git".into()),
                managed: true,
                on_disk: true,
                is_git: true,
                head: Some("abc".into()),
                pin_rev: None,
                pin_state: PinState::MissingPinFile,
                dirty: Some(false),
            }],
            progens: vec![],
        }
    }

    #[test]
    fn project_list_dto_locked_json_shape() {
        let ws = ws_one("alpha", "projects/alpha");
        let snap = snap_one("alpha", "projects/alpha");
        let dto = project_list_from(&ws, &snap);
        let v = serde_json::to_value(&dto).unwrap();
        let p = &v["projects"][0];
        assert_eq!(p["name"], "alpha");
        assert_eq!(p["path"], "projects/alpha");
        assert_eq!(p["url"], "https://example.com/a.git");
        assert_eq!(p["branch"], "main");
        assert_eq!(p["type"], "app");
        assert_eq!(p["on_disk"], true);
        assert_eq!(p["is_git"], true);
        assert_eq!(p["pin_state"], "missing_pin_file");
        // envelope key
        assert!(v.get("projects").unwrap().is_array());
    }

    #[test]
    fn project_list_missing_status_defaults() {
        let ws = ws_one("ghost", "projects/ghost");
        let snap = StatusSnapshot {
            root: "/tmp/ws".into(),
            projects: vec![],
            progens: vec![],
        };
        let dto = project_list_from(&ws, &snap);
        assert_eq!(dto.projects.len(), 1);
        assert!(!dto.projects[0].on_disk);
        assert!(!dto.projects[0].is_git);
        assert!(dto.projects[0].pin_state.is_none());
    }

    #[test]
    fn project_info_dto_uses_observation_origin() {
        let obs = EntityObservation {
            name: "alpha".into(),
            path: "projects/alpha".into(),
            url: Some("https://example.com/a.git".into()),
            managed: true,
            abs_path: Some(PathBuf::from("/tmp/ws/projects/alpha")),
            resolve_error: None,
            on_disk: true,
            is_git: true,
            head: Some("deadbeef".into()),
            origin: Some("https://example.com/a.git".into()),
            dirty: Some(false),
            pin_rev: None,
            pin_state: PinState::MissingPinFile,
        };
        let dto = project_info_from(
            "alpha",
            "projects/alpha",
            Some("https://example.com/a.git"),
            Some("main"),
            Some("app"),
            &obs,
        );
        let v = serde_json::to_value(&dto).unwrap();
        assert_eq!(v["name"], "alpha");
        assert_eq!(v["path"], "projects/alpha");
        assert_eq!(v["type"], "app");
        assert_eq!(v["origin"], "https://example.com/a.git");
        assert_eq!(v["head"], "deadbeef");
        assert_eq!(v["pin_state"], "missing_pin_file");
        assert_eq!(v["dirty"], false);
    }

    #[test]
    fn project_info_origin_none_when_not_git() {
        let obs = EntityObservation {
            name: "local".into(),
            path: "projects/local".into(),
            url: None,
            managed: false,
            abs_path: Some(PathBuf::from("/tmp/ws/projects/local")),
            resolve_error: None,
            on_disk: true,
            is_git: false,
            head: None,
            origin: Some("should-ignore".into()),
            dirty: None,
            pin_rev: None,
            pin_state: PinState::None,
        };
        let dto = project_info_from("local", "projects/local", None, None, None, &obs);
        assert!(dto.origin.is_none());
    }
}

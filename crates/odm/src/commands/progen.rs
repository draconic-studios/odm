//! `odm progen list` / `info` — config ⨝ status / vault → DTO.

use std::path::PathBuf;

use odm_core::{build_status, OdmError, PinState, StatusSnapshot, Workspace};
use odm_git::Git;
use odm_progen::{scoped_from_config, vault_info};
use serde::Serialize;

/// `odm progen list --json` envelope.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ProgenListDto {
    pub progens: Vec<ProgenListItem>,
}

/// One Progen row for list JSON (locked field names).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ProgenListItem {
    pub name: String,
    pub path: String,
    pub url: Option<String>,
    pub branch: Option<String>,
    pub on_disk: bool,
    pub is_git: bool,
    pub pin_state: Option<PinState>,
}

/// `odm progen info --json` (locked field names).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ProgenInfoDto {
    pub name: String,
    pub path: String,
    pub url: Option<String>,
    pub branch: Option<String>,
    pub on_disk: bool,
    pub note_count: usize,
    pub has_obsidian: bool,
    pub abs_path: PathBuf,
}

/// Pure projection: Workspace config ⨝ status snapshot → list DTO.
pub fn progen_list_from(ws: &Workspace, snap: &StatusSnapshot) -> ProgenListDto {
    let progens = ws
        .config
        .progens
        .iter()
        .map(|(name, e)| {
            let st = snap.progens.iter().find(|p| p.name == *name);
            ProgenListItem {
                name: name.clone(),
                path: e.path.clone(),
                url: e.url.clone(),
                branch: e.branch.clone(),
                on_disk: st.map(|s| s.on_disk).unwrap_or(false),
                is_git: st.map(|s| s.is_git).unwrap_or(false),
                pin_state: st.map(|s| s.pin_state),
            }
        })
        .collect();
    ProgenListDto { progens }
}

/// Library entrypoint: observe + progen list DTO.
pub fn list_progens<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
) -> Result<ProgenListDto, OdmError> {
    let snap = build_status(git, ws)?;
    Ok(progen_list_from(ws, &snap))
}

/// Library entrypoint: scoped vault + config → info DTO (bin does not compose scope).
pub fn progen_info(ws: &Workspace, name: &str) -> Result<ProgenInfoDto, OdmError> {
    let entry = ws
        .config
        .progens
        .get(name)
        .ok_or_else(|| OdmError::usage(format!("unknown progen '{name}'")))?;
    let sp = scoped_from_config(&ws.root, &ws.config, name)?;
    let info = vault_info(&sp)?;
    Ok(ProgenInfoDto {
        name: name.into(),
        path: entry.path.clone(),
        url: entry.url.clone(),
        branch: entry.branch.clone(),
        on_disk: info.on_disk,
        note_count: info.note_count,
        has_obsidian: info.has_obsidian,
        abs_path: info.path,
    })
}

/// Human multi-line list (beside DTO).
pub fn format_progen_list_human(ws: &Workspace, snap: &StatusSnapshot) -> String {
    if ws.config.progens.is_empty() {
        return "(no progens)\n".into();
    }
    let mut out = String::new();
    for (name, e) in &ws.config.progens {
        let managed = if e.url.is_some() { "managed" } else { "path" };
        let st = snap.progens.iter().find(|p| p.name == *name);
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
pub fn format_progen_info_human(dto: &ProgenInfoDto) -> String {
    let mut out = String::new();
    out.push_str(&format!("name: {}\n", dto.name));
    out.push_str(&format!("path: {}\n", dto.path));
    if let Some(u) = &dto.url {
        out.push_str(&format!("url: {u}\n"));
    }
    out.push_str(&format!("on_disk: {}\n", dto.on_disk));
    out.push_str(&format!("notes: {}\n", dto.note_count));
    out.push_str(&format!("obsidian: {}\n", dto.has_obsidian));
    out.push_str(&format!("abs: {}\n", dto.abs_path.display()));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use odm_core::{EntityStatus, PinState, ProgenEntry, WorkspaceConfig};

    #[test]
    fn progen_list_dto_locked_json_shape() {
        let mut progens = BTreeMap::new();
        progens.insert(
            "notes".into(),
            ProgenEntry {
                path: "progens/notes".into(),
                url: None,
                branch: None,
            },
        );
        let ws = Workspace {
            root: PathBuf::from("/tmp/ws"),
            config: WorkspaceConfig {
                progens,
                ..Default::default()
            },
            actions: BTreeMap::new(),
            generators: BTreeMap::new(),
        };
        let snap = StatusSnapshot {
            root: "/tmp/ws".into(),
            projects: vec![],
            progens: vec![EntityStatus {
                name: "notes".into(),
                path: "progens/notes".into(),
                url: None,
                managed: false,
                on_disk: true,
                is_git: false,
                head: None,
                pin_rev: None,
                pin_state: PinState::None,
                dirty: None,
                worktree_slots: None,
            }],
        };
        let dto = progen_list_from(&ws, &snap);
        let v = serde_json::to_value(&dto).unwrap();
        let p = &v["progens"][0];
        assert_eq!(p["name"], "notes");
        assert_eq!(p["path"], "progens/notes");
        assert_eq!(p["on_disk"], true);
        assert_eq!(p["is_git"], false);
        assert_eq!(p["pin_state"], "none");
        assert!(p.get("url").is_some());
        assert!(p.get("branch").is_some());
    }

    #[test]
    fn progen_info_dto_locked_json_shape() {
        let dto = ProgenInfoDto {
            name: "desk".into(),
            path: "progens/desk".into(),
            url: None,
            branch: None,
            on_disk: true,
            note_count: 3,
            has_obsidian: true,
            abs_path: PathBuf::from("/tmp/ws/progens/desk"),
        };
        let v = serde_json::to_value(&dto).unwrap();
        assert_eq!(v["name"], "desk");
        assert_eq!(v["path"], "progens/desk");
        assert_eq!(v["on_disk"], true);
        assert_eq!(v["note_count"], 3);
        assert_eq!(v["has_obsidian"], true);
        assert_eq!(v["abs_path"], "/tmp/ws/progens/desk");
    }
}

//! `odm agent pack` — list / install / link / rm DTOs and human formatting.

use odm_core::{PackEntry, PackMode};
use serde::Serialize;

/// `odm agent pack list --json` envelope.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PackListDto {
    pub packs: Vec<PackEntryDto>,
}

/// One pack in list or install/link JSON (stable field set).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PackEntryDto {
    pub name: String,
    pub source: String,
    pub path: String,
    pub mode: String,
}

impl From<&PackEntry> for PackEntryDto {
    fn from(e: &PackEntry) -> Self {
        Self {
            name: e.name.clone(),
            source: e.source.clone(),
            path: e.path.display().to_string(),
            mode: mode_str(e.mode).into(),
        }
    }
}

fn mode_str(mode: PackMode) -> &'static str {
    match mode {
        PackMode::Install => "install",
        PackMode::Link => "link",
    }
}

/// Build list DTO from core entries (already sorted by core).
pub fn pack_list_dto(entries: &[PackEntry]) -> PackListDto {
    PackListDto {
        packs: entries.iter().map(PackEntryDto::from).collect(),
    }
}

/// Human one-name-per-line list.
pub fn format_pack_list_human(dto: &PackListDto) -> String {
    if dto.packs.is_empty() {
        return "(no agent packs)\n".into();
    }
    let mut out = String::new();
    for p in &dto.packs {
        out.push_str(&p.name);
        out.push('\n');
    }
    out
}

/// Human success one-liner after install.
pub fn format_pack_install_human(entry: &PackEntry) -> String {
    format!("installed {} -> {}\n", entry.name, entry.path.display())
}

/// Human success one-liner after link.
pub fn format_pack_link_human(entry: &PackEntry) -> String {
    format!("linked {} -> {}\n", entry.name, entry.path.display())
}

/// Human success one-liner after rm.
pub fn format_pack_rm_human(entry: &PackEntry) -> String {
    format!("removed {} -> {}\n", entry.name, entry.path.display())
}

/// Single-entry JSON for install/link/rm (same fields as list item).
pub fn pack_entry_dto(entry: &PackEntry) -> PackEntryDto {
    PackEntryDto::from(entry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn entry(name: &str, mode: PackMode) -> PackEntry {
        PackEntry {
            name: name.into(),
            source: format!("packs/{name}"),
            path: PathBuf::from(format!("/home/agent/{name}")),
            mode,
        }
    }

    #[test]
    fn pack_list_dto_json_shape() {
        let entries = vec![entry("alpha", PackMode::Install), entry("zeta", PackMode::Link)];
        let dto = pack_list_dto(&entries);
        let v = serde_json::to_value(&dto).unwrap();
        let arr = v["packs"].as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["name"], "alpha");
        assert_eq!(arr[0]["source"], "packs/alpha");
        assert_eq!(arr[0]["path"], "/home/agent/alpha");
        assert_eq!(arr[0]["mode"], "install");
        assert_eq!(arr[1]["name"], "zeta");
        assert_eq!(arr[1]["mode"], "link");
    }

    #[test]
    fn empty_pack_list_human() {
        let dto = pack_list_dto(&[]);
        assert_eq!(format_pack_list_human(&dto), "(no agent packs)\n");
    }

    #[test]
    fn pack_list_human_names() {
        let dto = pack_list_dto(&[entry("a", PackMode::Install), entry("b", PackMode::Link)]);
        assert_eq!(format_pack_list_human(&dto), "a\nb\n");
    }

    #[test]
    fn install_link_human_and_dto() {
        let e = entry("core-desk", PackMode::Install);
        assert_eq!(
            format_pack_install_human(&e),
            "installed core-desk -> /home/agent/core-desk\n"
        );
        let e2 = entry("skills", PackMode::Link);
        assert_eq!(
            format_pack_link_human(&e2),
            "linked skills -> /home/agent/skills\n"
        );
        let v = serde_json::to_value(pack_entry_dto(&e)).unwrap();
        assert_eq!(v["name"], "core-desk");
        assert_eq!(v["mode"], "install");
        assert_eq!(v["path"], "/home/agent/core-desk");
        assert_eq!(v["source"], "packs/core-desk");
    }

    #[test]
    fn rm_human() {
        let e = entry("core-desk", PackMode::Install);
        assert_eq!(
            format_pack_rm_human(&e),
            "removed core-desk -> /home/agent/core-desk\n"
        );
    }
}

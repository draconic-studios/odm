//! `odm agent pack` — handlers, list / install / link / rm DTOs and human formatting.

use std::path::Path;

use odm_core::{pack_install, pack_link, pack_list, pack_rm, OdmError, PackEntry, PackMode};
use serde::Serialize;

use crate::ctx::Ctx;
use crate::present::{json_value, Present, Ready};

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
    /// `true` when destination has no path/symlink entry (same rule as doctor/status).
    pub missing: bool,
}

impl From<&PackEntry> for PackEntryDto {
    fn from(e: &PackEntry) -> Self {
        Self {
            name: e.name.clone(),
            source: e.source.clone(),
            path: e.path.display().to_string(),
            mode: mode_str(e.mode).into(),
            missing: e.is_missing(),
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

/// Human one-name-per-line list; suffix ` missing` when dest absent.
pub fn format_pack_list_human(dto: &PackListDto) -> String {
    if dto.packs.is_empty() {
        return "(no agent packs)\n".into();
    }
    let mut out = String::new();
    for p in &dto.packs {
        out.push_str(&p.name);
        if p.missing {
            out.push_str(" missing");
        }
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

impl Present for PackListDto {
    fn to_json(&self) -> Result<serde_json::Value, OdmError> {
        json_value(self)
    }
    fn to_human(&self) -> String {
        format_pack_list_human(self)
    }
}

impl Present for PackEntryDto {
    fn to_json(&self) -> Result<serde_json::Value, OdmError> {
        json_value(self)
    }
    fn to_human(&self) -> String {
        // Prefer install/link/rm-specific formatters in handlers.
        format!("{}\n", self.name)
    }
}

pub fn list_cmd(ctx: &Ctx) -> Result<Ready<PackListDto>, OdmError> {
    let entries = pack_list(&ctx.ws)?;
    let dto = pack_list_dto(&entries);
    let human = format_pack_list_human(&dto);
    Ok(Ready::ok(dto, human))
}

pub fn install_cmd(
    ctx: &Ctx,
    source: &Path,
    home: &Path,
    force: bool,
) -> Result<Ready<PackEntryDto>, OdmError> {
    let entry = pack_install(&ctx.ws, source, home, force)?;
    let human = format_pack_install_human(&entry);
    Ok(Ready::ok(PackEntryDto::from(&entry), human))
}

pub fn link_cmd(
    ctx: &Ctx,
    source: &Path,
    home: &Path,
    force: bool,
) -> Result<Ready<PackEntryDto>, OdmError> {
    let entry = pack_link(&ctx.ws, source, home, force)?;
    let human = format_pack_link_human(&entry);
    Ok(Ready::ok(PackEntryDto::from(&entry), human))
}

pub fn rm_cmd(ctx: &Ctx, name: &str) -> Result<Ready<PackEntryDto>, OdmError> {
    let entry = pack_rm(&ctx.ws, name)?;
    let human = format_pack_rm_human(&entry);
    Ok(Ready::ok(PackEntryDto::from(&entry), human))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn entry_at(name: &str, mode: PackMode, path: PathBuf) -> PackEntry {
        PackEntry {
            name: name.into(),
            source: format!("packs/{name}"),
            path,
            mode,
        }
    }

    fn entry(name: &str, mode: PackMode) -> PackEntry {
        entry_at(name, mode, PathBuf::from(format!("/home/agent/{name}")))
    }

    #[test]
    fn pack_list_dto_json_shape() {
        let dir = tempfile::tempdir().unwrap();
        let present = dir.path().join("alpha");
        fs::create_dir_all(&present).unwrap();
        let entries = vec![
            entry_at("alpha", PackMode::Install, present.clone()),
            entry("zeta", PackMode::Link),
        ];
        let dto = pack_list_dto(&entries);
        let v = serde_json::to_value(&dto).unwrap();
        let arr = v["packs"].as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["name"], "alpha");
        assert_eq!(arr[0]["source"], "packs/alpha");
        assert_eq!(arr[0]["path"], present.display().to_string());
        assert_eq!(arr[0]["mode"], "install");
        assert_eq!(arr[0]["missing"], false);
        assert_eq!(arr[1]["name"], "zeta");
        assert_eq!(arr[1]["mode"], "link");
        assert_eq!(arr[1]["missing"], true);
    }

    #[test]
    fn missing_false_when_path_exists() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pack");
        fs::create_dir_all(&path).unwrap();
        let v = serde_json::to_value(PackEntryDto::from(&entry_at(
            "a",
            PackMode::Install,
            path,
        )))
        .unwrap();
        assert_eq!(v["missing"], false);
    }

    #[test]
    fn missing_true_when_path_absent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("no-such");
        let v = serde_json::to_value(PackEntryDto::from(&entry_at(
            "a",
            PackMode::Install,
            path,
        )))
        .unwrap();
        assert_eq!(v["missing"], true);
    }

    #[test]
    fn missing_false_for_dangling_symlink() {
        let dir = tempfile::tempdir().unwrap();
        let link = dir.path().join("dangling");
        #[cfg(unix)]
        std::os::unix::fs::symlink(dir.path().join("gone-target"), &link).unwrap();
        #[cfg(not(unix))]
        {
            let _ = link;
            return;
        }
        assert!(link.symlink_metadata().is_ok());
        assert!(!link.exists());
        let v = serde_json::to_value(PackEntryDto::from(&entry_at(
            "a",
            PackMode::Link,
            link,
        )))
        .unwrap();
        assert_eq!(v["missing"], false);
    }

    #[test]
    fn empty_pack_list_human() {
        let dto = pack_list_dto(&[]);
        assert_eq!(format_pack_list_human(&dto), "(no agent packs)\n");
    }

    #[test]
    fn pack_list_human_names() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a");
        fs::create_dir_all(&a).unwrap();
        let b = dir.path().join("no-b");
        let dto = pack_list_dto(&[
            entry_at("a", PackMode::Install, a),
            entry_at("b", PackMode::Link, b),
        ]);
        assert_eq!(format_pack_list_human(&dto), "a\nb missing\n");
    }

    #[test]
    fn pack_list_human_present_and_missing() {
        let present = PackListDto {
            packs: vec![PackEntryDto {
                name: "a".into(),
                source: "s".into(),
                path: "/p".into(),
                mode: "install".into(),
                missing: false,
            }],
        };
        assert_eq!(format_pack_list_human(&present), "a\n");
        let missing = PackListDto {
            packs: vec![PackEntryDto {
                name: "a".into(),
                source: "s".into(),
                path: "/p".into(),
                mode: "install".into(),
                missing: true,
            }],
        };
        assert_eq!(format_pack_list_human(&missing), "a missing\n");
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
        let v = serde_json::to_value(PackEntryDto::from(&e)).unwrap();
        assert_eq!(v["name"], "core-desk");
        assert_eq!(v["mode"], "install");
        assert_eq!(v["path"], "/home/agent/core-desk");
        assert_eq!(v["source"], "packs/core-desk");
        assert_eq!(v["missing"], true);
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

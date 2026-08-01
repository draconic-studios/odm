//! Agent pack install / link / list — local filesystem only.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::Workspace;
use crate::error::OdmError;
use crate::io::atomic_write;
use crate::paths::{agent_packs_path, resolve_under_root};

/// How a pack was materialized under an agent home.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackMode {
    Install,
    Link,
}

/// One registered agent pack entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackEntry {
    pub name: String,
    /// Source string as given at install/link time.
    pub source: String,
    /// Resolved destination: `<home>/<name>`.
    pub path: PathBuf,
    pub mode: PackMode,
}

impl PackEntry {
    /// `true` when destination has no path/symlink entry.
    ///
    /// Dangling symlink counts as present (lexists via `symlink_metadata`).
    pub fn is_missing(&self) -> bool {
        self.path.symlink_metadata().is_err()
    }
}

/// List registered agent packs (sorted by name). Missing registry → empty.
pub fn pack_list(ws: &Workspace) -> Result<Vec<PackEntry>, OdmError> {
    let mut packs = load_registry(&ws.root)?;
    packs.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(packs)
}

/// Copy pack source directory contents into `<home>/<name>/` and record registry.
pub fn pack_install(
    ws: &Workspace,
    source: impl AsRef<Path>,
    home: impl AsRef<Path>,
    force: bool,
) -> Result<PackEntry, OdmError> {
    let source_ref = source.as_ref();
    let source_str = path_as_given(source_ref);
    let resolved = resolve_source(ws, source_ref)?;
    let name = pack_name(&resolved)?;
    let dest = home.as_ref().join(&name);

    prepare_dest_for_install(&dest, force)?;
    copy_tree(&resolved, &dest)?;

    let entry = PackEntry {
        name,
        source: source_str,
        path: dest,
        mode: PackMode::Install,
    };
    upsert_registry(&ws.root, &entry)?;
    Ok(entry)
}

/// Symlink `<home>/<name>` → absolute resolved source and record registry.
pub fn pack_link(
    ws: &Workspace,
    source: impl AsRef<Path>,
    home: impl AsRef<Path>,
    force: bool,
) -> Result<PackEntry, OdmError> {
    let source_ref = source.as_ref();
    let source_str = path_as_given(source_ref);
    let resolved = resolve_source(ws, source_ref)?;
    let abs_source = absolutize(&resolved)?;
    let name = pack_name(&resolved)?;
    let dest = home.as_ref().join(&name);

    prepare_dest_for_link(&dest, force)?;
    create_symlink(&abs_source, &dest)?;

    let entry = PackEntry {
        name,
        source: source_str,
        path: dest,
        mode: PackMode::Link,
    };
    upsert_registry(&ws.root, &entry)?;
    Ok(entry)
}

/// Remove a registered agent pack: drop registry entry and delete destination if present.
pub fn pack_rm(ws: &Workspace, name: &str) -> Result<PackEntry, OdmError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(OdmError::usage("pack name must not be empty"));
    }
    let mut packs = load_registry(&ws.root)?;
    let idx = packs.iter().position(|p| p.name == name).ok_or_else(|| {
        OdmError::not_found(format!("agent pack not found: {name}"))
    })?;
    let entry = packs.remove(idx);
    if entry.path.symlink_metadata().is_ok() {
        remove_dest(&entry.path)?;
    }
    save_registry(&ws.root, &packs)?;
    Ok(entry)
}

fn path_as_given(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn resolve_source(ws: &Workspace, source: &Path) -> Result<PathBuf, OdmError> {
    let resolved = if source.is_absolute() {
        source.to_path_buf()
    } else {
        let rel = source.to_str().ok_or_else(|| {
            OdmError::usage(format!(
                "pack source path is not valid UTF-8: {}",
                source.display()
            ))
        })?;
        if rel.is_empty() {
            return Err(OdmError::usage("pack source must not be empty"));
        }
        resolve_under_root(&ws.root, rel)?
    };

    if !resolved.exists() {
        return Err(OdmError::not_found(format!(
            "pack source not found: {}",
            source.display()
        )));
    }
    if !resolved.is_dir() {
        return Err(OdmError::usage(format!(
            "pack source is not a directory: {}",
            source.display()
        )));
    }
    Ok(resolved)
}

fn pack_name(resolved: &Path) -> Result<String, OdmError> {
    let name = resolved
        .file_name()
        .and_then(|s| s.to_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| OdmError::usage("pack name must not be empty"))?;
    if name == "." || name == ".." {
        return Err(OdmError::usage(format!("invalid pack name '{name}'")));
    }
    Ok(name.to_string())
}

fn absolutize(path: &Path) -> Result<PathBuf, OdmError> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    let cwd = std::env::current_dir()
        .map_err(|e| OdmError::operation(format!("failed to get cwd: {e}")))?;
    Ok(cwd.join(path))
}

fn is_dir_empty(path: &Path) -> Result<bool, OdmError> {
    let mut entries = fs::read_dir(path).map_err(|e| {
        OdmError::operation(format!("failed to read {}: {e}", path.display()))
    })?;
    Ok(entries.next().is_none())
}

fn ensure_home_parent(dest: &Path) -> Result<(), OdmError> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            OdmError::operation(format!(
                "failed to create pack home {}: {e}",
                parent.display()
            ))
        })?;
    }
    Ok(())
}

fn remove_dest(dest: &Path) -> Result<(), OdmError> {
    let meta = fs::symlink_metadata(dest).map_err(|e| {
        OdmError::operation(format!("failed to stat {}: {e}", dest.display()))
    })?;
    if meta.file_type().is_symlink() || meta.is_file() {
        fs::remove_file(dest).map_err(|e| {
            OdmError::operation(format!("failed to remove {}: {e}", dest.display()))
        })?;
    } else if meta.is_dir() {
        fs::remove_dir_all(dest).map_err(|e| {
            OdmError::operation(format!("failed to remove {}: {e}", dest.display()))
        })?;
    } else {
        fs::remove_file(dest).map_err(|e| {
            OdmError::operation(format!("failed to remove {}: {e}", dest.display()))
        })?;
    }
    Ok(())
}

fn prepare_dest_for_install(dest: &Path, force: bool) -> Result<(), OdmError> {
    ensure_home_parent(dest)?;

    let meta = match fs::symlink_metadata(dest) {
        Ok(m) => Some(m),
        Err(e) if e.kind() == io::ErrorKind::NotFound => None,
        Err(e) => {
            return Err(OdmError::operation(format!(
                "failed to stat {}: {e}",
                dest.display()
            )));
        }
    };

    match meta {
        None => {
            fs::create_dir_all(dest).map_err(|e| {
                OdmError::operation(format!("failed to create {}: {e}", dest.display()))
            })?;
        }
        Some(meta) => {
            let ft = meta.file_type();
            if ft.is_symlink() || meta.is_file() {
                if !force {
                    return Err(OdmError::operation(format!(
                        "pack destination already exists: {} (use --force)",
                        dest.display()
                    )));
                }
                remove_dest(dest)?;
                fs::create_dir_all(dest).map_err(|e| {
                    OdmError::operation(format!("failed to create {}: {e}", dest.display()))
                })?;
            } else if meta.is_dir() {
                if force {
                    remove_dest(dest)?;
                    fs::create_dir_all(dest).map_err(|e| {
                        OdmError::operation(format!("failed to create {}: {e}", dest.display()))
                    })?;
                } else if !is_dir_empty(dest)? {
                    return Err(OdmError::operation(format!(
                        "pack destination already exists: {} (use --force)",
                        dest.display()
                    )));
                }
                // empty dir: keep and copy into it
            } else if !force {
                return Err(OdmError::operation(format!(
                    "pack destination already exists: {} (use --force)",
                    dest.display()
                )));
            } else {
                remove_dest(dest)?;
                fs::create_dir_all(dest).map_err(|e| {
                    OdmError::operation(format!("failed to create {}: {e}", dest.display()))
                })?;
            }
        }
    }
    Ok(())
}

fn prepare_dest_for_link(dest: &Path, force: bool) -> Result<(), OdmError> {
    ensure_home_parent(dest)?;

    let meta = match fs::symlink_metadata(dest) {
        Ok(m) => Some(m),
        Err(e) if e.kind() == io::ErrorKind::NotFound => None,
        Err(e) => {
            return Err(OdmError::operation(format!(
                "failed to stat {}: {e}",
                dest.display()
            )));
        }
    };

    if let Some(meta) = meta {
        let ft = meta.file_type();
        if ft.is_symlink() || meta.is_file() {
            if !force {
                return Err(OdmError::operation(format!(
                    "pack destination already exists: {} (use --force)",
                    dest.display()
                )));
            }
            remove_dest(dest)?;
        } else if meta.is_dir() {
            if !force && !is_dir_empty(dest)? {
                return Err(OdmError::operation(format!(
                    "pack destination already exists: {} (use --force)",
                    dest.display()
                )));
            }
            // empty dir OK without force; non-empty only with force — remove either way
            remove_dest(dest)?;
        } else if !force {
            return Err(OdmError::operation(format!(
                "pack destination already exists: {} (use --force)",
                dest.display()
            )));
        } else {
            remove_dest(dest)?;
        }
    }
    Ok(())
}

/// Recursively copy `src` directory contents into `dst`.
fn copy_tree(src: &Path, dst: &Path) -> Result<(), OdmError> {
    let entries = fs::read_dir(src).map_err(|e| {
        OdmError::operation(format!("failed to read {}: {e}", src.display()))
    })?;
    for entry in entries {
        let entry = entry.map_err(|e| {
            OdmError::operation(format!("failed to read entry in {}: {e}", src.display()))
        })?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        let ft = entry.file_type().map_err(|e| {
            OdmError::operation(format!("failed to stat {}: {e}", from.display()))
        })?;

        if ft.is_dir() {
            fs::create_dir_all(&to).map_err(|e| {
                OdmError::operation(format!("failed to create {}: {e}", to.display()))
            })?;
            copy_tree(&from, &to)?;
        } else if ft.is_symlink() {
            let target = fs::read_link(&from).map_err(|e| {
                OdmError::operation(format!("failed to read symlink {}: {e}", from.display()))
            })?;
            if to.exists() || to.symlink_metadata().is_ok() {
                remove_dest(&to)?;
            }
            create_symlink(&target, &to)?;
        } else {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    OdmError::operation(format!(
                        "failed to create {}: {e}",
                        parent.display()
                    ))
                })?;
            }
            fs::copy(&from, &to).map_err(|e| {
                OdmError::operation(format!(
                    "failed to copy {} -> {}: {e}",
                    from.display(),
                    to.display()
                ))
            })?;
        }
    }
    Ok(())
}

fn create_symlink(target: &Path, link: &Path) -> Result<(), OdmError> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(target, link).map_err(|e| {
            OdmError::operation(format!(
                "failed to create symlink {} -> {}: {e}",
                link.display(),
                target.display()
            ))
        })
    }
    #[cfg(not(unix))]
    {
        let _ = (target, link);
        Err(OdmError::operation(
            "pack link requires symlink support (not available on this platform)".into(),
        ))
    }
}

fn load_registry(root: &Path) -> Result<Vec<PackEntry>, OdmError> {
    let path = agent_packs_path(root);
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(&path).map_err(|e| {
        OdmError::workspace(format!("failed to read {}: {e}", path.display()))
    })?;
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    let packs: Vec<PackEntry> = serde_json::from_str(&text).map_err(|e| {
        OdmError::workspace(format!("invalid agent pack registry {}: {e}", path.display()))
    })?;
    Ok(packs)
}

fn save_registry(root: &Path, packs: &[PackEntry]) -> Result<(), OdmError> {
    let path = agent_packs_path(root);
    let mut sorted = packs.to_vec();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));
    let json = serde_json::to_string_pretty(&sorted)
        .map_err(|e| OdmError::operation(format!("failed to serialize agent pack registry: {e}")))?;
    let mut body = json;
    body.push('\n');
    atomic_write(&path, &body)
}

fn upsert_registry(root: &Path, entry: &PackEntry) -> Result<(), OdmError> {
    let mut packs = load_registry(root)?;
    if let Some(existing) = packs.iter_mut().find(|p| p.name == entry.name) {
        *existing = entry.clone();
    } else {
        packs.push(entry.clone());
    }
    save_registry(root, &packs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::io::Write;

    use crate::config::WorkspaceConfig;
    use crate::error::exit_code;
    use crate::paths::agent_packs_path;

    fn empty_ws(root: PathBuf) -> Workspace {
        Workspace {
            root,
            config: WorkspaceConfig::default(),
            actions: BTreeMap::new(),
            generators: BTreeMap::new(),
        }
    }

    fn write_pack_src(dir: &Path, files: &[(&str, &str)]) {
        fs::create_dir_all(dir).unwrap();
        for (name, body) in files {
            let p = dir.join(name);
            if let Some(parent) = p.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            let mut f = fs::File::create(&p).unwrap();
            write!(f, "{body}").unwrap();
        }
    }

    #[test]
    fn list_empty_when_no_registry() {
        let dir = tempfile::tempdir().unwrap();
        let ws = empty_ws(dir.path().to_path_buf());
        let list = pack_list(&ws).unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn install_copy_registry_and_list() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let src = root.join("packs/core-desk");
        write_pack_src(&src, &[("SKILL.md", "# skill"), ("nested/a.txt", "a")]);

        let home = root.join("agent-home");
        let ws = empty_ws(root.to_path_buf());

        let entry = pack_install(&ws, Path::new("packs/core-desk"), &home, false).unwrap();
        assert_eq!(entry.name, "core-desk");
        assert_eq!(entry.source, "packs/core-desk");
        assert_eq!(entry.path, home.join("core-desk"));
        assert_eq!(entry.mode, PackMode::Install);

        assert_eq!(
            fs::read_to_string(home.join("core-desk/SKILL.md")).unwrap(),
            "# skill"
        );
        assert_eq!(
            fs::read_to_string(home.join("core-desk/nested/a.txt")).unwrap(),
            "a"
        );
        assert!(agent_packs_path(root).is_file());

        let list = pack_list(&ws).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], entry);
    }

    #[test]
    fn registry_survives_reload() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pack_src(&root.join("src/my-pack"), &[("f.txt", "v1")]);
        let home = root.join("home");
        let ws = empty_ws(root.to_path_buf());

        pack_install(&ws, Path::new("src/my-pack"), &home, false).unwrap();

        // Fresh workspace handle — only root matters for registry path.
        let ws2 = empty_ws(root.to_path_buf());
        let list = pack_list(&ws2).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "my-pack");
        assert_eq!(list[0].mode, PackMode::Install);
        assert_eq!(list[0].path, home.join("my-pack"));
    }

    #[test]
    #[cfg(unix)]
    fn link_symlink_to_absolute_source() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let src = root.join("packs/linked");
        write_pack_src(&src, &[("x.md", "link-me")]);
        let home = root.join("home");
        let ws = empty_ws(root.to_path_buf());

        let entry = pack_link(&ws, &src, &home, false).unwrap();
        assert_eq!(entry.name, "linked");
        assert_eq!(entry.mode, PackMode::Link);
        assert_eq!(entry.path, home.join("linked"));

        let meta = fs::symlink_metadata(&entry.path).unwrap();
        assert!(meta.file_type().is_symlink());
        let target = fs::read_link(&entry.path).unwrap();
        assert_eq!(target, src);
        assert_eq!(
            fs::read_to_string(entry.path.join("x.md")).unwrap(),
            "link-me"
        );

        let list = pack_list(&ws).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].mode, PackMode::Link);
    }

    #[test]
    fn force_replace_install() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pack_src(&root.join("packs/p"), &[("a.txt", "new")]);
        let home = root.join("home");
        let dest = home.join("p");
        fs::create_dir_all(&dest).unwrap();
        fs::write(dest.join("old.txt"), "stale").unwrap();
        fs::write(dest.join("a.txt"), "old").unwrap();

        let ws = empty_ws(root.to_path_buf());
        let err = pack_install(&ws, Path::new("packs/p"), &home, false).unwrap_err();
        assert!(matches!(err, OdmError::Operation(_)));
        assert_eq!(exit_code(&err), 3);

        let entry = pack_install(&ws, Path::new("packs/p"), &home, true).unwrap();
        assert_eq!(entry.mode, PackMode::Install);
        assert_eq!(fs::read_to_string(dest.join("a.txt")).unwrap(), "new");
        assert!(!dest.join("old.txt").exists());
    }

    #[test]
    fn without_force_fails_if_exists() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pack_src(&root.join("packs/p"), &[("a.txt", "x")]);
        let home = root.join("home");
        fs::create_dir_all(home.join("p")).unwrap();
        fs::write(home.join("p/keep.txt"), "y").unwrap();

        let ws = empty_ws(root.to_path_buf());
        let err = pack_install(&ws, Path::new("packs/p"), &home, false).unwrap_err();
        assert!(matches!(err, OdmError::Operation(_)));
        assert_eq!(exit_code(&err), 3);
        assert_eq!(fs::read_to_string(home.join("p/keep.txt")).unwrap(), "y");
    }

    #[test]
    fn missing_source_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let ws = empty_ws(root.to_path_buf());
        let err = pack_install(&ws, Path::new("no/such/pack"), root.join("home"), false)
            .unwrap_err();
        assert!(matches!(err, OdmError::NotFound(_)));
        assert_eq!(exit_code(&err), 4);
    }

    #[test]
    fn relative_escape_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let ws = empty_ws(root.to_path_buf());
        let err = pack_install(&ws, Path::new("../outside"), root.join("home"), false)
            .unwrap_err();
        assert!(matches!(err, OdmError::Workspace(_)));
        assert_eq!(exit_code(&err), 2);
    }

    #[test]
    fn install_allows_empty_dest_without_force() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pack_src(&root.join("packs/e"), &[("f.txt", "ok")]);
        let home = root.join("home");
        fs::create_dir_all(home.join("e")).unwrap();

        let ws = empty_ws(root.to_path_buf());
        let entry = pack_install(&ws, Path::new("packs/e"), &home, false).unwrap();
        assert_eq!(
            fs::read_to_string(entry.path.join("f.txt")).unwrap(),
            "ok"
        );
    }

    #[test]
    fn list_sorted_by_name() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pack_src(&root.join("packs/z-pack"), &[("z.txt", "z")]);
        write_pack_src(&root.join("packs/a-pack"), &[("a.txt", "a")]);
        let home = root.join("home");
        let ws = empty_ws(root.to_path_buf());

        pack_install(&ws, Path::new("packs/z-pack"), &home, false).unwrap();
        pack_install(&ws, Path::new("packs/a-pack"), &home, false).unwrap();

        let list = pack_list(&ws).unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].name, "a-pack");
        assert_eq!(list[1].name, "z-pack");
    }

    #[test]
    fn absolute_source_install() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let src = root.join("abs-pack");
        write_pack_src(&src, &[("t.txt", "abs")]);
        // home outside workspace
        let home_dir = tempfile::tempdir().unwrap();
        let home = home_dir.path();
        let ws = empty_ws(root.to_path_buf());

        let entry = pack_install(&ws, &src, home, false).unwrap();
        assert_eq!(entry.name, "abs-pack");
        assert_eq!(entry.path, home.join("abs-pack"));
        assert_eq!(
            fs::read_to_string(entry.path.join("t.txt")).unwrap(),
            "abs"
        );
        // registry still under workspace
        assert!(agent_packs_path(root).is_file());
    }

    #[test]
    #[cfg(unix)]
    fn force_replace_link_over_install() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pack_src(&root.join("packs/p"), &[("a.txt", "via-link")]);
        let home = root.join("home");
        let ws = empty_ws(root.to_path_buf());

        pack_install(&ws, Path::new("packs/p"), &home, false).unwrap();
        let err = pack_link(&ws, Path::new("packs/p"), &home, false).unwrap_err();
        assert_eq!(exit_code(&err), 3);

        let entry = pack_link(&ws, Path::new("packs/p"), &home, true).unwrap();
        assert_eq!(entry.mode, PackMode::Link);
        assert!(fs::symlink_metadata(&entry.path)
            .unwrap()
            .file_type()
            .is_symlink());
        let list = pack_list(&ws).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].mode, PackMode::Link);
    }

    #[test]
    fn rm_after_install_removes_dest_and_registry() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pack_src(&root.join("packs/core-desk"), &[("SKILL.md", "# skill")]);
        let home = root.join("home");
        let ws = empty_ws(root.to_path_buf());

        let installed = pack_install(&ws, Path::new("packs/core-desk"), &home, false).unwrap();
        assert!(installed.path.is_dir());

        let removed = pack_rm(&ws, "core-desk").unwrap();
        assert_eq!(removed, installed);
        assert!(fs::symlink_metadata(&installed.path).is_err());
        assert!(pack_list(&ws).unwrap().is_empty());
    }

    #[test]
    #[cfg(unix)]
    fn rm_after_link_removes_symlink() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pack_src(&root.join("packs/linked"), &[("x.md", "link-me")]);
        let home = root.join("home");
        let ws = empty_ws(root.to_path_buf());

        let linked = pack_link(&ws, Path::new("packs/linked"), &home, false).unwrap();
        assert!(fs::symlink_metadata(&linked.path)
            .unwrap()
            .file_type()
            .is_symlink());

        let removed = pack_rm(&ws, "linked").unwrap();
        assert_eq!(removed, linked);
        assert!(fs::symlink_metadata(&linked.path).is_err());
        // source pack untouched
        assert!(root.join("packs/linked/x.md").is_file());
        assert!(pack_list(&ws).unwrap().is_empty());
    }

    #[test]
    fn rm_unknown_name_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let ws = empty_ws(dir.path().to_path_buf());
        let err = pack_rm(&ws, "nope").unwrap_err();
        assert!(matches!(err, OdmError::NotFound(_)));
        assert_eq!(exit_code(&err), 4);
        assert!(err.to_string().contains("agent pack not found: nope"));
    }

    #[test]
    fn rm_empty_name_usage() {
        let dir = tempfile::tempdir().unwrap();
        let ws = empty_ws(dir.path().to_path_buf());
        for name in ["", "   ", "\t"] {
            let err = pack_rm(&ws, name).unwrap_err();
            assert!(matches!(err, OdmError::Usage(_)));
            assert_eq!(exit_code(&err), 1);
        }
    }

    #[test]
    fn rm_missing_dest_still_cleans_registry() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pack_src(&root.join("packs/stale"), &[("f.txt", "v")]);
        let home = root.join("home");
        let ws = empty_ws(root.to_path_buf());

        let installed = pack_install(&ws, Path::new("packs/stale"), &home, false).unwrap();
        fs::remove_dir_all(&installed.path).unwrap();
        assert!(fs::symlink_metadata(&installed.path).is_err());

        let removed = pack_rm(&ws, "stale").unwrap();
        assert_eq!(removed, installed);
        assert!(pack_list(&ws).unwrap().is_empty());
    }

    #[test]
    fn rm_one_pack_preserves_other() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_pack_src(&root.join("packs/keep-me"), &[("k.txt", "keep")]);
        write_pack_src(&root.join("packs/drop-me"), &[("d.txt", "drop")]);
        let home = root.join("home");
        let ws = empty_ws(root.to_path_buf());

        let keep = pack_install(&ws, Path::new("packs/keep-me"), &home, false).unwrap();
        let drop = pack_install(&ws, Path::new("packs/drop-me"), &home, false).unwrap();

        let removed = pack_rm(&ws, "drop-me").unwrap();
        assert_eq!(removed, drop);
        assert!(fs::symlink_metadata(&drop.path).is_err());
        assert_eq!(
            fs::read_to_string(keep.path.join("k.txt")).unwrap(),
            "keep"
        );

        let list = pack_list(&ws).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], keep);
    }
}

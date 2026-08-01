//! Workspace layout path policy — single owner for checkout, worktree, and index paths.

use std::path::{Component, Path, PathBuf};

use crate::error::OdmError;

/// ODM state directory: `<root>/.odm`.
pub fn odm_dir(root: &Path) -> PathBuf {
    root.join(".odm")
}

/// Workspace config path: `<root>/.odm/odm.config.yaml`.
pub fn config_path(root: &Path) -> PathBuf {
    odm_dir(root).join("odm.config.yaml")
}

/// Pin file path: `<root>/.odm/odm.lock.yaml`.
pub fn pin_path(root: &Path) -> PathBuf {
    odm_dir(root).join("odm.lock.yaml")
}

/// Agent pack registry path: `<root>/.odm/agent-packs.json`.
pub fn agent_packs_path(root: &Path) -> PathBuf {
    odm_dir(root).join("agent-packs.json")
}

/// Resolve config-relative `rel` under Workspace `root`.
/// Err if absolute or escapes via `..`.
pub fn resolve_under_root(root: &Path, rel: &str) -> Result<PathBuf, OdmError> {
    let p = Path::new(rel);
    if p.is_absolute() {
        return Err(OdmError::workspace(format!(
            "path must be relative, got '{rel}'"
        )));
    }
    let mut out = root.to_path_buf();
    for c in p.components() {
        match c {
            Component::CurDir => {}
            Component::Normal(s) => out.push(s),
            Component::ParentDir => {
                if !out.pop() || out.as_os_str().is_empty() {
                    return Err(OdmError::workspace(format!(
                        "path escapes Workspace root: '{rel}'"
                    )));
                }
                if !out.starts_with(root) {
                    return Err(OdmError::workspace(format!(
                        "path escapes Workspace root: '{rel}'"
                    )));
                }
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(OdmError::workspace(format!(
                    "path must be relative, got '{rel}'"
                )));
            }
        }
    }
    if !out.starts_with(root) {
        return Err(OdmError::workspace(format!(
            "path escapes Workspace root: '{rel}'"
        )));
    }
    Ok(out)
}

/// Primary checkout / Progen store absolute path (escape-safe).
pub fn abs_checkout(root: &Path, rel: &str) -> Result<PathBuf, OdmError> {
    resolve_under_root(root, rel)
}

/// Single path component used as a name (entity, worktree slot, …).
///
/// Non-empty after trim; no `/` `\` NUL; not `.` or `..`; no Windows drive prefix.
/// Returns the trimmed token, or a short reason string on failure.
pub fn parse_path_token(name: &str) -> Result<&str, &'static str> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("empty");
    }
    if trimmed == "." || trimmed == ".." {
        return Err("dot");
    }
    if trimmed.contains('/') || trimmed.contains('\\') || trimmed.contains('\0') {
        return Err("separator");
    }
    if trimmed.starts_with('/') {
        return Err("absolute");
    }
    let bytes = trimmed.as_bytes();
    if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
        return Err("drive");
    }
    Ok(trimmed)
}

/// Worktree slot working tree: `<root>/worktrees/<project>/<slot>`.
pub fn worktree_slot_path(root: &Path, project_name: &str, slot_name: &str) -> PathBuf {
    root.join("worktrees").join(project_name).join(slot_name)
}

/// ODM-side Progen index dir: `<root>/.odm/progen/<name>`.
pub fn progen_index_dir(root: &Path, progen_name: &str) -> PathBuf {
    odm_dir(root).join("progen").join(progen_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_under_root_happy_path() {
        let root = Path::new("/ws");
        let got = resolve_under_root(root, "projects/a").unwrap();
        assert_eq!(got, PathBuf::from("/ws/projects/a"));
    }

    #[test]
    fn resolve_under_root_rejects_escape() {
        let root = Path::new("/ws");
        assert!(resolve_under_root(root, "../outside").is_err());
        assert!(resolve_under_root(root, "a/../../outside").is_err());
    }

    #[test]
    fn resolve_under_root_rejects_absolute() {
        let root = Path::new("/ws");
        assert!(resolve_under_root(root, "/abs").is_err());
    }

    #[test]
    fn abs_checkout_matches_resolve() {
        let root = Path::new("/ws");
        assert_eq!(
            abs_checkout(root, "projects/a").unwrap(),
            resolve_under_root(root, "projects/a").unwrap()
        );
        assert!(abs_checkout(root, "../outside").is_err());
    }

    #[test]
    fn worktree_slot_path_shape() {
        let root = Path::new("/ws");
        assert_eq!(
            worktree_slot_path(root, "alpha", "slot1"),
            PathBuf::from("/ws/worktrees/alpha/slot1")
        );
    }

    #[test]
    fn progen_index_dir_shape() {
        let root = Path::new("/ws");
        assert_eq!(
            progen_index_dir(root, "desk"),
            PathBuf::from("/ws/.odm/progen/desk")
        );
    }

    #[test]
    fn layout_helpers() {
        let root = Path::new("/ws");
        assert_eq!(odm_dir(root), PathBuf::from("/ws/.odm"));
        assert_eq!(config_path(root), PathBuf::from("/ws/.odm/odm.config.yaml"));
        assert_eq!(pin_path(root), PathBuf::from("/ws/.odm/odm.lock.yaml"));
        assert_eq!(
            agent_packs_path(root),
            PathBuf::from("/ws/.odm/agent-packs.json")
        );
    }

    #[test]
    fn parse_path_token_rules() {
        assert_eq!(parse_path_token("  ok  ").unwrap(), "ok");
        assert!(parse_path_token("").is_err());
        assert!(parse_path_token("a/b").is_err());
        assert!(parse_path_token("..").is_err());
        assert!(parse_path_token(".").is_err());
    }
}

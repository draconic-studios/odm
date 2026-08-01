use std::fs;
use std::path::Path;

use crate::config::WorkspaceConfig;
use crate::error::OdmError;
use crate::io::atomic_write;

pub const BEGIN_MARKER: &str = "# >>> ODM managed (do not edit between markers)";
pub const END_MARKER: &str = "# <<< ODM managed";

const EPHEMERAL: &[&str] = &[".odm/cache/", ".odm/log/", ".odm/progen/", "worktrees/"];

/// Seed/update Workspace-root `.gitignore` managed block.
pub fn update_workspace_gitignore(
    root: &Path,
    config: &WorkspaceConfig,
) -> Result<(), OdmError> {
    if !config.manage_gitignore() {
        return Ok(());
    }
    let mut lines: Vec<String> = EPHEMERAL.iter().map(|s| (*s).to_string()).collect();
    for entry in config.projects.values() {
        if entry.is_managed() {
            lines.push(with_trailing_slash(&entry.path));
        }
    }
    for entry in config.progens.values() {
        if entry.is_managed() {
            lines.push(with_trailing_slash(&entry.path));
        }
    }
    // stable unique order
    lines.sort();
    lines.dedup();
    write_managed_block(&root.join(".gitignore"), &lines)
}

fn with_trailing_slash(path: &str) -> String {
    let p = path.trim_end_matches('/');
    format!("{p}/")
}

pub fn write_managed_block(gitignore_path: &Path, inner_lines: &[String]) -> Result<(), OdmError> {
    let existing = if gitignore_path.is_file() {
        fs::read_to_string(gitignore_path).map_err(|e| {
            OdmError::operation(format!(
                "failed to read {}: {e}",
                gitignore_path.display()
            ))
        })?
    } else {
        String::new()
    };

    let new_block = format_block(inner_lines);
    let updated = replace_or_append_block(&existing, &new_block);
    atomic_write(gitignore_path, &updated)
}

fn format_block(inner_lines: &[String]) -> String {
    let mut s = String::new();
    s.push_str(BEGIN_MARKER);
    s.push('\n');
    for line in inner_lines {
        s.push_str(line);
        s.push('\n');
    }
    s.push_str(END_MARKER);
    s.push('\n');
    s
}

fn replace_or_append_block(existing: &str, new_block: &str) -> String {
    if let Some((before, after)) = split_markers(existing) {
        let mut out = before;
        if !out.is_empty() && !out.ends_with('\n') {
            out.push('\n');
        }
        out.push_str(new_block);
        // after may start with leftover newline content
        let after = after.trim_start_matches('\n');
        if !after.is_empty() {
            if !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(after);
            if !out.ends_with('\n') {
                out.push('\n');
            }
        }
        return out;
    }

    // append
    let mut out = existing.to_string();
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    out.push_str(new_block);
    out
}

fn split_markers(text: &str) -> Option<(String, String)> {
    let begin = text.find(BEGIN_MARKER)?;
    let end_rel = text[begin..].find(END_MARKER)?;
    let end = begin + end_rel + END_MARKER.len();
    // consume trailing newline after end marker
    let mut after_start = end;
    if text[after_start..].starts_with('\n') {
        after_start += 1;
    }
    Some((text[..begin].to_string(), text[after_start..].to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProjectEntry;
    use tempfile::tempdir;

    #[test]
    fn seed_ephemeral_and_managed() {
        let dir = tempdir().unwrap();
        let mut cfg = WorkspaceConfig::default();
        cfg.projects.insert(
            "a".into(),
            ProjectEntry {
                path: "projects/alpha".into(),
                url: Some("u".into()),
                branch: None,
                type_: None,
            },
        );
        cfg.projects.insert(
            "local".into(),
            ProjectEntry {
                path: "local".into(),
                url: None,
                branch: None,
                type_: None,
            },
        );
        update_workspace_gitignore(dir.path(), &cfg).unwrap();
        let text = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
        assert!(text.contains(BEGIN_MARKER));
        assert!(text.contains(".odm/cache/"));
        assert!(text.contains("projects/alpha/"));
        assert!(!text.contains("local/"));
    }

    #[test]
    fn rewrite_idempotent_preserves_outside() {
        let dir = tempdir().unwrap();
        let gi = dir.path().join(".gitignore");
        fs::write(&gi, "user-line\n").unwrap();
        let cfg = WorkspaceConfig::default();
        update_workspace_gitignore(dir.path(), &cfg).unwrap();
        update_workspace_gitignore(dir.path(), &cfg).unwrap();
        let text = fs::read_to_string(&gi).unwrap();
        assert_eq!(text.matches(BEGIN_MARKER).count(), 1);
        assert!(text.starts_with("user-line\n"));
    }

    #[test]
    fn disabled_noop() {
        let dir = tempdir().unwrap();
        let mut cfg = WorkspaceConfig::default();
        cfg.manage_gitignore = Some(false);
        update_workspace_gitignore(dir.path(), &cfg).unwrap();
        assert!(!dir.path().join(".gitignore").exists());
    }
}

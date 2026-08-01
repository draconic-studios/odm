use std::path::{Path, PathBuf};

use regex::Regex;
use serde_json::Value;

/// Stable note identity within one store.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoteId(pub String);

impl NoteId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One Markdown note in a vault.
#[derive(Debug, Clone)]
pub struct NoteDoc {
    pub id: NoteId,
    pub path: PathBuf,
    /// Path relative to vault root, forward slashes.
    pub rel_path: String,
    pub title: Option<String>,
    pub body: String,
    pub frontmatter: Option<Value>,
    pub wikilinks: Vec<String>,
}

/// Parse YAML frontmatter + body from Markdown text.
pub fn parse_markdown(rel_path: &str, abs: &Path, text: &str) -> NoteDoc {
    let (fm_raw, body) = split_frontmatter(text);
    let frontmatter = fm_raw.and_then(|y| serde_yaml::from_str::<Value>(y).ok());
    let id = id_from_frontmatter(&frontmatter).unwrap_or_else(|| path_id(rel_path));
    let title = title_from(&frontmatter, &body, rel_path);
    let wikilinks = parse_wikilinks(&body);
    NoteDoc {
        id: NoteId(id),
        path: abs.to_path_buf(),
        rel_path: rel_path.to_string(),
        title,
        body,
        frontmatter,
        wikilinks,
    }
}

/// Extract `[[target]]` and `[[target|alias]]` targets (Obsidian-style).
pub fn parse_wikilinks(body: &str) -> Vec<String> {
    // Avoid code fences roughly by not matching across ][
    let re = Regex::new(r"\[\[([^\]|#]+)(?:[|#][^\]]*)?\]\]").expect("wikilink regex");
    let mut out = Vec::new();
    for cap in re.captures_iter(body) {
        let t = cap[1].trim();
        if !t.is_empty() && !out.iter().any(|x: &String| x == t) {
            out.push(t.to_string());
        }
    }
    out
}

fn split_frontmatter(text: &str) -> (Option<&str>, String) {
    let t = text;
    if !t.starts_with("---") {
        return (None, t.to_string());
    }
    let rest = &t[3..];
    let rest = rest.strip_prefix('\n').unwrap_or(rest);
    if let Some(end) = rest.find("\n---") {
        let yaml = &rest[..end];
        let after = &rest[end + 4..];
        let body = after.strip_prefix('\n').unwrap_or(after).to_string();
        return (Some(yaml), body);
    }
    (None, t.to_string())
}

fn id_from_frontmatter(fm: &Option<Value>) -> Option<String> {
    let obj = fm.as_ref()?.as_object()?;
    if let Some(v) = obj.get("id") {
        if let Some(s) = v.as_str() {
            if !s.is_empty() {
                return Some(s.to_string());
            }
        }
    }
    None
}

fn title_from(fm: &Option<Value>, body: &str, rel: &str) -> Option<String> {
    if let Some(obj) = fm.as_ref().and_then(|v| v.as_object()) {
        if let Some(s) = obj.get("title").and_then(|v| v.as_str()) {
            if !s.is_empty() {
                return Some(s.to_string());
            }
        }
    }
    for line in body.lines() {
        let t = line.trim();
        if let Some(h) = t.strip_prefix("# ") {
            let h = h.trim();
            if !h.is_empty() {
                return Some(h.to_string());
            }
        }
    }
    Path::new(rel)
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
}

fn path_id(rel: &str) -> String {
    let no_ext = rel.strip_suffix(".md").unwrap_or(rel);
    no_ext.replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn frontmatter_id_and_wikilinks() {
        let text = r#"---
id: n1
title: Hello
---
See [[Other Note]] and [[foo|bar]].
"#;
        let n = parse_markdown("a/b.md", &PathBuf::from("/x/a/b.md"), text);
        assert_eq!(n.id.as_str(), "n1");
        assert_eq!(n.title.as_deref(), Some("Hello"));
        assert_eq!(n.wikilinks, vec!["Other Note", "foo"]);
    }

    #[test]
    fn path_id_fallback() {
        let n = parse_markdown("notes/hi.md", &PathBuf::from("/v/notes/hi.md"), "hi\n");
        assert_eq!(n.id.as_str(), "notes/hi");
    }
}

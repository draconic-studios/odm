//! Local Generator materialize — copy a template directory under the Workspace.

use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{GeneratorDef, Workspace};
use crate::error::OdmError;
use crate::paths::resolve_under_root;

/// Result of a successful local generate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateOutcome {
    /// Number of files (and symlinks) written.
    pub copied: u32,
    /// Absolute destination path under the Workspace root.
    pub dest: PathBuf,
}

/// Resolve a Generator by name from the loaded Workspace.
pub fn generator<'a>(ws: &'a Workspace, name: &str) -> Result<&'a GeneratorDef, OdmError> {
    ws.generators
        .get(name)
        .ok_or_else(|| OdmError::usage(format!("unknown generator '{name}'")))
}

/// Materialize a local `template` directory into `dest_rel` under the Workspace root.
///
/// - Prefers `template` when both `template` and `url` are set.
/// - Url-only generators return a usage error (remote deferred).
/// - Without `force`, fails if dest exists as a file or non-empty directory.
/// - Empty dest directory is allowed without `force`.
/// - With `force`, overwrites files in place; does not delete unrelated extras.
pub fn generate_local(
    ws: &Workspace,
    name: &str,
    dest_rel: &str,
    force: bool,
) -> Result<GenerateOutcome, OdmError> {
    let def = generator(ws, name)?;

    let template_rel = def
        .template
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    let Some(template_rel) = template_rel else {
        return Err(OdmError::usage(format!(
            "generator '{name}' has no local template (remote generators deferred)"
        )));
    };

    let template_path = resolve_under_root(&ws.root, template_rel)?;
    if !template_path.exists() {
        return Err(OdmError::operation(format!(
            "generator '{name}' template does not exist: {template_rel}"
        )));
    }
    if !template_path.is_dir() {
        return Err(OdmError::operation(format!(
            "generator '{name}' template is not a directory: {template_rel}"
        )));
    }

    let dest = resolve_under_root(&ws.root, dest_rel)?;

    if dest.exists() {
        let meta = fs::symlink_metadata(&dest).map_err(|e| {
            OdmError::operation(format!("failed to stat {}: {e}", dest.display()))
        })?;
        if !meta.is_dir() {
            return Err(OdmError::operation(format!(
                "destination exists and is not a directory: {dest_rel}"
            )));
        }
        if !force && !is_dir_empty(&dest)? {
            return Err(OdmError::operation(format!(
                "destination is not empty: {dest_rel} (use --force)"
            )));
        }
    } else if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            OdmError::operation(format!(
                "failed to create parent of {}: {e}",
                dest.display()
            ))
        })?;
        fs::create_dir(&dest).map_err(|e| {
            OdmError::operation(format!("failed to create {}: {e}", dest.display()))
        })?;
    } else {
        fs::create_dir_all(&dest).map_err(|e| {
            OdmError::operation(format!("failed to create {}: {e}", dest.display()))
        })?;
    }

    let copied = copy_tree(&template_path, &dest)?;
    Ok(GenerateOutcome { copied, dest })
}

fn is_dir_empty(path: &Path) -> Result<bool, OdmError> {
    let mut entries = fs::read_dir(path).map_err(|e| {
        OdmError::operation(format!("failed to read {}: {e}", path.display()))
    })?;
    Ok(entries.next().is_none())
}

/// Recursively copy `src` directory contents into `dst`. Counts files and symlinks.
fn copy_tree(src: &Path, dst: &Path) -> Result<u32, OdmError> {
    let mut copied = 0u32;
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
            copied += copy_tree(&from, &to)?;
        } else if ft.is_symlink() {
            copy_symlink(&from, &to)?;
            copied += 1;
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
            copied += 1;
        }
    }
    Ok(copied)
}

fn copy_symlink(from: &Path, to: &Path) -> Result<(), OdmError> {
    let target = fs::read_link(from).map_err(|e| {
        OdmError::operation(format!("failed to read symlink {}: {e}", from.display()))
    })?;
    // Overwrite existing path when force allowed us to write into a non-empty dest.
    if to.exists() || to.symlink_metadata().is_ok() {
        let meta = fs::symlink_metadata(to).map_err(|e| {
            OdmError::operation(format!("failed to stat {}: {e}", to.display()))
        })?;
        if meta.is_dir() {
            fs::remove_dir_all(to).map_err(|e| {
                OdmError::operation(format!("failed to remove {}: {e}", to.display()))
            })?;
        } else {
            fs::remove_file(to).map_err(|e| {
                OdmError::operation(format!("failed to remove {}: {e}", to.display()))
            })?;
        }
    }
    symlink_at(&target, to).map_err(|e| {
        OdmError::operation(format!(
            "failed to create symlink {} -> {}: {e}",
            to.display(),
            target.display()
        ))
    })
}

#[cfg(unix)]
fn symlink_at(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(not(unix))]
fn symlink_at(target: &Path, link: &Path) -> std::io::Result<()> {
    // Best-effort on non-unix; surface failure as operation error to caller.
    std::os::windows::fs::symlink_file(target, link)
        .or_else(|_| std::os::windows::fs::symlink_dir(target, link))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::io::Write;

    use crate::config::WorkspaceConfig;
    use crate::error::exit_code;

    fn ws_with(
        root: PathBuf,
        generators: BTreeMap<String, GeneratorDef>,
    ) -> Workspace {
        Workspace {
            root,
            config: WorkspaceConfig::default(),
            actions: BTreeMap::new(),
            generators,
        }
    }

    fn gen_template(template: &str) -> GeneratorDef {
        GeneratorDef {
            template: Some(template.into()),
            url: None,
        }
    }

    fn setup_template(root: &Path, rel: &str, files: &[(&str, &str)]) -> PathBuf {
        let tpl = root.join(rel);
        fs::create_dir_all(&tpl).unwrap();
        for (name, body) in files {
            let p = tpl.join(name);
            if let Some(parent) = p.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            let mut f = fs::File::create(&p).unwrap();
            write!(f, "{body}").unwrap();
        }
        tpl
    }

    #[test]
    fn happy_copy() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        setup_template(root, "templates/pkg", &[("README.md", "hello"), (".keep", "")]);

        let mut gens = BTreeMap::new();
        gens.insert("pkg".into(), gen_template("templates/pkg"));
        let ws = ws_with(root.to_path_buf(), gens);

        let out = generate_local(&ws, "pkg", "out/pkg", false).unwrap();
        assert_eq!(out.copied, 2);
        assert_eq!(out.dest, root.join("out/pkg"));
        assert_eq!(
            fs::read_to_string(root.join("out/pkg/README.md")).unwrap(),
            "hello"
        );
        assert!(root.join("out/pkg/.keep").is_file());
    }

    #[test]
    fn nested_paths() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        setup_template(
            root,
            "t",
            &[("a/b/c.txt", "deep"), ("a/top.txt", "top")],
        );

        let mut gens = BTreeMap::new();
        gens.insert("n".into(), gen_template("t"));
        let ws = ws_with(root.to_path_buf(), gens);

        let out = generate_local(&ws, "n", "dest/nested", false).unwrap();
        assert_eq!(out.copied, 2);
        assert_eq!(
            fs::read_to_string(root.join("dest/nested/a/b/c.txt")).unwrap(),
            "deep"
        );
        assert_eq!(
            fs::read_to_string(root.join("dest/nested/a/top.txt")).unwrap(),
            "top"
        );
    }

    #[test]
    fn force_overwrite() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        setup_template(root, "t", &[("f.txt", "new"), ("only_tpl.txt", "tpl")]);

        let mut gens = BTreeMap::new();
        gens.insert("g".into(), gen_template("t"));
        let ws = ws_with(root.to_path_buf(), gens);

        fs::create_dir_all(root.join("out")).unwrap();
        fs::write(root.join("out/f.txt"), "old").unwrap();
        fs::write(root.join("out/extra.txt"), "keep").unwrap();

        let err = generate_local(&ws, "g", "out", false).unwrap_err();
        assert!(err.to_string().contains("not empty"));
        assert_eq!(exit_code(&err), 3);

        let out = generate_local(&ws, "g", "out", true).unwrap();
        assert_eq!(out.copied, 2);
        assert_eq!(fs::read_to_string(root.join("out/f.txt")).unwrap(), "new");
        assert_eq!(
            fs::read_to_string(root.join("out/only_tpl.txt")).unwrap(),
            "tpl"
        );
        // Unrelated extra file preserved.
        assert_eq!(
            fs::read_to_string(root.join("out/extra.txt")).unwrap(),
            "keep"
        );
    }

    #[test]
    fn empty_dest_dir_ok_without_force() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        setup_template(root, "t", &[("a.txt", "x")]);
        fs::create_dir_all(root.join("empty")).unwrap();

        let mut gens = BTreeMap::new();
        gens.insert("g".into(), gen_template("t"));
        let ws = ws_with(root.to_path_buf(), gens);

        let out = generate_local(&ws, "g", "empty", false).unwrap();
        assert_eq!(out.copied, 1);
        assert_eq!(fs::read_to_string(root.join("empty/a.txt")).unwrap(), "x");
    }

    #[test]
    fn reject_dest_file() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        setup_template(root, "t", &[("a.txt", "x")]);
        fs::write(root.join("file"), "nope").unwrap();

        let mut gens = BTreeMap::new();
        gens.insert("g".into(), gen_template("t"));
        let ws = ws_with(root.to_path_buf(), gens);

        let err = generate_local(&ws, "g", "file", true).unwrap_err();
        assert!(err.to_string().contains("not a directory"));
    }

    #[test]
    fn reject_escape() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        setup_template(root, "t", &[("a.txt", "x")]);

        let mut gens = BTreeMap::new();
        gens.insert("g".into(), gen_template("t"));
        let ws = ws_with(root.to_path_buf(), gens);

        let err = generate_local(&ws, "g", "../outside", false).unwrap_err();
        assert!(err.to_string().contains("escape"));
        assert_eq!(exit_code(&err), 2);

        // Template path escape also rejected.
        let mut gens2 = BTreeMap::new();
        gens2.insert(
            "bad".into(),
            GeneratorDef {
                template: Some("../outside".into()),
                url: None,
            },
        );
        let ws2 = ws_with(root.to_path_buf(), gens2);
        let err2 = generate_local(&ws2, "bad", "out", false).unwrap_err();
        assert!(err2.to_string().contains("escape"));
    }

    #[test]
    fn missing_template() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();

        let mut gens = BTreeMap::new();
        gens.insert("g".into(), gen_template("missing/tpl"));
        let ws = ws_with(root.to_path_buf(), gens);

        let err = generate_local(&ws, "g", "out", false).unwrap_err();
        assert!(err.to_string().contains("does not exist"));
        assert_eq!(exit_code(&err), 3);
    }

    #[test]
    fn unknown_name() {
        let dir = tempfile::tempdir().unwrap();
        let ws = ws_with(dir.path().to_path_buf(), BTreeMap::new());
        let err = generate_local(&ws, "nope", "out", false).unwrap_err();
        assert!(err.to_string().contains("unknown generator"));
        assert_eq!(exit_code(&err), 1);
        assert!(matches!(err, OdmError::Usage(_)));
    }

    #[test]
    fn url_only_generator_error() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();

        let mut gens = BTreeMap::new();
        gens.insert(
            "remote".into(),
            GeneratorDef {
                template: None,
                url: Some("https://example.com/gen.git".into()),
            },
        );
        let ws = ws_with(root.to_path_buf(), gens);

        let err = generate_local(&ws, "remote", "out", false).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("remote") || msg.contains("deferred") || msg.contains("template"));
        assert!(msg.to_lowercase().contains("remote") || msg.contains("deferred"));
        assert_eq!(exit_code(&err), 1);
    }

    #[test]
    fn prefer_template_when_both_set() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        setup_template(root, "local", &[("ok.txt", "yes")]);

        let mut gens = BTreeMap::new();
        gens.insert(
            "both".into(),
            GeneratorDef {
                template: Some("local".into()),
                url: Some("https://example.com/gen.git".into()),
            },
        );
        let ws = ws_with(root.to_path_buf(), gens);

        let out = generate_local(&ws, "both", "dest", false).unwrap();
        assert_eq!(out.copied, 1);
        assert_eq!(fs::read_to_string(root.join("dest/ok.txt")).unwrap(), "yes");
    }

    #[test]
    fn empty_template_dir_success() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("empty_tpl")).unwrap();

        let mut gens = BTreeMap::new();
        gens.insert("e".into(), gen_template("empty_tpl"));
        let ws = ws_with(root.to_path_buf(), gens);

        let out = generate_local(&ws, "e", "out", false).unwrap();
        assert_eq!(out.copied, 0);
        assert!(root.join("out").is_dir());
    }

    #[test]
    fn generator_lookup() {
        let dir = tempfile::tempdir().unwrap();
        let mut gens = BTreeMap::new();
        gens.insert("x".into(), gen_template("t"));
        let ws = ws_with(dir.path().to_path_buf(), gens);
        assert_eq!(
            generator(&ws, "x").unwrap().template.as_deref(),
            Some("t")
        );
        assert!(generator(&ws, "y").is_err());
    }

    #[cfg(unix)]
    #[test]
    fn copies_symlink_as_symlink() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let tpl = root.join("t");
        fs::create_dir_all(&tpl).unwrap();
        fs::write(tpl.join("target.txt"), "data").unwrap();
        std::os::unix::fs::symlink("target.txt", tpl.join("link.txt")).unwrap();

        let mut gens = BTreeMap::new();
        gens.insert("s".into(), gen_template("t"));
        let ws = ws_with(root.to_path_buf(), gens);

        let out = generate_local(&ws, "s", "out", false).unwrap();
        assert_eq!(out.copied, 2);
        let link = root.join("out/link.txt");
        assert!(link.symlink_metadata().unwrap().file_type().is_symlink());
        assert_eq!(fs::read_link(&link).unwrap(), PathBuf::from("target.txt"));
        assert_eq!(fs::read_to_string(&link).unwrap(), "data");
    }
}

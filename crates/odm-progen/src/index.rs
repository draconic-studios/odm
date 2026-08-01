use std::fs;
use std::path::{Path, PathBuf};

use odm_core::{odm_dir, OdmError, Workspace};
use rusqlite::{params, Connection};

use crate::scope::ScopedProgen;
use crate::vault::walk_notes;

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS meta (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS notes (
  id TEXT PRIMARY KEY,
  rel_path TEXT NOT NULL,
  title TEXT NOT NULL,
  body TEXT NOT NULL
);
CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
  id UNINDEXED,
  title,
  body
);
CREATE TABLE IF NOT EXISTS links (
  src_id TEXT NOT NULL,
  target TEXT NOT NULL
);
"#;

#[derive(Debug, Clone)]
pub struct IndexStats {
    pub progen: String,
    pub notes: usize,
    pub links: usize,
}

/// `.odm/progen/<name>/`
pub fn index_dir(ws_root: &Path, progen_name: &str) -> PathBuf {
    odm_dir(ws_root).join("progen").join(progen_name)
}

fn index_db_path(ws_root: &Path, progen_name: &str) -> PathBuf {
    index_dir(ws_root, progen_name).join("index.db")
}

/// Rebuild disposable index for one Progen from vault files.
pub fn reindex_progen(ws: &Workspace, sp: &ScopedProgen) -> Result<IndexStats, OdmError> {
    let notes = walk_notes(&sp.path)?;
    let dir = index_dir(&ws.root, &sp.name);
    fs::create_dir_all(&dir).map_err(|e| {
        OdmError::operation(format!("create index dir {}: {e}", dir.display()))
    })?;
    let db = index_db_path(&ws.root, &sp.name);
    if db.exists() {
        let _ = fs::remove_file(&db);
    }

    let conn = Connection::open(&db)
        .map_err(|e| OdmError::operation(format!("open index: {e}")))?;
    conn.execute_batch(SCHEMA)
        .map_err(|e| OdmError::operation(format!("index schema: {e}")))?;

    let mut link_count = 0usize;
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| OdmError::operation(e.to_string()))?;
    {
        let mut ins = tx
            .prepare(
                "INSERT INTO notes (id, rel_path, title, body) VALUES (?1, ?2, ?3, ?4)",
            )
            .map_err(|e| OdmError::operation(e.to_string()))?;
        let mut ins_fts = tx
            .prepare("INSERT INTO notes_fts (id, title, body) VALUES (?1, ?2, ?3)")
            .map_err(|e| OdmError::operation(e.to_string()))?;
        let mut ins_link = tx
            .prepare("INSERT INTO links (src_id, target) VALUES (?1, ?2)")
            .map_err(|e| OdmError::operation(e.to_string()))?;

        for n in &notes {
            let title = n.title.as_deref().unwrap_or("");
            ins.execute(params![n.id.as_str(), n.rel_path, title, n.body])
                .map_err(|e| OdmError::operation(format!("index insert: {e}")))?;
            ins_fts
                .execute(params![n.id.as_str(), title, n.body])
                .map_err(|e| OdmError::operation(format!("fts insert: {e}")))?;
            for t in &n.wikilinks {
                ins_link
                    .execute(params![n.id.as_str(), t])
                    .map_err(|e| OdmError::operation(e.to_string()))?;
                link_count += 1;
            }
        }
    }
    tx.execute(
        "INSERT OR REPLACE INTO meta (key, value) VALUES ('schema', '1')",
        [],
    )
    .map_err(|e| OdmError::operation(e.to_string()))?;
    tx.commit()
        .map_err(|e| OdmError::operation(e.to_string()))?;

    Ok(IndexStats {
        progen: sp.name.clone(),
        notes: notes.len(),
        links: link_count,
    })
}

pub fn open_index(ws_root: &Path, progen_name: &str) -> Result<Connection, OdmError> {
    let db = index_db_path(ws_root, progen_name);
    if !db.exists() {
        return Err(OdmError::operation(format!(
            "index missing for progen '{progen_name}' (run `odm progen reindex`)"
        )));
    }
    Connection::open(&db).map_err(|e| OdmError::operation(format!("open index: {e}")))
}

/// Ensure index exists; rebuild if missing.
pub fn ensure_index(ws: &Workspace, sp: &ScopedProgen) -> Result<(), OdmError> {
    let db = index_db_path(&ws.root, &sp.name);
    if !db.exists() {
        reindex_progen(ws, sp)?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct IndexedNote {
    pub id: String,
    pub rel_path: String,
    pub title: String,
    pub body: String,
}

pub fn load_all_notes(conn: &Connection) -> Result<Vec<IndexedNote>, OdmError> {
    let mut stmt = conn
        .prepare("SELECT id, rel_path, title, body FROM notes ORDER BY rel_path")
        .map_err(|e| OdmError::operation(e.to_string()))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(IndexedNote {
                id: row.get(0)?,
                rel_path: row.get(1)?,
                title: row.get(2)?,
                body: row.get(3)?,
            })
        })
        .map_err(|e| OdmError::operation(e.to_string()))?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| OdmError::operation(e.to_string()))?);
    }
    Ok(out)
}

pub fn search_fts(
    conn: &Connection,
    query: &str,
    limit: usize,
) -> Result<Vec<IndexedNote>, OdmError> {
    if query.trim().is_empty() {
        let mut all = load_all_notes(conn)?;
        all.truncate(limit);
        return Ok(all);
    }
    let q = query.replace('"', "\"\"");
    let mut stmt = conn
        .prepare(
            "SELECT id, title, body FROM notes_fts WHERE notes_fts MATCH ?1 LIMIT ?2",
        )
        .map_err(|e| OdmError::operation(e.to_string()))?;
    let fts_rows: Result<Vec<(String, String, String)>, _> = stmt
        .query_map(params![q, limit as i64], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(|e| OdmError::operation(e.to_string()))?
        .collect();
    let fts_rows = fts_rows.map_err(|e| OdmError::operation(e.to_string()))?;

    let mut out = Vec::new();
    for (id, _title, _body) in fts_rows {
        if let Some(n) = get_indexed(conn, &id)? {
            out.push(n);
        }
    }
    Ok(out)
}

pub fn get_indexed(conn: &Connection, id: &str) -> Result<Option<IndexedNote>, OdmError> {
    let mut stmt = conn
        .prepare("SELECT id, rel_path, title, body FROM notes WHERE id = ?1")
        .map_err(|e| OdmError::operation(e.to_string()))?;
    let mut rows = stmt
        .query_map(params![id], |row| {
            Ok(IndexedNote {
                id: row.get(0)?,
                rel_path: row.get(1)?,
                title: row.get(2)?,
                body: row.get(3)?,
            })
        })
        .map_err(|e| OdmError::operation(e.to_string()))?;
    if let Some(r) = rows.next() {
        return Ok(Some(r.map_err(|e| OdmError::operation(e.to_string()))?));
    }

    let md = if id.ends_with(".md") {
        id.to_string()
    } else {
        format!("{id}.md")
    };
    let mut stmt = conn
        .prepare(
            "SELECT id, rel_path, title, body FROM notes
             WHERE rel_path = ?1 OR rel_path = ?2 OR title = ?3
             LIMIT 1",
        )
        .map_err(|e| OdmError::operation(e.to_string()))?;
    let mut rows = stmt
        .query_map(params![md.as_str(), id, id], |row| {
            Ok(IndexedNote {
                id: row.get(0)?,
                rel_path: row.get(1)?,
                title: row.get(2)?,
                body: row.get(3)?,
            })
        })
        .map_err(|e| OdmError::operation(e.to_string()))?;
    match rows.next() {
        Some(r) => Ok(Some(r.map_err(|e| OdmError::operation(e.to_string()))?)),
        None => Ok(None),
    }
}

pub fn outgoing_links(conn: &Connection, src_id: &str) -> Result<Vec<String>, OdmError> {
    let mut stmt = conn
        .prepare("SELECT target FROM links WHERE src_id = ?1 ORDER BY target")
        .map_err(|e| OdmError::operation(e.to_string()))?;
    let rows = stmt
        .query_map(params![src_id], |row| row.get(0))
        .map_err(|e| OdmError::operation(e.to_string()))?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| OdmError::operation(e.to_string()))?);
    }
    Ok(out)
}

pub fn resolve_link_target(conn: &Connection, target: &str) -> Result<Option<IndexedNote>, OdmError> {
    get_indexed(conn, target)
}

pub fn backlinks_for(conn: &Connection, target: &str) -> Result<Vec<String>, OdmError> {
    let mut stmt = conn
        .prepare(
            "SELECT DISTINCT src_id FROM links WHERE target = ?1 ORDER BY src_id",
        )
        .map_err(|e| OdmError::operation(e.to_string()))?;
    let rows = stmt
        .query_map(params![target], |row| row.get(0))
        .map_err(|e| OdmError::operation(e.to_string()))?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| OdmError::operation(e.to_string()))?);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::ensure_vault;
    use odm_core::{ProgenEntry, WorkspaceConfig};
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    #[test]
    fn reindex_and_search() {
        let d = tempdir().unwrap();
        let root = d.path();
        let vault = root.join("mem");
        ensure_vault(&vault).unwrap();
        fs::write(
            vault.join("alpha.md"),
            "---\nid: a1\ntitle: Alpha\n---\nUniqueZebra word and [[Beta]].\n",
        )
        .unwrap();
        fs::write(
            vault.join("beta.md"),
            "---\nid: b1\ntitle: Beta\n---\nOther.\n",
        )
        .unwrap();

        let mut progens = BTreeMap::new();
        progens.insert(
            "main".into(),
            ProgenEntry {
                path: "mem".into(),
                url: None,
                branch: None,
            },
        );
        fs::create_dir_all(root.join(".odm")).unwrap();
        let ws = Workspace {
            root: root.to_path_buf(),
            config: WorkspaceConfig {
                progens,
                ..Default::default()
            },
            actions: BTreeMap::new(),
            generators: BTreeMap::new(),
        };
        let sp = ScopedProgen {
            name: "main".into(),
            path: vault,
        };
        let stats = reindex_progen(&ws, &sp).unwrap();
        assert_eq!(stats.notes, 3);
        let conn = open_index(root, "main").unwrap();
        let hits = search_fts(&conn, "UniqueZebra", 10).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].id, "a1");
    }
}

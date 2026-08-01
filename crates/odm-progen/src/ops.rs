use odm_core::{OdmError, Workspace};
use serde::Serialize;

use crate::index::{
    ensure_index, get_indexed, load_all_notes, open_index, outgoing_links, resolve_link_target,
    search_fts, IndexedNote,
};
use crate::scope::{resolve_read_scope, resolve_single_read, ScopedProgen};
use crate::vault::vault_info;

#[derive(Debug, Clone, Serialize)]
pub struct FindHit {
    pub progen: String,
    pub id: String,
    pub path: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LsHit {
    pub progen: String,
    pub id: String,
    pub path: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetResult {
    pub progen: String,
    pub id: String,
    pub path: String,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContextHit {
    pub progen: String,
    pub anchor: GetResult,
    pub outgoing: Vec<LsHit>,
    pub incoming: Vec<LsHit>,
}

/// Federated find across scoped Progens.
pub fn find_notes(
    ws: &Workspace,
    query: &str,
    progens: &[String],
    groups: &[String],
    limit_per: usize,
) -> Result<Vec<FindHit>, OdmError> {
    let scope = resolve_read_scope(ws, progens, groups)?;
    let mut hits = Vec::new();
    for sp in &scope {
        ensure_index(ws, sp)?;
        let conn = open_index(&ws.root, &sp.name)?;
        for n in search_fts(&conn, query, limit_per)? {
            hits.push(FindHit {
                progen: sp.name.clone(),
                id: n.id.clone(),
                path: n.rel_path.clone(),
                title: n.title.clone(),
                snippet: snippet(&n.body, query),
            });
        }
    }
    Ok(hits)
}

/// List notes in one Progen (single-root).
pub fn list_notes(ws: &Workspace, progen: Option<&str>) -> Result<Vec<LsHit>, OdmError> {
    let sp = resolve_single_read(ws, progen)?;
    ensure_index(ws, &sp)?;
    let conn = open_index(&ws.root, &sp.name)?;
    Ok(load_all_notes(&conn)?
        .into_iter()
        .map(|n| to_ls(&sp, &n))
        .collect())
}

/// Get one note. Id may be `name:id` or bare id with optional --progen.
pub fn get_note(
    ws: &Workspace,
    id_arg: &str,
    progen: Option<&str>,
) -> Result<GetResult, OdmError> {
    let (sp, id) = resolve_id(ws, id_arg, progen)?;
    ensure_index(ws, &sp)?;
    let conn = open_index(&ws.root, &sp.name)?;
    let n = get_indexed(&conn, &id)?.ok_or_else(|| {
        OdmError::not_found(format!("no such note: {id} in progen '{}'", sp.name))
    })?;
    Ok(to_get(&sp, &n))
}

/// Note body only (single-root).
pub fn note_body(
    ws: &Workspace,
    id_arg: &str,
    progen: Option<&str>,
) -> Result<GetResult, OdmError> {
    get_note(ws, id_arg, progen)
}

/// Sorted note paths in one Progen (single-root).
pub fn note_tree(ws: &Workspace, progen: Option<&str>) -> Result<Vec<String>, OdmError> {
    let hits = list_notes(ws, progen)?;
    let mut paths: Vec<String> = hits.into_iter().map(|h| h.path).collect();
    paths.sort();
    Ok(paths)
}

/// Notes that wikilink to id (single-root).
pub fn note_backlinks(
    ws: &Workspace,
    id_arg: &str,
    progen: Option<&str>,
) -> Result<Vec<LsHit>, OdmError> {
    Ok(context_notes(ws, id_arg, progen)?.incoming)
}

/// In-store wikilink neighborhood (no cross-store walk).
pub fn context_notes(
    ws: &Workspace,
    id_arg: &str,
    progen: Option<&str>,
) -> Result<ContextHit, OdmError> {
    let (sp, id) = resolve_id(ws, id_arg, progen)?;
    ensure_index(ws, &sp)?;
    let conn = open_index(&ws.root, &sp.name)?;
    let n = get_indexed(&conn, &id)?.ok_or_else(|| {
        OdmError::not_found(format!("no such note: {id} in progen '{}'", sp.name))
    })?;
    let anchor = to_get(&sp, &n);

    let mut outgoing = Vec::new();
    for t in outgoing_links(&conn, &n.id)? {
        if let Some(tn) = resolve_link_target(&conn, &t)? {
            outgoing.push(to_ls(&sp, &tn));
        } else {
            outgoing.push(LsHit {
                progen: sp.name.clone(),
                id: t.clone(),
                path: String::new(),
                title: t,
            });
        }
    }

    // Incoming: notes that wikilink to this note's id, title, or path stem
    let mut incoming_ids = std::collections::BTreeSet::new();
    let stem = path_stem(&n.rel_path);
    for key in [&n.id, &n.title, &stem] {
        if key.is_empty() {
            continue;
        }
        for src in crate::index::backlinks_for(&conn, key)? {
            incoming_ids.insert(src);
        }
    }
    incoming_ids.remove(&n.id);
    let mut incoming = Vec::new();
    for src in incoming_ids {
        if let Some(sn) = get_indexed(&conn, &src)? {
            incoming.push(to_ls(&sp, &sn));
        }
    }

    Ok(ContextHit {
        progen: sp.name.clone(),
        anchor,
        outgoing,
        incoming,
    })
}

fn path_stem(rel: &str) -> String {
    rel.strip_suffix(".md").unwrap_or(rel).to_string()
}

fn resolve_id(
    ws: &Workspace,
    id_arg: &str,
    progen: Option<&str>,
) -> Result<(ScopedProgen, String), OdmError> {
    if let Some((name, id)) = id_arg.split_once(':') {
        if ws.config.progens.contains_key(name) {
            let sp = resolve_single_read(ws, Some(name))?;
            return Ok((sp, id.to_string()));
        }
    }
    let sp = resolve_single_read(ws, progen)?;
    Ok((sp, id_arg.to_string()))
}

fn to_ls(sp: &ScopedProgen, n: &IndexedNote) -> LsHit {
    LsHit {
        progen: sp.name.clone(),
        id: n.id.clone(),
        path: n.rel_path.clone(),
        title: n.title.clone(),
    }
}

fn to_get(sp: &ScopedProgen, n: &IndexedNote) -> GetResult {
    GetResult {
        progen: sp.name.clone(),
        id: n.id.clone(),
        path: n.rel_path.clone(),
        title: n.title.clone(),
        body: n.body.clone(),
    }
}

fn snippet(body: &str, query: &str) -> Option<String> {
    if query.is_empty() {
        return None;
    }
    let lower = body.to_lowercase();
    let q = query.to_lowercase();
    let idx = lower.find(&q)?;
    let start = idx.saturating_sub(40);
    let end = (idx + q.len() + 40).min(body.len());
    let mut s = body[start..end].replace('\n', " ");
    if start > 0 {
        s.insert_str(0, "…");
    }
    if end < body.len() {
        s.push('…');
    }
    Some(s)
}

pub fn format_find_human(hits: &[FindHit]) -> String {
    if hits.is_empty() {
        return "(no matches)\n".into();
    }
    let mut out = String::new();
    for h in hits {
        out.push_str(&format!(
            "{}\t{}\t{}\t{}\n",
            h.progen, h.id, h.path, h.title
        ));
    }
    out
}

pub fn format_ls_human(hits: &[LsHit]) -> String {
    if hits.is_empty() {
        return "(no notes)\n".into();
    }
    let mut out = String::new();
    for h in hits {
        out.push_str(&format!("{}\t{}\t{}\n", h.id, h.path, h.title));
    }
    out
}

pub fn format_get_human(g: &GetResult) -> String {
    format!(
        "---\nprogen: {}\nid: {}\npath: {}\ntitle: {}\n---\n{}",
        g.progen, g.id, g.path, g.title, g.body
    )
}

pub fn format_context_human(c: &ContextHit) -> String {
    let mut out = String::new();
    out.push_str(&format!("# context {}\n\n", c.anchor.id));
    out.push_str(&format_get_human(&c.anchor));
    out.push_str("\n\n## outgoing\n");
    if c.outgoing.is_empty() {
        out.push_str("(none)\n");
    } else {
        for h in &c.outgoing {
            out.push_str(&format!("- {} ({})\n", h.id, h.title));
        }
    }
    out.push_str("\n## incoming\n");
    if c.incoming.is_empty() {
        out.push_str("(none)\n");
    } else {
        for h in &c.incoming {
            out.push_str(&format!("- {} ({})\n", h.id, h.title));
        }
    }
    out
}

/// Store-side doctor checks for one or all progens.
pub fn doctor_progens(
    ws: &Workspace,
    progen: Option<&str>,
) -> Result<Vec<ProgenDoctorCheck>, OdmError> {
    let scope = if let Some(n) = progen {
        vec![resolve_single_read(ws, Some(n))?]
    } else if ws.config.progens.is_empty() {
        return Ok(vec![]);
    } else {
        resolve_read_scope(ws, &[], &[])?
    };
    let mut checks = Vec::new();
    for sp in scope {
        let info = vault_info(&sp)?;
        checks.push(ProgenDoctorCheck {
            progen: sp.name.clone(),
            id: "vault_path".into(),
            ok: info.on_disk,
            message: if info.on_disk {
                format!("vault present ({})", sp.path.display())
            } else {
                format!("vault missing: {}", sp.path.display())
            },
        });
        let idx = crate::index::index_dir(&ws.root, &sp.name).join("index.db");
        let idx_ok = idx.is_file();
        checks.push(ProgenDoctorCheck {
            progen: sp.name.clone(),
            id: "index".into(),
            ok: idx_ok,
            message: if idx_ok {
                format!("index present ({} notes)", info.note_count)
            } else {
                "index missing (run `odm progen reindex`)".into()
            },
        });
    }
    Ok(checks)
}

#[derive(Debug, Clone, Serialize)]
pub struct ProgenDoctorCheck {
    pub progen: String,
    pub id: String,
    pub ok: bool,
    pub message: String,
}

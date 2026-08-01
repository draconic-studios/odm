use odm_core::{OdmError, Workspace};
use serde::Serialize;

use crate::scope::{resolve_read_scope, resolve_write_progen};
use crate::store::{ContextHit, FindHit, GetResult, LsHit};
use crate::vault::vault_info;

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
        vec![resolve_write_progen(ws, Some(n))?]
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
        let idx = odm_core::progen_index_dir(&ws.root, &sp.name).join("index.db");
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

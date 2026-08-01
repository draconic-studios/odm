//! `odm find` — thin library entrypoint over progen find.

use odm_core::{OdmError, Workspace};
use odm_progen::{find_notes, FindHit};
use serde::Serialize;

/// `odm find --json` envelope.
#[derive(Debug, Clone, Serialize)]
pub struct FindDto {
    pub hits: Vec<FindHit>,
}

/// Library entrypoint: find notes across progen scope.
pub fn find_notes_dto(
    ws: &Workspace,
    query: &str,
    progens: &[String],
    groups: &[String],
    limit: usize,
) -> Result<FindDto, OdmError> {
    let hits = find_notes(ws, query, progens, groups, limit)?;
    Ok(FindDto { hits })
}

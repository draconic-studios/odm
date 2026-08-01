//! `odm status` — thin library entrypoint over core observation.

use odm_core::{build_status, OdmError, StatusSnapshot, Workspace};
use odm_git::Git;

/// Library-callable status snapshot (same shape as `odm status --json`).
pub fn status_snapshot<R: odm_git::CommandRunner>(
    git: &Git<R>,
    ws: &Workspace,
) -> Result<StatusSnapshot, OdmError> {
    build_status(git, ws)
}

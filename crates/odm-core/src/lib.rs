//! `odm-core` — Workspace config, pin, discovery, and path policy.

mod config;
mod discover;
mod doctor;
mod error;
mod gitignore;
mod init;
mod io;
mod lifecycle;
mod pin;
mod progen_lifecycle;
mod status;
mod url_match;

pub use config::{
    config_path, load_workspace, odm_dir, parse_config_yaml, pin_path, save_config,
    validate_and_load_bundles, ActionDef, ActionTask, GeneratorDef, ProjectEntry, ProgenEntry,
    Workspace, WorkspaceConfig,
};
pub use discover::discover_root;
pub use doctor::{
    format_doctor_human, run_doctor, CheckStatus, DoctorCheck, DoctorReport,
};
pub use error::{exit_code, OdmError};
pub use gitignore::{
    apply_managed_gitignore, desired_ancestor_lines, desired_block, desired_workspace_lines,
    resolve_under_root, update_workspace_gitignore, workspace_gitignore_has_drift, BEGIN_MARKER,
    END_MARKER,
};
pub use init::{init_workspace, InitOptions, InitResult};
pub use lifecycle::{
    abs_checkout, all_managed, entity_disk_info, materialize, pin_apply, pin_status, project_add,
    project_git, project_rm, resolve_clone_url, resolve_managed, sort_by_depth, sync_managed,
    EntityDiskInfo, ManagedEntity, MaterializeOutcome, PinApplyResult, PinStatusEntry,
    PinStatusReport, SyncResult,
};
pub use pin::{
    is_full_sha, load_pin, parse_pin_yaml, prune_pins, save_pin, PinEntry, PinFile,
};
pub use progen_lifecycle::{path_buf_to_rel, progen_add, progen_rm};
pub use status::{
    build_status, compute_pin_state, format_status_human, EntityStatus, PinState, StatusSnapshot,
};
pub use url_match::{normalize_git_url, urls_match, urls_match_with_root};

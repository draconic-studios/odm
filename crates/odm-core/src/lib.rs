//! `odm-core` — Workspace config, pin, discovery, and path policy.

mod config;
mod discover;
mod doctor;
mod error;
mod gitignore;
mod init;
mod io;
mod lifecycle;
mod observation;
mod paths;
mod pin;
mod progen_lifecycle;
mod status;
mod url_match;

pub use config::{
    load_workspace, parse_config_yaml, save_config, validate_and_load_bundles, ActionDef,
    ActionTask, GeneratorDef, ProjectEntry, ProgenEntry, Workspace, WorkspaceConfig,
};
pub use discover::discover_root;
pub use doctor::{
    format_doctor_human, run_doctor, CheckStatus, DoctorCheck, DoctorReport,
};
pub use error::{exit_code, OdmError};
pub use gitignore::{
    apply_managed_gitignore, desired_ancestor_lines, desired_block, desired_workspace_lines,
    update_workspace_gitignore, workspace_gitignore_has_drift, BEGIN_MARKER, END_MARKER,
};
pub use init::{init_workspace, InitOptions, InitResult};
pub use lifecycle::{
    all_managed, entity_disk_info, materialize, pin_apply, pin_status, project_add, project_git,
    project_rm, resolve_clone_url, resolve_managed, sort_by_depth, sync_managed, EntityDiskInfo,
    ManagedEntity, MaterializeOutcome, PinApplyResult, PinStatusEntry, PinStatusReport, SyncResult,
};
pub use paths::{
    abs_checkout, config_path, odm_dir, pin_path, progen_index_dir, resolve_under_root,
    worktree_slot_path,
};
pub use observation::{
    observe_entity, observe_workspace, EntityObservation, WorkspaceObservation,
};
pub use pin::{
    is_full_sha, load_pin, parse_pin_yaml, prune_pins, save_pin, PinEntry, PinFile,
};
pub use progen_lifecycle::{path_buf_to_rel, progen_add, progen_rm};
pub use status::{
    build_status, compute_pin_state, format_status_human, status_from_observation, EntityStatus,
    PinState, StatusSnapshot,
};
pub use url_match::{normalize_git_url, urls_match, urls_match_with_root};

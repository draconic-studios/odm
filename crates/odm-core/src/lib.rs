//! `odm-core` — Workspace config, pin, discovery, and path policy.

mod agent_pack;
mod checkout;
mod config;
mod discover;
mod doctor;
mod doctor_worktree;
mod error;
mod generate;
mod gitignore;
mod init;
mod io;
mod membership;
mod observation;
mod paths;
mod pin;
mod pin_maintain;
mod status;
mod url_match;
mod worktree;

pub use agent_pack::{pack_install, pack_link, pack_list, PackEntry, PackMode};
pub use checkout::{
    all_managed, materialize, resolve_clone_url, resolve_managed, sort_by_depth, sync_managed,
    ManagedEntity, MaterializeOutcome, SyncResult,
};
pub use config::{
    load_workspace, parse_config_yaml, save_config, validate_and_load_bundles, ActionDef,
    ActionTask, GeneratorDef, ProjectEntry, ProgenEntry, Workspace, WorkspaceConfig,
};
pub use discover::discover_root;
pub use doctor::{
    format_doctor_human, run_doctor, CheckStatus, DoctorCheck, DoctorReport,
};
pub use error::{exit_code, OdmError};
pub use generate::{generate_local, generator, GenerateOutcome};
pub use gitignore::{
    apply_managed_gitignore, desired_ancestor_lines, desired_block, desired_workspace_lines,
    update_workspace_gitignore, workspace_gitignore_has_drift, BEGIN_MARKER, END_MARKER,
};
pub use init::{init_workspace, InitOptions, InitResult};
pub use membership::{
    membership_add, membership_rm, path_buf_to_rel, progen_add, progen_rm, project_add,
    project_git, project_rm, MembershipEntry, MembershipKind,
};
pub use observation::{
    observe_entity, observe_workspace, EntityObservation, WorkspaceObservation,
};
pub use paths::{
    abs_checkout, agent_packs_path, config_path, odm_dir, pin_path, progen_index_dir,
    resolve_under_root, worktree_slot_path,
};
pub use pin::{
    is_full_sha, load_pin, parse_pin_yaml, prune_pins, save_pin, PinEntry, PinFile,
};
pub use pin_maintain::{
    pin_apply, pin_status, PinApplyResult, PinStatusEntry, PinStatusReport,
};
pub use status::{
    build_status, compute_pin_state, format_status_human, status_from_observation, EntityStatus,
    PinState, StatusSnapshot,
};
pub use url_match::{normalize_git_url, urls_match, urls_match_with_root};
pub use worktree::{
    validate_slot_name, worktree_add, worktree_list, worktree_prune, worktree_rm,
    WorktreeListOutcome, WorktreePruneOutcome, WorktreeSlotInfo, WorktreeSlotOutcome,
};

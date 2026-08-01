//! `odm-core` — Workspace config, pin, discovery, and path policy.

mod config;
mod discover;
mod error;
mod gitignore;
mod init;
mod io;
mod pin;
mod url_match;

pub use config::{
    config_path, load_workspace, odm_dir, parse_config_yaml, pin_path, save_config,
    validate_and_load_bundles, ActionDef, GeneratorDef, ProjectEntry, ProgenEntry, Workspace,
    WorkspaceConfig,
};
pub use discover::discover_root;
pub use error::{exit_code, OdmError};
pub use gitignore::{update_workspace_gitignore, BEGIN_MARKER, END_MARKER};
pub use init::{init_workspace, InitOptions, InitResult};
pub use pin::{
    is_full_sha, load_pin, parse_pin_yaml, prune_pins, save_pin, PinEntry, PinFile,
};
pub use url_match::{normalize_git_url, urls_match, urls_match_with_root};

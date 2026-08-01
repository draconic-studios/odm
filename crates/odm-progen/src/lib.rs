//! `odm-progen` — federation/scope + Obsidian-compatible vault store ops.

mod index;
mod note;
mod ops;
mod scope;
mod vault;

pub use index::{index_dir, reindex_progen, IndexStats};
pub use note::{parse_wikilinks, NoteDoc, NoteId};
pub use ops::{
    context_notes, doctor_progens, find_notes, format_context_human, format_find_human,
    format_get_human, format_ls_human, get_note, list_notes, note_backlinks, note_body, note_tree,
    ContextHit, FindHit, GetResult, LsHit, ProgenDoctorCheck,
};
pub use scope::{resolve_read_scope, resolve_single_read, resolve_write_progen, ScopedProgen};
pub use vault::{ensure_vault, vault_info, vault_path, VaultInfo};

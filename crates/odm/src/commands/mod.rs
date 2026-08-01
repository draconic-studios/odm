//! Library command entrypoints returning serializable DTOs.

mod agent_pack;
mod find;
mod generate;
mod materialize;
mod project;
mod progen;
mod run;
mod status;
mod worktree;

pub use agent_pack::{
    format_pack_install_human, format_pack_link_human, format_pack_list_human, pack_entry_dto,
    pack_list_dto, PackEntryDto, PackListDto,
};
pub use find::{find_notes_dto, FindDto};
pub use generate::{
    format_generate_run_human, format_generator_list_human, list_generators_dto, GenerateRunDto,
    GeneratorListDto, GeneratorListItem,
};
pub use materialize::{
    format_progen_add_human, format_project_add_human, materialize_json, materialize_json_opt,
    materialize_sync_human, MaterializeLabel,
};
pub use project::{
    format_project_info_human, format_project_list_human, list_projects, project_info,
    project_info_from, project_list_from, ProjectInfoDto, ProjectListDto, ProjectListItem,
};
pub use progen::{
    format_progen_info_human, format_progen_list_human, list_progens, progen_info, progen_list_from,
    ProgenInfoDto, ProgenListDto, ProgenListItem,
};
pub use run::{
    format_action_list_human, list_actions_dto, ActionListDto, ActionListItem, ActionTaskDto,
};
pub use status::status_snapshot;
pub use worktree::{
    format_worktree_add_human, format_worktree_list_human, format_worktree_prune_all_human,
    format_worktree_prune_human, format_worktree_rm_human, worktree_list_dto,
    worktree_prune_all_dto, worktree_prune_dto, worktree_slot_action_dto, WorktreeListDto,
    WorktreePruneAllDto, WorktreePruneDto, WorktreeSlotActionDto, WorktreeSlotDto,
};

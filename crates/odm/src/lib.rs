//! ODM application library — command DTOs and multi-crate composition.
//!
//! The binary is a thin adapter: parse argv, call these entrypoints, print, exit codes.

pub mod commands;

pub use commands::{
    find_notes_dto, format_progen_add_human, format_project_add_human, list_actions_dto,
    list_projects, list_progens, materialize_json, materialize_json_opt, materialize_sync_human,
    project_info, project_info_from, project_list_from, progen_info, progen_list_from,
    status_snapshot, ActionListDto, ActionListItem, ActionTaskDto, FindDto, MaterializeLabel,
    ProjectInfoDto, ProjectListDto, ProjectListItem, ProgenInfoDto, ProgenListDto, ProgenListItem,
};

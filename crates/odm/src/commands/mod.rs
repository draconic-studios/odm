//! Library command entrypoints returning serializable DTOs.

mod find;
mod materialize;
mod project;
mod progen;
mod run;
mod status;

pub use find::{find_notes_dto, FindDto};
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

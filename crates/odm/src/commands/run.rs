//! `odm run` (list) — Action list DTO.

use odm_actions::list_actions;
use odm_core::Workspace;
use serde::Serialize;

/// `odm run --json` (no action name) envelope.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ActionListDto {
    pub actions: Vec<ActionListItem>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ActionListItem {
    pub name: String,
    pub tasks: Vec<ActionTaskDto>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ActionTaskDto {
    pub run: String,
    pub dir: Option<String>,
}

/// Library entrypoint: list configured actions as a serializable DTO.
pub fn list_actions_dto(ws: &Workspace) -> ActionListDto {
    let actions = list_actions(ws)
        .into_iter()
        .map(|(name, def)| ActionListItem {
            name: name.to_string(),
            tasks: def
                .tasks
                .iter()
                .map(|t| ActionTaskDto {
                    run: t.run.clone(),
                    dir: t.dir.clone(),
                })
                .collect(),
        })
        .collect();
    ActionListDto { actions }
}

/// Human one-name-per-line list (beside DTO).
pub fn format_action_list_human(dto: &ActionListDto) -> String {
    if dto.actions.is_empty() {
        return "(no actions)\n".into();
    }
    let mut out = String::new();
    for a in &dto.actions {
        out.push_str(&a.name);
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use odm_core::{ActionDef, ActionTask, WorkspaceConfig};

    #[test]
    fn action_list_dto_shape() {
        let mut actions = BTreeMap::new();
        actions.insert(
            "hello".into(),
            ActionDef {
                tasks: vec![ActionTask {
                    run: "echo hello-desk".into(),
                    dir: None,
                }],
            },
        );
        let ws = Workspace {
            root: PathBuf::from("/tmp/ws"),
            config: WorkspaceConfig::default(),
            actions,
            generators: BTreeMap::new(),
        };
        let dto = list_actions_dto(&ws);
        let v = serde_json::to_value(&dto).unwrap();
        let a = &v["actions"][0];
        assert_eq!(a["name"], "hello");
        assert_eq!(a["tasks"][0]["run"], "echo hello-desk");
        assert!(a["tasks"][0].get("dir").is_some());
    }
}

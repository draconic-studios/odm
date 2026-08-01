//! `odm run` — Action list + run JSON DTOs.

use odm_actions::{list_actions, RunResult};
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

/// `odm run <action> --json` envelope. Streams are concatenated across tasks in run order.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ActionRunDto {
    pub action: String,
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
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

/// Build run JSON envelope: concatenate captured task streams in order (empty string when none).
pub fn action_run_dto(action: impl Into<String>, result: &RunResult) -> ActionRunDto {
    let stdout = result
        .tasks
        .iter()
        .filter_map(|t| t.stdout.as_deref())
        .collect();
    let stderr = result
        .tasks
        .iter()
        .filter_map(|t| t.stderr.as_deref())
        .collect();
    ActionRunDto {
        action: action.into(),
        exit_code: result.exit_code,
        stdout,
        stderr,
    }
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

    #[test]
    fn action_run_dto_concatenates_streams() {
        use odm_actions::{RunResult, TaskResult};
        let result = RunResult {
            exit_code: 0,
            tasks: vec![
                TaskResult {
                    exit_code: 0,
                    stdout: Some("a\n".into()),
                    stderr: Some("e1".into()),
                },
                TaskResult {
                    exit_code: 0,
                    stdout: Some("b\n".into()),
                    stderr: Some("e2".into()),
                },
            ],
        };
        let dto = action_run_dto("chain", &result);
        let v = serde_json::to_value(&dto).unwrap();
        assert_eq!(v["action"], "chain");
        assert_eq!(v["exitCode"], 0);
        assert_eq!(v["stdout"], "a\nb\n");
        assert_eq!(v["stderr"], "e1e2");
    }

    #[test]
    fn action_run_dto_empty_streams() {
        use odm_actions::{RunResult, TaskResult};
        let result = RunResult {
            exit_code: 7,
            tasks: vec![TaskResult {
                exit_code: 7,
                stdout: Some(String::new()),
                stderr: Some(String::new()),
            }],
        };
        let dto = action_run_dto("fail", &result);
        assert_eq!(dto.stdout, "");
        assert_eq!(dto.stderr, "");
        assert_eq!(dto.exit_code, 7);
    }
}

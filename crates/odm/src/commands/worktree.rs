//! `odm project worktree` DTOs and human formatters.

use odm_core::{WorktreeListOutcome, WorktreeSlotOutcome};
use serde::Serialize;

/// `odm project worktree list --json`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorktreeListDto {
    pub project: String,
    pub slots: Vec<WorktreeSlotDto>,
}

/// One slot in list JSON.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorktreeSlotDto {
    pub name: String,
    pub path: String,
}

/// `odm project worktree add|rm --json`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorktreeSlotActionDto {
    pub project: String,
    pub slot: String,
    pub path: String,
}

pub fn worktree_list_dto(out: &WorktreeListOutcome) -> WorktreeListDto {
    WorktreeListDto {
        project: out.project.clone(),
        slots: out
            .slots
            .iter()
            .map(|s| WorktreeSlotDto {
                name: s.name.clone(),
                path: s.path.clone(),
            })
            .collect(),
    }
}

pub fn worktree_slot_action_dto(out: &WorktreeSlotOutcome) -> WorktreeSlotActionDto {
    WorktreeSlotActionDto {
        project: out.project.clone(),
        slot: out.slot.clone(),
        path: out.path.clone(),
    }
}

pub fn format_worktree_list_human(out: &WorktreeListOutcome) -> String {
    let mut s = String::new();
    for slot in &out.slots {
        s.push_str(&slot.name);
        s.push('\n');
    }
    s
}

pub fn format_worktree_add_human(out: &WorktreeSlotOutcome) -> String {
    format!("added worktree slot {} -> {}", out.slot, out.path)
}

pub fn format_worktree_rm_human(out: &WorktreeSlotOutcome) -> String {
    format!("removed worktree slot {} ({})", out.slot, out.path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use odm_core::{WorktreeListOutcome, WorktreeSlotInfo, WorktreeSlotOutcome};

    #[test]
    fn list_dto_json_shape() {
        let out = WorktreeListOutcome {
            project: "alpha".into(),
            slots: vec![
                WorktreeSlotInfo {
                    name: "a".into(),
                    path: "worktrees/alpha/a".into(),
                },
                WorktreeSlotInfo {
                    name: "b".into(),
                    path: "worktrees/alpha/b".into(),
                },
            ],
        };
        let dto = worktree_list_dto(&out);
        let v = serde_json::to_value(&dto).unwrap();
        assert_eq!(v["project"], "alpha");
        assert_eq!(v["slots"][0]["name"], "a");
        assert_eq!(v["slots"][0]["path"], "worktrees/alpha/a");
        assert_eq!(v["slots"][1]["name"], "b");
    }

    #[test]
    fn action_dto_json_shape() {
        let out = WorktreeSlotOutcome {
            project: "alpha".into(),
            slot: "s1".into(),
            path: "worktrees/alpha/s1".into(),
        };
        let v = serde_json::to_value(worktree_slot_action_dto(&out)).unwrap();
        assert_eq!(v["project"], "alpha");
        assert_eq!(v["slot"], "s1");
        assert_eq!(v["path"], "worktrees/alpha/s1");
    }

    #[test]
    fn list_human_one_name_per_line() {
        let out = WorktreeListOutcome {
            project: "alpha".into(),
            slots: vec![
                WorktreeSlotInfo {
                    name: "a".into(),
                    path: "worktrees/alpha/a".into(),
                },
                WorktreeSlotInfo {
                    name: "b".into(),
                    path: "worktrees/alpha/b".into(),
                },
            ],
        };
        assert_eq!(format_worktree_list_human(&out), "a\nb\n");
    }
}

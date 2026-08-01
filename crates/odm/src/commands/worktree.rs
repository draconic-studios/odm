//! `odm project worktree` DTOs and human formatters.

use odm_core::{WorktreeListOutcome, WorktreePruneOutcome, WorktreeSlotOutcome};
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

/// `odm project worktree prune --json`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorktreePruneDto {
    pub project: String,
    pub pruned: Vec<WorktreeSlotDto>,
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

pub fn worktree_prune_dto(out: &WorktreePruneOutcome) -> WorktreePruneDto {
    WorktreePruneDto {
        project: out.project.clone(),
        pruned: out
            .pruned
            .iter()
            .map(|s| WorktreeSlotDto {
                name: s.name.clone(),
                path: s.path.clone(),
            })
            .collect(),
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

pub fn format_worktree_prune_human(out: &WorktreePruneOutcome) -> String {
    let mut s = if out.pruned.is_empty() {
        "pruned 0 orphan worktree dirs".to_string()
    } else {
        let names: Vec<&str> = out.pruned.iter().map(|p| p.name.as_str()).collect();
        format!(
            "pruned {} orphan worktree dir{}: {}",
            out.pruned.len(),
            if out.pruned.len() == 1 { "" } else { "s" },
            names.join(", ")
        )
    };
    if !out.skipped_nonempty.is_empty() {
        let names: Vec<&str> = out
            .skipped_nonempty
            .iter()
            .map(|p| p.name.as_str())
            .collect();
        s.push_str(&format!(
            "\nskipped non-empty orphan{} (use --force): {}",
            if out.skipped_nonempty.len() == 1 {
                ""
            } else {
                "s"
            },
            names.join(", ")
        ));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use odm_core::{
        WorktreeListOutcome, WorktreePruneOutcome, WorktreeSlotInfo, WorktreeSlotOutcome,
    };

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

    #[test]
    fn prune_dto_json_shape() {
        let out = WorktreePruneOutcome {
            project: "alpha".into(),
            pruned: vec![WorktreeSlotInfo {
                name: "stale".into(),
                path: "worktrees/alpha/stale".into(),
            }],
            skipped_nonempty: vec![],
        };
        let v = serde_json::to_value(worktree_prune_dto(&out)).unwrap();
        assert_eq!(v["project"], "alpha");
        assert_eq!(v["pruned"][0]["name"], "stale");
        assert_eq!(v["pruned"][0]["path"], "worktrees/alpha/stale");
        assert!(v.get("skipped_nonempty").is_none());
    }

    #[test]
    fn prune_human_zero_and_skipped() {
        let empty = WorktreePruneOutcome {
            project: "alpha".into(),
            pruned: vec![],
            skipped_nonempty: vec![],
        };
        assert_eq!(
            format_worktree_prune_human(&empty),
            "pruned 0 orphan worktree dirs"
        );

        let partial = WorktreePruneOutcome {
            project: "alpha".into(),
            pruned: vec![WorktreeSlotInfo {
                name: "empty".into(),
                path: "worktrees/alpha/empty".into(),
            }],
            skipped_nonempty: vec![WorktreeSlotInfo {
                name: "full".into(),
                path: "worktrees/alpha/full".into(),
            }],
        };
        let h = format_worktree_prune_human(&partial);
        assert!(h.contains("pruned 1 orphan worktree dir: empty"));
        assert!(h.contains("skipped non-empty orphan (use --force): full"));
    }
}

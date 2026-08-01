//! Shared MaterializeOutcome → string labels (JSON vs human sync differ).

use odm_core::MaterializeOutcome;

/// Locked JSON label for a materialize outcome (`cloned` | `already_present`).
pub type MaterializeLabel = &'static str;

/// JSON contract: `cloned` | `already_present`.
pub fn materialize_json(outcome: MaterializeOutcome) -> MaterializeLabel {
    match outcome {
        MaterializeOutcome::Cloned => "cloned",
        MaterializeOutcome::AlreadyPresent => "already_present",
    }
}

/// Optional JSON materialize field (project/progen add with `--no-clone` → null).
pub fn materialize_json_opt(outcome: Option<MaterializeOutcome>) -> Option<MaterializeLabel> {
    outcome.map(materialize_json)
}

/// Human sync table cell: `cloned` | `present` (not the JSON `already_present`).
pub fn materialize_sync_human(outcome: MaterializeOutcome) -> MaterializeLabel {
    match outcome {
        MaterializeOutcome::Cloned => "cloned",
        MaterializeOutcome::AlreadyPresent => "present",
    }
}

/// Human line after `project add` (single shared MaterializeOutcome mapping).
pub fn format_project_add_human(name: &str, outcome: Option<MaterializeOutcome>) -> String {
    match outcome {
        Some(MaterializeOutcome::Cloned) => format!("added project {name} (cloned)"),
        Some(MaterializeOutcome::AlreadyPresent) => {
            format!("added project {name} (already present)")
        }
        None => format!("added project {name}"),
    }
}

/// Human line after `progen add` (single shared MaterializeOutcome mapping).
pub fn format_progen_add_human(name: &str, outcome: Option<MaterializeOutcome>) -> String {
    match outcome {
        Some(MaterializeOutcome::Cloned) => format!("added progen {name} (cloned vault)"),
        Some(MaterializeOutcome::AlreadyPresent) => {
            format!("added progen {name} (already present)")
        }
        None => format!("added progen {name} (vault ready)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_and_sync_human_labels_differ_for_already_present() {
        assert_eq!(materialize_json(MaterializeOutcome::Cloned), "cloned");
        assert_eq!(
            materialize_json(MaterializeOutcome::AlreadyPresent),
            "already_present"
        );
        assert_eq!(
            materialize_sync_human(MaterializeOutcome::AlreadyPresent),
            "present"
        );
        assert_eq!(
            materialize_json_opt(Some(MaterializeOutcome::Cloned)),
            Some("cloned")
        );
        assert_eq!(materialize_json_opt(None), None);
    }

    #[test]
    fn add_human_messages_cover_all_outcomes() {
        assert_eq!(
            format_project_add_human("a", Some(MaterializeOutcome::Cloned)),
            "added project a (cloned)"
        );
        assert_eq!(
            format_project_add_human("a", Some(MaterializeOutcome::AlreadyPresent)),
            "added project a (already present)"
        );
        assert_eq!(format_project_add_human("a", None), "added project a");
        assert_eq!(
            format_progen_add_human("n", Some(MaterializeOutcome::Cloned)),
            "added progen n (cloned vault)"
        );
        assert_eq!(
            format_progen_add_human("n", Some(MaterializeOutcome::AlreadyPresent)),
            "added progen n (already present)"
        );
        assert_eq!(
            format_progen_add_human("n", None),
            "added progen n (vault ready)"
        );
    }
}

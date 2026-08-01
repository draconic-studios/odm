//! `odm generate` — Generator list DTO and human formatting.

use odm_core::Workspace;
use serde::Serialize;

/// `odm generate --json` (no name) envelope.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GeneratorListDto {
    pub generators: Vec<GeneratorListItem>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GeneratorListItem {
    pub name: String,
    /// Present as JSON `null` when unset (do not skip).
    pub template: Option<String>,
    /// Present as JSON `null` when unset (do not skip).
    pub url: Option<String>,
}

/// Library entrypoint: list configured generators as a serializable DTO (sorted by name).
pub fn list_generators_dto(ws: &Workspace) -> GeneratorListDto {
    let generators = ws
        .generators
        .iter()
        .map(|(name, def)| GeneratorListItem {
            name: name.clone(),
            template: def.template.clone(),
            url: def.url.clone(),
        })
        .collect();
    GeneratorListDto { generators }
}

/// Human one-name-per-line list (beside DTO).
pub fn format_generator_list_human(dto: &GeneratorListDto) -> String {
    if dto.generators.is_empty() {
        return "(no generators)\n".into();
    }
    let mut out = String::new();
    for g in &dto.generators {
        out.push_str(&g.name);
        out.push('\n');
    }
    out
}

/// Human success one-liner after materialize.
pub fn format_generate_run_human(name: &str, dest: &str, copied: u32) -> String {
    format!("generated {name} -> {dest} ({copied} files)\n")
}

/// `odm generate <name> --json` envelope.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GenerateRunDto {
    pub generator: String,
    pub dest: String,
    pub copied: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use odm_core::{GeneratorDef, Workspace, WorkspaceConfig};

    fn ws_with(generators: BTreeMap<String, GeneratorDef>) -> Workspace {
        Workspace {
            root: PathBuf::from("/tmp/ws"),
            config: WorkspaceConfig::default(),
            actions: BTreeMap::new(),
            generators,
        }
    }

    #[test]
    fn generator_list_dto_json_nulls_and_sorted_names() {
        let mut generators = BTreeMap::new();
        generators.insert(
            "zeta".into(),
            GeneratorDef {
                template: Some("t/z".into()),
                url: None,
            },
        );
        generators.insert(
            "alpha".into(),
            GeneratorDef {
                template: None,
                url: Some("https://example.com/g".into()),
            },
        );
        let dto = list_generators_dto(&ws_with(generators));
        let v = serde_json::to_value(&dto).unwrap();
        let arr = v["generators"].as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["name"], "alpha");
        assert!(arr[0]["template"].is_null());
        assert_eq!(arr[0]["url"], "https://example.com/g");
        assert_eq!(arr[1]["name"], "zeta");
        assert_eq!(arr[1]["template"], "t/z");
        assert!(arr[1]["url"].is_null());
    }

    #[test]
    fn empty_generator_list_human() {
        let dto = list_generators_dto(&ws_with(BTreeMap::new()));
        assert_eq!(format_generator_list_human(&dto), "(no generators)\n");
    }

    #[test]
    fn generate_run_dto_shape() {
        let dto = GenerateRunDto {
            generator: "pkg".into(),
            dest: "out/pkg".into(),
            copied: 3,
        };
        let v = serde_json::to_value(&dto).unwrap();
        assert_eq!(v["generator"], "pkg");
        assert_eq!(v["dest"], "out/pkg");
        assert_eq!(v["copied"], 3);
    }

    #[test]
    fn format_generate_run_human_line() {
        assert_eq!(
            format_generate_run_human("pkg", "out/pkg", 2),
            "generated pkg -> out/pkg (2 files)\n"
        );
    }
}

---
id: issues-17
title: "Config and pin serde model"
description: "Lock Rust types, validation, deny-unknown, and write-back policy for config and pin YAML."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
  - ready-for-agent
---

# Config and pin serde model

## Question

What are the exact Rust types and validation rules for `.odm/odm.config.yaml` and `.odm/odm.lock.yaml` (full v1 schema load, deny unknown fields), and what is the write-back policy when CLI mutates config or auto-maintains pins (formatting, key order, drop of removed pins)?

## Blocked by

_(none — frontier)_

## Answer

**Recommended lock (docs + implement):**

### Types (`odm-core`)

- **`WorkspaceConfig`**: `name: Option<String>`, `manage_gitignore: Option<bool>` (default **true** via `fn manage_gitignore(&self) -> bool`), `projects` / `progens` / `progen_groups` / `actions` / `generators` as `BTreeMap` (stable sorted key order). All structs: `#[serde(deny_unknown_fields)]`.
- **`ProjectEntry`**: required `path: String`; optional `url`, `branch`, `type_` (`#[serde(rename = "type")]`).
- **`ProgenEntry`**: required `path`; optional `url`, `branch`.
- **`progen_groups`**: `BTreeMap<String, Vec<String>>` (Progen names).
- **`actions` / `generators`**: `BTreeMap<String, String>` (bundle path relative to Workspace root).
- **Action bundle file**: map name → `ActionDef { run: String, dir: Option<String> }` (deny unknown).
- **Generator bundle file**: map name → `GeneratorDef { template: Option<String>, url: Option<String> }` (deny unknown; post-validate at least one of template/url).
- **`PinFile`**: `version: u32` (must be `1`), `pins: BTreeMap<String, PinEntry>`.
- **`PinEntry`**: `rev: String` (40-char lowercase hex), `url: String`, `branch: Option<String>`.

### Load / validation

1. Deserialize YAML with deny-unknown.
2. Post-validate: non-empty entity names and paths; paths relative (no absolute, no empty); `progen_groups` members exist in `progens`; every declared action/generator bundle path exists and loads; merge action/generator names across bundles — duplicate names → error; pin `version == 1`; pin `rev` full SHA shape when present.
3. Bundle load is **eager** on config load (relative to Workspace root).
4. Missing optional maps = empty. Empty config file body / minimal `{}` valid bootstrap.

### Write-back

- Serialize with `serde_yaml`; **BTreeMap** → sorted keys; no comment preservation day one.
- Omit `manage_gitignore` when writing if value is default `true` (cleaner files); always write explicit `false`.
- **Atomic write**: write temp sibling (`*.tmp` / unique) then `rename` over target.
- Pin auto-maintain: drop pins whose names are no longer managed in config; add/update on successful materialize/sync leaving defined HEAD; path-only entities never pinned.
- Config mutations (`project add`/`rm`): rewrite full config file (sorted), not surgical line edits.

### Out of scope day one

- Comment/key-order round-trip preservation beyond BTreeMap sort
- File locking under concurrent CLI
- JSON config twin

## Comments

Parent map: [[issues-14-implement-core-map]]

Recommended decision locked for agent implement 2026-08-01.

Landed 2026-08-01 in `crates/odm-core` (`config.rs`, `pin.rs`, `io.rs`) + unit tests.

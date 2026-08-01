---
id: issues-106
title: "pack list missing CLI integration"
description: "Bin tests: agent pack list --json missing false/true after install and dest delete."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# pack list missing CLI integration

## Description

Wire is already in DTO (105). Add bin-level coverage so `odm agent pack list` / `--json` and install/link JSON expose `missing` correctly end-to-end.

## Affected

- `crates/odm/tests/cli_agent_pack.rs` (or adjacent) — integration tests
- CLI path only if 105 left a gap (should be thin: already uses `pack_list_dto` / `pack_entry_dto`)
- Watch `cli_agent_pack.rs` LOC ≤1000 / ≤1250

## Impact

Without bin tests, DTO unit coverage can drift from real CLI JSON.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-105-pack-list-missing-dto]]

## Agent Brief

**Category:** test  
**Summary:** Integration-test `odm agent pack list` (and install JSON) for `missing` true/false after real install and dest deletion.

**Bindings:**

- Parent map: [[issues-104-post-v1-pack-list-missing-map]]
- DTO work: [[issues-105-pack-list-missing-dto]]
- Existing patterns in `cli_agent_pack.rs` (install → list → rm; doctor pack_missing gate)

**Desired behavior:**

1. **Install then list:** `agent pack install <src> --home <h>` → `list --json` → that pack has `missing: false`; human list line is bare name (no ` missing` suffix).
2. **Delete dest then list:** remove `<home>/<name>` (file/dir) without `pack rm` → `list --json` → `missing: true`; human line ends with ` missing`.
3. **Install `--json`:** response object includes `missing: false` (same field set as list items).
4. **Optional:** after `pack rm`, pack gone from list (existing coverage may already assert this).
5. Do not change doctor tests except if shared helpers need a tweak.
6. Keep test file within LOC limits; extract helpers if needed.
7. No docs (107). No core-desk (108).
8. `cargo test` green.

**Acceptance criteria:**

- [ ] Bin test: present pack → list JSON `missing: false`
- [ ] Bin test: deleted dest → list JSON `missing: true` + human ` missing`
- [ ] Install JSON includes `missing: false`
- [ ] File size within limits; `cargo test` green

**Out of scope:**

- Docs / CHANGELOG
- core-desk dogfood
- Status/doctor product changes
- Link-mode dangling symlink matrix (unit tests in 105 suffice unless cheap)

## Acceptance

- [ ] Agent Brief acceptance criteria all met

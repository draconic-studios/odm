---
id: issues-91
title: "CLI integration: doctor pack_missing warn"
description: "Bin-level test: registry pack with deleted dest → odm doctor --json includes pack_missing warn."
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

# CLI integration: doctor pack_missing warn

## Description

[[issues-86-doctor-pack-missing]] landed unit tests for `pack_missing_checks`. There is no `crates/odm/tests` coverage that runs the real `odm doctor --json` binary path for pack_missing. Add a focused integration test.

## Affected

- `crates/odm/tests/cli_agent_pack.rs` (preferred — pack lifecycle home) **or** small addition beside existing doctor smoke tests
- Uses real CLI + temp workspace; no production code change expected unless a wiring bug is found

## Impact

Regressions in doctor CLI wiring for packs would only be caught by unit tests of the helper, not the bin.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** test  
**Summary:** Integration test proves `odm doctor --json` emits `pack_missing:<name>` after install + delete dest; present pack does not; `--fix` does not clear the warn by editing registry.

**Bindings:**

- Parent map: [[issues-89-post-v1-status-packs-map]]
- Existing pack CLI helpers in `cli_agent_pack.rs` (install/list/rm patterns)
- Doctor JSON shape already used in `core_desk.rs` / init smoke (`ok`, checks with `id` / `status` / `fixable`)
- Unit behavior lock from [[issues-86-doctor-pack-missing]]

**Desired behavior:**

1. Temp workspace: `odm init` (or fixture pattern used in cli_agent_pack).
2. `odm agent pack install <src> --home <home>` → success.
3. `odm doctor --json` → no check id starting with `pack_missing:` for that pack.
4. Delete the installed dest directory (leave registry intact).
5. `odm doctor --json` → includes check `id` = `pack_missing:<name>`, warn status, `fixable: false` (match existing doctor JSON field names).
6. `odm doctor --fix` then `doctor --json` again → pack_missing **still** present; registry file still lists the pack.
7. Optional: after `odm agent pack rm <name>`, pack_missing gone.
8. No product/docs changes unless a real bug forces a one-line fix (stay minimal).
9. `cargo test` green; clippy clean if any rustc code touched.

**Acceptance criteria:**

- [ ] Integration test covers present → no pack_missing; deleted dest → pack_missing warn
- [ ] Asserts fixable false / warn; `--fix` does not remove registry entry or clear warn
- [ ] No drive-by refactors; `cargo test` green

**Out of scope:**

- Status agent_packs field (ticket 90)
- core-desk README dogfood (ticket 93)
- New doctor check types

## Acceptance

- [ ] Agent Brief acceptance criteria all met

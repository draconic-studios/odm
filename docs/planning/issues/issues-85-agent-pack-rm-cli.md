---
id: issues-85
title: "CLI: odm agent pack rm"
description: "Wire odm agent pack rm <name> over pack_rm with human + JSON output and integration tests."
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

# CLI: odm agent pack rm

## Description

After core `pack_rm` lands ([[issues-84-agent-pack-rm-core]]), expose `odm agent pack rm <name>` with the same human/JSON honesty as install/link/list.

## Affected

- `crates/odm/src/cli.rs` — `PackCmd` / clap
- `crates/odm/src/main.rs` — dispatch
- `crates/odm/src/commands/agent_pack.rs` — DTO + human format
- `crates/odm/tests/cli_agent_pack.rs` — integration

## Impact

Users cannot uninstall packs from the CLI without hand-editing registry and homes.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-84-agent-pack-rm-core]] (closed)

## Agent Brief

**Category:** feat  
**Summary:** `odm agent pack rm <name>` calls core `pack_rm`; human + `--json` match install/link entry shape.

**Bindings:**

- Parent map: [[issues-82-post-v1-pack-lifecycle-hardening-map]]
- Core: `pack_rm` from [[issues-84-agent-pack-rm-core]]
- Existing pack CLI patterns in `commands/agent_pack.rs` and `cli_agent_pack.rs`
- Exit spine: unknown name → `4`; workspace/config → `2`; fs/registry op fail → `3`; usage → `1`

**Desired behavior:**

1. **Clap:** `odm agent pack rm <name>` (subcommand alongside list/install/link). No extra flags in v1.
2. **Workspace required** (same discovery as other pack commands).
3. **Human success:** one line, e.g. `removed <name> -> <path>` (path display consistent with install/link style).
4. **`--json` success:** single entry object `{ "name", "source", "path", "mode" }` (same fields as list items / install-link JSON) — the removed entry.
5. **Errors:** unknown name → exit `4` with clear message; empty name → usage `1`.
6. **Integration tests:** install then rm (list empty after); rm unknown → exit 4; optional unix link then rm.
7. Help text / clap about strings mention rm.
8. Docs/CHANGELOG full honesty is [[issues-87-pack-lifecycle-docs-honesty]] — a minimal code comment is enough here; do not skip tests.

**Acceptance criteria:**

- [ ] `odm agent pack rm <name>` works human + `--json`
- [ ] Exit codes honest (4 unknown, etc.)
- [ ] Integration tests cover success + unknown
- [ ] `cargo test` green
- [ ] list/install/link behavior unchanged

**Out of scope:**

- Doctor pack_missing ([[issues-86-doctor-pack-missing]])
- core-desk README dogfood ([[issues-88-core-desk-pack-rm-dogfood]])
- Full reference doc pass ([[issues-87-pack-lifecycle-docs-honesty]])
- `--keep-files`, force flags, multi-name rm

## Acceptance

- [ ] Agent Brief acceptance criteria all met

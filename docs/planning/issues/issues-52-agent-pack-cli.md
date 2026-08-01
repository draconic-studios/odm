---
id: issues-52
title: "Agent pack CLI"
description: "Wire odm agent pack list|install|link over core; keep start/prompt stubs."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
  - ready-for-agent
---

# Agent pack CLI

## Description

Replace trailing-var-arg `agent pack` stub with real subcommands calling core pack APIs. Emit human and `--json` output. Leave `agent start` and `agent prompt` as not-implemented.

## Affected

- `crates/odm/src/cli.rs` (`AgentCmd`)
- `crates/odm/src/main.rs` / `commands/`
- Stub expectations in `crates/odm/tests/progen_vault.rs` (or wherever agent stubs are asserted)

## Impact

Users cannot install/link packs from the CLI without this.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-51-agent-pack-core]]

## Agent Brief

**Category:** feature  
**Summary:** Thin CLI adapter for agent pack list/install/link.

**Bindings:**

- Map [[issues-50-agent-packs-map]] standing prefs + JSON shapes
- Core API from [[issues-51-agent-pack-core]]
- Exit spine: `docs` / existing `exit_code` mapping
- `cli.md` reserved names (update deferred to docs ticket if preferred — behavior must match map)

**Current behavior:**

- `odm agent pack …` → `not implemented: agent pack` exit 1
- clap: single `Pack { rest: Vec<String> }`

**Desired behavior:**

1. **Clap:**
   ```text
   odm agent pack list
   odm agent pack install <source> --home <path> [--force]
   odm agent pack link <source> --home <path> [--force]
   ```
   Nested subcommand under `agent` (e.g. `AgentCmd::Pack { cmd: PackCmd::… }`).
2. **list:** core `pack_list`; human one name/line or `(no agent packs)`; JSON `{ "packs": [ { "name", "source", "path", "mode" } ] }`.
3. **install/link:** require `--home`; call core; human one-liner e.g. `installed <name> -> <path>` / `linked <name> -> <path>`; JSON single object same fields as list entry (or `{ "pack": { … } }` — pick one and test it).
4. **start/prompt:** still `not_implemented`.
5. **Update stub tests** that expected all agent verbs exit 1 — pack success paths move to integration ticket; pack with missing workspace still fails honestly; `agent start`/`prompt` remain exit 1.
6. No integration matrix in this ticket beyond fixing broken unit/cli smoke if any.

**Acceptance criteria:**

- [ ] `odm agent pack list|install|link` wired to core
- [ ] `--home` / `--force` flags work
- [ ] Human + `--json` shapes stable and tested at least lightly (unit or small CLI test)
- [ ] `agent start` / `agent prompt` still not-implemented exit 1
- [ ] `cargo test` green
- [ ] No remote/marketplace; no start/prompt implementation

**Out of scope:**

- Full integration dogfood / docs promotion ([[issues-53-agent-pack-integration-and-docs]])
- Changing core pack semantics

## Acceptance

- [ ] Agent Brief acceptance criteria all met

## Comments

Seeded with map issues-50.

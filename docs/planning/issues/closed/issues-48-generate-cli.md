---
id: issues-48
title: "odm generate CLI list and run"
description: "Wire odm generate list + materialize with --dest/--force and JSON shapes; drop not_implemented stub."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
---

# odm generate CLI list and run

## Description

CLI `Generate` is a stub (`not_implemented`). After core materialize lands, wire list and run with flags and JSON.

## Affected

- `crates/odm` (`cli.rs`, `main.rs`, commands module)
- Stub test `generate_and_agent_stubs_exit_1` must stop expecting generate exit 1 for implemented paths

## Impact

Users cannot scaffold from declared generators.

## Proposed Fix

See Agent Brief.

## Blocked by

- ~~[[issues-47-generator-materialize-core]]~~ (closed)

## Agent Brief

**Category:** feature  
**Summary:** Implement `odm generate` per map standing prefs using core materialize API.

**Bindings:**

- Parent map [[issues-45-generators-map]]
- Core API from [[issues-47-generator-materialize-core]]
- Exit spine: `docs/reference/cli.md` / existing `OdmError` mapping
- Mirror `odm run` list/dispatch patterns where natural

**Current behavior:**

- `Commands::Generate { name: Option<String> }` → always not_implemented
- No `--dest` / `--force` flags

**Desired behavior:**

1. **Clap:**
   - `odm generate` — list
   - `odm generate <name> --dest <path> [--force]`
   - `--dest` required when `name` is present; if name present without dest → clap/usage error
2. **List (no name):**
   - Human: one generator name per line, sorted
   - `--json`: `{ "generators": [ { "name", "template", "url" } ] }` with `null` for absent optional fields
3. **Run:**
   - Call core materialize; human success one-liner e.g. `generated <name> -> <dest> (<n> files)`
   - `--json`: `{ "generator", "dest", "copied" }`
   - Unknown name / url-only / path errors → existing error mapper (usage `1`, workspace `2`, operation as today)
4. **Tests:** update/remove generate cases in stub test; add unit or command-level tests if pattern exists; full integration can wait for issues-49 but CLI must not leave generate always exit 1
5. Agent stubs remain not_implemented

**Acceptance criteria:**

- [x] `odm generate` lists; no longer not_implemented
- [x] `odm generate <name> --dest …` materializes via core
- [x] `--force` passed through
- [x] JSON list + run shapes match map
- [x] Stub test no longer requires generate exit 1 for list/run happy paths
- [x] `cargo test` green
- [x] No remote fetch; no agent pack work

**Out of scope:**

- core-desk dogfood generator (issues-49)
- Long reference doc rewrite (issues-49 can finish; brief cli.md note OK if tiny)
- `url` implementation

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Wired `odm generate` as a thin CLI adapter over `odm_core::generate_local`.

- **Clap:** `Generate { name, dest, force }` — `--dest`/`--force` require `name`; name without dest → usage exit 1
- **List:** `commands/generate.rs` DTOs — human one-name-per-line (or `(no generators)`); JSON `{ "generators": [{ name, template, url }] }` with nulls for absent optionals
- **Run:** `generate_local(&ws, name, dest_rel, force)`; human `generated <name> -> <dest> (<n> files)`; JSON `{ generator, dest, copied }` using user-relative dest
- **Tests:** unit tests for list/run JSON shapes; progen_vault stub updated (list succeeds; agent stubs still not_implemented)
- Full e2e dogfood/docs left to [[issues-49-generate-integration-and-docs]]

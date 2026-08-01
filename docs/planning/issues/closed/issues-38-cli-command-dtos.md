---
id: issues-38
title: "CLI command DTOs and thin bin"
description: "Move product shapes and multi-crate composition out of main into library command modules returning serializable DTOs."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - architecture
  - deepen
---

# CLI command DTOs and thin bin

## Description

Architecture says the bin is parse, UX, exit codes. `main` also owns MaterializeOutcome string mapping (×3), Project/Progen list joins, pin labels, ScopedProgen hand-builds, and composition across core/progen/actions. Deepen command **modules** that return DTOs; bin becomes a thin **adapter** (**leverage** for tests and future non-CLI clients).

Domain: Workspace, Project, Progen, Action, Pin file.  
Architecture: **interface** of each command is the DTO + errors; bin only prints.

## Affected

- `odm` binary main / output
- Possibly thin lib surface used only by the bin (same package or small app lib)
- Integration tests that assert JSON contracts

## Impact

JSON contracts exist only as assert_cmd tests; every new client reimplements joins; main is the largest shallow composition root.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-34-workspace-observation-depth]] — list/info/status should project observation DTOs, not re-sample
- [[issues-36-managed-entity-membership]] — add/sync composition and vault wiring should already be library-side
- [[issues-37-progen-facade-engine]] — progen command block should call façade, not reimplement scope loops

## Agent Brief

**Category:** enhancement  
**Summary:** Extract command orchestration and serializable DTOs from the CLI binary so main only parses, invokes library commands, formats, and sets exit codes.

**Current behavior:**
- cli parse module is appropriately thin; output helper encodes JSON and error envelopes.
- main discovers workspace, loads config, constructs git, and inlines:
  - MaterializeOutcome → human/json strings in multiple commands
  - Project list and Progen list nearly duplicated (config + status + pin label)
  - pin_state human labels only in the bin
  - Manual construction of progen scope types for info
  - Repeated single-`--progen` checks and reindex loops (should shrink once façade lands)
- Presentation locality is inconsistent: some formatters in core, some in progen, some only in main.

**Desired behavior:**
- For each shipped command family that today embeds join logic in main, a library entrypoint returns a serializable DTO (or structured error) covering the success payload already locked for `--json` where applicable:
  - init, status, doctor, pin status/apply, project list/info/add/rm, progen list/info/add/rm and store ops, sync, run list/run
- Bin responsibilities only: parse argv, call library, print human or JSON, map errors to exit codes (existing spine).
- Duplicated MaterializeOutcome mapping and pin label mapping live once next to the DTO.
- Prefer reusing observation/membership/façade outputs rather than rebuilding entity rows in the bin.
- Pick one locality rule for human formatters going forward: either beside DTOs in one present path, or only in the bin — do not add a third home. Migrating every legacy formatter is best-effort; new code must follow the rule.
- assert_cmd tests remain the E2E gate; add library-level tests for DTO construction where it removes brittle setup.

**Key interfaces:**
- Existing JSON shapes (core JSON lock, progen/actions CLI output) — field names stable
- Exit code spine unchanged
- Error envelope under `--json` unchanged
- Workspace discovery and `--root` / global flags unchanged

**Acceptance criteria:**
- [x] main contains no duplicated MaterializeOutcome → string tables (single shared mapping)
- [x] Project list and Progen list JSON are produced by library DTOs, not ad-hoc maps only in main
- [x] Bin does not hand-build progen scope/store types when a library helper exists
- [x] All existing CLI integration tests pass without relaxing assertions
- [x] At least the heaviest command paths (status, project list, progen find/list, run list) have library-callable entrypoints returning DTOs
- [x] `cargo test` and `cargo clippy -- -D warnings` clean for touched crates

**Out of scope:**
- New commands (generate, agent, worktree) beyond keeping stubs thin
- Redesigning JSON schemas
- Full extraction of every format_*_human in core/progen in one go
- MCP/TUI clients (only enable them via DTOs)

## Answer

Added `crates/odm` library (`odm::commands`) with serializable DTOs and entrypoints: `materialize_*` (single MaterializeOutcome label map), `status_snapshot`, `list_projects`/`project_info`, `list_progens`/`progen_info` (via `scoped_from_config`), `find_notes_dto`, `list_actions_dto`. Human formatters for new list/info/add paths live beside DTOs. Bin is a thin adapter (parse → call → print/exit). JSON field contracts and exit codes unchanged; unit + integration tests green.

## Comments

From architecture review 2026-08-01 (candidate #5, Worth exploring).

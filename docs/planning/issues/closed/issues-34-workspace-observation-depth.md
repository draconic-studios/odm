---
id: issues-34
title: "Workspace observation depth"
description: "One observation module for pin/checkout facts shared by status, doctor, and pin status."
status: closed
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - architecture
  - deepen
---

# Workspace observation depth

## Description

Pin drift and per-entity disk/git facts are one domain concern, but status, lifecycle pin reporting, and doctor each sample git and derive pin state separately (enum vs string machines). Deepen a single Workspace observation so reporters project from one fact source.

Domain: Workspace, Project, Progen, Pin file, Primary checkout.  
Architecture: deepen the observation **module**; one **seam** for sampling; status/doctor/pin status as thin projections (**leverage** / **locality**).

## Affected

- `odm-core` status, doctor, lifecycle pin/entity reporting
- CLI consumers of status / pin status / doctor JSON and human output
- Tests that assert `pin_state` / pin entry `state`

## Impact

Divergent pin-state edge cases; sealed `Git::new()` in status/doctor blocks injectable tests; every new reporter re-implements sampling.

## Proposed Fix

See Agent Brief.

## Blocked by

_(none)_

## Agent Brief

**Category:** enhancement  
**Summary:** Introduce one Workspace observation depth that samples each managed entity once and derives a single `PinState`; make status, pin status, and doctor project from it with an injectable git adapter.

**Current behavior:**
- Status builds entity rows and uses a pure `PinState` enum (`none` | `missing_path` | `unpinned` | `in_sync` | `drift` | `missing_pin_file`).
- Lifecycle pin status / entity disk info re-implement pin state as strings with different edge-case order (e.g. missing path when pin file absent).
- Doctor walks projects/progens again, constructs its own git client, and does not share the status snapshot.
- Status and doctor construct the real git runner internally; lifecycle already accepts an injectable git port.

**Desired behavior:**
- One observation function (or small module) accepts Workspace root, loaded config, optional pin file, and a git adapter. For each declared Project and Progen it produces a stable snapshot: resolved path, on_disk, is_git, head, origin, dirty, pin_rev, pin_state, and any other fields already required by locked JSON shapes for status / pin status / project|progen info.
- `PinState` (or equivalent) is the only pin-drift classifier. Lifecycle pin status and any entity-info helpers use it — no parallel string state machines.
- `odm status`, `odm pin status`, and `odm doctor` derive their existing public JSON/human contracts from this snapshot (or from pure projections of it). Check IDs, severities, and exit behavior for doctor stay as already locked.
- Status and doctor accept the same style of injectable git adapter lifecycle already uses (or share a thin workspace session type). Unit tests can exercise pin_state and doctor classification without a real git binary for pure classification paths; integration tests may still use real git.
- Deleting the duplicate pin-state branches in lifecycle does not change documented `pin_state` / pin entry `state` string values for agents.

**Key interfaces:**
- `PinState` — single source of truth for pin drift labels already exposed in JSON
- Observation snapshot type(s) covering Project and Progen rows
- Git port already used by lifecycle materialize/sync — observation must use it, not a hard-coded constructor
- Stable JSON shapes from closed core JSON / doctor matrix decisions — preserve field names and enums

**Acceptance criteria:**
- [x] Exactly one implementation path derives pin drift labels used by status and pin status
- [x] Status and doctor do not construct a non-injectable git client for entity sampling
- [x] `odm status --json` and `odm pin status --json` field names and `pin_state` / `state` enum values remain compatible with locked shapes
- [x] Doctor check ids, severities, and `--fix` allowlist unchanged
- [x] Unit tests cover pin_state edge cases (missing pin file, missing path, unpinned, in_sync, drift) through the shared classifier
- [x] Existing core-desk / CLI integration expectations for status and doctor still pass
- [x] `cargo test` and `cargo clippy -- -D warnings` clean for touched crates

**Out of scope:**
- Splitting lifecycle membership / materialize (separate issue)
- Path escape policy unification (separate issue)
- Moving human formatters out of core
- Worktree slot observation
- Changing doctor check matrix or JSON schema_version

## Answer

Shipped in `6f6db5c` (`feat(core): workspace observation shared by status, doctor, pin`). Swarm cycle verified ACs; no further product code required.

- **`observe_workspace` / `observe_entity`** (`odm-core::observation`): one sample per declared Project/Progen with injectable `&Git<R: CommandRunner>`
- **`compute_pin_state` + `PinState`**: sole pin-drift classifier; status `pin_state` and pin status `state` project via serde / `as_str()`
- **Projections**: `build_status` / `status_from_observation`, `pin_status`, `run_doctor` / entity path checks — no parallel string machines
- **`Git::new()`** only at CLI boundary and tests; core APIs take `&Git<R>`
- **Tests**: `pin_state_matrix`, observation pin attach, core-desk / CLI status+doctor smoke green

Residual (YAGNI): CLI `project info` may still re-query origin; `EntityObservation` already carries it for a later polish.

## Comments

From architecture review 2026-08-01 (candidate #1, Strong). Top recommendation.

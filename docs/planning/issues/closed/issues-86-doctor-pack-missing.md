---
id: issues-86
title: "Doctor warn for missing agent pack paths"
description: "odm doctor warns pack_missing:<name> when a registry entry path does not exist on disk."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# Doctor warn for missing agent pack paths

## Description

`env-gen-packs.md` deferred “status/doctor pack reports”. Minimal AFK slice: when `.odm/agent-packs.json` lists a pack whose `path` is missing, `odm doctor` emits a **warn** (not fixable). Complements pack rm ([[issues-84-agent-pack-rm-core]]) for observation before cleanup.

## Affected

- `crates/odm-core/src/doctor.rs` and/or new `doctor_pack.rs` sibling (prefer extract if doctor.rs would exceed 1000)
- `crates/odm-core/src/agent_pack.rs` — reuse `pack_list` / registry load only
- Unit tests for doctor checks; optional thin CLI coverage if doctor integration pattern exists

## Impact

Stale registry entries are invisible until list is inspected carefully; agents get no doctor signal.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** `run_doctor` includes warn checks `pack_missing:<name>` for each registry pack whose destination path does not exist.

**Bindings:**

- Parent map: [[issues-82-post-v1-pack-lifecycle-hardening-map]]
- `pack_list` / registry; doctor check model (`DoctorCheck`, `CheckStatus::Warn`, `fixable`)
- Prior warn-only pattern: worktree orphan/dirty (`fixable: false`, `--fix` no-op)

**Desired behavior:**

1. After existing checks (or in a clear pack section), load pack registry via existing list/load API.
2. For each entry (stable sorted by name): if `path` does **not** exist → one check:
   - **id:** `pack_missing:<name>`
   - **status:** Warn
   - **fixable:** `false`
   - **message:** clear, includes name and path
3. Path exists (file, dir, or symlink — even broken symlink may “exist” as a link; treat `path.exists()` / symlink_metadata consistently: **prefer** “missing” when neither the path nor a symlink entry is present; if symlink exists but target missing, still **no** pack_missing for v1 — only absent path entry).
4. Empty registry / missing registry file → no pack checks.
5. `--fix` does **not** remove registry entries or files for these checks.
6. Unit tests: missing path → warn id; present path after install → no pack_missing; fixable false.
7. Keep `doctor.rs` ≤1000 if adding non-trivial code — extract `doctor_pack.rs` like `doctor_worktree.rs` when needed.
8. No status command changes.

**Acceptance criteria:**

- [x] Missing registry path → `pack_missing:<name>` warn, not fixable
- [x] Present path → no that check
- [x] `--fix` does not alter packs/registry for this check
- [x] Unit tests cover missing vs present
- [x] File sizes ≤1000 target / ≤1250 hard
- [x] `cargo test` green; clippy `-D warnings` clean on workspace

**Out of scope:**

- Pack fields on `odm status`
- Auto-rm on doctor --fix
- Marketplace / manifest
- CLI pack rm (separate ticket)

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Landed `doctor_pack.rs` with `pack_missing_checks` wired into `collect_checks` after worktree checks.

- Registry via `pack_list`; missing path (no path/symlink entry via `symlink_metadata`) → `pack_missing:<name>` Warn, `fixable: false`
- Dangling symlink present → no pack_missing; empty/missing/corrupt registry → soft-skip
- `--fix` does not touch registry or pack paths
- Unit tests: missing, present, empty, dangling symlink, corrupt soft-skip, doctor --fix no-op

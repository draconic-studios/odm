---
id: issues-63
title: "Split doctor worktree orphan checks out of doctor.rs"
description: "Extract worktree orphan doctor checks (+ tests) so doctor.rs stays ≤1000 LOC target."
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

# Split doctor worktree orphan checks out of doctor.rs

## Description

`crates/odm-core/src/doctor.rs` is ~984 LOC (target ≤1000, hard 1250). Worktree orphan scan + unit tests live in the same file. Extract to a sibling module without behavior change.

## Affected

- `crates/odm-core/src/doctor.rs`
- New module e.g. `crates/odm-core/src/doctor_worktree.rs` (or `doctor/worktree_orphan.rs` if you prefer a small `doctor/` dir — pick one style consistent with crate; prefer flat `doctor_worktree.rs` unless `doctor/` already exists)
- `crates/odm-core/src/lib.rs` — `mod` + re-exports only if needed (prefer `pub(crate)` use from doctor)
- Unit tests move with the orphan helpers

## Impact

Next doctor feature will breach the file-size target; harder reviews.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** refactor  
**Summary:** Move `worktree_orphan_checks` and its unit tests out of `doctor.rs`; `run_doctor` still calls the same function; public doctor API and check ids unchanged.

**Bindings:**

- Existing `worktree_orphan_checks` / orphan tests in `doctor.rs`
- `worktree_list`, `validate_slot_name`, `paths::worktree_slot_path`
- Parent map: [[issues-60-post-v1-polish-map]]
- Closed behavior: [[issues-57-doctor-worktree-orphans]]

**Behavior lock (no product change):**

1. Check id `worktree_orphan:<project>:<slot>`, `Warn`, `fixable: false`, configured Projects only — unchanged.
2. `DoctorReport` / `run_doctor` / `--fix` allowlist unchanged.
3. Public exports from `odm_core` stay the same for doctor types used by CLI (do not force new public orphan API unless already public).
4. `wc -l` on `doctor.rs` after move: **≤900** preferred, must be **≤1000**. New file also ≤1000.
5. TDD-friendly: move tests with code; all existing orphan tests still pass (same assertions).
6. `cargo test` + `cargo clippy --workspace --all-targets -- -D warnings` clean for touched crates.

**Acceptance criteria:**

- [x] Orphan logic lives outside monolithic `doctor.rs` body
- [x] `doctor.rs` ≤1000 LOC; no file >1250
- [x] Existing orphan unit tests pass (relocated OK)
- [x] No intentional behavior/JSON/check-id change
- [x] `cargo test` green; clippy `-D warnings` clean on workspace

**Out of scope:**

- New doctor checks
- status worktree fields ([[issues-65-status-worktree-slots]])
- Docs beyond a one-line comment if needed
- Splitting unrelated doctor sections

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Extracted `worktree_orphan_checks` + orphan unit tests into flat `crates/odm-core/src/doctor_worktree.rs` (`pub(crate)` only). `collect_checks` calls `crate::doctor_worktree::worktree_orphan_checks`. Public doctor API unchanged. LOC: `doctor.rs` 611, `doctor_worktree.rs` 389. `cargo test` + clippy `-D warnings` green.

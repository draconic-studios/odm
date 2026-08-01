---
id: issues-83
title: "Split worktree.rs to meet ≤1000 LOC target"
description: "Extract worktree unit tests (and/or prune helpers) so odm-core worktree module stays under file-size target."
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

# Split worktree.rs to meet ≤1000 LOC target

## Description

`crates/odm-core/src/worktree.rs` is ~1109 LOC (target ≤1000, hard 1250). Production APIs are fine; the bulk is an inline `mod tests`. Extract without behavior change (same pattern spirit as [[issues-63-doctor-split-worktree-orphan]]).

## Affected

- `crates/odm-core/src/worktree.rs`
- New sibling e.g. `crates/odm-core/src/worktree_tests.rs` via `#[cfg(test)] #[path = "worktree_tests.rs"] mod tests;` **or** split prune helpers into `worktree_prune.rs` if you prefer production split — pick one approach; prefer **test extract** if production stays ≤500 LOC
- `crates/odm-core/src/lib.rs` only if new public modules need wiring (prefer no public API change)

## Impact

Next worktree feature will breach the hard limit; harder reviews.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** refactor  
**Summary:** Bring `worktree.rs` under the ≤1000 LOC target with no product behavior change; keep all existing unit tests green (relocated OK).

**Bindings:**

- Parent map: [[issues-82-post-v1-pack-lifecycle-hardening-map]]
- Public API today: `validate_slot_name`, `worktree_list` / `add` / `rm` / `prune` / `prune_all`, outcome/info types — re-exported from `lib.rs`
- Prior split pattern: [[issues-63-doctor-split-worktree-orphan]] / `doctor_worktree.rs`

**Behavior lock (no product change):**

1. All public worktree functions and types keep the same signatures and semantics.
2. Unit tests retain the same assertions (may move file).
3. After change: `wc -l` on every touched `.rs` file **≤1000** preferred, must be **≤1250**. `worktree.rs` itself must be **≤1000**.
4. `cargo test` green; `cargo clippy --workspace --all-targets -- -D warnings` clean.
5. No CLI/docs/CHANGELOG required (pure refactor).

**Acceptance criteria:**

- [x] `crates/odm-core/src/worktree.rs` ≤1000 LOC
- [x] No new/edited Rust file >1250 LOC
- [x] Public worktree API unchanged for CLI/callers
- [x] Existing worktree unit tests still pass (relocated OK)
- [x] `cargo test` green; clippy `-D warnings` clean on workspace

**Out of scope:**

- New worktree features (prune flags, dirty, orphans)
- Pack / doctor product work
- Docs beyond a one-line module comment if needed

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Extracted inline `mod tests` from `crates/odm-core/src/worktree.rs` into sibling `worktree_tests.rs` via `#[cfg(test)] #[path = "worktree_tests.rs"] mod tests;`. Production stays in `worktree.rs` (~384 LOC); tests ~726 LOC. Public API and `lib.rs` re-exports unchanged. `cargo test` green; clippy `-D warnings` clean.

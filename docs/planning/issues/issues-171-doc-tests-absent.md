---
id: issues-171
title: "Zero doc-tests across all crates — docs can drift from public API"
description: "No crate has any `cargo test` doc-test; public API surfaces lack executable examples, so docs can silently drift."
status: open
issue-type: observation
severity: low
tags:
  - planning
  - issue
---

# Zero doc-tests across all crates — docs can drift from public API

## Description

`cargo test --workspace` reports `0 passed; 0 failed` for doc-tests in every
crate (`odm`, `odm-core`, `odm-progen`, `odm-git`, `odm-actions`). None of the
public API surfaces carry `///` doc examples (no `///` comments found in
`crates/odm-core/src/lib.rs`), so nothing executes the documented API as part
of the suite.

## Affected

- All workspace crates: `odm-core`, `odm-progen`, `odm-git`, `odm-actions`, `odm`
- Public lib seams (config, store, worktree, membership, inventory, paths, ops)

## Observed

- `cargo test --workspace` → 5× `Doc-tests <crate>` blocks, each `0 passed`
- Grep for `///` in public `lib.rs` surfaces finds none
- No `#![deny(missing_docs)]` or similar gate anywhere

## Impact

Without doc-tests, documented examples (if added) would be unverified and the
public API has no executable usage contract. Low severity today — no drift has
happened yet — but the seam is unprotected.

## Proposed Fix

- Add `///` doc examples with `assert`s to the highest-value public seams
  (config load, store open/find, worktree slot resolve, inventory walk)
- Prefer examples that double as behavior documentation; keep within file size
  budget (≤1000 LOC per file; split into new modules if needed)
- Optionally consider `#![deny(missing_docs)]` on `odm-core` only — evaluate
  noise before applying workspace-wide

## Comments

_(none yet)_

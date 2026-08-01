---
id: issues-98
title: "core-desk dogfood status worktree orphans"
description: "core-desk README + integration gate: status shows orphan then prune clears."
status: closed
issue-type: feature-request
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# core-desk dogfood status worktree orphans

## Description

Dogfood Workspace should document and gate orphan observation: create orphan dir → `status --json` lists it under project `worktree_orphans` → prune removes → status empty orphans.

## Affected

- `examples/core-desk/README.md` — worktree section
- `crates/odm/tests/core_desk.rs` — new or extended gate (file ≤1000/1250)

## Impact

Without dogfood, orphan status can regress unnoticed.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-96-project-info-worktree-orphans]]

## Agent Brief

**Category:** test  
**Summary:** Extend core-desk dogfood so orphan dirs appear on status (and optionally info) JSON and disappear after prune.

**Bindings:**

- Parent map: [[issues-94-post-v1-status-orphans-map]]
- Prior gates: `core_desk_prune_all_and_slot_dirty_gate`, orphan mkdir patterns in core-desk README
- Field shape from 95/96

**Desired behavior:**

1. **README:** in worktree dogfood section, show:
   - `mkdir -p worktrees/alpha/stale-orphan` (already present for doctor)
   - `odm status --json` → project alpha `worktree_orphans` contains `stale-orphan`
   - optional: `odm project info alpha --json` same
   - `odm project worktree prune alpha` (or prune --all) clears empty orphan
   - status orphans empty afterward
   - Keep doctor warn notes; do not claim doctor `--fix` deletes orphans.
2. **Integration test:** gate that after sync + mkdir orphan:
   - `status --json` has orphan name/path for alpha
   - prune removes empty orphan
   - status `worktree_orphans` empty (or omits that name)
   - Prefer one focused test; reuse existing core-desk temp harness helpers.
3. No product code unless a trivial bug blocks the gate (fix minimally; prefer filing if large).
4. `cargo test` green; file sizes OK.

**Acceptance criteria:**

- [x] core-desk README documents status orphans + prune clear
- [x] Integration gate asserts orphan on status then cleared after prune
- [x] Does not require doctor `--fix` to delete orphans
- [x] `cargo test` green; core_desk.rs within size limits

**Out of scope:**

- Reference docs (ticket 97)
- New worktree product features

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

core-desk dogfood for status worktree orphans:

- **README** orphan section: after mkdir, `status --json` shows alpha `worktree_orphans` (`stale-orphan`); optional info parity; after prune status orphans empty; doctor `--fix` still does not delete orphans.
- **Gate** `core_desk_status_orphans_gate`: sync → mkdir orphan → status/info assert `{name,path}` no dirty → prune alpha → disk gone → status without stale-orphan.
- Docs + tests only (693 LOC core_desk.rs); full `cargo test` green.

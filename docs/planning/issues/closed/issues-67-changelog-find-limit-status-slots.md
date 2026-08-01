---
id: issues-67
title: "CHANGELOG honesty for find --limit and status worktree_slots"
description: "Record landed find --limit and status worktree_slots in CHANGELOG Unreleased."
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

# CHANGELOG honesty for find --limit and status worktree_slots

## Description

`odm find --limit` and `odm status` registered `worktree_slots` landed (commits after polish map) but `CHANGELOG.md` **[Unreleased]** does not mention them. Operators reading the changelog miss two user-visible surfaces.

## Affected

- `CHANGELOG.md` only

## Impact

Docs drift; release notes incomplete when cutting the next version.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** docs  
**Summary:** Docs-only honesty. No Rust changes.

**Bindings:**

- Parent map: [[issues-66-post-v1-dogfood-slot-depth-map]]
- Landed behavior: `find --limit` (default 200, reject 0); status project `worktree_slots: [{name,path}]`
- Existing Unreleased bullets for generate / worktree / pack / prompt / doctor orphan — **append**, do not rewrite history of 0.1.0

**Behavior lock:**

1. Under `## [Unreleased]` → `### Added` (or `### Changed` if more accurate), add bullets that:
   - **`odm find --limit`** — max hits per Progen store; default **200**; `0` → usage exit `1`.
   - **`odm status` worktree slots** — each project JSON includes registered `worktree_slots` (`name` + `path`); human lists names when non-empty; orphans remain doctor-only.
2. Do not claim doctor dirty-slot or prune until those tickets land.
3. No other file churn required. Optional one-line touch to `phased-delivery.md` only if it still omits find limit / status slots in a way that contradicts reality — prefer **CHANGELOG-only** unless an obvious false claim exists.
4. `cargo test` still green (no code change expected).

**Acceptance criteria:**

- [x] CHANGELOG Unreleased mentions `find --limit` with default 200
- [x] CHANGELOG Unreleased mentions status registered `worktree_slots`
- [x] No false claims about unbuilt prune/dirty-doctor features
- [x] `cargo test` green

**Out of scope:**

- Rust / CLI changes
- Version bump or cutting a release
- Rewriting the 0.1.0 section

## Acceptance

Mirror Agent Brief checklist.

## Answer

Appended two **Added** bullets under `[Unreleased]` in `CHANGELOG.md`:

- **`odm find --limit`** — default 200; `0` → usage exit 1
- **`odm status` worktree slots** — registered `name`+`path`; human lists names; orphans stay doctor-only

No prune/dirty-doctor claims. Docs-only; `cargo test` green. CHANGELOG-only (no phased-delivery touch).

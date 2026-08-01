---
id: issues-61
title: "phased-delivery + CHANGELOG honesty after prompt/orphan"
description: "Fix phased-delivery still listing agent prompt as deferred; record doctor orphan + prompt in CHANGELOG Unreleased."
status: closed
issue-type: observation
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# phased-delivery + CHANGELOG honesty after prompt/orphan

## Description

`docs/reference/phased-delivery.md` still lists **Agent `start` / `prompt`** under deferred/sketch and says start/prompt remain open in the Actions historical note, but **`odm agent prompt` is v1 thin landed**. Doctor worktree orphan **warn** also landed and is missing from `CHANGELOG.md` [Unreleased].

## Affected

- `docs/reference/phased-delivery.md`
- `CHANGELOG.md` [Unreleased]
- Optionally one-line cross-check vs README status (do not rewrite README unless still wrong)

## Impact

Agents and humans treat prompt as unimplemented; release notes omit doctor orphan.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** docs  
**Summary:** Docs-only honesty. No Rust changes.

**Bindings:**

- Truth: `cli.md` full vs sketch (prompt full/thin; start sketch); `env-gen-packs.md` agent prompt section; `worktrees.md` doctor orphan warn; closed [[issues-57-doctor-worktree-orphans]], [[issues-58-agent-prompt-thin]], [[issues-59-agent-prompt-integration-docs]]
- Parent map: [[issues-60-post-v1-polish-map]]

**Edits:**

1. **`phased-delivery.md` Phase spine “Still deferred / sketch”:**
   - Remove `prompt` from deferred agent line — keep **`agent start`** only (or “Agent `start` (prompt is v1 thin — see Phase spine landed)”).
   - Ensure Phase spine **landed** list mentions agent prompt thin if not already (one bullet or clause).
   - Fix historical Actions “Out of this phase” line that still says “start/prompt … still open” so prompt is not claimed open.
2. **`CHANGELOG.md` [Unreleased]:**
   - Add doctor orphan warn under Added or Changed (Warn checks `worktree_orphan:…`, not fixable).
   - Confirm prompt bullet remains accurate; do not claim start implemented.
3. **Do not** invent new product features or reopen deferred worktree/generate/pack items.

**Verify:**

- `rg -n "prompt" docs/reference/phased-delivery.md` — no claim that prompt is still deferred/unimplemented as current truth.
- `rg -n "orphan|worktree_orphan|agent prompt" CHANGELOG.md` — Unreleased covers both.
- No `cargo` required if docs-only; if you touch nothing Rust, skip tests. If accidental Rust touch → `cargo test`.

**Acceptance criteria:**

- [x] phased-delivery deferred list does not group landed prompt with start as both deferred
- [x] phased-delivery landed/spine text acknowledges prompt v1 thin
- [x] CHANGELOG Unreleased mentions doctor worktree orphan warn
- [x] CHANGELOG still says agent start stubbed / not implemented
- [x] No Rust behavior change

**Out of scope:**

- cli.md find/context wording ([[issues-62-cli-find-context-docs-honesty]])
- Code, version bump, release

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Docs honesty only:

- **`phased-delivery.md`**: Phase spine landed now lists agent `prompt` v1 thin + doctor worktree orphan warn; deferred line is Agent `start` only (prompt pointed at landed); Actions historical out-of-phase note no longer claims prompt still open.
- **`CHANGELOG.md` [Unreleased]**: Added doctor worktree orphan warn (`worktree_orphan:…`, not fixable); existing agent prompt / start-stubbed bullets unchanged.
- README already accurate; no Rust changes.

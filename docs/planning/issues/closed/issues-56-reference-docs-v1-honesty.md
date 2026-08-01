---
id: issues-56
title: "Reference docs v1 honesty (architecture, multi-git, cli progen, init -i)"
description: "Align architecture/multi-git/config/cli with landed worktree/pack/generate; honest progen façade + init --interactive."
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

# Reference docs v1 honesty (architecture, multi-git, cli progen, init -i)

## Description

Post-0.1.0 features landed, but several **reference** docs still call worktrees/packs “sketch” or document a full progen store façade and `init --interactive` as if implemented. README/phased-delivery were fixed in [[issues-54-readme-post-010-docs-drift]]; this ticket finishes the honesty pass on remaining refs.

## Affected

- `docs/reference/architecture.md`
- `docs/reference/multi-git.md`
- `docs/reference/config.md` (only if it still marks generators/packs as pure sketch incorrectly)
- `docs/reference/cli.md` (progen store hot set; `init --interactive`)
- Optionally `docs/reference/worktrees.md` Related line if it still says packs are pure sketch only

## Impact

Implementers and agents treat stubs as shipped or miss that worktree/pack are v1.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** docs  
**Summary:** Docs-only honesty pass. No Rust behavior changes.

**Bindings:**

- Truth sources: `CHANGELOG.md` [Unreleased], `cli.md` full vs sketch matrix after packs, actual CLI (`ProgenCmd` in `crates/odm/src/cli.rs`), `main.rs` `init --interactive` → `not_implemented`
- Parent map: [[issues-55-post-v1-hardening-map]]
- Do not reopen design fundamentals; only correct depth markers and implemented lists

**Current drift (fix these):**

1. **architecture.md**
   - Layout comment / Related still say Worktree slots **(sketch)** while v1 is implemented
   - Crate blurb still implies agent pack is sketch-only in `odm-agent` while packs live in `odm-core` as v1 local
2. **multi-git.md**
   - Related / intro still “Worktree slots (sketch)” — should point to v1 + deferred in `worktrees.md`
3. **cli.md**
   - `init --interactive` documented under **full** as if it prompts; code exits `1` not-implemented — mark **not implemented / deferred** (keep flag reserved) OR note exit `1` until implemented
   - Progen “Re-exported / mapped store commands (hot set)” lists `refs`, `task|issue|idea|…`, `archive`, `log`, `glossary`, `plan`, `watch` as if present — **only** implemented today: lifecycle list/add/rm/info + store `get`, `body`, `ls`, `tree`, `backlinks`, `reindex`, `doctor` (plus top-level `find` / `context`). Split **implemented** vs **reserved/deferred**
4. **config.md** — only if a line still claims generators are sketch-only with no v1 local note; leave true deferred items alone

**Desired behavior:**

- Worktree + generate local + agent pack local described as **v1 landed** where those docs mention them; deferred remnants stay deferred
- `init --interactive`: honest not-implemented
- Progen façade list matches code
- No code changes; `cargo test` still green

**Acceptance criteria:**

- [x] `rg -n "Worktree slots \\(sketch\\)" docs/reference` finds no stale claims (or only historical phase text clearly marked)
- [x] architecture Related/crate notes do not call pack install/link/list a pure stub
- [x] cli.md progen section distinguishes implemented vs reserved verbs
- [x] cli.md states `init --interactive` is not implemented (exit 1) or deferred
- [x] No Rust/source changes under `crates/`
- [x] `cargo test` green

**Out of scope:**

- Implementing interactive init, agent prompt/start, extra progen verbs
- README (already done in 54)
- Graph productization

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Docs-only honesty pass on reference docs:

- **architecture.md**: worktree layout + Related → v1 + deferred; crate blurb notes packs live in `odm-core` (no `odm-agent` crate yet).
- **multi-git.md**: non-goals + Related → worktree v1 + deferred.
- **cli.md**: `init --interactive` marked not-implemented (exit 1); progen store façade split into implemented vs reserved/deferred; full/sketch matrix updated.
- **worktrees.md**: Related packs line → v1 local + start/prompt sketch.
- **config.md**: left alone (deferred items still true).
- No `crates/` changes; `cargo test` green.

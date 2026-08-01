---
id: issues-73
title: "phased-delivery + CHANGELOG honesty after slot depth"
description: "Record prune, dirty-slot doctor, find --limit, status/info worktree_slots; stop claiming GC deferred."
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

# phased-delivery + CHANGELOG honesty after slot depth

## Description

Map [[issues-66-post-v1-dogfood-slot-depth-map]] landed prune, dirty-slot doctor, find `--limit`, status + **project info** `worktree_slots`. `CHANGELOG.md` already has most Unreleased bullets but **omits project info slots**. `docs/reference/phased-delivery.md` Phase spine still says worktree is only add/list/rm, lists orphan warn alone, and still names **GC** under deferred worktree items — false now that `project worktree prune` exists.

## Affected

- `CHANGELOG.md`
- `docs/reference/phased-delivery.md`

## Impact

Agents and humans reading phased-delivery think orphan GC is still unbuilt; release notes miss `project info` slots.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** docs  
**Summary:** Docs-only honesty. No Rust changes.

**Bindings:**

- Parent map: [[issues-72-post-v1-honesty-dogfood-map]]
- Truth sources: `docs/reference/worktrees.md`, `docs/reference/cli.md`, existing CHANGELOG Unreleased bullets for prune / dirty / find limit / status slots
- Landed: `project worktree prune`, `worktree_dirty:*` doctor warn, `find --limit`, status + **info** registered `worktree_slots`

**Behavior lock:**

1. **`CHANGELOG.md` [Unreleased] → Added** (append, do not rewrite 0.1.0):
   - Bullet for **`odm project info` worktree slots** — same registered shape as status (`name` + `path`); empty array when none / non-git / soft-fail; human `worktrees: …` when non-empty.
   - Do **not** duplicate prune/dirty/find/status bullets if already present and accurate; only add missing info-slots claim.
2. **`docs/reference/phased-delivery.md`** post-0.1.0 Phase spine:
   - Worktree slots v1 line includes **prune** (not only add/list/rm) and points at `worktrees.md`.
   - Landed list includes: doctor **dirty-slot** warn; **find `--limit`**; status **and** project info registered `worktree_slots` (or fold into clear bullets without inventing features).
   - Deferred worktree line must **not** list bare **GC** as open work — prune landed; deferred remains config slots, pin↔slot, auto-prune on doctor, branch templates, multi-project prune, etc. (match `worktrees.md` Deferred).
3. No other files required. Do not touch `architecture.md` unless it still claims prune unbuilt (it should not).
4. `cargo test` green (no code change expected).

**Acceptance criteria:**

- [ ] CHANGELOG Unreleased mentions project info registered `worktree_slots`
- [ ] phased-delivery Phase spine mentions prune (not only add/list/rm)
- [ ] phased-delivery does not list orphan GC as still deferred
- [ ] phased-delivery records dirty-slot doctor and/or find limit / status+info slots as landed (honest, not exhaustive essay)
- [ ] `cargo test` green

**Out of scope:**

- Rust / CLI changes
- Version bump or cutting a release
- core-desk / root README (other tickets)
- Implementing deferred product

## Acceptance

Mirror Agent Brief checklist.

## Answer

Docs-only honesty after map-66 slot depth:

- **CHANGELOG** `[Unreleased]` Added: `odm project info` registered `worktree_slots` (`name` + `path`), same shape as status; empty when none / non-git / soft-fail; human `worktrees: …` when non-empty. Existing prune/dirty/find/status bullets left intact.
- **phased-delivery** Phase spine: worktree v1 includes **prune**; landed dirty-slot doctor, `find --limit`, status + project info `worktree_slots`; deferred worktree line drops bare **GC** and lists config slots / pin↔slot / auto-prune on doctor / branch templates / multi-project prune (prune + orphan/dirty warns noted as landed).

`cargo test` green (no Rust changes).

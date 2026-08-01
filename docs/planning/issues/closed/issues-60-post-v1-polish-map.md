---
id: issues-60
title: "Post-v1 polish map (docs honesty, doctor split, find limit, status slots)"
description: "Wayfinder map: remaining docs drift after prompt land, doctor file-size split, find --limit, status worktree slots."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Post-v1 polish map (docs honesty, doctor split, find limit, status slots)

## Destination

After post-v1 hardening (map [[issues-55-post-v1-hardening-map]]) closed, close the next AFK-ready polish slice:

1. **Docs honesty** — `phased-delivery.md` / `CHANGELOG` no longer treat landed `agent prompt` as deferred; doctor orphan warn recorded in Unreleased; `cli.md` find/context match real flags.
2. **Doctor module size** — keep `doctor.rs` under the ≤1000 LOC target by extracting worktree orphan checks.
3. **`odm find --limit`** — expose the existing per-store limit (today hardcoded `200`).
4. **`odm status` worktree slots** — report **registered** slots per Project (not orphans; doctor keeps orphan warn).

**Status:** complete (all child tickets closed).

## Notes

- **Authority:** `cli.md`, `worktrees.md`, `phased-delivery.md`, `env-gen-packs.md`, `CHANGELOG.md`, actual CLI in `crates/odm/src/cli.rs` / `main.rs`.
- **Prereqs landed:** worktree v1, generate local, agent pack local, agent prompt thin, doctor orphan warn.
- **Execution:** ticket close = decision + code/tests/docs as scoped. TDD for code tickets.
- **Standing prefs (AFK defaults):**
  - Docs tickets are docs-only (no Rust unless ticket says otherwise).
  - Status slots = registered git worktrees only (same filter as `worktree list`); no orphan/dirty in status.
  - Find `--limit` default remains `200` when flag omitted.
  - Do not implement graph, env, generate remote, pack marketplace, `agent start`, init interactive, GC/prune, pin↔slot, config-declared slots.

## Decisions so far

- Child tickets: [[issues-61-phased-delivery-changelog-honesty]] (closed), [[issues-62-cli-find-context-docs-honesty]] (closed), [[issues-63-doctor-split-worktree-orphan]] (closed), [[issues-64-find-limit-flag]] (closed), [[issues-65-status-worktree-slots]] (closed).
- Order preference: 61 → 62 (docs, unblocked) in parallel with 63; 64 and 65 independent of docs; no hard edges unless noted on tickets.
- **61:** `phased-delivery.md` + CHANGELOG Unreleased honesty — prompt v1 thin + doctor orphan warn recorded; `agent start` remains deferred/stubbed.
- **62:** `cli.md` find/context match shipped clap — no facet-flags/`--depth`; default find limit 200 documented; no Rust.
- **63:** `worktree_orphan_checks` + tests extracted to `doctor_worktree.rs` (`pub(crate)`); `doctor.rs` 611 LOC; public doctor API unchanged.
- **64:** `odm find --limit` (default 200, reject 0 usage exit 1) wired to existing per-store `limit_per`; cli.md updated.
- **65:** `odm status` project entities include registered `worktree_slots` (name/path via `worktree_list`); soft-fail → `[]`; progens omit field; human lists names when non-empty; orphans remain doctor-only.

## Not yet specified

- _(none for this map — deferred product work stays out of scope)_

## Out of scope

- `odm agent start`
- `init --interactive`
- Graph, env, generate remote/templating
- Pack manifest/marketplace
- Worktree GC/prune, config slots, pin↔slot, doctor dirty-slot checks
- Release version bump / GitHub release

## Blocked by

None

## Answer

All four destination items landed via children 61–65: docs honesty (phased-delivery/CHANGELOG + cli find/context), doctor worktree orphan extract, `find --limit`, and status registered worktree slots. Map complete.

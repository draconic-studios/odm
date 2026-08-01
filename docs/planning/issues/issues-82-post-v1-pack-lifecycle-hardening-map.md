---
id: issues-82
title: "Post-v1 pack lifecycle + worktree module hardening map"
description: "Wayfinder map: agent pack rm, doctor pack_missing warn, worktree.rs LOC split, docs + core-desk dogfood."
status: open
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Post-v1 pack lifecycle + worktree module hardening map

## Destination

After worktree observation map [[issues-77-post-v1-worktree-observation-map]] closed, pull the next AFK-ready hardening slice:

1. **Worktree module size** — `crates/odm-core/src/worktree.rs` is over the ≤1000 LOC target (~1109); split so production + tests stay maintainable.
2. **Agent pack `rm`** — install/link/list without remove leaves registry + home debris; add local `odm agent pack rm <name>`.
3. **Doctor pack missing** — warn when a registry entry’s `path` is gone (minimal doctor pack report from deferred list).
4. **Docs + dogfood** — reference docs / CHANGELOG / README honesty; core-desk exercises pack rm (+ doctor missing if easy).

## Notes

- **Authority:** `docs/reference/env-gen-packs.md`, `cli.md`, `worktrees.md`, `AGENTS.md` file-size target, landed `agent_pack` + `doctor_worktree` patterns.
- **Prereqs:** maps 50 (packs v1), 77 (worktree observation) closed.
- **Execution:** ticket close = decision + code/tests/docs as scoped. TDD for code tickets.
- **Standing prefs (AFK defaults):**
  - Do not implement graph, env, generate remote/templating, pack marketplace/manifest, config-declared packs, `agent start`, `init --interactive`.
  - Do not implement worktree deferred product (config slots, pin↔slot, auto-prune on doctor, branch templates, status orphan listing, global `--wt` depth).
  - Pack `rm` removes registry entry and best-effort deletes dest (install tree or link symlink); unknown name → exit `4`; missing dest still drops registry (exit `0`).
  - Doctor `pack_missing` is warn-only (`fixable: false`); `--fix` does not edit registry or delete anything pack-related.
  - No new crate; stay in `odm-core` + thin CLI.

## Decisions so far

- Child tickets: [[issues-83-worktree-module-split]], [[issues-84-agent-pack-rm-core]], [[issues-85-agent-pack-rm-cli]], [[issues-86-doctor-pack-missing]], [[issues-87-pack-lifecycle-docs-honesty]], [[issues-88-core-desk-pack-rm-dogfood]].
- Prefer order: 83 and 84 and 86 unblocked in parallel; 85 blocked by 84; 87 blocked by 84+85+86; 88 blocked by 84+85.
- **83 closed:** test extract — `worktree_tests.rs` via `#[path]`; `worktree.rs` ~384 LOC; public API unchanged.
- **84 closed:** `pack_rm(ws, name)` — registry drop + best-effort dest delete; missing dest ok; unknown → not_found/4; re-exported; unit tests; no CLI.
- **85 closed:** CLI `odm agent pack rm <name>` — human + `--json` entry DTO; unknown → exit 4; integration tests (install/link→rm).
- **86 closed:** `pack_missing_checks` in `doctor_pack.rs` — warn `pack_missing:<name>` when registry path absent (lexists via symlink_metadata); fixable false; `--fix` no-op; unit tests.
- **87 closed:** docs honesty — cli/env-gen-packs/phased-delivery/CHANGELOG/README cover pack rm + doctor pack_missing; README worktree quick includes `prune --all`; architecture untouched.

## Not yet specified

- _(none for this map — further deferred product stays out of scope)_

## Out of scope

- `odm agent start`, `init --interactive`
- Graph, env, generate remote/templating, pack marketplace/manifest/config declarations
- Worktree deferred product beyond module split
- Status listing of packs or pack fields on `odm status`
- Release version bump / GitHub release

## Blocked by

None

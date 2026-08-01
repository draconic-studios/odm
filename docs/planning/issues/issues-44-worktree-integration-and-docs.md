---
id: issues-44
title: "Worktree integration tests and docs"
description: "End-to-end git worktree tests plus reference doc updates for implemented slot CLI."
status: open
issue-type: feature-request
severity: low
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
  - ready-for-agent
---

# Worktree integration tests and docs

## Description

After lifecycle + `project git --wt` land, lock behavior with real-git integration tests and bring `worktrees.md` / `cli.md` in line so they are not “stub only”.

## Affected

- `crates/odm/tests/*`
- `docs/reference/worktrees.md`
- `docs/reference/cli.md` (project worktree section depth marker)
- Optional: `CHANGELOG.md` Unreleased note (if present pattern) — only if repo already uses Unreleased; else skip
- Optional one-liner in `examples/core-desk/README.md` — only if it stays offline-honest (slots need a real git primary; core-desk alpha may be path-only — **do not** force git into desk if fixtures are non-git)

## Impact

Regressions in worktree argv/path policy would slip; docs would still say sketch-only.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-42-worktree-slot-lifecycle]]
- [[issues-43-project-git-wt-resolve]]

## Agent Brief

**Category:** test + docs  
**Summary:** Add integration coverage for worktree add/list/rm and `project git --wt`; update reference docs to describe implemented v1 behavior without claiming deferred features.

**Bindings:**
- Implemented behavior from [[issues-41-odm-git-worktree-ops]], [[issues-42-worktree-slot-lifecycle]], [[issues-43-project-git-wt-resolve]]
- Map [[issues-40-worktree-slots-map]]
- Prior harness patterns: `crates/odm/tests/cli_init.rs`, `actions_run.rs`

**Integration tests (temp dir, real git on PATH):**
1. init workspace → project add with url **or** local bare/file remote fixture (match existing test style) so primary is a git repo
2. `project worktree add <proj> slot1` → directory `worktrees/<proj>/slot1` exists; `git rev-parse --is-inside-work-tree` ok
3. `project worktree list <proj>` shows `slot1` (human and `--json`)
4. `project git <proj> --wt slot1 -- rev-parse --show-toplevel` resolves to slot path
5. `project worktree rm <proj> slot1` removes worktree; list empty
6. `project git --wt missing` fails non-zero without creating paths
7. Non-git project (path-only checkout without .git) → worktree add fails

Skip or `#[ignore]` only if environment lacks git — prefer require git like other tests.

**Docs:**
- `worktrees.md`: keep sketch banner **or** retitle to note “v1 implemented + deferred”; document actual CLI flags (`list|add|rm`, `--branch`, `--force`); keep Deferred list honest (config slots, GC, pin↔slot, doctor orphans, branch templates)
- `cli.md`: mark `project worktree` as **full** (v1) or “implemented subset” with pointer to worktrees.md; `--wt` on `project git` no longer sketch-only for path binding
- Do **not** invent flag tables beyond what code supports
- No markdown tables (AGENTS.md)

**Acceptance criteria:**
- [ ] Integration test file (or module) covers add/list/rm + git --wt happy path and missing-slot failure
- [ ] `cargo test` green including new tests
- [ ] `worktrees.md` and `cli.md` describe implemented commands; deferred items still explicit
- [ ] Map [[issues-40-worktree-slots-map]] Decisions/Notes updated if anything drifted; close map only if all child tickets closed (this ticket may close the map in `## Answer` / Comments when done)

**Out of scope:**
- Generators, agent packs, graph
- doctor/status slot reporting
- core-desk structural change unless already git-backed and trivial

## Acceptance

- [ ] Agent Brief acceptance criteria all met

## Comments

Parent map: [[issues-40-worktree-slots-map]]

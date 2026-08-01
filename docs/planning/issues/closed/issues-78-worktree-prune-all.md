---
id: issues-78
title: "project worktree prune --all multi-project orphan GC"
description: "Add odm project worktree prune --all to prune orphans across every configured Project."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
---

# project worktree prune --all multi-project orphan GC

## Description

Per-project `odm project worktree prune <project>` landed ([[issues-71-worktree-orphan-prune]]). `worktrees.md` Deferred still lists **Multi-Project or Workspace-level … prune-all-projects**. Operators with many Projects need one command to GC orphans workspace-wide without scripting.

## Affected

- `crates/odm-core/src/worktree.rs` (or thin wrapper)
- CLI: `crates/odm/src/cli.rs`, `main.rs`, `commands/worktree.rs`
- Tests: core unit + `crates/odm/tests/cli_worktree.rs`
- Docs touch: `cli.md`, `worktrees.md`; CHANGELOG Unreleased bullet (full honesty pass is [[issues-80-worktree-observation-docs-honesty]])

## Impact

Multi-project desks accumulate orphans per project; agents must loop project names today.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** `odm project worktree prune --all [--force]` prunes orphans for every configured Project using the same rules as per-project prune.

**Bindings:**

- Parent map: [[issues-77-post-v1-worktree-observation-map]]
- Orphan definition + empty/`--force` semantics: **identical** to `worktree_prune` / [[issues-71-worktree-orphan-prune]]
- Never delete registered worktrees or Primary

**Behavior lock:**

1. **CLI clap:**
   ```text
   odm project worktree prune <project> [--force]
   odm project worktree prune --all [--force]
   ```
   - Positional `project` and `--all` are **mutually exclusive**.
   - Exactly one required: either `project` **or** `--all` (clap group / required_unless).
   - Bare `prune` with neither → clap usage exit `1`.
   - `prune --all PROJECT` or both → usage error.

2. **`--all` core behavior:**
   - Iterate `ws.config.projects` keys in **sorted name order**.
   - For each project: if primary missing / not a git repo → **skip** (no hard fail; same soft spirit as doctor scanning). Do not add a failure row.
   - For each git project: call existing `worktree_prune` (or shared helper) with the same `force` flag.
   - Aggregate all `pruned` and `skipped_nonempty` across projects.

3. **Exit codes:**
   - `0` if no non-empty orphans left skipped.
   - `3` if any project contributed `skipped_nonempty` (partial empties still removed) — same as single-project.
   - Unknown single `project` still usage `1`; non-git single project still operation `3` (unchanged).

4. **JSON (`--all` success path):**
   ```json
   {
     "all": true,
     "pruned": [ { "project", "name", "path" } ],
     "skipped_nonempty": [ { "project", "name", "path" } ]
   }
   ```
   - `path` remains `worktrees/<project>/<slot>`.
   - Include `skipped_nonempty` always (empty array when none) so agents can see partial failure without scraping stderr.
   - Single-project JSON **unchanged**: `{ "project", "pruned": [ { "name", "path" } ] }` (no need to add skipped to single-project JSON unless already present — do not break existing shape).

5. **Human (`--all`):** summarize total pruned count/names (qualified `project/slot` or `project:slot`) and note skipped non-empty if any.

6. **TDD:**
   - Two projects each with an empty orphan → `--all` removes both; exit `0`.
   - One non-empty orphan without `--force` → exit `3`; empty orphans in other projects still removed.
   - `--force` removes non-empty across projects.
   - Registered slots never deleted.
   - Non-git project in config does not fail `--all`.
   - Mutual exclusion: `prune --all alpha` / bare `prune` → usage.

7. **Docs (minimal in this ticket):** command tree line in `cli.md` + one sentence in `worktrees.md` CLI section for `--all`. CHANGELOG Unreleased bullet. Spine/deferred cleanup can wait for issues-80 if cleaner.

8. `cargo test` green.

**Acceptance criteria:**

- [x] `prune --all` removes empty orphans across configured git projects
- [x] Exit `3` when any non-empty orphan remains without `--force` (partial OK)
- [x] `--force` recursive-deletes orphans only; registered untouched
- [x] `project` and `--all` mutually exclusive; bare prune usage error
- [x] Non-git projects skipped under `--all` without failing the run
- [x] JSON/human shapes stable; cli.md + worktrees.md + CHANGELOG touched
- [x] Single-project prune behavior and JSON unchanged
- [x] `cargo test` green

**Out of scope:**

- Auto prune on `doctor --fix`
- Config-declared slots, pin↔slot, branch templates
- Dirty observation (issues-79)
- core-desk dogfood (issues-81)
- Status orphan listing

## Acceptance

Mirror Agent Brief checklist.

## Answer

Landed `worktree_prune_all` + CLI `odm project worktree prune --all [--force]`.

- Core walks config projects in sorted order, reuses `worktree_prune`, soft-skips NotFound/Operation (missing primary / non-git / list fail).
- Aggregate JSON `{ all, pruned[{project,name,path}], skipped_nonempty[...] }`; human uses qualified `project/slot`.
- Exit `0` / `3` on any skipped nonempty (same as single-project). Single-project prune JSON/behavior unchanged.
- Clap: `project` XOR `--all` (`required_unless_present` / `conflicts_with`); bare/`--all`+project → clap usage (exit 2, repo convention).
- Docs: `cli.md`, `worktrees.md`, CHANGELOG Unreleased.
- Tests: core unit + CLI integration green under full `cargo test`.

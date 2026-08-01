---
id: issues-41
title: "odm-git worktree shell ops"
description: "Add git worktree add/list/remove to odm-git behind CommandRunner."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
  - ready-for-agent
---

# odm-git worktree shell ops

## Description

`odm-git` has clone/fetch/run but no worktree primitives. Slot lifecycle needs shell-out `git worktree add|list|remove` with the same absolute-path and error rules as existing ops.

## Affected

- `crates/odm-git` (`Git`, `GitError`, tests)
- Downstream: core worktree lifecycle (next ticket)

## Impact

Without this, CLI worktree commands cannot share the injectable runner / error map.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feature  
**Summary:** Extend `odm-git::Git` with worktree add, list, and remove operations used by ODM slot orchestration. Keep shell-out-only; no libgit2.

**Bindings:**
- `docs/reference/worktrees.md` (git owns worktree mechanics)
- Closed [[issues-16-odm-git-shell-contract]] (absolute paths, `git -C`, `CommandRunner`, `GitError`)
- Parent map [[issues-40-worktree-slots-map]]

**Current behavior:**
- `Git` exposes is_repo, init, clone, fetch, head_sha, is_clean, origin_url, checkout_detached, run
- No worktree subcommands

**Desired behavior:**
- **`worktree_add(primary: &Path, slot_path: &Path, branch: Option<&str>)`**
  - Both paths absolute (existing `require_absolute`)
  - `branch` **None:** `git -C <primary> worktree add -- <slot_path>` (plain git default branch/path behavior)
  - `branch` **Some(b):** `git -C <primary> worktree add -b <b> -- <slot_path>` (create new branch `b` at slot; if git fails because branch exists, surface `Failed` — do not invent reset policy)
  - Parent dirs of `slot_path`: **caller** creates if needed (git may require parent to exist — document; core will `create_dir_all` on `worktrees/<project>/` only)
  - Capture stderr on failure like other library ops
- **`worktree_list(primary: &Path) -> Result<Vec<WorktreeEntry>, GitError>`**
  - Run `git -C <primary> worktree list --porcelain` (or equivalent stable parse)
  - Each entry: at least `path` (absolute as git reports) and optional `head` / `branch` if cheap to parse; minimum required for ODM: **path** string
  - Parse robustly; malformed lines → `Parse` or skip with tests locking chosen behavior (prefer fail on garbage porcelain if rare)
- **`worktree_remove(primary: &Path, slot_path: &Path, force: bool)`**
  - `git -C <primary> worktree remove [--force] -- <slot_path>`
  - force true → pass `--force`
- Errors: reuse `NotAbsolute`, `Failed{operation, …}`, `GitNotFound`; operation names like `worktree_add` / `worktree_list` / `worktree_remove`
- Unit tests with fake `CommandRunner` asserting argv shapes and parse of a fixture porcelain blob
- Do **not** change clone/fetch semantics

**Acceptance criteria:**
- [x] Public `worktree_add` / `worktree_list` / `worktree_remove` on `Git`
- [x] Absolute path enforcement on primary and slot paths
- [x] Argv contracts covered by unit tests (including `-b` and `--force`)
- [x] Porcelain list parse returns paths for a multi-entry fixture
- [x] `cargo test -p odm-git` and workspace `cargo test` green
- [x] No CLI wiring in this ticket

**Out of scope:**
- ODM slot name validation / `worktrees/` layout
- CLI commands
- Prune / lock repair / `worktree move`
- Pin interaction

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Landed `WorktreeEntry` plus `Git::worktree_add` / `worktree_list` / `worktree_remove` in `odm-git`, shell-out via `CommandRunner` with the same absolute-path and `GitError` rules as existing ops.

- **add:** `git -C <primary> worktree add [-b <branch>] -- <slot_path>`; parent dirs are caller responsibility
- **list:** porcelain parse → `path` + optional `head` / `branch`; garbage record without path → `Parse`
- **remove:** `git -C <primary> worktree remove [--force] -- <slot_path>`
- Unit tests in `crates/odm-git/tests/worktree_ops.rs` with fake runner (argv, abs paths, porcelain, failures)
- Unblocks [[issues-42-worktree-slot-lifecycle]]

## Comments

Parent map: [[issues-40-worktree-slots-map]]

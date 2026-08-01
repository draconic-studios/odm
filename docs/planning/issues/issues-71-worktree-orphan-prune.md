---
id: issues-71
title: "project worktree prune removes orphan slot dirs"
description: "odm project worktree prune deletes doctor-warned orphan dirs under worktrees/<project>/."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# project worktree prune removes orphan slot dirs

## Description

Doctor warns on orphan `worktrees/<project>/<slot>/` dirs that are not registered git worktrees, but cleanup is manual filesystem work. `worktrees.md` Deferred includes GC/prune. Add an **explicit** prune command (not `doctor --fix`) so agents can remove orphans safely.

## Affected

- `crates/odm-core/src/worktree.rs` (or adjacent module)
- CLI: `crates/odm/src/cli.rs`, `main.rs`, commands/worktree DTOs
- Integration tests `cli_worktree.rs`
- `docs/reference/cli.md`, `worktrees.md`; CHANGELOG Unreleased

## Impact

Orphans accumulate after failed adds or manual deletes of git worktrees; no first-class cleanup path.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** `odm project worktree prune <project> [--force]` removes **orphan** slot directories only.

**Bindings:**

- Parent map: [[issues-66-post-v1-dogfood-slot-depth-map]]
- Orphan definition: **same as doctor** — directory under `worktrees/<project>/` with a valid slot name that is **not** in `worktree_list` registered set; configured Project; primary must be git (else operation error exit `3` like other worktree verbs)
- Never delete registered worktrees; never touch Primary; never delete `worktrees/<other-project>/`

**Behavior lock:**

1. **CLI:**
   ```text
   odm project worktree prune <project> [--force]
   ```
2. **Default (no `--force`):** remove orphan slot dirs only if they are **empty** (or only contain ignorable empty structure — prefer: `remove_dir` succeeds only when empty). Non-empty orphan → skip that slot with a clear message, overall exit `3` if any orphan was skipped as non-empty (or collect and fail at end). Prefer: if any non-empty orphan exists without force → exit `3` and delete nothing **or** delete empties and fail if any non-empty remained — **lock: delete all empty orphans; if any non-empty orphan remains, exit `3` after empties removed** (partial progress OK) unless you choose all-or-nothing — **prefer partial: remove empties, then exit 3 if any non-empty orphan left**.
3. **`--force`:** delete orphan dirs recursively (`remove_dir_all`) even if non-empty. Still **never** delete registered worktree paths.
4. **No orphans:** success exit `0`; human message like `pruned 0 orphan worktree dirs`; JSON `{ "project", "pruned": [] }`.
5. **JSON success:** `{ "project": "<name>", "pruned": [ { "name", "path" } ] }` paths relative `worktrees/<project>/<slot>`.
6. **Human:** list pruned slot names or count.
7. Unknown project → usage exit `1`. Non-git primary → operation exit `3`.
8. Best-effort remove empty `worktrees/<project>/` if it becomes empty after prune (same spirit as `worktree rm`).
9. TDD: empty orphan removed without force; non-empty without force → not fully cleaned + non-zero; force removes non-empty orphan; registered slot path untouched even with force.
10. Docs: cli.md command tree + worktrees.md (prune is the manual GC; doctor --fix still does not delete).
11. CHANGELOG Unreleased bullet.
12. `cargo test` green.

**Acceptance criteria:**

- [ ] `project worktree prune <project>` removes empty orphan dirs
- [ ] Non-empty orphan without `--force` does not recursive-delete; exit non-zero if any remain
- [ ] `--force` recursive-deletes orphans only
- [ ] Registered worktrees never deleted
- [ ] JSON + human shapes stable; cli.md + worktrees.md + CHANGELOG updated
- [ ] `cargo test` green

**Out of scope:**

- Auto prune on doctor --fix
- Prune all projects in one command (YAGNI; per-project only)
- Config-declared slots, pin↔slot
- Dirty registered slot cleanup (doctor warn only)

## Acceptance

Mirror Agent Brief checklist.

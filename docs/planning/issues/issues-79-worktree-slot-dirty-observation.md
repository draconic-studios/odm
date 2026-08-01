---
id: issues-79
title: "worktree slot dirty on list/status/info"
description: "Expose dirty bool on registered worktree slots in list, status, and project info."
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

# worktree slot dirty on list/status/info

## Description

Doctor warns on dirty registered slots (`worktree_dirty:…`). `status` / `project info` / `worktree list` only expose `{ name, path }`. `worktrees.md` Deferred lists **status obligations for … dirty slots**. Agents need dirty on the observation surfaces without parsing doctor output.

## Affected

- `crates/odm-core/src/worktree.rs` — `WorktreeSlotInfo`
- `status.rs`, project info path, list path
- CLI DTOs/human formatters: `commands/worktree.rs`, `commands/status.rs`, `commands/project.rs`
- Tests: core unit + CLI integration as needed
- Docs touch: `cli.md`, `worktrees.md`; CHANGELOG Unreleased (full honesty → issues-80)

## Impact

Dirty slots are invisible to `status --json` consumers until doctor runs.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Registered worktree slots include dirty observation on list, status, and project info.

**Bindings:**

- Parent map: [[issues-77-post-v1-worktree-observation-map]]
- Dirty definition: same as doctor — `git.is_clean(abs_slot_path)` → dirty when `Ok(false)`; probe errors → unknown
- Orphans: still **not** on status/list (doctor + prune only)

**Behavior lock:**

1. **Extend `WorktreeSlotInfo`:**
   ```rust
   pub struct WorktreeSlotInfo {
       pub name: String,
       pub path: String,
       /// `Some(true)` dirty, `Some(false)` clean, `None` if cleanliness probe failed.
       pub dirty: Option<bool>,
   }
   ```
   - Serde JSON: always emit `dirty` (`true` / `false` / `null`).

2. **Where filled:**
   - `worktree_list`: after collecting registered slots, for each slot probe `is_clean` on absolute slot path; set `dirty` accordingly. Soft: on `is_clean` err → `dirty: None` (do not fail list).
   - `build_status` / `project info` already use `worktree_list` (or equivalent) — they inherit the field. Soft-fail empty list stays `[]` with no slots.

3. **Prune/add/rm outcomes:** do **not** require `dirty` on action/prune DTOs. If `WorktreeSlotInfo` is reused in prune `pruned` arrays, set `dirty: None` (unknown / irrelevant) rather than probing deleted paths. Prefer a slim prune row type **only if** adding `dirty` to prune JSON would break tests — otherwise `dirty: null` on prune rows is OK and simpler.

4. **Human output:**
   - `worktree list`: one slot per line; if `dirty == Some(true)` suffix ` dirty` (e.g. `feat dirty`); clean/unknown → name only (unknown does not claim dirty).
   - `status` / `project info`: when listing slot names, mark dirty slots the same way (`feat dirty`) or `worktrees: feat dirty, other`. Keep primary entity `dirty` field unchanged.

5. **JSON shapes:**
   - List: `{ "project", "slots": [ { "name", "path", "dirty" } ] }`
   - Status projects / project info: `worktree_slots: [ { "name", "path", "dirty" } ]`
   - Update unit tests that construct `WorktreeSlotInfo` literals.

6. **TDD:**
   - Clean registered slot → `dirty: Some(false)` / JSON `false`.
   - Dirty registered slot → `dirty: Some(true)`; human list shows ` dirty`.
   - `is_clean` error → `dirty: None` / JSON `null`; list still succeeds.
   - Status JSON includes `dirty` on slot objects.
   - Doctor behavior unchanged (no requirement to dedupe logic into one function, but DRY if natural — e.g. shared helper `slot_dirty(git, path) -> Option<bool>`).

7. **Docs (minimal):** cli.md status/list/info JSON field notes; worktrees.md rules one-liner that status/list/info report dirty on registered slots. CHANGELOG Unreleased bullet.

8. `cargo test` green.

**Acceptance criteria:**

- [ ] `WorktreeSlotInfo` includes `dirty: Option<bool>` with JSON true/false/null
- [ ] `worktree list`, `status`, `project info` populate dirty via `is_clean` (soft on probe err)
- [ ] Human list/status/info mark dirty slots; clean/unknown unmarked as dirty
- [ ] Orphans still absent from status/list
- [ ] Existing doctor dirty warn still passes
- [ ] cli.md + worktrees.md + CHANGELOG touched
- [ ] `cargo test` green

**Out of scope:**

- Cleaning/stashing dirty slots
- Doctor `--fix` changes
- Multi-project prune (issues-78)
- Status orphan listing
- core-desk dogfood (issues-81)

## Acceptance

Mirror Agent Brief checklist.

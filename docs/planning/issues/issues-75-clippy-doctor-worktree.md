---
id: issues-75
title: "Clippy clean doctor_worktree"
description: "Fix clippy for_kv_map and single_match warnings in odm-core doctor_worktree.rs."
status: open
issue-type: feature-request
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# Clippy clean doctor_worktree

## Description

`cargo clippy --all-targets` warns on `crates/odm-core/src/doctor_worktree.rs`:

- `for_kv_map` — iterate `ws.config.projects.keys()` instead of `(project, _)`
- `single_match` — prefer `if let Ok(false) = git.is_clean(...)` over match with empty arms

No behavior change intended.

## Affected

- `crates/odm-core/src/doctor_worktree.rs`

## Impact

Noise in local clippy; small maintainability debt.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** chore / refactor  
**Summary:** Apply clippy-suggested cleanups only in `doctor_worktree.rs`. Keep doctor orphan/dirty semantics identical.

**Bindings:**

- Parent map: [[issues-72-post-v1-honesty-dogfood-map]]
- Reproduce: `cargo clippy --all-targets -p odm-core -- -D warnings` (or project-wide clippy) should currently warn on those two lints
- Existing unit tests in the same file cover orphan + dirty behavior — must stay green

**Behavior lock:**

1. Fix `for_kv_map` and `single_match` (and any identical duplicates in the same file) without changing check ids, messages, fixable flags, or soft-skip on probe errors.
2. Do not drive-by refactor other modules.
3. `cargo test` green.
4. `cargo clippy --all-targets -p odm-core` has **no** warnings from `doctor_worktree.rs` (prefer zero warnings for that package if easy).

**Acceptance criteria:**

- [ ] `for_kv_map` / `single_match` warnings gone from `doctor_worktree.rs`
- [ ] Doctor orphan/dirty tests still pass
- [ ] `cargo test` green
- [ ] No intentional behavior change to check ids or fixable flags

**Out of scope:**

- Enabling `-D warnings` in CI (no CI)
- Broad clippy campaign across the monorepo
- Docs changes

## Acceptance

Mirror Agent Brief checklist.

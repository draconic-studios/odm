---
id: issues-143
title: "odm-git real-git worktree integration test"
description: "worktree_ops.rs is mock-only; add one real-git tempfile test for add/list/remove at the git crate seam."
status: reviewing
issue-type: feature-request
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
---

# odm-git real-git worktree integration test

## Description

`crates/odm-git/tests/worktree_ops.rs` uses RecordingRunner only. Real git worktree behavior is covered higher up; crate seam can drift.

## Affected

- `crates/odm-git/tests/` new or extend git_ops/worktree

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** test  
**Summary:** One tempfile integration: init repo, commit, worktree_add, worktree_list, worktree_remove against real git.

**Bindings:**

- Parent: [[issues-120-test-coverage-map]]
- Pattern: `tests/git_ops.rs`

**Desired behavior:**

1. Real git required (skip or fail clearly if missing — match git_ops style).
2. add slot path; list contains it; remove cleans.
3. Keep mock tests for argv contracts.
4. No network.

**Acceptance criteria:**

- [ ] Real-git worktree round-trip test
- [ ] `cargo test -p odm-git` green

**Out of scope:** non-interactive env product change (133).

## Acceptance

- [ ] Agent Brief acceptance criteria all met

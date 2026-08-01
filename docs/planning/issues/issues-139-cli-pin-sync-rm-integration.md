---
id: issues-139
title: "CLI integration: pin force, sync named, project/progen rm"
description: "Thin binary coverage for pin apply --force, named sync, and membership rm --delete/--force."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
---

# CLI integration: pin force, sync named, project/progen rm

## Description

Pin apply force, named sync fail paths, and project/progen rm delete/force are unit-covered at best; binary seam is smoke-only via core_desk happy path.

## Affected

- New or extended: `crates/odm/tests/cli_pin.rs`, `cli_sync.rs`, or `cli_membership.rs`
- Commands: pin apply/status, sync, project rm, progen rm

## Proposed Fix

See Agent Brief.

## Blocked by

None (pairs well with 138)

## Agent Brief

**Category:** test  
**Summary:** assert_cmd integration tests for mutate paths that lack CLI gates.

**Bindings:**

- Parent: [[issues-120-test-coverage-map]]
- Patterns: `cli_init.rs`, `cli_worktree.rs` harness helpers

**Desired behavior:**

1. **pin:** dirty tree apply → exit 3; `--force` → 0; `--json pin status` has stable fields; named subset if easy.
2. **sync:** `sync <known>` ok; unknown name → exit 1; optional JSON results shape.
3. **project rm:** default keeps tree; `--delete` removes clean tree; dirty needs `--force`.
4. **progen rm:** undeclare; `--delete` removes path when clean.
5. New file(s) if existing files near LOC limit — do not push `core_desk.rs` over 1000.
6. Temp bare fixtures or reuse init patterns.

**Acceptance criteria:**

- [ ] Each area has at least one success + one primary failure CLI test
- [ ] `cargo test -p odm` green
- [ ] File size limits respected

**Out of scope:** full exit matrix (140); core-desk dogfood script (145).

## Acceptance

- [ ] Agent Brief acceptance criteria all met

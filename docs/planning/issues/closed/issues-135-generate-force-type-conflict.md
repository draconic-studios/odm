---
id: issues-135
title: "generate --force handles file/dir type conflicts"
description: "force overwrite fails mid-copy on file↔directory conflicts leaving a partial tree; symlinks already remove conflicts."
status: closed
issue-type: bug
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
---

# generate --force handles file/dir type conflicts

## Description

`copy_tree` under force uses `fs::copy` / `create_dir_all` without removing conflicting opposite types. Symlink path removes dest first. Partial trees on type conflict.

## Affected

- `crates/odm-core/src/generate.rs` `copy_tree` / force path

## Impact

`--force` is not reliable when dest layout diverges by type; leaves debris.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Before writing each path under force, if dest exists with conflicting type (file vs dir), remove it (file or `remove_dir_all`) like symlink handling; or fail-fast before any copy with clear error — prefer **per-path remove then write** for force semantics consistency.

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]
- Contrast `copy_symlink` conflict removal

**Desired behavior:**

1. force + dest dir where template has file → succeeds (dir removed, file written).
2. force + dest file where template has dir → succeeds.
3. Without force, existing non-empty dest still requires force (unchanged).
4. dry-run still writes nothing.
5. Unit tests for both type conflicts under force.

**Acceptance criteria:**

- [x] Type conflicts succeed under --force
- [x] Tests cover file↔dir both ways
- [x] `cargo test -p odm-core` green

**Out of scope:** remote generators; variable substitution.

## Acceptance

- [x] Agent Brief acceptance criteria all met

---
id: issues-127
title: "Align missing --wt / project path exit codes to 4"
description: "run treats missing worktree/project path as usage exit 1; project git and docs use not_found exit 4."
status: open
issue-type: bug
severity: high
tags:
  - planning
  - issue
  - ready-for-agent
---

# Align missing --wt / project path exit codes to 4

## Description

`cli.md` / `worktrees.md`: missing slot → exit `4`. `project git` uses `OdmError::not_found`. `odm-actions` `resolve_cwd` uses `OdmError::usage` for missing worktree path and missing project dir → exit `1`.

## Affected

- `crates/odm-actions/src/lib.rs` resolve_cwd
- `crates/odm/tests/actions_run.rs`
- Docs already specify 4

## Impact

Agents cannot use “exit 4 = create slot then retry” on `run`.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Missing on-disk project primary or worktree slot path → `not_found` (exit 4). Unknown **names** stay `usage` (exit 1).

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]
- Spec: `docs/reference/cli.md` exit codes; `worktrees.md` missing slot → 4
- Contrast: `membership.rs` project_git already not_found

**Desired behavior:**

1. Unknown project **name** → usage / exit 1 (unchanged).
2. Known project, path missing on disk → not_found / exit 4.
3. Known project, `--wt` slot missing on disk → not_found / exit 4.
4. `--wt` without `--project` → usage (unchanged).
5. Integration tests in `actions_run.rs` assert exit 4 for missing slot and missing project path.
6. Unit tests in odm-actions if present.

**Acceptance criteria:**

- [ ] Missing slot on `run` → exit 4
- [ ] Missing project path on `run` → exit 4
- [ ] Unknown project name still exit 1
- [ ] Tests green

**Out of scope:** clap parse exit (129); dual global/local `--wt` precedence (can fold tiny hard-error into 132 if easy).

## Acceptance

- [ ] Agent Brief acceptance criteria all met

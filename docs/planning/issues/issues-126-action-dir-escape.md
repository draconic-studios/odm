---
id: issues-126
title: "Action task dir must stay under Workspace root"
description: "task.dir uses root.join without resolve_under_root; actions can run with cwd outside the Workspace."
status: open
issue-type: bug
severity: high
tags:
  - planning
  - issue
  - ready-for-agent
---

# Action task dir must stay under Workspace root

## Description

`odm-actions` `resolve_cwd` does `ws.root.join(dir)` for task `dir`. No `..`/absolute check. Bundle load does not validate `dir` either. Spec: dir is relative to Workspace root.

## Affected

- `crates/odm-actions/src/lib.rs` ~76
- `crates/odm-core/src/config.rs` action task load (~275–281)

## Impact

Compromised or careless Action YAML runs shell tasks outside the Workspace.

## Proposed Fix

See Agent Brief.

## Blocked by

None (can land with or after 124)

## Agent Brief

**Category:** fix  
**Summary:** Validate and resolve action `dir` under Workspace root at load and/or runtime via `resolve_under_root`.

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]
- Prefer validate at **bundle load** (fail workspace load) **and** resolve safely at runtime (defense in depth)

**Desired behavior:**

1. `dir: ../outside` or absolute → reject (workspace error on load, or operation/usage at run if only runtime).
2. Prefer load-time reject so `odm doctor`/`status` surfaces bad config early.
3. Valid `dir: projects/alpha` still works when path exists.
4. Unit tests in odm-core (load) and/or odm-actions (resolve).
5. Missing-but-valid-relative dir can keep existing missing-path behavior.

**Acceptance criteria:**

- [ ] Escaping/absolute `dir` rejected
- [ ] In-workspace relative `dir` still works
- [ ] Tests cover escape + happy path
- [ ] `cargo test -p odm-core -p odm-actions` green

**Out of scope:** Windows shell portability; run --json stdio (128).

## Acceptance

- [ ] Agent Brief acceptance criteria all met

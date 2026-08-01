---
id: issues-125
title: "membership_add validates path escape before save"
description: "project/progen add accepts ../ paths, writes config, then load_workspace fails — bricks the Workspace."
status: closed
issue-type: bug
severity: high
tags:
  - planning
  - issue
  - ready-for-agent
---

# membership_add validates path escape before save

## Description

`membership_add` rejects empty/absolute paths but not `../outside`. Config is saved; later `load_workspace` rejects escape. Workspace becomes unusable until manual YAML edit.

## Affected

- `crates/odm-core/src/membership.rs`
- `config.rs` `validate_rel_path`

## Impact

One bad `project add` / `progen add` bricks the Workspace for all commands.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Validate membership paths with the same `validate_rel_path` / `resolve_under_root` rules as config load **before** mutate/save.

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]
- Contrast load validation in `config.rs`

**Desired behavior:**

1. `project_add` / `progen_add` with path `../outside` → error, **config unchanged**.
2. Absolute path still rejected (existing).
3. Valid relative path still adds.
4. Unit tests: escape rejected + config file unchanged; happy path unchanged.
5. No full transactional rollback of failed clone required in this ticket (documented write-then-clone stays); only pre-save path validation.

**Acceptance criteria:**

- [x] Escaping path never written to config
- [x] Unit tests prove config unchanged on reject
- [x] `cargo test -p odm-core` green

**Out of scope:** clone rollback on materialize failure; name uniqueness (130).

## Acceptance

- [x] Agent Brief acceptance criteria all met

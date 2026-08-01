---
id: issues-124
title: "Action/generator bundle paths must resolve under root"
description: "Bundle paths use root.join without resolve_under_root; absolute and .. paths read outside the Workspace."
status: closed
issue-type: bug
severity: high
tags:
  - planning
  - issue
---

# Action/generator bundle paths must resolve under root

## Description

`load_action_bundles` / `load_generator_bundles` join bundle paths with `root.join(rel)` only. Absolute paths and `../` escape the Workspace contrary to config docs (“relative to Workspace root”).

## Affected

- `crates/odm-core/src/config.rs` ~248, ~294
- `docs/reference/config.md`

## Observed

`actions: { core: /tmp/evil.yaml }` or `../outside/actions.yaml` can load if the file parses.

## Impact

Untrusted or mistaken config can pull task definitions from outside the Workspace.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Resolve every action/generator bundle path with `resolve_under_root`; reject absolute/`..` escapes; unit tests.

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]
- Helper: `paths::resolve_under_root` (same as project path validation)
- Project paths already validated via `validate_rel_path`

**Desired behavior:**

1. Bundle path must be relative and stay under Workspace root after resolve.
2. Absolute path → workspace error (exit 2 at CLI).
3. `../` escape → workspace error.
4. Happy path: relative `actions/core.yaml` still loads.
5. Unit tests in `config.rs` for absolute, escape, and valid relative.
6. File size limits; no docs CHANGELOG required unless message strings need cli.md mention (optional one line in config.md if already documenting relative-only).

**Acceptance criteria:**

- [x] Absolute and escaping bundle paths rejected on load
- [x] Valid relative bundles still merge
- [x] Unit tests cover both failure modes
- [x] `cargo test -p odm-core` green

**Out of scope:** action `dir` escape (126), membership path (125).

## Acceptance

- [x] Agent Brief acceptance criteria all met

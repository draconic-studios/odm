---
id: issues-130
title: "Entity names unique and path-safe"
description: "Projects and Progens may share names (pin/managed collision); names may contain / breaking worktree paths."
status: open
issue-type: bug
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
---

# Entity names unique and path-safe

## Description

No uniqueness across `projects` and `progens` maps — same name collides on pin keys and `resolve_managed`. Entity names only need non-empty; `/` and `..` allow `worktrees/<name>/` path escape/nesting. Slot names already validate tokens.

## Affected

- `crates/odm-core/src/config.rs` validate
- `crates/odm-core/src/membership.rs` add
- `crates/odm-core/src/paths.rs` worktree paths
- `crates/odm-core/src/worktree.rs` `validate_slot_name` (pattern to mirror)

## Impact

Config can create ambiguous pins and unsafe worktree paths.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Reject cross-map duplicate names; validate entity names like slot tokens (no `/` `\` `.` `..` empty).

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]
- Slot validation: `worktree.rs` `validate_slot_name`
- cli.md: names are tokens not paths

**Desired behavior:**

1. Config load: if a name exists in both projects and progens → workspace error.
2. membership add: reject name already used in the other map.
3. Entity name validation: non-empty; no path separators; not `.` or `..`; prefer same charset as slots (read and mirror).
4. Unit tests: clash on load; clash on add; bad name `a/b` and `..` rejected; good names pass.
5. Docs: one line in config.md that names are unique across Projects and Progens and are path tokens.

**Acceptance criteria:**

- [ ] Cross-map duplicate names rejected
- [ ] Unsafe entity names rejected at load and add
- [ ] Tests cover both
- [ ] `cargo test -p odm-core` green

**Out of scope:** renaming existing bad configs automatically; slot rename.

## Acceptance

- [ ] Agent Brief acceptance criteria all met

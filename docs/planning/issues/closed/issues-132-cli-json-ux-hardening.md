---
id: issues-132
title: "CLI JSON/UX hardening batch"
description: "Prune JSON skipped_nonempty; neutral single-progen errors; name:id vs --progen conflict; dual --wt hard error; prune DTO dirty null."
status: closed
issue-type: bug
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
---

# CLI JSON/UX hardening batch

## Description

Several small CLI honesty bugs from the swarm audit, batched for one session:

1. Single-project `worktree prune --json` omits `skipped_nonempty` (human has it; `--all` has it).
2. Prune entry DTO reuses slot DTO → extra `"dirty": null` vs docs `{name,path}`.
3. Read paths use `resolve_write_progen` → error says “write requires --progen…”.
4. `name:id` silently wins over conflicting `--progen`.
5. Global `--wt` + `project git --wt` different values: local wins silently.

## Affected

- `crates/odm/src/commands/worktree.rs`
- `crates/odm-progen/src/scope.rs`, `store.rs`
- `crates/odm/src/main.rs` project git wt merge
- docs cli.md / worktrees.md

## Impact

Agents get opaque exit 3, wrong-store context with exit 0, misleading errors.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Fix the five UX/JSON items above with tests; keep each change minimal.

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]

**Desired behavior:**

1. `WorktreePruneDto` includes `skipped_nonempty: [{name,path}]` (mirror `--all`).
2. Pruned/skipped entries are `{name,path}` only (no dirty).
3. Rename/split resolver message for reads: e.g. “requires --progen <name> when multiple progens are configured” (no “write”).
4. If `name:id` prefix progen ≠ `--progen` value → usage error exit 1.
5. If global `--wt` and `project git --wt` both set and differ → usage error; if equal, OK.
6. Tests: prune JSON skips; multi-progen context message; conflicting name:id; dual wt.
7. Docs one-liners where shapes change.

**Acceptance criteria:**

- [x] All five behaviors implemented
- [x] Tests cover each
- [x] `cargo test -p odm -p odm-progen` green

**Out of scope:** clap exit (129); run stdio (128); wt missing exit (127).

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Resolution

- Prune DTO: `skipped_nonempty` always present; entries `{name,path}` only.
- `resolve_single_progen` neutral multi-progen message; reads use it.
- `name:id` vs `--progen` mismatch → usage 1.
- Conflicting repeated `--wt` (argv scan; clap global Append loses split positions) → usage 1.

---
id: issues-131
title: "Progen index freshness and duplicate id errors"
description: "Index rebuilds only when missing (stale after edits); duplicate note ids abort whole reindex with opaque SQL error."
status: open
issue-type: bug
severity: high
tags:
  - planning
  - issue
  - ready-for-agent
---

# Progen index freshness and duplicate id errors

## Description

`ensure_index` rebuilds only if `index.db` is missing — vault edits stay invisible until `odm progen reindex`. Duplicate frontmatter `id`s make INSERT fail and abort the whole reindex with a weak error.

## Affected

- `crates/odm-progen/src/index.rs` — `ensure_index`, `reindex_progen`
- `store.rs` open path
- doctor “index present” messaging (`ops.rs`) may mislead when stale

## Impact

Agents get stale memory after note edits; one bad note bricks indexing.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Detect stale index on open (mtime or content watermark) and rebuild; on duplicate ids, fail reindex with both paths listed.

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]
- Domain: disposable index; explicit `reindex` remains valid

**Desired behavior:**

1. **Freshness:** Store a watermark (vault tree mtime max, or cheap hash of paths+mtimes) in index `meta`. On open/ensure, if vault newer → rebuild. Prefer mtime walk (fast enough for v1 desks).
2. Edit a note body after index → next `find` sees change without manual reindex.
3. **Duplicates:** Before/during insert, if two files share `id`, error naming both `rel_path`s (hard fail, no silent last-wins).
4. Unit tests: edit → find updates; two files same id → clear error.
5. Doctor may keep “index present”; optional improve message only if tiny.

**Acceptance criteria:**

- [ ] Stale index auto-rebuilds on ensure/open
- [ ] Duplicate id error lists both paths
- [ ] Unit tests for both
- [ ] `cargo test -p odm-progen` green
- [ ] Explicit `reindex` still works

**Out of scope:** watch daemon; FTS escape (123); snippet (122).

## Acceptance

- [ ] Agent Brief acceptance criteria all met

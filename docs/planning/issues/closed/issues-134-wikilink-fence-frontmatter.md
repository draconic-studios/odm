---
id: issues-134
title: "Wikilinks skip code fences; surface bad frontmatter"
description: "parse_wikilinks indexes links inside fenced code; invalid YAML frontmatter is silently dropped causing id drift."
status: closed
issue-type: bug
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
---

# Wikilinks skip code fences; surface bad frontmatter

## Description

1. `parse_wikilinks` scans full body; fenced `[[NotALink]]` becomes real edges.
2. `parse_markdown` uses `serde_yaml::from_str(...).ok()` — bad FM → silent path-id fallback.

## Affected

- `crates/odm-progen/src/note.rs`
- context/backlinks/reindex quality

## Impact

Noisy graph; silent identity bugs on typos.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Strip fenced code before wikilink scan; report invalid frontmatter on reindex (hard error or per-file warning — prefer **hard error listing path** for v1 honesty).

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]

**Desired behavior:**

1. Fenced blocks (``` … ```) do not contribute wikilinks; real body links still do.
2. Optional: skip inline single-backtick spans if easy.
3. Invalid YAML between `---` markers → reindex error including file path (not silent drop).
4. Unit tests: fence false positive gone; bad FM errors; happy FM+link still works.

**Acceptance criteria:**

- [x] No links from fenced code
- [x] Bad frontmatter fails reindex with path
- [x] Tests cover both
- [x] `cargo test -p odm-progen` green

**Out of scope:** full CommonMark parser; title-collision backlinks (low priority observation).

## Acceptance

- [x] Agent Brief acceptance criteria all met

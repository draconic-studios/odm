---
id: issues-97
title: "docs honesty for status/info worktree orphans"
description: "Document landed worktree_orphans on status and project info; strike deferred status orphan listing where implemented."
status: open
issue-type: feature-request
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# docs honesty for status/info worktree orphans

## Description

After code tickets land orphan listing on status and project info, reference docs and CHANGELOG/README must not still claim orphans are status-invisible / deferred-only.

## Affected

- `docs/reference/worktrees.md` — Rules + Deferred
- `docs/reference/cli.md` — status / project info / full vs sketch matrix
- `docs/reference/phased-delivery.md` — Phase spine landed + deferred bullet
- `CHANGELOG.md` [Unreleased]
- `README.md` only if it still implies orphans are doctor-only with no status mention (minimal)

## Impact

Docs drift confuses agents and humans about observation surfaces.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-96-project-info-worktree-orphans]]

## Agent Brief

**Category:** docs  
**Summary:** Record landed `worktree_orphans` on `odm status` projects and `odm project info`; keep doctor warn + prune as cleanup path; remove “status orphan listing” from pure-deferred TODO where implemented.

**Bindings:**

- Parent map: [[issues-94-post-v1-status-orphans-map]]
- Truth: implementation from 95+96 (JSON field names, human lines, empty/soft-fail)
- Prior honesty tickets pattern: [[issues-92-status-packs-docs-honesty]], [[issues-80-worktree-observation-docs-honesty]]

**Desired behavior:**

1. **worktrees.md:** Rules section states status/info report registered slots **and** orphan dirs (observation); doctor warn + prune remain cleanup. Deferred: strike or parenthetical that status/info orphan listing landed; leave config slots, pin↔slot, auto-prune, branch templates, global `--wt` depth deferred.
2. **cli.md:** status snapshot blurb includes per-project orphans; project info blurb; sketch/deferred matrix no longer lists status orphan listing as unimplemented without landed note.
3. **phased-delivery.md:** Phase spine bullet for status/info `worktree_orphans`; deferred worktree bullet updated.
4. **CHANGELOG [Unreleased] Added:** concise bullet for status + project info orphan listing.
5. **README:** only if needed for honesty (one line max).
6. No code changes. `cargo test` still green (sanity).
7. `rg` sanity: no remaining claim that orphans are **only** doctor-visible without acknowledging status/info.

**Acceptance criteria:**

- [ ] worktrees.md / cli.md / phased-delivery honest about landed orphans on status + info
- [ ] Deferred lists no longer treat status orphan listing as pure TODO
- [ ] CHANGELOG [Unreleased] records the feature
- [ ] Remaining deferred worktree items still listed
- [ ] No product code changes; `cargo test` green

**Out of scope:**

- core-desk dogfood (ticket 98)
- Implementing other deferred worktree features

## Acceptance

- [ ] Agent Brief acceptance criteria all met

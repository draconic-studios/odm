---
id: issues-80
title: "Docs honesty for prune --all and slot dirty observation"
description: "Align worktrees/cli/phased-delivery/CHANGELOG/README with prune --all and slot dirty fields."
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

# Docs honesty for prune --all and slot dirty observation

## Description

After [[issues-78-worktree-prune-all]] and [[issues-79-worktree-slot-dirty-observation]] land, reference spine and consumer docs may still list multi-project prune / status dirty as pure Deferred or omit them from Phase spine.

## Affected

- `docs/reference/worktrees.md`
- `docs/reference/cli.md`
- `docs/reference/phased-delivery.md`
- `CHANGELOG.md`
- Root `README.md` only if Status/quickstart still omit useful one-liners (YAGNI if already adequate)

## Impact

Agents treat deferred items as unshipped or miss new JSON fields.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-78-worktree-prune-all]]
- [[issues-79-worktree-slot-dirty-observation]]

## Agent Brief

**Category:** docs  
**Summary:** Docs-only honesty pass for multi-project prune and slot dirty observation. No Rust changes unless a doc example is wrong and already contradicted by tests (prefer docs fix only).

**Bindings:**

- Parent map: [[issues-77-post-v1-worktree-observation-map]]
- Authority: landed CLI/JSON from 78+79; do not invent flags.

**Behavior lock:**

1. **`worktrees.md`:**
   - CLI section documents `prune --all [--force]` and dirty on list/status/info.
   - **Deferred** no longer lists multi-project prune or status dirty-slot observation as open (or rewrites those bullets to “landed”).
   - Keep deferred: config slots, branch templates, auto-prune on doctor --fix, pin↔slot, status orphan listing, global `--wt` depth.

2. **`cli.md`:**
   - Command tree / worktree section: `--all`.
   - Status / project info / worktree list JSON mention `dirty` on slot objects.
   - Full vs sketch matrix if it still claims those deferred.

3. **`phased-delivery.md`:**
   - Post-0.1.0 Phase spine bullets for prune `--all` and slot dirty on status/list/info.
   - Deferred list no longer claims multi-project prune / status dirty as open work.

4. **`CHANGELOG.md` [Unreleased]:**
   - Ensure bullets exist for both features (add only if 78/79 missed them; dedupe if duplicated).

5. **Root README:** optional one-liner only if Status/quickstart is misleading without it. Do not claim `agent start` or other deferred product.

6. No code changes. `cargo test` still green (run once to confirm workspace).

**Acceptance criteria:**

- [ ] worktrees.md Deferred no longer lists multi-project prune or status dirty observation as TODO
- [ ] cli.md documents prune --all and dirty field
- [ ] phased-delivery Phase spine + deferred aligned
- [ ] CHANGELOG accurate (no duplicate contradictory bullets)
- [ ] No false “shipped” claims for still-deferred items
- [ ] `cargo test` green

**Out of scope:**

- core-desk dogfood (issues-81)
- New features
- Release version bump

## Acceptance

Mirror Agent Brief checklist.

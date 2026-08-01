---
id: issues-87
title: "Docs honesty: pack rm + doctor pack_missing + README prune --all"
description: "Update cli/env-gen-packs/phased-delivery/CHANGELOG/README for pack rm, pack_missing doctor, and prune --all surface."
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

# Docs honesty: pack rm + doctor pack_missing + README prune --all

## Description

After pack rm + doctor pack_missing land, reference docs and consumer-facing README/CHANGELOG must not still say pack is list/install/link only. README still omits `worktree prune --all` in the quick surface block.

## Affected

- `docs/reference/cli.md` — pack command tree, doctor blurb, full/sketch matrix
- `docs/reference/env-gen-packs.md` — pack CLI + deferred list (doctor pack report partial land)
- `docs/reference/phased-delivery.md` — Phase spine / deferred bullets if needed
- `docs/reference/architecture.md` — only if pack/doctor sentences are stale
- `CHANGELOG.md` [Unreleased]
- `README.md` — status line + worktree quick commands (`prune --all`)

## Impact

Agents and humans treat docs as truth; drift causes wrong expectations.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-84-agent-pack-rm-core]]
- [[issues-85-agent-pack-rm-cli]]
- [[issues-86-doctor-pack-missing]]

## Agent Brief

**Category:** docs  
**Summary:** Make docs match landed pack rm + pack_missing doctor; mention `prune --all` on README quick surface.

**Bindings:**

- Parent map: [[issues-82-post-v1-pack-lifecycle-hardening-map]]
- Behavior from 84–86 (read code if unsure)
- Prior honesty tickets: [[issues-80-worktree-observation-docs-honesty]], [[issues-56-reference-docs-v1-honesty]]

**Edits (checklist):**

1. **cli.md:** `odm agent pack rm <name>`; JSON/human; exit 4 unknown; doctor section mentions `pack_missing:<name>` warn (not fixable); command tree / full matrix include pack rm; deferred list no longer claims “no doctor pack reports” without nuance (marketplace/manifest still deferred; full status pack reports still deferred).
2. **env-gen-packs.md:** pack CLI includes rm; deferred: drop or narrow “status/doctor pack reports” — doctor missing-path warn landed; status pack reports still deferred.
3. **phased-delivery.md:** Phase spine bullet for pack rm and/or doctor pack_missing if other post-0.1.0 lands are listed that way; deferred stays honest.
4. **CHANGELOG [Unreleased] Added:** pack rm; doctor pack_missing warn.
5. **README:** status/quickstart mention pack rm if packs are mentioned; worktree block includes `prune --all` alongside per-project prune.
6. **architecture.md:** touch only if a sentence falsely denies doctor pack checks.
7. No Rust behavior changes in this ticket. `cargo test` still green (sanity).

**Acceptance criteria:**

- [ ] cli.md documents pack rm + pack_missing doctor warn
- [ ] env-gen-packs.md pack section includes rm; deferred narrowed
- [ ] CHANGELOG Unreleased bullets present
- [ ] README mentions prune --all; pack surface not “install/link/list only” if it lists pack verbs
- [ ] No false “not implemented” for pack rm
- [ ] `cargo test` green

**Out of scope:**

- core-desk README dogfood steps ([[issues-88-core-desk-pack-rm-dogfood]])
- New features
- Version bump / release

## Acceptance

- [ ] Agent Brief acceptance criteria all met

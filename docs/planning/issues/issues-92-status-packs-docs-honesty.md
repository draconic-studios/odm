---
id: issues-92
title: "Docs honesty: status agent_packs + deferred lists"
description: "Update cli/env-gen-packs/phased-delivery/CHANGELOG/README so status pack reports are landed truth."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# Docs honesty: status agent_packs + deferred lists

## Description

After [[issues-90-status-agent-packs]] lands, reference docs and CHANGELOG still call status pack reports deferred. Align docs with code.

## Affected

- `docs/reference/cli.md` — `odm status` section + sketch/deferred matrix
- `docs/reference/env-gen-packs.md` — deferred bullet for status pack reports
- `docs/reference/phased-delivery.md` — Phase spine / deferred if it mentions status packs
- `CHANGELOG.md` Unreleased
- `README.md` only if status blurb should mention packs (keep short; no false claims)

## Impact

Agents and humans trust docs over code; deferred lies cause wrong tickets.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-90-status-agent-packs]]

## Agent Brief

**Category:** docs  
**Summary:** Document landed `odm status` `agent_packs` inventory; remove “status pack reports” from deferred-as-TODO where implemented; keep doctor pack_missing and remaining pack deferred items honest.

**Bindings:**

- Parent map: [[issues-89-post-v1-status-packs-map]]
- Behavior from closed [[issues-90-status-agent-packs]] (JSON shape + human + missing flag)
- Prior honesty tickets: [[issues-87-pack-lifecycle-docs-honesty]], [[issues-80-worktree-observation-docs-honesty]]

**Desired edits:**

1. **cli.md status:** state that status includes top-level `agent_packs: [ { name, source, path, mode, missing } ]`; human Agent packs section; empty array when none; missing aligns with doctor path rule; doctor still owns warn checks.
2. **cli.md sketch/deferred matrix:** status pack reports moved to full/landed parenthetical; do not claim marketplace/manifest/config packs landed.
3. **env-gen-packs.md:** deferred list — strike or parenthetical “status pack reports landed on `odm status`”; keep marketplace/manifest/config/status-vs-doctor distinction clear; doctor pack_missing stays documented.
4. **phased-delivery.md:** if deferred/spine mentions status packs as open, mark landed; do not invent unrelated spine bullets.
5. **CHANGELOG Unreleased Added:** bullet for status `agent_packs` (name/source/path/mode/missing).
6. **README:** optional one-phrase status note only if current status line is misleading; do not expand quickstart unless necessary.
7. No code changes. `rg` sanity: no remaining “status pack reports” as pure TODO without landed note.
8. Do not touch architecture.md unless a single stale sentence is clearly wrong (prefer leave).

**Acceptance criteria:**

- [ ] cli.md status documents agent_packs JSON + human
- [ ] env-gen-packs / cli deferred lists no longer claim status pack reports unimplemented
- [ ] CHANGELOG Unreleased mentions status agent_packs
- [ ] Remaining deferred (marketplace, manifest, config packs, agent start) still explicit
- [ ] No product code in this ticket

**Out of scope:**

- core-desk dogfood (ticket 93)
- Implementing other deferred pack features
- Release version bump

## Acceptance

- [ ] Agent Brief acceptance criteria all met

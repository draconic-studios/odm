---
id: issues-107
title: "docs honesty pack list missing"
description: "cli.md / env-gen-packs / CHANGELOG (and README if needed) document pack list missing field."
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

# docs honesty pack list missing

## Description

After 105/106 land `missing` on pack list JSON/human, reference docs and CHANGELOG must match code. Status already documents `agent_packs[].missing`; pack list section still omits it.

## Affected

- `docs/reference/cli.md` — agent pack list / install/link/rm JSON field set
- `docs/reference/env-gen-packs.md` — pack list bullet if it restates JSON shape
- `CHANGELOG.md` — Unreleased Added/Changed
- `README.md` only if it claims pack list shape (likely no change)
- `docs/reference/phased-delivery.md` only if it lists pack list observation as deferred (unlikely)

## Impact

Docs drift confuses agents implementing against cli.md.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-105-pack-list-missing-dto]]
- [[issues-106-pack-list-missing-cli]]

## Agent Brief

**Category:** docs  
**Summary:** Document `missing` on `odm agent pack list` (and shared entry JSON) consistently with status/doctor path rules.

**Bindings:**

- Parent map: [[issues-104-post-v1-pack-list-missing-map]]
- Landed behavior from 105/106
- Prior honesty tickets (e.g. [[issues-92-status-packs-docs-honesty]]) for tone

**Desired behavior:**

1. **cli.md** pack section:
   - List `--json` items: `{ name, source, path, mode, missing }`
   - `missing` = true when dest has no path/symlink entry (same as status/doctor); dangling symlink not missing
   - Human list: one name per line; ` missing` suffix when missing
   - install/link/rm `--json` same fields as list items (including `missing`)
2. **env-gen-packs.md:** if list shape is summarized, add `missing`; do not claim list lacks observation.
3. **CHANGELOG Unreleased:** note pack list/entry JSON `missing` + human suffix.
4. **README / phased-delivery:** touch only if currently wrong; no version bump.
5. No code changes unless a one-line comment; no core-desk (108).
6. Markdown: no tables (AGENTS.md).

**Acceptance criteria:**

- [ ] cli.md documents list/entry `missing` + human suffix + shared install/link/rm fields
- [ ] env-gen-packs.md not contradictory
- [ ] CHANGELOG Unreleased mentions the observation
- [ ] No false “deferred” claim for pack list missing

**Out of scope:**

- Product code
- core-desk dogfood
- Marketplace/manifest/`agent start` promotion

## Acceptance

- [ ] Agent Brief acceptance criteria all met

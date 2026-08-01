---
id: issues-102
title: "Docs honesty for generate --dry-run"
description: "Record landed generate --dry-run in reference docs, CHANGELOG, README; strike pure-deferred dry-run."
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

# Docs honesty for generate --dry-run

## Description

After CLI dry-run lands, reference docs still list dry-run as pure deferred and omit the flag from generate surfaces.

## Affected

- `docs/reference/cli.md` — generate section + full vs sketch matrix if needed
- `docs/reference/env-gen-packs.md` — Generators CLI + Deferred
- `docs/reference/phased-delivery.md` — only if generate bullet should mention dry-run (keep short)
- `CHANGELOG.md` — Unreleased Added
- `README.md` — only if generate blurb should mention `--dry-run` (one short phrase max; no false claims)

## Impact

Agents treat dry-run as unshipped or miss the flag.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-100-generate-dry-run-core]]
- [[issues-101-generate-dry-run-cli]]

## Agent Brief

**Category:** docs  
**Summary:** Document landed `odm generate … --dry-run`; remove dry-run from pure-deferred TODO where implemented; keep remote/templating deferred honest.

**Bindings:**

- Parent map: [[issues-99-post-v1-generate-dry-run-map]]
- Behavior from closed 100+101 (flag, no-write, `copied` count, JSON `dry_run`, human `would generate`)
- Prior honesty pattern: [[issues-97-status-orphans-docs-honesty]]

**Edits:**

1. **cli.md generate:** document `--dry-run` on run form; state no filesystem writes; same validation as real run; JSON includes `dry_run` bool; human would-generate line; real run `dry_run: false`.
2. **env-gen-packs.md:** CLI block includes dry-run; Deferred — strike or parenthetical that dry-run landed; keep remote/`template.toml`/prompts/Nx deferred.
3. **phased-delivery.md:** optional one-phrase on generate local bullet if it lists features; do not claim Ship-gate change.
4. **CHANGELOG Unreleased Added:** bullet for `odm generate --dry-run` (no-write preview + count / JSON `dry_run`).
5. **README:** optional short mention next to generate lines; do not bloat.
6. No Rust/product changes. `rg` sanity: no remaining “dry-run” as pure deferred without landed note.
7. `cargo test` still green (no code change expected).

**Acceptance criteria:**

- [ ] cli.md documents `--dry-run` behavior + JSON/human
- [ ] env-gen-packs.md Deferred no longer treats dry-run as pure TODO
- [ ] CHANGELOG Unreleased records the feature
- [ ] Remote/templating remain clearly deferred
- [ ] No product code required; tests still green

**Out of scope:**

- core-desk dogfood (ticket 103)
- Version bump / release cut
- Implementing remote generators

## Acceptance

- [ ] Agent Brief acceptance criteria all met

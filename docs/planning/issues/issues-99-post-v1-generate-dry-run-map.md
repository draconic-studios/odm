---
id: issues-99
title: "Post-v1 generate dry-run map"
description: "Wayfinder map: odm generate --dry-run (count without write); docs + core-desk dogfood."
status: open
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Post-v1 generate dry-run map

## Destination

After status orphans map [[issues-94-post-v1-status-orphans-map]] closed, pull the next AFK-ready slice from `env-gen-packs.md` **Deferred** (Generators):

1. **Generate dry-run core** — materialize path validates like a real run but writes nothing; report file count that would be copied.
2. **CLI `--dry-run`** — wire flag on `odm generate <name> --dest …`; human + JSON shapes.
3. **Docs + dogfood** — reference honesty; core-desk exercises dry-run then real generate.

## Notes

- **Authority:** `docs/reference/env-gen-packs.md` (Generators Deferred: dry-run mode), `cli.md` generate section, landed `generate_local` / `GenerateRunDto`.
- **Prereqs:** generators map [[issues-45-generators-map]] closed (local template v1).
- **Execution:** ticket close = decision + code/tests/docs as scoped. TDD for code tickets.
- **Standing prefs (AFK defaults):**
  - Do not implement graph, env, generate **remote** / `template.toml` / prompts / vars, pack marketplace/manifest, config-declared packs/slots, `agent start`, `init --interactive`.
  - Do not implement worktree deferred product (config slots, pin↔slot, auto-prune on doctor, branch templates, global `--wt` depth).
  - Dry-run uses the **same validation** as a real run (template resolve, dest relative/no escape, dest not a file, non-empty dest requires `--force`).
  - Dry-run **never** creates dirs or copies files (including under `--force`).
  - `copied` = number of files that **would** be written (same counting rules as real copy).
  - JSON always includes `"dry_run": bool` on run envelope (true for dry-run; false for real run — additive field).
  - Human dry-run: `would generate <name> -> <dest> (<n> files)`; real run stays `generated …`.
  - Url-only generators still exit `1` (deferred remote) whether or not `--dry-run`.
  - No new crate; stay in `odm-core` + thin CLI/formatters.
  - File size ≤1000 target / ≤1250 hard.

## Decisions so far

- Child tickets: [[issues-100-generate-dry-run-core]], [[issues-101-generate-dry-run-cli]], [[issues-102-generate-dry-run-docs-honesty]], [[issues-103-core-desk-generate-dry-run-dogfood]].
- Prefer order: 100 unblocked first; 101 blocked by 100; 102 blocked by 100+101; 103 blocked by 100+101.
- **100 closed:** `generate_local(..., force, dry_run)`; dry-run validates + `count_tree` (files/symlinks), zero writes even with force; real path unchanged when `dry_run=false`; CLI still passes `false` until 101.
- **101 closed:** CLI `--dry-run` on generate-run; `GenerateRunDto.dry_run`; human `would generate` / `generated`; integration coverage in `cli_generate.rs`.

## Not yet specified

_(none for this slice — AFK defaults lock dry-run behavior)_

## Out of scope

- `odm agent start`, `init --interactive`
- Graph, env, generate remote/templating/prompts/vars, pack marketplace/manifest/config declarations
- Worktree deferred product
- Nx/schematics interop
- Release version bump / GitHub release

## Blocked by

None

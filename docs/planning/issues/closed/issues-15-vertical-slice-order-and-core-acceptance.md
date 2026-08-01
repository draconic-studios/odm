---
id: issues-15
title: "Vertical slice order and core acceptance"
description: "Lock build order of core slices and the checklist that closes this implement map."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# Vertical slice order and core acceptance

## Question

In what order should Implement core land as vertical slices (each shippable/testable), and what concrete acceptance checklist means this map’s destination is met so the map can close — beyond the phase-2 bullets in `phased-delivery.md`?

## Blocked by

_(none — frontier)_

## Answer

### Slice shape

Thin **vertical** slices (bin wired → real behavior → tests). Unbuilt core verbs may appear in help but must exit **`1`** with a clear **not implemented** message — never silent no-op. Harness asserts only what the current slice claims.

### Order

1. **Skeleton** — Cargo workspace: `crates/odm` (bin), `odm-core`, `odm-git`; `odm --help` works
2. **Config / discovery / `init`** — load full v1 schema (deny unknown), Workspace walk/`--root`, `.odm/` bootstrap
3. **Git + materialize / sync + `project add` / `list`** — shell-out clone/fetch; managed Project path
4. **Pin** — auto-create/maintain when Workspace is git; `pin status` / `pin apply`
5. **`status` + gitignore manage** — snapshot; `manage_gitignore` markers
6. **`doctor`** — ODM-side checks + mechanical `--fix`
7. **`project` remainder** — `rm` / `info` / `git` passthrough (+ pin auto-maintain on HEAD change)
8. **Gate** — `examples/core-desk` (local bare fixtures + README) + integration harness

Contract tickets (git shell API, serde model, exit codes, JSON shapes, etc.) feed these slices; they do not replace the order.

### Acceptance checklist (map close)

All must be true; human gate = confirming checklist then closing the parent map:

1. Real crates `odm` / `odm-core` / `odm-git`; `cargo test` green locally
2. Core command cut works: `init`, `sync`, `pin`, `status`, `doctor`, `project` list/add/rm/info/git; globals `--root`, `--json`, exit codes `0`–`4` spine
3. Multi-git: plain clone/fetch/pin apply; no submodules; origin mismatch fails
4. `examples/core-desk` with local bare fixtures + README dogfood path
5. Integration harness covers at least: init → add managed project from fixture → sync → pin status/apply → status → doctor
6. Full v1 config schema loads (deny unknown); `progens`/actions/generators maps load; no progen/run/generate/agent CLI
7. No phase 3–5 scope (progen façade, actions productization, Ship release, sketches as commitments)

Mirrored in short form under phase 2 in `docs/reference/phased-delivery.md`.

## Comments

Parent map: [[issues-14-implement-core-map]]

Grilled with maintainer; locked 2026-08-01.

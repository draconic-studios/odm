---
id: issues-163
title: "Docs + dogfood honesty for agent start"
description: "Promote agent start from sketch in reference docs/README/website; update core-desk/todo dogfood; CHANGELOG; close map 158."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
---

# Docs + dogfood honesty for agent start

## Description

After CLI start lands, honesty surfaces still call `agent start` a pure sketch / exit-1 stub. Dogfood scripts assert exit 1. Promote docs and flip dogfood to a real one-shot start against a Project with `true` (or equivalent offline program).

## Affected

- `docs/reference/cli.md` — start section + full vs sketch matrix + command tree comment
- `docs/reference/env-gen-packs.md` — start section; mixed-depth blurb
- `docs/reference/architecture.md` / `phased-delivery.md` / `worktrees.md` — only if they still say start is pure stub (keep short)
- `CHANGELOG.md` — Unreleased
- `README.md` — status line (start no longer pure sketch)
- `website/` — shipped-vs-sketch copy if it names start as sketch only
- `examples/core-desk/scripts/dogfood.sh` + README; `examples/todo` dogfood/probe/REVIEW if present
- Parent map [[issues-158-agent-start-map]] — Answer + close when destination met

## Impact

Agents and humans treat start as unshipped after code landed.

## Proposed Fix

See Agent Brief.

## Blocked by

None (unblocked — [[issues-161-agent-start-lib]] and [[issues-162-agent-start-cli]] closed)

## Agent Brief

**Category:** docs  
**Summary:** Document landed `odm agent start` v1; update dogfood; CHANGELOG; close map 158 with Answer. No product scope creep.

**Bindings:**

- Parent map: [[issues-158-agent-start-map]] Decisions + map-close acceptance
- Behavior locked by [[issues-161-agent-start-lib]] / [[issues-162-agent-start-cli]]
- Markdown: never tables (`AGENTS.md`)
- Dogfood must stay offline (use `true` / `false` / `echo` only)

**Desired behavior:**

1. **cli.md:** Promote `odm agent start` to **v1** (one-shot exec; `--project` required; optional `--wt`; argv program; human inherit; `--json` shape; exit passthrough). Move out of pure sketch matrix; note deferred (runtime matrix, pack auto-apply, prompt compose, session lifecycle, serve/MCP).
2. **env-gen-packs.md:** Same honesty; env injection still deferred/sketch.
3. **architecture / phased-delivery / worktrees:** One-line honesty if they still say start-only stub.
4. **README + website:** Status/features no longer claim start is unimplemented sketch if shipped; keep other deferred surfaces honest.
5. **CHANGELOG [Unreleased]:** Added `odm agent start` v1 bullet.
6. **Dogfood:**
   - core-desk: replace `expect_exit 1 odm agent start` with a real start against an existing project, e.g. `odm --project <name> agent start -- true` exit 0 (and optional fail passthrough if cheap)
   - todo scripts/REVIEW: same honesty
   - Update example README tour one-liners that mention start exit 1
7. **Close map 158:** write `## Answer`, `status: closed`, move `issues-158-agent-start-map.md` to `closed/`; refresh `docs/planning/issues/Index.md` (drop from Maps/Frontier or mark closed).
8. `cargo test` still green; no new product flags.

**Acceptance criteria:**

- [x] Reference docs describe start v1 behavior; not pure sketch
- [x] Deferred list still names runtime matrix / marketplace / serve / pack-auto / prompt-compose
- [x] Dogfood no longer requires start exit 1; proves start exit 0 with `true`
- [x] CHANGELOG + README/website honesty
- [x] Map 158 Answer + closed to `closed/`
- [x] Index updated
- [x] `cargo test` green

**Out of scope:**

- Implementing new start flags beyond what 161/162 shipped
- Release tag/publish
- Remote generate, marketplace, serve/MCP, interactive init

## Answer

Promoted `odm agent start` to v1 one-shot across `cli.md`, `env-gen-packs.md`, architecture/phased-delivery/worktrees, README, CHANGELOG, and website (features/cli/guide-agents/index). Dogfood/probe/REVIEW flip to `odm --project … agent start -- true` (exit 0) + `false` passthrough. Closed parent map [[issues-158-agent-start-map]].

## Comments

Minted from [[issues-158-agent-start-map]] 2026-08-02.

- Add tag `ready-for-agent` when [[issues-161-agent-start-lib]] and [[issues-162-agent-start-cli]] are closed (blocked until then).
- 2026-08-02: Unblocked — [[issues-162-agent-start-cli]] closed; tagged `ready-for-agent`.
- 2026-08-02: Docs/dogfood honesty landed; closed with map 158.

---
id: issues-76
title: "README honesty for slot-depth surfaces"
description: "Root README status/quickstart mention prune, find --limit, and project info slots without false deferred claims."
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

# README honesty for slot-depth surfaces

## Description

Root `README.md` Status and Generators/worktree quickstart cover worktree add/list and generate, but omit **`project worktree prune`**, **`find --limit`**, and **project info / status slot** shape that operators now have. Consumers scanning README miss post-map-66 surfaces (core-desk is deeper; root should stay short but honest).

## Affected

- `README.md` (repo root)

## Impact

First-touch docs lag shipped CLI.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** docs  
**Summary:** Docs-only root README touch. No Rust changes.

**Bindings:**

- Parent map: [[issues-72-post-v1-honesty-dogfood-map]]
- Truth: `docs/reference/cli.md`, `worktrees.md`, existing README Status line
- Prefer **minimal** edits — not a second cli.md

**Behavior lock:**

1. **Status** line (top): keep v0.1.0 spine + worktree/generate/pack/prompt; optionally note doctor orphan/dirty warns + prune exist without turning Status into a changelog. Do not claim `agent start` shipped.
2. **Quickstart** worktree block: add one line for `odm project worktree prune <project>` (and optional `--force` mention). Keep add/list.
3. **Progen** or find area: show `odm find <query> --limit 5` once (or note default 200) without removing bare `odm find`.
4. Optional one-liner that `odm status` / `odm project info` report registered worktree slots — only if it fits without bloat.
5. Point to `docs/reference/cli.md` / core-desk for depth (already linked).
6. `cargo test` green (no code change).

**Acceptance criteria:**

- [ ] README mentions `project worktree prune`
- [ ] README mentions `find --limit` (or equivalent)
- [ ] Status/quickstart does not claim deferred features (agent start, graph, auto-prune on doctor) as shipped
- [ ] `cargo test` green

**Out of scope:**

- Rewriting install/docs tree
- examples/core-desk (ticket 74)
- CHANGELOG / phased-delivery (ticket 73)
- Version bump

## Acceptance

Mirror Agent Brief checklist.

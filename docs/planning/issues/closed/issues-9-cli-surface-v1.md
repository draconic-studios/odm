---
id: issues-9
title: "CLI surface v1"
description: "Lock odm command tree (odm progen, projects, context, find, …) and global flags."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# CLI surface v1

## Question

What is the v1 CLI command tree and global flags (`--json`, `--root`, `--project`, `--progen`, `--wt`) for design docs — with `odm progen` (not brain), and which commands are full vs sketch — written to `docs/reference/cli.md`?

## Blocked by

- [[issues-5-config-schema-spine]]
- [[issues-6-progen-scope-and-federation]]

## Answer

Documented in `docs/reference/cli.md`.

- Tree: `init`; top-level `sync` / `pin` / `status` / `doctor`; `project` (+ sketch `worktree`); `progen` lifecycle + store façade; top-level `find` / `context`; `run <action>`; sketch `generate` / `agent`.
- No `ops` namespace; no top-level action verbs; no serve/MCP; never “brain”.
- Globals: `--root`, `--json`, `--project`, `--progen`, `--progen-group`, `--wt` (names only).
- `project git <name> -- <args>` passthrough; pin auto-maintain on HEAD change (not sync).
- Entity summary verb `info`; node `get` under `progen`.
- Exit codes `0`/`1`/`2`/`3`/`4`; `run` passthrough when executed.
- Full vs sketch matrix in cli.md; sketch depth follow-up [[issues-12-sketch-sections-depth]].

## Comments

Parent map: [[issues-1-odm-design-docs-map]]

Grilled with maintainer.

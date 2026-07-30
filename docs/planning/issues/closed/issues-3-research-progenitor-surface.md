---
id: issues-3
title: "Research progenitor crates and store contract"
description: "AFK research: progen-* crate layout, store format, CLI, multi-root gaps at progenitor path."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-research
---

# Research progenitor crates and store contract

## Question

What does the progenitor project actually expose today (crates, store layout, frontmatter, index, CLI, multi-root support, task/context APIs) that ODM must integrate vs façade — facts only, from `/Users/jaredhembrow/Projects/draconic/progenitor` and its docs?

## Blocked by

None.

## Comments

Parent map: [[issues-1-odm-design-docs-map]]

Research file: [[progenitor-surface]] (`docs/reference/research/progenitor-surface.md`) on branch `research/progenitor-surface`.

## Answer

Progenitor exposes a layered Rust stack (`progem-data` → `parser` → `core` → `index` → `ops` → `cli`/`serve`): Markdown+YAML frontmatter store with path-from-kind placement, disposable SQLite/FTS index, unified `set` CRUD, query/context/prompt/doctor, and loopback WS JSON-RPC. Single root per process only (`--root` / `PROGEN_MEMORY_DIR` / `./memory`); no multi-store API. `progen-client` is gone — call ops crates, CLI, or `progen serve`. Full findings: `docs/reference/research/progenitor-surface.md`.

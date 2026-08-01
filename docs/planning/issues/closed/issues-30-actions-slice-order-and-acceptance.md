---
id: issues-30
title: "Actions slice order and acceptance"
description: "Decision: vertical slice order and phase-4 map-close checklist."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# Actions slice order and acceptance

## Question

What is the vertical implement order for phase 4, and when is the Actions map closable?

## Blocked by

_(none — progen map closed)_

## Answer

**Slice order** (vertical; TDD at each seam):

1. Fix Action bundle shape to `tasks: [{ run, dir? }]` in `odm-core` (load/validate + unit tests)
2. `odm-actions` crate: resolve cwd + run action (shell-out, sequential tasks, exit passthrough)
3. CLI `odm run` list + dispatch + `--project` / `--wt` / `--json` / extra args
4. Dogfood: core-desk `actions/core.yaml` + config pointer
5. Integration tests gate (`odm run` against temp core-desk)

**Map-close checklist (phase gate):**

- [ ] Action bundles load from config pointers; missing path → exit `2`; duplicate names → exit `2`
- [ ] `odm run` lists; `odm run <name>` shells out; unknown → exit `1`
- [ ] Multi-task sequential fail-fast; exit code passthrough when executed
- [ ] Cwd rules: task dir / workspace / `--project` / `--wt`
- [ ] `--json` list + run shapes per `cli.md`
- [ ] core-desk dogfood actions offline
- [ ] `cargo test` green including run integration scenarios

## Comments

Autonomous chart 2026-08-01 from `config.md` / `cli.md` / `architecture.md` / `phased-delivery.md`.

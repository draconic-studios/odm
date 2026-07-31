---
id: issues-8
title: ".odm directory contract"
description: "Lock what lives under .odm/, tracked vs gitignored, and that it never owns managed trees."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# .odm directory contract

## Question

Exactly what belongs under `.odm/` (state + config only), what is git-tracked vs ignored, allowed exceptions, and confirmation it does not own project checkouts or progen vaults — documented (config.md and/or architecture.md)?

## Blocked by

- [[issues-5-config-schema-spine]]

## Answer

Documented in `docs/reference/architecture.md` (ODM state directory section); pointers in `config.md`, `multi-git.md`, `CONTEXT.md`.

- `.odm/`: `odm.config.yaml` + `odm.lock.yaml` tracked; `cache/`, `log/`, `progen/<name>/` ignored.
- Never under `.odm/`: Primary checkouts, Progen stores, worktrees, agent pack payloads.
- Worktree slots: `worktrees/<project>/<slot>/` (lazy); gitignored.
- `manage_gitignore`: explicit ephemeral paths only (no ignore-all-`.odm`).
- Root discovery: `--root`, else walk up for `.odm/odm.config.yaml` (start at parent if cwd inside `.odm/`), stop at `$HOME`; miss → error. `init` excepted.
- Users may add files under `.odm/`; ODM leaves unknowns alone.

## Comments

Parent map: [[issues-1-odm-design-docs-map]]

Grilled with maintainer.

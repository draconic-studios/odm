---
id: issues-7
title: "Multi-git lifecycle and pin file"
description: "Lock plain-clone model, optional workspace git, opt-in odm.lock.yaml — no submodules."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# Multi-git lifecycle and pin file

## Question

What is the documented multi-repo model — plain clones via url+path, sync/add/rm semantics, optional workspace git, opt-in pin file, explicit non-use of submodules — in `docs/reference/multi-git.md`?

## Blocked by

- [[issues-2-domain-glossary]]
- [[issues-5-config-schema-spine]]

## Answer

Documented in `docs/reference/multi-git.md`; config path amended in `docs/reference/config.md` and `CONTEXT.md`.

- Plain clones only; no submodules. Managed = Project/Progen with `url`; path-only never touched by git.
- Sync = ensure present + fetch only (no HEAD move). Add = config then clone (`--no-clone` defer). Rm = un-declare; `--delete` if clean (force if dirty).
- Clone: remote default HEAD or optional `branch`; full history; fail on origin mismatch / non-git path.
- Parallel branches = multiple entries (same url, different path/branch). Nested managed paths OK; depth order; fail-fast.
- Workspace: `odm init` git-inits by default; `manage_gitignore` default on.
- Config/pin live under `.odm/` (`.odm/odm.config.yaml`, `.odm/odm.lock.yaml`) — amends spine root path.
- Pin auto-created on first materialize if Workspace is git; auto-maintains SHAs; pin apply = detached HEAD at rev.
- CLI names deferred to [[issues-9-cli-surface-v1]].

## Comments

Parent map: [[issues-1-odm-design-docs-map]]

Grilled with maintainer.

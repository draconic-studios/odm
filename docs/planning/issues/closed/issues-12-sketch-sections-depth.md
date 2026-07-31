---
id: issues-12
title: "Sketch sections depth"
description: "Lock how deep worktrees, graph, env, gen, packs sketches go — and explicit non-goals."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# Sketch sections depth

## Question

For sketch-only areas (worktrees, graph/tags, env, generators, agent packs), what minimum content is required in their reference docs so implementers are not blocked, and what is explicitly deferred — including Serve/MCP as out of scope?

## Blocked by

- [[issues-10-vision-and-architecture-narrative]]
- [[issues-9-cli-surface-v1]]

## Answer

Locked in `docs/reference/worktrees.md`, `graph.md`, `env-gen-packs.md`.

- **Sketch bar** — intent, placement/ownership, CLI names reserved, explicit deferred/non-goals. Not a Ship gate.
- **Worktrees** — parallel trees; path `worktrees/<project>/<slot>/`; `list|add|rm`; `--wt` no auto-create; git Project only; Primary ≠ slot; orphans OK on project rm; no branch templates/config/pin/GC in sketch.
- **Graph** — optional workspace code↔doc index (informal); ODM cache vs Progen in-store graph; no `odm graph` CLI; no CONTEXT term yet; ingest choice deferred.
- **Env** — paragraph non-goal only; no keys, CLI, or CONTEXT entity.
- **Generators** — `odm generate <name>`; config pointers already locked; materialize one-liner; template.toml/prompts/Nx deferred.
- **Packs / agent** — install|link|list to agent-native homes; Workspace-scoped ops; `start` shell-out; `prompt` thin progen wrap; manifest/Windows/marketplace deferred.
- **Absences** — serve/MCP/daemon, plugin host, top-level pack/env, cross-store graph API; no status/doctor obligations for sketch features.
- **No** CONTEXT changes; **no** ADR.

## Comments

Parent map: [[issues-1-odm-design-docs-map]]

Grilled with maintainer; three sketch docs written.

---
id: issues-55
title: "Post-v1 hardening + agent prompt thin map"
description: "Wayfinder map: docs honesty after packs/worktree/generate, doctor worktree orphans, thin agent prompt."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Post-v1 hardening + agent prompt thin map

## Destination

After worktree/generate/pack v1 landed, close the next AFK-ready slice:

1. **Docs honesty** — reference docs no longer call landed v1 surfaces “sketch” or list unimplemented progen verbs as if shipped.
2. **Doctor worktree orphans** — warn (not fail) when `worktrees/<project>/<slot>/` dirs exist but are not registered git worktrees (or project folder has stray dirs).
3. **`odm agent prompt` thin v1** — single-note agent work-package markdown from existing Progen context (not a second engine); `agent start` stays stub.

**Status:** destination met (2026-08-01). All child tickets closed.

## Notes

- **Authority:** `cli.md`, `worktrees.md` Deferred, `env-gen-packs.md` agent start/prompt, `architecture.md`, `multi-git.md`, `progen.md`.
- **Prereqs landed:** worktree v1, generate local, agent pack local, README/phased-delivery honesty ([[issues-54-readme-post-010-docs-drift]]).
- **Execution:** ticket close = decision + code/tests/docs as scoped. TDD for code tickets.
- **Standing prefs (AFK defaults):**
  - Docs ticket is docs-only.
  - Doctor orphan check is **Warn**, `fixable: false`, id prefix `worktree_orphan:`; no auto-delete on `--fix`.
  - Agent prompt reuses `context_notes` / `ContextHit` + human formatter (header may say `agent prompt`); JSON = same shape as `odm context --json` (serialize `ContextHit`). Scope = same as `context` / single-progen rules already used for get/context.
  - No new crate; CLI thin; keep `agent start` not-implemented.
  - Do not implement graph, env, generate remote, pack marketplace, init interactive UX, or agent start runtime matrix.

## Decisions so far

- Child tickets: [[issues-56-reference-docs-v1-honesty]], [[issues-57-doctor-worktree-orphans]], [[issues-58-agent-prompt-thin]], [[issues-59-agent-prompt-integration-docs]].
- [[issues-56-reference-docs-v1-honesty]] closed — reference docs honesty: worktree/pack v1 markers, progen façade implemented vs reserved, `init --interactive` not-implemented.
- [[issues-57-doctor-worktree-orphans]] closed — doctor Warn on orphan `worktrees/<project>/<slot>` dirs (`worktree_orphan:…`, fixable false); configured Projects only; docs honesty in worktrees/cli/phased-delivery.
- Orphan scan lock: configured Projects only (ignore unknown names under `worktrees/`).
- [[issues-58-agent-prompt-thin]] closed — `odm agent prompt <id>` thin alias of context via shared `run_context_prompt`; typed `id`; JSON = `ContextHit`; human reuses `format_context_human`; `agent start` still stub.
- Human header lock: reuse `# context <id>` (no separate agent-prompt header).
- [[issues-59-agent-prompt-integration-docs]] closed — integration proof (`agent_prompt_is_thin_context_alias`) + docs/CHANGELOG honesty for prompt v1 thin; start remains sketch; destination complete.

## Not yet specified

- _(none — map complete)_

## Out of scope

- `odm agent start`
- `init --interactive` implementation (docs may mark deferred only)
- Graph, env, generate remote/templating
- Pack manifest/marketplace
- Pin↔slot, GC/prune, config-declared slots
- Release version bump / GitHub release

## Blocked by

None

## Answer

Map destination met. Docs honesty (56), doctor worktree orphans warn (57), thin `odm agent prompt` (58), and integration tests + docs lock (59) all closed. `agent start` remains intentionally stubbed.

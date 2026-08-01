---
id: issues-54
title: "README and phased-delivery post-0.1.0 drift"
description: "Align README quickstart and phased-delivery sketch list with landed generate + worktree; agent still sketch."
status: closed
issue-type: observation
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
---

# README and phased-delivery post-0.1.0 drift

## Description

After 0.1.0, **generate** (local template) and **project worktree** landed, but root `README.md` quickstart/status still reads like spine-only, and `docs/reference/phased-delivery.md` still lists worktrees/generators as sketch-only in places. Docs drift confuses humans and agents.

## Affected

- `README.md`
- `docs/reference/phased-delivery.md`
- Optionally `docs/reference/install.md` only if it still claims generate/worktree stubs

## Impact

Readers miss shipped features; phased-delivery narrative is stale vs CHANGELOG.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** docs  
**Summary:** Honest docs pass — no code/behavior changes.

**Bindings:**

- `CHANGELOG.md` [Unreleased] + [0.1.0] truth
- `docs/reference/cli.md` full vs sketch matrix (generate + worktree full; agent sketch)
- Do not invent agent-pack docs here if [[issues-53-agent-pack-integration-and-docs]] will own pack promotion — only fix statements that are already false today.

**Current behavior (drift examples):**

- README status/quickstart omits `odm generate` and `odm project worktree`
- phased-delivery still groups worktrees/generators with pure sketches in summary bullets

**Desired behavior:**

1. **README:** Status line acknowledges post-0.1.0 worktree slots + local generate (keep agent as not ready / sketch). Quickstart or secondary snippet shows:
   - `odm generate` list / generate with `--dest` (one or two lines)
   - `odm project worktree list|add` mention or pointer to cli.md
   - Dogfood section may mention `generators/` under core-desk if present
2. **phased-delivery.md:** Update sketch-only lists so worktrees v1 and generate local template are not described as unbuilt sketches; point deferred remnants to `worktrees.md` / `env-gen-packs.md`. Do not claim agent packs shipped unless 53 already closed (if still open, agent stays sketch).
3. **No code changes.** No new features.
4. Verify with eyes + `rg` that you did not reintroduce “generate stub” / “worktree not implemented” as current truth.

**Acceptance criteria:**

- [x] README mentions generate + worktree as available (agent not oversold)
- [x] phased-delivery no longer lists landed worktree/generate v1 as sketch-only blockers
- [x] No Rust/code changes
- [x] `cargo test` still green (sanity)

**Out of scope:**

- Implementing agent packs
- Rewriting vision/architecture
- Release version bump

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Docs-only honesty pass aligned with CHANGELOG [Unreleased] and `cli.md` matrix:

- **README:** Status notes post-0.1.0 worktree slots + local generate; pack local v1; start/prompt sketch. Quickstart snippet for `odm generate` and `project worktree list|add`; dogfood shows core-desk generate; pointer to cli.md for packs.
- **phased-delivery.md:** Phase spine splits landed (worktree v1, generate local, pack local) vs still-deferred sketches; historical Actions out-of-phase bullets annotated; Related points to mixed-depth refs.
- **install.md:** unchanged (no stub claims). No Rust diffs. `cargo test` green.

## Comments

Seeded by swarm 2026-08-01 empty-frontier seed (hardening spine).

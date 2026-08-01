---
id: issues-62
title: "cli.md find/context honesty (no fake facets/--depth)"
description: "Align odm find and odm context docs with real CLI flags; document default find limit 200."
status: open
issue-type: observation
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# cli.md find/context honesty (no fake facets/--depth)

## Description

`docs/reference/cli.md` documents:

- `odm find [query] [facet-flags…]` and “FTS + **facets**”
- `odm context` “Thin-pass upstream scope flags (`--depth`, etc.)”

Actual CLI (`crates/odm/src/cli.rs`): `Find { query }`, `Context { id }` — **no** facet flags, **no** `--depth`. Find uses hardcoded limit `200` in `main.rs`.

## Affected

- `docs/reference/cli.md` (`odm find`, `odm context` sections; full vs sketch only if needed)
- Optionally `docs/reference/progen.md` only if it claims facet CLI that does not exist (leave alone if already honest)

## Impact

Implementers add phantom flags or agents pass flags that clap rejects.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** docs  
**Summary:** Docs-only. Match cli.md to shipped find/context. Do **not** implement facets or depth in this ticket.

**Bindings:**

- `crates/odm/src/cli.rs` — `Commands::Find`, `Commands::Context`
- `crates/odm/src/main.rs` — find path uses limit `200`
- Parent map: [[issues-60-post-v1-polish-map]]
- Sibling: [[issues-64-find-limit-flag]] will add `--limit`; docs here should state **current** default 200 and may say “limit is fixed at 200 until `--limit` lands” **or** after 64 merges, 64 updates the flag — write so either order works: document default 200; if `--limit` not in cli.rs yet, do not document the flag as shipped.

**Edits:**

1. **`odm find`:**
   - Synopsis: `odm find [query] [--progen …] [--progen-group …] [--json]` (drop `[facet-flags…]` unless implemented).
   - Body: federated **FTS** (not “+ facets” as shipped). Empty query behavior stays as implemented/tests say.
   - Note default max hits per store / overall as code does today (`200` in main) without inventing a flag if absent.
2. **`odm context`:**
   - Remove or reword “Thin-pass upstream scope flags (`--depth`, etc.)” — depth is **not** implemented; neighborhood is fixed one-hop as `ContextHit` (anchor/outgoing/incoming).
   - Keep disambiguation rules (`--progen`, `name:id`) that are real.
3. Do not change full vs sketch matrix except if find/context wording there is wrong.

**Verify:**

- `rg -n "facet|--depth" docs/reference/cli.md` — no claim those are shipped CLI.
- Cross-check clap: no facet/depth args.
- Docs-only → no cargo required.

**Acceptance criteria:**

- [ ] find synopsis has no facet-flags placeholder as current CLI
- [ ] find text does not claim facets as implemented
- [ ] context text does not claim `--depth` (or other unshipped upstream flags) as available
- [ ] default find limit 200 is documented honestly
- [ ] No Rust change

**Out of scope:**

- Implementing facets, depth, or `--limit` (limit is [[issues-64-find-limit-flag]])
- progen store engine changes

## Acceptance

- [ ] Agent Brief acceptance criteria all met

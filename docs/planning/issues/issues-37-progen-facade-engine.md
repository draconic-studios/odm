---
id: issues-37
title: "Progen façade engine depth"
description: "Deepen public Progen store handle and federation; hide index ensure/open; delete thin aliases."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - architecture
  - deepen
---

# Progen façade engine depth

## Description

Progen internals (note parse, SQLite FTS index, scope, vault scaffold) are deep, but the public surface is a flat free-function bag that re-runs ensure_index → open Connection on every op. Architecture calls for a swap-ready façade; that **seam** is not yet a real **adapter** boundary. Deepen a store handle + federation **module**.

Domain: Progen, Progen group, Workspace, ODM state directory.  
Architecture: small **interface**, large **implementation**; SQLite as adapter; two adapters only when a second engine exists (don't invent a fake second adapter).

## Affected

- `odm-progen` public ops, index, scope
- CLI progen find/ls/get/body/tree/backlinks/context/reindex/doctor
- Integration tests for progen vault

## Impact

Callers and tests re-learn index lifecycle; façade is not swap-ready; thin aliases inflate the surface; CLI owns reindex fan-out and flag cardinality.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-35-workspace-path-policy]] — index dir must come from shared Workspace path policy

## Agent Brief

**Category:** enhancement  
**Summary:** Deepen the Progen public interface into a store-oriented façade that owns index lifecycle and federation, without changing user-visible CLI semantics or Obsidian vault compatibility.

**Current behavior:**
- Public API is many free functions (find, list, get, body, tree, backlinks, context, reindex, formatters, scope helpers).
- Each read path resolves scope, then for every store: ensure index, open DB, query, merge.
- Some symbols are pure aliases (e.g. body vs get, dual single-progen resolve names).
- Index module speaks rusqlite Connection across internal module edges; swapping the engine means rewriting ops+index together.
- Federation find is loop+concat; context stays in-store only (correct product rule).
- CLI repeats “at most one `--progen`” and reindex-over-scope loops.

**Desired behavior:**
- A deep façade type (name flexible: store handle / engine session) obtained from Workspace + scoped Progen that:
  - Ensures/opens index once per handle lifetime (or equivalent explicit open)
  - Exposes find/list/get/context/reindex (and any other product ops) without callers touching Connection
- Federation sits at scope level: find/list across a resolved read scope using N handles; write/single-read still require a single Progen when multiple are configured (existing scope rules).
- Thin aliases removed or folded; one clear name per behavior.
- Single-progen flag / write-target rules are enforceable via the façade or scope helpers so CLI does not copy-paste cardinality checks.
- Reindex-over-scope is a library operation the CLI calls once.
- Obsidian vault ensure and note/wikilink behavior unchanged.
- Human formatters may remain temporarily but must not be required to use the façade; prefer DTOs the CLI already prints. Do not expand formatter surface.
- Internal SQLite details stay behind the façade. A formal `StoreEngine` trait is optional: only introduce if it clarifies the seam without a second unused adapter. Prefer a concrete deep type over a hypothetical port.

**Key interfaces:**
- Scope resolution: default-all, group union, write needs `--progen` when multi (existing progen scope rules)
- FindHit / note DTOs already returned to CLI — preserve JSON field contracts agents rely on
- Index location under `.odm/progen/<name>/` via path policy
- Vault ensure non-clobber semantics for `.obsidian/`

**Acceptance criteria:**
- [ ] Callers of find/list/get/context/reindex do not open or ensure the index themselves
- [ ] No public pure-alias ops that only rename another public op
- [ ] Federated find and in-store context semantics unchanged
- [ ] CLI progen commands keep the same flags, exit codes, and JSON/human success shapes
- [ ] Library supports reindex for a full read scope in one call
- [ ] Progen integration tests pass; add/adjust unit tests on the façade interface (not Connection)
- [ ] `cargo test` and `cargo clippy -- -D warnings` clean for touched crates

**Out of scope:**
- Replacing SQLite with upstream progenitor engine (seam readiness only)
- Cross-store ranking/global order beyond current merge behavior
- Graph/backlink engine redesign beyond existing context/backlinks behavior
- Moving all human formatters in this ticket
- Progen lifecycle add/rm ownership (membership issue) except consuming path helpers

## Comments

From architecture review 2026-08-01 (candidate #4, Worth exploring).

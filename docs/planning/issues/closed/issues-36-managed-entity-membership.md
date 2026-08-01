---
id: issues-36
title: "Managed-entity membership depth"
description: "Collapse project/progen lifecycle mirror; split lifecycle into checkout, pin maintain, and membership depths."
status: closed
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - ready-for-agent
  - architecture
  - deepen
---

# Managed-entity membership depth

## Description

`lifecycle` is a hot god-module mixing materialize/sync, pin maintain/apply, and project CRUD. Progen add/rm is a shallow mirror plus vault callback wired from the CLI. Deepen by domain: managed checkout, pin maintenance, Workspace membership with kind-specific hooks — not CLI verbs and not a second file of copy-paste.

Domain: Workspace, Project, Progen, Progen group, Pin file, Primary checkout.  
Architecture: delete the shallow progen mirror (**deletion test**); membership **module** owns choreography; vault/index hooks leave the bin.

## Affected

- `odm-core` lifecycle and progen_lifecycle
- CLI `project add|rm`, `progen add|rm`, sync, pin apply
- Progen vault scaffold composition

## Impact

Near file-size ceiling; drift between project and progen ops; CLI is the only place that knows vault must run on progen add.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-34-workspace-observation-depth]] — pin status / entity disk reporting should already share observation before further lifecycle splits touch those paths
- [[issues-35-workspace-path-policy]] — membership materialize/rm should use shared path helpers

## Agent Brief

**Category:** enhancement  
**Summary:** Reshape multi-git lifecycle into deep modules for managed checkout, pin maintenance, and membership mutation; unify Project and Progen add/rm choreography with progen-specific hooks; stop requiring the CLI to inject vault scaffold per call.

**Current behavior:**
- One large lifecycle surface owns materialize, sync ordering, pin auto-maintain after HEAD, pin apply, project add/rm/git, and entity disk info.
- Progen add/rm largely duplicates project add/rm: validate, mutate config, save, gitignore, optional materialize, pin maintain; progen adds vault `FnOnce`, group membership cleanup, and `.odm/progen/<name>` index delete.
- CLI wires `ensure_vault` into progen add.
- `remove_path` and similar helpers are duplicated across project/progen paths.

**Desired behavior:**
- **Managed checkout** depth: materialize (clone vs present vs origin match), sync fetch ordering, depth-sort of nested paths, clone URL resolve — shared for Project and managed Progen stores.
- **Pin maintenance** depth: when pins are written/pruned after successful operations; pin apply remains explicit. Auto-maintain policy stays product-correct but is readable in one place.
- **Workspace membership** depth: add/rm of Project or Progen is one choreography parameterized by kind:
  - Shared: name validation, config insert/remove, save, gitignore apply, optional materialize, pin maintain/prune
  - Progen-only hooks: vault scaffold, strip Progen group members, remove ODM-side progen index dir (via path policy helper)
  - Project-only hooks: none beyond shared, unless already required
- Vault scaffold composition is not the CLI’s job: either progen crate owns `progen add` wrapping core membership + vault, or core accepts a registered/vault adapter once — not a per-invocation closure threaded from main.
- Public CLI behavior and exit codes for `project add|rm`, `progen add|rm`, `sync`, `pin apply` remain compatible (including materialize outcome messaging).
- Dead or unused parallel entity-info paths that observation already replaced stay deleted rather than re-homed.

**Key interfaces:**
- Shared membership mutation for entity kind Project | Progen
- Managed checkout materialize/sync used by membership and `odm sync`
- Pin maintain/apply module boundary clear enough that pin policy can be reasoned about without reading project CRUD
- Vault ensure behavior already used for Obsidian-compatible empty vaults (README, non-clobber `.obsidian`, gitignore) must still run on progen add when creating store path

**Acceptance criteria:**
- [x] No shallow near-duplicate progen add/rm module that only renames project ops
- [x] CLI main does not pass a one-off vault closure into core for progen add (composition lives in a library crate)
- [x] Project and Progen add/rm still update config, gitignore, pins, and materialize consistently with prior semantics
- [x] Progen rm still drops group membership and ODM progen index dir
- [x] Sync / pin apply behavior and tests remain green
- [x] lifecycle-sized files stay under the repo hard LOC limit (1250) after the split
- [x] `cargo test` and `cargo clippy -- -D warnings` clean for touched crates

**Out of scope:**
- Re-deriving pin state (owned by observation issue)
- Redefining path layout (owned by path policy issue)
- Worktree slot lifecycle commands
- Agent packs
- Changing Workspace config schema keys

## Answer

Split the lifecycle god-module into three deep modules and moved vault composition into `odm-progen`:

- **`checkout.rs`** (451 LOC) — `ManagedEntity`, materialize, sync, depth-sort, clone URL resolve
- **`pin_maintain.rs`** (349 LOC) — `maintain_pins_after`, pin status/apply; `pin.rs` stays pure IO
- **`membership.rs`** (core, 490 LOC) — unified `membership_add`/`membership_rm` parameterized by `MembershipKind::{Project,Progen}`; progen hooks strip groups + drop ODM progen index dir; no vault `FnOnce`
- **`odm-progen::membership`** (120 LOC) — `add_progen` / `rm_progen` compose core membership + `ensure_vault` with prior vault timing
- **Deleted** `lifecycle.rs`, `progen_lifecycle.rs`; dropped unused `entity_disk_info` export
- CLI `main` calls `odm_progen::{add_progen,rm_progen}` — no vault closure into core

Regression tests: `progen_rm_strips_group_and_index_dir`, `path_only_add_scaffolds_vault_without_bin_closure`, `managed_no_clone_skips_vault`. Full `cargo test` + clippy `-D warnings` green.

## Comments

From architecture review 2026-08-01 (candidate #2, Strong).

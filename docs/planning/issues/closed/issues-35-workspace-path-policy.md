---
id: issues-35
title: "Workspace path policy module"
description: "Single core path module for primary checkout, worktree slot, progen index, and safe under-root resolve."
status: closed
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - architecture
  - deepen
---

# Workspace path policy module

## Description

Workspace layout path rules are split: weak `root.join` for checkouts, escape-safe resolve living under gitignore, Worktree slot paths hard-coded in actions, and Progen index dir duplicated in core rm vs progen index. Deepen one path **module** in core so all callers share layout truth (**locality**).

Domain: Workspace, Primary checkout, Worktree slot, ODM state directory, Progen.  
Architecture: one **interface** for path resolution; actions/progen/lifecycle become consumers, not owners.

## Affected

- `odm-core` path helpers (lifecycle checkout join, gitignore under-root resolve, progen index cleanup)
- `odm-actions` cwd resolution for `--wt`
- `odm-progen` index directory placement
- Future worktree commands

## Impact

Doctor can fail escape paths that lifecycle would still join; worktree layout can drift when slots land; index cleanup can miss the engine’s path rule.

## Proposed Fix

See Agent Brief.

## Blocked by

_(none)_

## Agent Brief

**Category:** enhancement  
**Summary:** Own all Workspace layout path resolution in odm-core behind a small path policy interface; migrate lifecycle, doctor/status, actions cwd, and progen index paths to it.

**Current behavior:**
- Primary checkout absolute path is often `workspace_root.join(config_relative_path)` with no `..` guard.
- Escape-safe resolve (relative, no `..`, under root) exists for doctor/status path checks but lives with gitignore marker logic.
- Action run with `--wt` builds `worktrees/<project>/<slot>/` inside the actions crate.
- Progen ODM-side index path under `.odm/progen/<name>/` is constructed in more than one crate for create vs delete.

**Desired behavior:**
- Core exports a cohesive path policy (names can vary) covering at least:
  - Primary checkout (and Progen store path) from config-relative path — safe under Workspace root (reject or error on escape; same rule doctor already uses for `path_declared`)
  - Worktree slot working tree: `worktrees/<project-name>/<slot-name>/` under Workspace root (not under `.odm/`)
  - ODM progen index/cache dir: `.odm/progen/<progen-name>/`
  - Optional helpers already implied by architecture: `odm_dir`, config/pin paths stay consistent with existing layout
- Lifecycle materialize/sync/add/rm, status, doctor, actions cwd, and progen index open/delete all use these helpers — no parallel string joins for the same concepts.
- Config load validation and runtime resolve agree: a path that fails `path_declared` must not be joinable into a checkout operation via the public path helpers.
- Actions `resolve_cwd` consumes core worktree/primary helpers; it no longer owns the worktrees layout string.
- Documented layout in architecture / CONTEXT remains the source of truth; code matches it.

**Key interfaces:**
- Safe resolve of config `path` → absolute path under Workspace root
- `worktree_slot_path(root, project_name, slot_name)` (or equivalent)
- `progen_index_dir(root, progen_name)` (or equivalent) used by both engine and lifecycle rm
- Actions cwd priority unchanged: task `dir` > worktree slot > project primary > workspace root

**Acceptance criteria:**
- [x] One core implementation defines Primary checkout resolve, Worktree slot path, and Progen index dir
- [x] Doctor `path_declared` and lifecycle checkout resolution share the same escape rules
- [x] Actions worktree cwd uses core Worktree slot path helper
- [x] Progen index create and lifecycle/progen rm target the same directory helper
- [x] Unit tests cover escape rejection and worktree/index path shape
- [x] Existing CLI/integration tests for actions cwd and progen still pass
- [x] `cargo test` and `cargo clippy -- -D warnings` clean for touched crates

**Out of scope:**
- Implementing full worktree add/rm commands
- Changing on-disk layout paths (only consolidate ownership)
- Action stdio / RunResult deepening (separate issue)
- Agent pack paths

## Answer

Shipped as `feat(core): workspace path policy module` (this commit).

- **`odm-core::paths`**: single owner for `resolve_under_root`, `abs_checkout` (Result), `worktree_slot_path`, `progen_index_dir`, plus `odm_dir` / config / pin helpers
- **Config load** `validate_rel_path` shares escape rules with runtime resolve
- **Lifecycle / progen / CLI / actions** use Result-based checkout; actions `--wt` uses `worktree_slot_path`; progen index open + `progen_rm` use `progen_index_dir`
- **Tests**: paths unit suite (escape + shapes), config escape reject, doctor escape sample; full `cargo test` green

## Comments

From architecture review 2026-08-01 (candidate #3, Strong).

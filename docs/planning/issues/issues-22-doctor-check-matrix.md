---
id: issues-22
title: "Doctor check matrix"
description: "Lock odm doctor checks and mechanical --fix repairs for core (ODM-side only)."
status: reviewing
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
  - ready-for-agent
---

# Doctor check matrix

## Question

What exact checks does `odm doctor` run in core (config load, declared paths, gitignore drift, pin consistency basics, …), what severities/exit behavior apply, and which repairs may `--fix` perform without destructive git rewrites?

## Blocked by

- [[issues-16-odm-git-shell-contract]]
- [[issues-17-config-and-pin-serde-model]]
- [[issues-21-gitignore-manage-markers]]

## Answer

**Recommended lock:**

### Severities / exit

- Check `status`: `pass` | `warn` | `fail`
- Command exit: `0` if no `fail` (warns OK); `3` if any `fail`; config/discovery problems before checks → `2`
- No fetch in doctor

### Check matrix (`id` → rule)

| id | severity if bad | rule |
|----|-----------------|------|
| `config_load` | fail | Config already loaded to run doctor; always pass if we got here (kept for JSON stability) |
| `odm_layout` | warn | `.odm/cache`, `.odm/log`, `.odm/progen` dirs exist (create on --fix) |
| `path_declared` | fail per entity | Each project/progen `path` is relative and resolves under Workspace root (no escape) |
| `path_exists` | warn | Declared path missing on disk (managed or not) |
| `managed_git` | fail | Managed entry path exists but is not a git repo |
| `origin_match` | fail | Managed git present; origin missing or normalized URL ≠ config url |
| `gitignore_drift` | warn | `manage_gitignore` and Workspace is git: on-disk managed block ≠ desired (markers per issues-21) |
| `pin_version` | fail | Pin file present but `version != 1` or invalid serde |
| `pin_unknown` | warn | Pin keys not in current managed set |
| `pin_rev_format` | fail | Pin rev not 40-char lowercase hex |
| `pin_url_mismatch` | warn | Pin entry url normalized ≠ config url for that name |

### `--fix` allowlist (mechanical only)

- Create missing `.odm/cache`, `.odm/log`, `.odm/progen` directories
- Rewrite managed gitignore blocks to desired content (issues-21)
- **Do not**: checkout/reset/clean, rewrite remotes, delete trees, rewrite pin file contents, drop pins, edit config

### Out of scope

Store-content doctor, worktree slots, agent packs, network checks.

## Comments

Parent map: [[issues-14-implement-core-map]]

Recommended decision locked for agent implement 2026-08-01.

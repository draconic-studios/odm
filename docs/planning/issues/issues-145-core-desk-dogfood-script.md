---
id: issues-145
title: "core-desk scripts/dogfood.sh full tour"
description: "Fail-fast shell script exercising all shipped odm commands against a temp copy of core-desk."
status: open
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - ready-for-agent
---

# core-desk scripts/dogfood.sh full tour

## Description

Manual README tour is long and incomplete. Need a single offline script that proves the product surface end-to-end and is the source of truth for “full demo.”

## Affected

- `examples/core-desk/scripts/dogfood.sh` (new)
- `examples/core-desk/README.md` — quick start + pointer to script

## Proposed Fix

See Agent Brief.

## Blocked by

[[issues-144-core-desk-assets-full-surface]]

## Agent Brief

**Category:** feat  
**Summary:** Executable dogfood tour with set -euo pipefail; temp copy of core-desk; ODM binary path configurable.

**Bindings:**

- Parent: [[issues-121-full-capability-demo-map]]
- Phases from swarm audit proposal (sync → pin → status → doctor → project git → worktree → progen façade → find groups → context/prompt → run → generate → packs → optional membership)

**Desired behavior:**

1. Usage: from monorepo root after `cargo build -p odm`, `ODM=target/debug/odm examples/core-desk/scripts/dogfood.sh` (or discover relative binary).
2. Copies core-desk to temp dir; `git init` + user identity for pin paths.
3. Phases with clear echo headers; fail-fast.
4. Must exercise: sync, pin status/apply, status --json, doctor, project list/info/git, worktree add/list/prune, find + --progen-group, context, agent prompt, progen get/body/tree/backlinks/ls/doctor/reindex, run hello/fail/chain + --project, generate dry-run/real/force, pack install/list/link/rm, agent start exit 1 honesty.
5. Cleanup temp on success (or trap).
6. README: Quick start (~10 lines) + “Full tour: scripts/dogfood.sh”.
7. No network; relative fixtures only.
8. jq optional — prefer `odm --json` + greptest/python -c if jq missing, or require jq and document.

**Acceptance criteria:**

- [ ] Script runs green on clean machine with rust+git
- [ ] Covers all shipped full commands listed above
- [ ] README points to script
- [ ] Does not modify the committed core-desk tree (temp copy)

**Out of scope:** Integration test gate (146); fixing product bugs found (147).

## Acceptance

- [ ] Agent Brief acceptance criteria all met

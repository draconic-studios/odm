---
id: issues-172
title: "Confirm agent-pack references remain only in historical docs"
description: "After the agent verb removal (12fa7e3), verify no live references to agent-pack/agent start remain outside historical docs (closed issues, CHANGELOG, review logs)."
status: open
issue-type: observation
severity: low
tags:
  - planning
  - issue
---

# Confirm agent-pack references remain only in historical docs

## Description

Post-`refactor(odm): remove agent verb (pack/prompt/start)` (12fa7e3), a scan
found `agent-pack` / `agent_pack` / `agent start` references only in historical
material:

- `CHANGELOG.md` (prior release entries)
- `docs/planning/issues/closed/` (closed issue notes, e.g. issues-158, issues-86, issues-50, issues-51)
- `docs/planning/issues/Index.md` (closed map list)
- `docs/logs/reviews/*` and `docs/reference/research/progenitor-surface.md`

No references remain in Rust code or the live website.

## Affected

- `CHANGELOG.md`, `docs/planning/issues/Index.md`, closed issue notes,
  review/research logs, `docs/logs/reviews/thermo-nuclear-code-quality-review.*`

## Observed

- `rg -li "agent.pack|agent.start|agent_start"` on the repo (excluding
  `target/`, `.git/`) returns only the historical files above
- `rg` for `agent_pack|agent-pack|agent start|AgentStart` in `*.rs` returns nothing
- Website e2e (32 tests) and Rust suite (398 tests) all pass after the removal

## Impact

Low — historical docs are the audit trail and should not be rewritten. Risk is
only if future readers mistake closed-issue references for live capability, or
if a later regeneration re-introduces the terms.

## Proposed Fix

- No code change required; treat historical mentions as intended (audit trail,
  per issue-tracker rules: don't delete closed issues)
- Optional: add a one-line note in `Index.md`'s closed-map entry or the
  CHANGELOG v0.1.2 entry stating the `agent` verb was removed, so readers of
  older entries have the context
- Re-run the scan above as a cheap regression check after any doc regeneration

## Comments

_(none yet)_

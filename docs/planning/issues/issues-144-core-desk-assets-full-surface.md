---
id: issues-144
title: "core-desk assets: multi-progen, groups, scoped actions"
description: "Expand examples/core-desk so all shipped surfaces can be dogfooded offline (second progen, groups, vault ids, project-scoped actions, gitignore)."
status: reviewing
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - ready-for-agent
---

# core-desk assets: multi-progen, groups, scoped actions

## Description

core-desk cannot demo federation, `--progen-group`, store façade backlinks, or `run --project/--wt` realistically. Assets need expansion before script/tour gate.

## Affected

- `examples/core-desk/.odm/odm.config.yaml`
- `examples/core-desk/progens/`
- `examples/core-desk/actions/core.yaml`
- `examples/core-desk/.gitignore`
- vault note frontmatter

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Expand core-desk fixtures/config for full shipped-CLI demo (assets only; script is 145).

**Bindings:**

- Parent: [[issues-121-full-capability-demo-map]]
- Existing layout in `examples/core-desk/README.md`

**Desired behavior:**

1. **Second Progen** `ops` at `progens/ops/` with note id `ops-note`, unique token `OpsUniqueToken`.
2. **progen_groups:** `default: [notes]`, `all-docs: [notes, ops]` (keep or replace current default).
3. **Vault graph:** `README.md` (or notes README) gets `id: readme` so wikilinks/backlinks work with Welcome.
4. **Actions:** add `in-alpha` (or similar) that checks a file in project tree; document invocation with `--project alpha` and `--wt`.
5. Optional `dir: projects/alpha` action if it stays clear.
6. **.gitignore:** ignore `worktrees/`, `out/`, `.odm/progen/`, `.odm/cache/`, `.odm/agent-packs.json` (keep projects/ + lock).
7. Do not break existing core_desk integration tests — update fixtures/URLs carefully; run `cargo test -p odm` and fix any path assumptions.
8. README can get a short “assets” note; full tour text is 145.

**Acceptance criteria:**

- [ ] Two progens + groups in config
- [ ] Distinct FTS tokens in each progen
- [ ] Wikilink targets have ids
- [ ] Project-scoped action present
- [ ] gitignore complete for dogfood debris
- [ ] Existing tests green (update if needed)

**Out of scope:** dogfood.sh (145); full tour test (146); managed Progen bare remote (stretch — skip unless trivial).

## Acceptance

- [ ] Agent Brief acceptance criteria all met

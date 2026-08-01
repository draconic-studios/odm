---
id: issues-136
title: "Docs honesty: Releases, AGENTS Pages, progen federation"
description: "Install docs claim empty GitHub Releases; AGENTS forbids Actions while Pages uses them; progen.md overclaims context/ls federation."
status: closed
issue-type: bug
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
---

# Docs honesty: Releases, AGENTS Pages, progen federation

## Description

Swarm docs audit found honesty gaps:

1. README / install.md / website: GitHub Releases primary — repo has **zero** releases.
2. `AGENTS.md` “No CI/CD or GitHub Actions” vs `.github/workflows/pages.yml` + README.
3. `progen.md` lists `context` and `ls` as federating; only `find` federates.
4. README Docs index omits `worktrees.md`, `env-gen-packs.md`.
5. website `guide-actions.html` unclosed paren (tiny).

## Affected

- README.md, AGENTS.md, docs/reference/install.md, progen.md
- website/install.html, guide-actions.html

## Impact

Users chase missing binaries; agents may delete Pages workflow; wrong federation expectations.

## Proposed Fix

See Agent Brief.

## Blocked by

None (cutting a real release is optional human step; docs must not claim empty channel)

## Agent Brief

**Category:** chore / docs  
**Summary:** Fix honesty without inventing a release. Narrow AGENTS. Fix progen federation wording. Link missing docs. Fix HTML typo.

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]
- Live site uses Actions Pages from `website/` on main

**Desired behavior:**

1. **Install:** Lead with build-from-source / `cargo install --path crates/odm`. Releases section = “when published” or remove primary claim until a tag exists. Align website install.html.
2. **AGENTS.md:** e.g. “No product CI test matrix; GitHub Actions allowed only for GitHub Pages deploy of `website/`.”
3. **progen.md:** Federating reads = `find` only (and any true fan-out). `context`, `progen ls/tree/get` = single-root.
4. **README Docs:** add worktrees.md, env-gen-packs.md (label graph.md sketch if linked).
5. **guide-actions.html:** fix unclosed paren.
6. No new GitHub release required in this ticket (human may cut 0.2.0 later).
7. Markdown in issues/ADRs: no tables; reference docs may keep existing tables.

**Acceptance criteria:**

- [x] No primary claim of existing Release assets
- [x] AGENTS allows Pages workflow only
- [x] progen.md federation accurate
- [x] README docs links updated
- [x] HTML typo fixed

**Out of scope:** Actually publishing a release; Playwright (116–118); pack list missing docs (107).

## Acceptance

- [x] Agent Brief acceptance criteria all met

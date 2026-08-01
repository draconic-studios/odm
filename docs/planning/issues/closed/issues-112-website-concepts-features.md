---
id: issues-112
title: "website concepts and features pages"
description: "Fill website concepts.html + features.html from CONTEXT.md and README/vision."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# website concepts and features pages

## Description

Add visitor-facing **Concepts** (domain vocabulary) and **Features** (what shipped) pages so the site explains the product model without dumping the whole vault.

## Affected

- `website/concepts.html`, `website/features.html`
- Shared nav/CSS from [[issues-110-website-shell-landing]]

## Impact

Landing alone does not teach Workspace / Project / Progen / worktree slot / agent pack.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-110-website-shell-landing]]

## Agent Brief

**Category:** feat  
**Summary:** Author concepts + features pages from CONTEXT.md, vision.md, and README Status — short, accurate, link out for depth.

**Bindings:**

- Parent map: [[issues-109-project-website-github-pages-map]]
- Shell: [[issues-110-website-shell-landing]]
- Sources: `CONTEXT.md`, `docs/reference/vision.md`, root `README.md` Status + command overview
- Optional skim: `docs/reference/architecture.md` (summary only — do not paste whole doc)

**Desired behavior:**

1. **`concepts.html`:** plain-language definitions for at least:
   - Workspace, Project, Progen, Progen group
   - ODM state directory (`.odm/`), Primary checkout, Worktree slot
   - Agent pack, Workspace config, Pin file, Action, Generator
   - Use CONTEXT.md meanings; include “Avoid” only if it helps clarity (optional, keep short).
2. **`features.html`:** honest capability list:
   - Shipped: multi-git Projects + pins, Progen find/context, Actions (`odm run`), doctor/status, worktree slots, local `odm generate` (+ dry-run if README says so), agent pack install/link/list/rm, thin `odm agent prompt`
   - Sketch / not done: `agent start`, pack marketplace, remote generate — label clearly as not shipped
3. Same nav + CSS; relative paths.
4. Link to GitHub `CONTEXT.md` and vision/architecture reference paths for full text.
5. No tables in any markdown you add; HTML may use simple lists (and simple HTML tables only if truly needed — prefer lists).
6. No GitHub Actions; no npm; no crate changes.

**Acceptance criteria:**

- [ ] `concepts.html` covers core CONTEXT terms listed above
- [ ] `features.html` separates shipped vs sketch/deferred honestly
- [ ] Nav/CSS consistent; relative links; GitHub deep links present
- [ ] No product claims beyond repo docs

**Out of scope:**

- Install/quickstart (111)
- Publish script (113)
- Root README honesty (114)
- Interactive demos / playground

## Acceptance

- [ ] Agent Brief acceptance criteria all met

---
id: issues-110
title: "website shell and landing page"
description: "Create website/ static shell: shared CSS, layout, index landing from vision/README."
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

# website shell and landing page

## Description

Add a pure-static project site root at `website/` with shared styling and a landing page that introduces ODM and points visitors to install and the GitHub repo.

## Affected

- New tree: `website/` (`index.html`, `assets/style.css`, optional shared nav snippet pattern)
- No Rust crates; no `.github/workflows`

## Impact

No public marketing/docs surface for the product beyond the README on GitHub.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Scaffold `website/` with shared CSS + landing page aligned with vision/README; relative links only.

**Bindings:**

- Parent map: [[issues-109-project-website-github-pages-map]]
- Copy/facts from: root `README.md`, `docs/reference/vision.md`, `CONTEXT.md` (terms only if mentioned on landing)
- Repo: `https://github.com/hembrow-innovations/odm`
- Expected Pages URL (document in comment only until 114): `https://hembrow-innovations.github.io/odm/`

**Desired behavior:**

1. **Layout:** `website/index.html`, `website/assets/style.css`. Optional `website/assets/` only — no build step.
2. **Landing content (minimum):**
   - Product name **ODM** + one-line from vision/README (poly-repo workspace OS for humans and AI agents).
   - Short “who it’s for” (humans + agents, same Workspace).
   - Primary CTAs: link to `install.html` (stub OK if 111 not done — create placeholder `install.html` with “coming” only if needed; prefer real heading + “see GitHub README” fallback), link to GitHub repo, link to releases if already referenced in README.
   - Status blurb honest to README Status line (v0.1.0 spine + post-0.1 worktrees/generate/packs) — do not overclaim.
3. **Nav:** site-wide nav on index: Home, Install, Quickstart, Concepts, Features (hrefs to pages 111/112 will add; create minimal placeholder pages with title + “Content in follow-up” **or** only link pages that exist — prefer **create thin placeholder HTML files** for install/quickstart/concepts/features so nav never 404s).
4. **CSS:** readable, responsive (mobile-friendly), system fonts OK; dark-friendly optional but not required. No external CDN fonts/scripts required for first paint (no tracking).
5. **Paths:** all CSS and internal links **relative** (e.g. `assets/style.css`, `install.html`) so project Pages subpath works.
6. **`website/README.md`:** 5–15 lines — purpose, open `index.html` locally, publish is ticket 113, no Actions.
7. Do not put site under `docs/`. Do not add GitHub Actions.
8. No npm/package.json.

**Acceptance criteria:**

- [ ] `website/index.html` + `website/assets/style.css` exist and load via relative paths
- [ ] Landing states what ODM is, CTAs to install + GitHub
- [ ] Nav present; placeholder or real sibling pages so nav targets exist
- [ ] `website/README.md` explains local preview
- [ ] No `.github/workflows`; no Node toolchain

**Out of scope:**

- Full install/quickstart/concepts copy (111, 112)
- Publish script / gh-pages (113)
- README root honesty (114)
- Enabling Pages in GitHub UI (115)

## Acceptance

- [ ] Agent Brief acceptance criteria all met

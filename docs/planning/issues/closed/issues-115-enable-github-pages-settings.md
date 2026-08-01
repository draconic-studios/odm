---
id: issues-115
title: "Enable GitHub Pages for odm repo"
description: "Human: Settings → Pages → deploy gh-pages / root; verify site URL loads."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-human
  - wayfinder
  - wayfinder-task
---

# Enable GitHub Pages for odm repo

## Description

GitHub repo settings must point Pages at the published `gh-pages` branch. Agents cannot complete org/repo admin UI; a human with admin access does this once.

## Affected

- GitHub repo `hembrow-innovations/odm` → Settings → Pages
- Verification of `https://hembrow-innovations.github.io/odm/`

## Impact

Until enabled, publish script updates `gh-pages` but the public site URL does not serve.

## Proposed Fix

See steps below.

## Blocked by

- [[issues-113-website-pages-publish]]
- Prefer content tickets [[issues-111-website-install-quickstart]] and [[issues-112-website-concepts-features]] done before first public enable (optional but recommended)

## Steps (human)

1. Ensure `website/` content is ready and `./scripts/pages-publish.sh` has been run with push (`ODM_PAGES_PUSH=1` or documented flag) so `origin/gh-pages` exists with `index.html` at root.
2. GitHub → **Settings** → **Pages**.
3. **Build and deployment:** Deploy from a **branch** (not Actions).
4. Branch: **`gh-pages`**, folder: **`/` (root)** → Save.
5. Wait for Pages build; open `https://hembrow-innovations.github.io/odm/`.
6. Spot-check: Home, Install, Quickstart, Concepts, Features, CSS loads.
7. Comment on this issue / close when live; confirm [[issues-114-website-docs-honesty]] URL matches.

## Acceptance

- [x] Pages source folder is `/` (root) (no Actions workflow)
- [ ] `https://hembrow-innovations.github.io/odm/` serves landing — pending site content + first publish (110–113)
- [ ] Nav pages load; no broken CSS — pending content

## Answer

Human enabled GitHub Pages with deploy folder **`/` (root)** (2026-08-01). Site content and `gh-pages` publish still land via [[issues-110-website-shell-landing]]–[[issues-113-website-pages-publish]]; full URL verify after first publish.

## Out of scope

- Custom domain
- Actions-based deploy
- Writing site content (agent tickets 110–112)

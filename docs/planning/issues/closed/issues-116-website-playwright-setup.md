---
id: issues-116
title: "Website Playwright setup"
description: "Add Playwright toolchain for website/ static site (local config, deps, scripts, docs)."
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

# Website Playwright setup

## Description

The project site lives under `website/` (static HTML/CSS, deployed via `.github/workflows/pages.yml`). There is no browser test harness. Add Playwright so later tickets can smoke-test and review the site.

## Affected

- New: root or `website/` Node package for Playwright only (prefer **repo-root** `package.json` scoped to site tests, or `website/package.json` — pick one place and document it)
- `playwright.config.*`, `.gitignore` entries for Playwright artifacts
- `website/README.md` (how to install/run)
- Optional: document in root `README.md` under Development (one line)

## Impact

No automated way to verify nav, pages, or regressions on the public site.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** chore  
**Summary:** Install and configure Playwright against static `website/` served locally; no product HTML changes required beyond ignoring test artifacts.

**Bindings:**

- Site: `website/` (see `website/README.md`, closed map [[issues-109-project-website-github-pages-map]])
- Deploy: `.github/workflows/pages.yml` (do not expand into full CI matrix unless a single optional job is clearly needed — default **local-only** scripts; no requirement to wire CI in this ticket)
- Standing: prefer minimal surface; site stays pure static (no SSG)

**Desired behavior:**

1. **Package location:** one `package.json` (recommend `website/package.json` so Rust root stays clean) with devDependency `@playwright/test` (current stable).
2. **Config:** `playwright.config.ts` (or `.mjs`) that:
   - `testDir` → e.g. `website/e2e` or `website/tests`
   - Serves `website/` via `webServer` (e.g. `npx serve website -l 4173` or `python3 -m http.server` — Node `serve` is fine if added as devDep)
   - `baseURL` matches that server
   - Screenshot/video on failure only; output dirs gitignored
3. **Scripts:** `test:e2e` / `test:e2e:ui` (or equivalent) in that package.json.
4. **Install path documented:** `npm install` (or `pnpm`/`npm` — match whatever you introduce; **one** package manager, lockfile committed).
5. **Browsers:** document `npx playwright install` (chromium sufficient for v1).
6. **Placeholder test:** one trivial passing test (e.g. home title contains `ODM`) so setup is verified green.
7. **Gitignore:** `test-results/`, `playwright-report/`, `blob-report/`, playwright cache if any under site tree.
8. Do **not** implement full suite (117) or redesign (118).
9. Do **not** add Rust/crate changes. Avoid broad monorepo Node tooling beyond this package.

**Acceptance criteria:**

- [x] Playwright installed with lockfile; config serves `website/`
- [x] One smoke test passes locally after documented install steps
- [x] `website/README.md` documents install + run
- [x] Artifacts gitignored; no CI requirement unless already trivial
- [x] No full page matrix (that is 117)

**Out of scope:**

- Full e2e coverage ([[issues-117-website-playwright-tests]])
- UX review/redesign ([[issues-118-website-playwright-review-improve]])
- Visual regression baselines (optional later; not required here)
- Testing the live github.io URL as primary (local static server is source of truth)

## Acceptance

- [x] Agent Brief acceptance criteria all met

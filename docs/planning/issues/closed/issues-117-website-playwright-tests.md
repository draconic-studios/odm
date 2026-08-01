---
id: issues-117
title: "Website Playwright smoke tests"
description: "E2E coverage for website/ pages: nav, key content, internal links, no broken assets."
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

# Website Playwright smoke tests

## Description

With Playwright configured (116), add a real smoke suite so every public page loads, primary nav works, and critical content/claims stay present.

## Affected

- `website/e2e/` (or path chosen in 116)
- Possibly small `data-testid` / stable selectors on HTML if needed (prefer role/text/accessible selectors first)
- `website/README.md` if script names change

## Impact

Site regressions (broken nav, missing install path, 404 assets) ship unnoticed.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-116-website-playwright-setup]]

## Agent Brief

**Category:** test  
**Summary:** Playwright smoke tests for all `website/*.html` pages and shared chrome (nav, CSS, CTAs).

**Bindings:**

- Setup: [[issues-116-website-playwright-setup]]
- Pages (current set): `index`, `install`, `quickstart`, `concepts`, `features`, `guides`, `guide-workspace`, `guide-projects`, `guide-progen`, `guide-worktrees`, `guide-actions`, `guide-agents`, `cli`, `config`
- Product honesty: do not assert sketch features as shipped; mirror README Status / features page

**Desired behavior:**

1. **All pages load:** each HTML file returns 200 via in-app navigation or direct `goto`; `<h1>` (or main landmark) visible.
2. **Home:** title/lead mentions ODM; CTAs to Install and Quickstart (or GitHub) work.
3. **Nav:** from home, each primary nav target is reachable; `aria-current="page"` (or equivalent) set on the active page when present.
4. **Install:** shows release install path and/or build-from-source; link to GitHub releases present.
5. **Quickstart:** shows `odm init` (or equivalent bootstrap) in a code block.
6. **Concepts:** at least Workspace, Project, Progen terms present.
7. **Features:** distinguishes shipped vs sketch (e.g. agent start / deferred called out).
8. **Guides hub + each guide:** loads; guide hub links resolve without 404.
9. **CLI + Config:** load; config example or key list visible.
10. **Assets:** `assets/style.css` loads (response ok) on at least home + one deep page.
11. **No console errors** on home and install (filter known benign if any; fail on page errors).
12. **Mobile smoke (optional but preferred):** one test with a mobile viewport — nav usable or content readable (no horizontal-break assertion unless easy).
13. Keep tests stable: prefer `getByRole`, `getByText`; avoid brittle CSS class chains.
14. Suite green via documented `npm test` / `test:e2e` from 116.
15. No large visual redesign (118). Minimal HTML tweaks only if required for a11y selectors.

**Acceptance criteria:**

- [x] Every `website/*.html` page covered by at least one load assertion
- [x] Nav + install + quickstart + concepts + features critical content asserted
- [x] CSS asset check; suite documented and green locally
- [x] No dependency on live github.io for CI/local default

**Out of scope:**

- Playwright install/config (116)
- UX polish / content rewrite (118)
- Screenshot golden files / Percy
- Testing odm CLI binary

## Acceptance

- [x] Agent Brief acceptance criteria all met

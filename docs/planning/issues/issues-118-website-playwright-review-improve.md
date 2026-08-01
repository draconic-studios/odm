---
id: issues-118
title: "Website Playwright review and improve"
description: "Use Playwright (and manual pass) to review website UX/a11y/content gaps; fix issues found."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# Website Playwright review and improve

## Description

After smoke tests exist, actively **review** the site with Playwright (headed/UI mode, a11y checks, link audit) and **improve** HTML/CSS/copy so newcomers can understand and use ODM. Land fixes with tests updated.

## Affected

- `website/**/*.html`, `website/assets/style.css`
- E2E tests from 117 (extend/adjust as behavior improves)
- `website/README.md` if preview/test notes change
- Honesty vs root `README.md` / `docs/reference/*` — no new product claims

## Impact

Site may pass smoke checks but still have weak IA, a11y gaps, inconsistent nav, or unclear onboarding.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-116-website-playwright-setup]]
- [[issues-117-website-playwright-tests]]

## Agent Brief

**Category:** feat  
**Summary:** Review `website/` with Playwright-assisted checks; fix UX/a11y/content issues; keep e2e green and product claims honest.

**Bindings:**

- Tests: [[issues-117-website-playwright-tests]]
- Setup: [[issues-116-website-playwright-setup]]
- Authority for facts: root `README.md`, `CONTEXT.md`, `docs/reference/vision.md`, `install.md`, `cli.md`
- Live URL optional cross-check only: https://hembrow-innovations.github.io/odm/ (local server remains source of truth)

**Review checklist (run and record findings in PR/commit body or `website/REVIEW.md` temporary notes — prefer fixing over long writeups; delete scratch notes before finish or keep a short “Fixed” list in commit message):**

1. **Information architecture:** Can a new user go Install → Quickstart → first success path without confusion? Fix ordering/CTAs/copy if not.
2. **Navigation:** Consistent side nav on all pages; current page indicated; mobile layout usable.
3. **Accessibility:** headings hierarchy; skip link works; focus visible; contrast adequate on body/text/buttons; images (if any) have alt; interactive elements keyboard-reachable.
4. **Playwright a11y (preferred):** add `@axe-core/playwright` (or equivalent) scan on home + install + one guide; fix serious/critical violations found.
5. **Broken / external links:** crawl internal links; spot-check key external GitHub links (don’t flake on network — soft-skip external failures if offline).
6. **Content gaps:** missing “what is ODM”, install verify, dogfood pointer, shipped vs sketch honesty; fill from repo docs without inventing features.
7. **Polish:** spacing, code block overflow on small screens, button/link affordances; keep design system coherent (existing CSS variables).
8. **Performance light touch:** no huge assets; no unnecessary third-party scripts/fonts unless justified.

**Desired behavior after fixes:**

1. Concrete HTML/CSS (and minor copy) improvements landed.
2. E2E suite still green; new assertions for any new critical UI.
3. A11y scan clean of serious/critical on scanned pages (or document residual with reason).
4. No Node runtime required for the published static site (devDependencies for tests only).
5. No GitHub Actions expansion required beyond existing pages deploy (optional: add e2e job only if cheap and non-flaky — default skip CI e2e).

**Acceptance criteria:**

- [ ] Written or commit-summarized review against checklist above
- [ ] Material improvements merged for issues found (not “no issues” without evidence)
- [ ] A11y serious/critical addressed on home + install (+ one guide)
- [ ] Playwright suite green
- [ ] Product claims still match README/reference

**Out of scope:**

- Playwright initial setup (116) / first smoke suite authorship (117) except extending tests
- Blog, i18n, CMS, analytics
- Redesigning the odm CLI or Rust crates
- Custom domain

## Acceptance

- [ ] Agent Brief acceptance criteria all met

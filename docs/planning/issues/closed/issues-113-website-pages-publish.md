---
id: issues-113
title: "website GitHub Pages publish script"
description: "Add scripts/pages-publish.sh to publish website/ to orphan gh-pages; document usage (no Actions)."
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

# website GitHub Pages publish script

## Description

Provide a **local** publish path so `website/` becomes the content of branch `gh-pages` for GitHub project Pages — without GitHub Actions.

## Affected

- `scripts/pages-publish.sh` (new)
- `website/README.md` (publish section)
- Possibly root `.gitignore` only if a temp dir is used and might leak (prefer mktemp)

## Impact

Site exists in-repo but cannot be attached as the GitHub Pages site without a repeatable publish step.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-110-website-shell-landing]]

## Agent Brief

**Category:** feat  
**Summary:** Shell script publishes `website/` tree to `gh-pages` (orphan or update) for GitHub Pages; document; no Actions.

**Bindings:**

- Parent map: [[issues-109-project-website-github-pages-map]]
- Site root: `website/`
- Standing rule: **No CI/CD or GitHub Actions**
- Human Pages settings: [[issues-115-enable-github-pages-settings]] **closed** — folder `/` (root) already enabled; this ticket still must create/push `gh-pages` with `website/` contents
- Existing script style: `scripts/release-build.sh` (read for tone/safety patterns)

**Desired behavior:**

1. **`scripts/pages-publish.sh`:**
   - Run from repo root (or detect root via script location).
   - Require clean check of only what’s needed: fail if `website/` missing or empty of `index.html`.
   - Publish **contents** of `website/` to branch **`gh-pages`** at branch root (so Pages “Deploy from branch → gh-pages → /” serves `index.html`).
   - Prefer: worktree or temp dir + orphan branch create/update; commit message like `chore(pages): publish website`.
   - Push to `origin gh-pages` only when `ODM_PAGES_PUSH=1` (or `--push` flag); default is local branch update + print next steps (mirrors release-build optional publish pattern).
   - Never force-push `main`. Avoid `git push --force` unless updating orphan history is required — if force needed on `gh-pages` only, document it and gate behind explicit env e.g. `ODM_PAGES_FORCE=1`.
   - No network except optional git push.
2. **`website/README.md`:** document:
   - `./scripts/pages-publish.sh`
   - `ODM_PAGES_PUSH=1 ./scripts/pages-publish.sh`
   - Expected site URL after human enables Pages: `https://hembrow-innovations.github.io/odm/`
   - Pointer that enabling Pages in GitHub Settings is [[issues-115-enable-github-pages-settings]] / human.
3. Do **not** add `.github/workflows/*`.
4. Script should be `set -euo pipefail` (or equivalent safe bash); executable bit set.
5. Do not modify Rust code.

**Acceptance criteria:**

- [ ] `scripts/pages-publish.sh` exists, safe defaults, publishes `website/` → `gh-pages` root
- [ ] Push is opt-in via env or flag
- [ ] `website/README.md` documents publish + expected URL
- [ ] No GitHub Actions added
- [ ] Script does not force-push `main`

**Out of scope:**

- Enabling Pages in GitHub UI (115)
- Root README / install.md link updates (114)
- Content page authorship (111, 112) — script may run with shell-only site
- Custom domain

## Acceptance

- [ ] Agent Brief acceptance criteria all met

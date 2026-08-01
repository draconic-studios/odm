---
id: issues-114
title: "website docs honesty README and install"
description: "Link project website from README and install.md; note Pages publish path."
status: closed
issue-type: feature-request
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# website docs honesty README and install

## Description

Once the site and publish path exist, point consumers at the project website from root README and install reference — without claiming the live URL works before Pages is enabled.

## Affected

- `README.md`
- `docs/reference/install.md`
- Optionally `CHANGELOG.md` Unreleased / next note (only if repo habit is to log docs)
- `website/README.md` only if cross-link needed

## Impact

Site is easy to miss; install path stays GitHub-only.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-113-website-pages-publish]]

## Agent Brief

**Category:** docs  
**Summary:** Honest README + install.md links to website source (`website/`) and published URL; do not claim Pages is live if unknown.

**Bindings:**

- Parent map: [[issues-109-project-website-github-pages-map]]
- Publish: [[issues-113-website-pages-publish]]
- Human enable: [[issues-115-enable-github-pages-settings]]
- URL: `https://hembrow-innovations.github.io/odm/`
- In-repo site: `website/`

**Desired behavior:**

1. **`README.md`:**
   - Near top or Docs section: **Website** link to `https://hembrow-innovations.github.io/odm/` with note that source is `website/` and publish is `scripts/pages-publish.sh`.
   - If Pages may not be enabled yet, phrase as “Project site (GitHub Pages): …” and “source: `website/`” so a 404 is a settings issue not a docs lie — one short sentence is enough.
2. **`docs/reference/install.md`:** link to website install page path on Pages (`…/odm/install.html`) and/or in-repo `website/install.html`.
3. Do not add GitHub Actions.
4. Do not rewrite vision/architecture.
5. Keep markdown style: no tables (use `- **label**: text` if needed).

**Acceptance criteria:**

- [ ] README mentions website URL + `website/` source
- [ ] install.md links to site install and/or `website/install.html`
- [ ] Wording stays honest if Pages not yet enabled
- [ ] No Actions / no crate changes

**Out of scope:**

- Enabling Pages (115)
- New site content pages
- Full docs portal migration

## Acceptance

- [ ] Agent Brief acceptance criteria all met

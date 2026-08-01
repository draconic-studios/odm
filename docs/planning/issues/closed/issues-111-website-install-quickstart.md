---
id: issues-111
title: "website install and quickstart pages"
description: "Fill website install.html + quickstart.html from install.md and README quickstart."
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

# website install and quickstart pages

## Description

Replace placeholders with real **Install** and **Quickstart** pages on the static site, sourced from existing reference docs — not a second source of truth that drifts.

## Affected

- `website/install.html`, `website/quickstart.html`
- Shared nav/CSS from [[issues-110-website-shell-landing]]

## Impact

Visitors cannot install or try ODM from the project site alone.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-110-website-shell-landing]]

## Agent Brief

**Category:** feat  
**Summary:** Author `install.html` and `quickstart.html` from `docs/reference/install.md` + README; keep claims honest; link to GitHub for deep detail.

**Bindings:**

- Parent map: [[issues-109-project-website-github-pages-map]]
- Shell: [[issues-110-website-shell-landing]]
- Sources: `docs/reference/install.md`, root `README.md` Install/Quickstart sections
- Releases: `https://github.com/hembrow-innovations/odm/releases`

**Desired behavior:**

1. **`install.html`:**
   - Requirements: Rust 1.70+ (from source), git on PATH; Actions need Unix shell (as README).
   - Primary: download release tarball, extract `odm` onto PATH, `odm --version`.
   - Secondary: build from source (`cargo build -p odm --release` / `cargo install --path crates/odm`).
   - Link to full install doc on GitHub (`docs/reference/install.md` blob/main URL).
2. **`quickstart.html`:**
   - Commands from README: `odm init`, project add, `sync`, `pin status`, `status`, `doctor`.
   - Mention Progen find/context and `odm run` briefly.
   - Mention worktree/generate/agent pack only at README honesty level (one short section or bullets — not full CLI reference).
   - Link to `docs/reference/cli.md` on GitHub for full surface.
3. Same nav + CSS as shell; relative paths; responsive.
4. Do not invent flags or commands not in README/cli.md.
5. No publish script changes unless a nav label fix is required.
6. No GitHub Actions; no npm.

**Acceptance criteria:**

- [ ] `install.html` covers release install + from-source; links to GitHub install doc + releases
- [ ] `quickstart.html` covers init → sync → status/doctor path from README
- [ ] Shared nav/CSS consistent with 110; relative links
- [ ] No overclaims vs README Status

**Out of scope:**

- Concepts/features pages (112)
- gh-pages publish (113)
- Root README website URL (114)
- Full CLI reference mirror

## Acceptance

- [ ] Agent Brief acceptance criteria all met

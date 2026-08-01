---
id: issues-109
title: "Project website GitHub Pages map"
description: "Wayfinder map: static project site under website/ deployable as GitHub Pages (no Actions)."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Project website GitHub Pages map

## Destination

Ship a **public project website** for ODM that can be the repo’s GitHub Pages site:

1. **Static site shell + landing** under `website/` (no Node, no SSG, no GitHub Actions).
2. **Content pages** — install, quickstart, concepts (domain terms), feature overview aligned with README/vision.
3. **Publish path** — local script to push `website/` to orphan `gh-pages` branch; README/docs honesty for URL.
4. **Human enable** — once: GitHub repo Settings → Pages → deploy from `gh-pages` / root.

## Notes

- **Authority:** root `README.md`, `CONTEXT.md`, `docs/reference/vision.md`, `docs/reference/install.md`, `docs/reference/cli.md`.
- **Prereqs:** none (orthogonal to pack-list map [[issues-104-post-v1-pack-list-missing-map]]).
- **Execution:** ticket close = decision + code/docs as scoped. Prefer small files; keep each HTML page self-contained via shared CSS.
- **Standing prefs (AFK defaults):**
  - **No CI/CD / GitHub Actions** (repo standing rule).
  - Site lives at repo root **`website/`** — not under `docs/` (Obsidian vault / planning issues stay private-by-structure; do not make vault the Pages root).
  - **Pure static** HTML + one CSS file (+ optional tiny JS only if needed for nav). No npm, no mdbook, no Vite.
  - **Relative asset/link paths** so the same tree works from `file://`, local static server, and project Pages base `https://hembrow-innovations.github.io/odm/`.
  - Do not invent product claims beyond README / reference docs; link to GitHub for deep docs.
  - Publish script must not force-push `main`; only update `gh-pages` (or create orphan branch).
  - Do not commit secrets. Do not add `.github/workflows/`.
  - File size ≤1000 target / ≤1250 hard per file.

## Decisions so far

- Child tickets: [[issues-110-website-shell-landing]], [[issues-111-website-install-quickstart]], [[issues-112-website-concepts-features]], [[issues-113-website-pages-publish]], [[issues-114-website-docs-honesty]], [[issues-115-enable-github-pages-settings]].
- Prefer order: 110 first; 111 and 112 blocked by 110 (parallel after); 113 blocked by 110 (can land before full content); 114 blocked by 113; 115 human blocked by 113.
- **115 closed early:** human enabled Pages deploy folder **`/` (root)**.
- **110–114 closed:** full static site (home, install, quickstart, concepts, features, guides, CLI, config); README/install honesty.
- **Site home:** `website/` on `main`; publish contents to Pages branch root via `scripts/pages-publish.sh` (Pages folder `/`).

## Not yet specified

_(none — AFK defaults lock stack and layout)_

## Out of scope

- GitHub Actions / CI deploy
- Blog, search, i18n, analytics, CMS
- Hosting full `docs/reference/` or vault as the site
- Changing Rust crates / CLI behavior
- Custom domain (unless human later asks)
- Pack marketplace, agent start, generate remote

## Blocked by

None

# ODM project website

Static HTML/CSS under `website/` on **main** only. No second branch.

## Local preview

```bash
# from repo root
python3 -m http.server 8080 --directory website
# → http://127.0.0.1:8080/
```

## Browser tests (Playwright)

Local-only smoke tests against a static server of this directory. Package manager: **npm** (lockfile committed).

```bash
cd website
npm install
npx playwright install chromium
npm run test:e2e
# optional UI mode:
npm run test:e2e:ui
```

Config: `playwright.config.ts` (serves `.` on port 4173 via `serve`). Tests live in `e2e/`:

- **`e2e/home.spec.ts`** — harness smoke (title)
- **`e2e/smoke.spec.ts`** — all `website/*.html` pages, nav, install/quickstart/concepts/features content, CSS asset, mobile viewport
- **`e2e/a11y.spec.ts`** — `@axe-core/playwright` on home + install + one guide (serious/critical clean); skip-link + start-here assertions
- **`e2e/links.spec.ts`** — crawl internal `a[href]` targets and in-page anchors

Artifacts (`test-results/`, `playwright-report/`, etc.) are gitignored. No live github.io dependency. DevDependencies only — the published static site needs no Node runtime.

## GitHub Pages

GitHub branch deploy only supports `/` or `/docs`, not `/website`. This repo deploys `website/` via a single workflow: `.github/workflows/pages.yml`.

**Settings → Pages → Build and deployment → Source: GitHub Actions**

Push to `main` (when `website/**` changes) publishes https://hembrow-innovations.github.io/odm/

## Content sources

- root `README.md`, `CONTEXT.md`
- `docs/reference/` (vision, install, cli, config, multi-git, progen, worktrees)

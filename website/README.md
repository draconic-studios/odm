# ODM project website

Static HTML/CSS under `website/` on **main** only. No second branch.

## Local preview

```bash
# from repo root
python3 -m http.server 8080 --directory website
# → http://127.0.0.1:8080/
```

## GitHub Pages

GitHub branch deploy only supports `/` or `/docs`, not `/website`. This repo deploys `website/` via a single workflow: `.github/workflows/pages.yml`.

**Settings → Pages → Build and deployment → Source: GitHub Actions**

Push to `main` (when `website/**` changes) publishes https://hembrow-innovations.github.io/odm/

## Content sources

- root `README.md`, `CONTEXT.md`
- `docs/reference/` (vision, install, cli, config, multi-git, progen, worktrees)

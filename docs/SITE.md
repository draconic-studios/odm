# ODM project website

Static HTML/CSS for the public project site, living at the root of `docs/` so GitHub Pages can serve **main** + **`/docs`** (no second branch, no Actions).

## Local preview

```bash
# from repo root
python3 -m http.server 8080 --directory docs
# → http://127.0.0.1:8080/
```

## Publish

Push `main`. Pages builds from this folder automatically.

URL: https://hembrow-innovations.github.io/odm/

## Layout

- Site pages: `docs/*.html`, `docs/assets/`
- Design docs (markdown): `docs/reference/`, `docs/agents/`, `docs/planning/`, `docs/adr/`

## Content sources

- root `README.md`, `CONTEXT.md`
- `docs/reference/` (vision, install, cli, config, multi-git, progen, worktrees)

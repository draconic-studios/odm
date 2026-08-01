# ODM project website

Static HTML/CSS for the public project site. No Node, no SSG, no GitHub Actions.

## Local preview

```bash
# from repo root
python3 -m http.server 8080 --directory website
# → http://127.0.0.1:8080/
```

## GitHub Pages

Source tree is `website/` on `main`. Publish the **contents** of this folder to the branch/folder your Pages settings use (typically branch with folder `/`).

```bash
# update local gh-pages from website/ (no push)
./scripts/pages-publish.sh

# push gh-pages
ODM_PAGES_PUSH=1 ./scripts/pages-publish.sh
```

Expected URL: https://hembrow-innovations.github.io/odm/

## Content sources

- root `README.md`, `CONTEXT.md`
- `docs/reference/` (vision, install, cli, config, multi-git, progen, worktrees)

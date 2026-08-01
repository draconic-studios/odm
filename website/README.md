# ODM project website

Static HTML/CSS for the public project site. No Node, no SSG, no GitHub Actions.

## Local preview

Open any page in a browser, or serve the directory:

```bash
# from repo root
python3 -m http.server 8080 --directory website
# → http://127.0.0.1:8080/
```

All internal links and CSS use relative paths so the tree works on GitHub project Pages at `https://hembrow-innovations.github.io/odm/`.

## Publish to GitHub Pages

Pages is configured to deploy from branch **`gh-pages`**, folder **`/` (root)**.

```bash
# update local gh-pages branch from website/ (no push)
./scripts/pages-publish.sh

# push to origin
ODM_PAGES_PUSH=1 ./scripts/pages-publish.sh
```

After push, the site is at: https://hembrow-innovations.github.io/odm/

## Content sources

Copy stays aligned with repo docs:

- root `README.md`, `CONTEXT.md`
- `docs/reference/` (vision, install, cli, config, multi-git, progen, worktrees)

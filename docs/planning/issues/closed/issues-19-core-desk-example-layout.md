---
id: issues-19
title: "core-desk example layout"
description: "Prototype examples/core-desk: fixtures, config, README dogfood path for core only."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-prototype
  - ready-for-agent
---

# core-desk example layout

## Question

What is the on-disk layout of `examples/core-desk` — bare fixture repos, Workspace config, paths, and README dogfood steps — so core multi-git and status/doctor can be exercised offline?

## Blocked by

_(none — frontier)_

## Answer

**Recommended lock:**

```text
examples/core-desk/
  README.md
  fixtures/
    README.md          # how fixtures are built/refreshed
    alpha.git/         # bare git repo (committed)
    beta.git/          # bare git repo (committed)
  .odm/
    odm.config.yaml    # pre-seeded consumer Workspace config
  .gitignore           # optional seed; harness may also exercise manage_gitignore
```

### Rules

- **No managed checkouts committed** (`projects/alpha`, `projects/beta` absent in git) — produced by `odm sync`.
- **No pin file committed** — created on first materialize when Workspace is a git repo.
- Fixtures are **bare** repos with at least one commit on `main` (file `README.md` inside each).
- Config URLs use **absolute `file://` paths resolved at dogfood/harness time** OR relative path form that harness rewrites — **harness must rewrite fixture URLs** to absolute `file://` of the temp copy (portable; avoids committed machine-absolute paths).
- Committed config uses placeholder-relative urls:

```yaml
name: core-desk
projects:
  alpha:
    path: projects/alpha
    url: fixtures/alpha.git
    branch: main
  beta:
    path: projects/beta
    url: fixtures/beta.git
    branch: main
```

Core/git clone accepts path URLs the same way `git clone fixtures/alpha.git` does when cwd is Workspace root; materialize runs with paths relative to Workspace root. Prefer passing url as-is to `git clone` with clone target absolute.

- Example tree is **not** required to be its own git repo in the product monorepo; harness `git init`s the temp copy when testing pin/gitignore.
- **No progens/actions/generators** in this example (core-only dogfood).

### README dogfood steps

1. From repo root, copy or `cd examples/core-desk` (after `cargo build -p odm`).
2. `git init` (if exercising pin/gitignore).
3. `odm --root . sync` (or `odm sync` from that dir once discovery works).
4. `odm pin status` / `odm status` / `odm doctor`.
5. Optional: `odm project list`, `odm pin apply`.

### Fixture bootstrap script

`examples/core-desk/fixtures/README.md` documents rebuilding bare repos with a short shell snippet (git init --bare, clone temp, commit, push). Committed bare repos must be enough for offline `cargo test` without network.

## Comments

Parent map: [[issues-14-implement-core-map]]

Recommended decision locked for agent implement 2026-08-01.

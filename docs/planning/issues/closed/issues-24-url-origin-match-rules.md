---
id: issues-24
title: "URL origin match rules"
description: "Lock how config url is compared to git remote origin for materialize/already-cloned."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
  - ready-for-agent
---

# URL origin match rules

## Question

How does ODM normalize and compare a managed entry’s config `url` to `remote.origin.url` so “already materialized” vs “origin mismatch → fail” is deterministic across SSH/HTTPS/`file://` and trailing `.git` variants?

## Blocked by

- [[issues-16-odm-git-shell-contract]]

## Answer

**Recommended lock** (pure functions in `odm-core`, e.g. `url_match.rs`):

### `normalize_git_url(raw: &str) -> String`

1. Trim whitespace.
2. If SCP-like `user@host:path` (no `://`): rewrite to `ssh://user@host/path` (path may omit leading `/` — use `ssh://user@host/path` with single slash after host when path has no leading slash: `git@github.com:org/repo` → `ssh://git@github.com/org/repo`).
3. Parse as URL-ish: split scheme, authority, path (lightweight; no heavy dependency required).
4. Lowercase **scheme** and **host** only; leave path case as-is (except Windows `file://` drive letter — lowercase drive).
5. Strip default ports (`:443` for https, `:80` for http, `:22` for ssh).
6. Strip trailing `/` from path repeatedly.
7. Strip trailing `.git` from path (once, case-sensitive `.git`).
8. Strip empty userinfo only if entirely empty; **keep** explicit usernames (`https://user@host/...` stays distinct from no-user).
9. `file://` and bare filesystem paths: canonicalize when the path exists on disk; if not, normalize lexical `//` and `/./`; compare in canonical form when both sides canonicalize.
10. Relative path urls (core-desk fixtures): normalize by stripping trailing `.git` and `./` prefix; comparison of config url vs origin should also try resolving relative config url against Workspace root before compare when origin is absolute `file://`.

### `urls_match(a, b) -> bool`

`normalize_git_url(a) == normalize_git_url(b)` after optional workspace-root resolution for relative sides.

### Non-goals (explicit)

- **Do not** treat `https://host/org/repo` and `git@host:org/repo` as equal (different transports = mismatch).
- **Do not** rewrite remotes on mismatch — fail materialize.
- Missing origin (`OriginMissing`) → mismatch / fail (not “match”).

### Pin file urls

Store config url **as written** in pin entries; when comparing pin url to config url, use same `urls_match`.

## Comments

Parent map: [[issues-14-implement-core-map]]

Recommended decision locked for agent implement 2026-08-01.

Landed 2026-08-01 in `crates/odm-core/src/url_match.rs` + unit tests.

---
id: issues-16
title: "odm-git shell contract"
description: "Lock odm-git public API: git invocations, cwd rules, and error classification for core."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# odm-git shell contract

## Question

What public API does `odm-git` expose for core — which git subcommands/args, working-directory rules, stdout parsing (rev-parse, status, origin URL), and how are failures classified for upper layers — given shell-out-only and no libgit2?

## Blocked by

_(none — frontier)_

## Answer

Locked in crate `crates/odm-git` (`Git` + `CommandRunner` + `GitError`).

- **Shape:** `Git` with injectable `CommandRunner` (default `ProcessRunner`); program `"git"` on PATH; `with_program` for tests. No libgit2.
- **Ops:** `is_repo`, `init`, `clone(url, path, branch?)`, `fetch`, `head_sha`, `is_clean`, `origin_url`, `checkout_detached`, `run` (passthrough argv).
- **Paths:** absolute required (`NotAbsolute`); library ops use `git -C <path>` (clone/init take path as git arg). No `mkdir -p` in crate.
- **Semantics:** full history clone (`-b` if branch); fetch default remote; clean = empty `status --porcelain=v1 -uall`; HEAD = full 40-char lowercase hex SHA; `is_repo` → bool (missing/non-repo = false); `checkout_detached` does not gate on dirty (core policy); empty `run` args → `EmptyArgs`.
- **I/O:** library ops capture stdout/stderr; `Failed` attaches trimmed stderr (stdout fallback). `run` inherits stdio. No TTY progress day one.
- **Errors:** `NotAbsolute`, `GitNotFound`, `NotARepo`, `OriginMissing`, `Failed{operation,path,code,stderr}`, `Parse{…}`, `EmptyArgs`. Origin-match and dirty/force stay in **core**. Exit-code map → [[issues-18-cli-error-and-exit-code-spine]]; URL normalize → [[issues-24-url-origin-match-rules]].

## Comments

Parent map: [[issues-14-implement-core-map]]

Grilled with maintainer; code + integration tests landed 2026-08-01.

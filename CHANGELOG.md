# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **`find --progen-group` CLI integration tests** — multi-progen fixture: group narrows hits, unknown group exit `1`, `--progen` ∪ `--progen-group`, JSON `progen` field (`crates/odm/tests/cli_progen_group.rs`).
- **odm-progen unit edges** — direct tests for `format_find_human` / `format_context_human`, `doctor_progens` (missing vault / index present), vault walk nested + dot-dir skip, note title/wikilink edges.
- **odm-git worktree real-git test** — tempfile round-trip (`worktree_add` / `worktree_list` / `worktree_remove`) against real git at the crate seam; mock argv tests unchanged.
- **Local coverage script** — `./scripts/coverage.sh` runs `cargo llvm-cov --workspace` (HTML + lcov under `target/coverage/`); install hint if missing; not wired to CI.
- **Website Playwright** — local e2e harness under `website/` (`npm run test:e2e`; chromium).
- **Website Playwright smoke suite** — `website/e2e/smoke.spec.ts` covers all `website/*.html` pages (load, nav, install/quickstart/concepts/features content, CSS asset, mobile viewport).
- **Website a11y + link crawl** — `@axe-core/playwright` scans (home/install/guide-workspace) plus internal link/anchor crawl; site UX polish (start-here path, contrast, focus-visible, guide meta/crumbs).
- **`odm generate`** — v1 local template materialize: list Generators from bundles; `generate <name> --dest <path> [--force]` copies a local template tree under the Workspace (remote/url-only run deferred with a clear error).
- **`odm generate --dry-run`** — no-write preview: same validation as a real run; reports file count that would be copied; human `would generate …`; JSON includes `dry_run: true` (real runs emit `dry_run: false`).

- **`odm project worktree`** — Worktree slot add/list/rm and `--wt` path binding (no longer a not-implemented stub).
- **`odm agent pack`** — v1 local install/link/list/rm into an agent home (`--home`); Workspace registry `.odm/agent-packs.json`; `rm` drops registry entry and best-effort deletes dest (missing dest still OK; unknown name → exit `4`); `agent start` remains a not-implemented stub.
- **`odm agent prompt`** — v1 thin context work-package: packages one note’s Progen neighborhood to stdout (same path/JSON as `odm context`); `agent start` still stubbed.
- **`odm doctor` worktree orphan warn** — Warn checks `worktree_orphan:<project>:<slot>` for configured-project dirs under `worktrees/` that are not registered git worktrees (`fixable: false`; `--fix` does not delete).
- **`odm doctor` worktree dirty-slot warn** — Warn checks `worktree_dirty:<project>:<slot>` for registered worktree slots with a dirty working tree (`fixable: false`; `--fix` does not clean or stash).
- **`odm doctor` pack missing-path warn** — Warn checks `pack_missing:<name>` when a registered agent pack path has no path/symlink on disk (`fixable: false`; `--fix` does not edit registry or delete pack paths).
- **`odm project worktree prune`** — remove orphan slot dirs under `worktrees/<project>/` (same orphan definition as doctor). Default deletes empty orphans only (exit `3` if any non-empty orphan remains after partial cleanup); `--force` recursive-deletes orphans. Never deletes registered worktrees. Doctor `--fix` still does not delete orphans.
- **`odm project worktree prune --all`** — prune orphans across every configured Project (same empty/`--force` rules); skips missing primary / non-git projects; exit `3` on any skipped non-empty without `--force`.
- **`odm find --limit`** — max hits per Progen store (default **200**); `0` is rejected with usage exit `1`.
- **`odm status` worktree slots** — each project includes registered `worktree_slots` (`name` + `path` + `dirty`); human output lists slot names when non-empty (dirty slots get a ` dirty` suffix).
- **`odm status` / `project info` worktree orphans** — each project includes `worktree_orphans` (`name` + `path`; same orphan definition as doctor/prune); human `orphans: …` when non-empty; empty array when none / soft-fail. Observation only — doctor warn + `worktree prune` remain cleanup.
- **`odm status` agent packs** — top-level `agent_packs` (`name` + `source` + `path` + `mode` + `missing`) from the Workspace registry; human **Agent packs:** section when non-empty (` missing` suffix when path absent). Empty array when none / missing registry / soft-fail. Doctor still owns `pack_missing` warn.
- **`odm agent pack list` / entry `missing`** — list JSON packs and install/link/rm `--json` entry objects include `missing` (same path/symlink rule as status `agent_packs` / doctor `pack_missing`; dangling symlink not missing); human list appends ` missing` when dest absent.
- **`odm project info` worktree slots** — registered `worktree_slots` (`name` + `path` + `dirty`), same shape as status; empty array when none / non-git / soft-fail; human `worktrees: …` when non-empty.
- **Worktree slot dirty observation** — `worktree list`, `status`, and `project info` probe registered slot cleanliness via `git status` (`dirty`: `true` / `false` / `null` on probe error). Human list marks dirty slots with a ` dirty` suffix.

### Fixed

- **`odm run --json` stdout/stderr** — envelope always includes captured `stdout` and `stderr` strings (concatenated across tasks in order) so agents can debug failures without a second non-JSON run; capture still keeps process stdout clean JSON.
- **Entity name uniqueness / path safety** — Project and Progen names must be unique across both maps and path-safe tokens (no `/` `\` `.` `..`); enforced on config load and membership add.
- **`odm run` missing cwd paths** — known project path or `--wt` slot missing on disk now exits `4` (`not_found`); unknown project names still exit `1` (`usage`).
- **Docs honesty** — install leads with build-from-source (Releases = when published); `AGENTS.md` allows Pages-only Actions; `progen.md` / `cli.md` federation = `find` only; README docs links; guide-actions HTML paren.
- **Progen wikilinks in code** — fenced ```/~~~ blocks and inline `` `code` `` no longer contribute graph edges; invalid YAML frontmatter hard-fails reindex with the note path.
- **Clap usage exit codes** — parse failures (unknown command / bad flags) exit `1` (not clap’s default `2`), matching library usage errors; with `--json` on argv, stdout is the standard `{ ok: false, error: { code: "usage", … } }` envelope.
- **`odm-git` non-interactive lifecycle** — captured git ops set `GIT_TERMINAL_PROMPT=0` so clone/fetch fail fast instead of hanging on auth prompts; `Git::run` passthrough stays user-facing.
- **Progen index freshness** — `ensure_index` rebuilds when vault note paths/mtimes change (meta `vault_fp`); edits and deletes show up on next open/find without manual `reindex`.
- **Progen duplicate note ids** — reindex fails with both `rel_path`s named instead of an opaque UNIQUE constraint error.
- **`project add` / `progen add` path escape** — membership paths validated with `resolve_under_root` before mutate/save so `../` escapes never brick the Workspace config.
- **Action/generator bundle paths** — absolute and `..` escapes rejected via `resolve_under_root` (workspace error); relative bundles under the Workspace root still load.
- **Action task `dir`** — load rejects absolute/`..` escape via `resolve_under_root` (workspace error); runtime `resolve_cwd` uses the same policy (usage error) so cwd cannot leave the Workspace.
- **`odm find` FTS queries** — plain-text terms are quoted for FTS5 so `AND`/`OR`/punctuation no longer cause syntax errors; multi-word is AND of terms.
- **`odm find` snippets** — no longer panic on multi-byte UTF-8 bodies (CJK/emoji); window start/end floored/ceiled to char boundaries.

### Changed

- `examples/core-desk` includes a sample Generator (`hello` → `templates/hello`) and a tiny demo agent pack (`agent-packs/demo`).
- `examples/core-desk` assets expand: second Progen `ops` + `all-docs` group, vault note ids (`readme` / `ops-note`), project-scoped `in-alpha` actions, gitignore for dogfood debris.
- `examples/core-desk` README + `core_desk_pack_list_missing_gate` dogfood pack list `missing` (install → dest delete → rm).
- Reference docs: `generate`, `agent pack`, and `agent prompt` documented as landed v1 (local / thin); remote/marketplace and `agent start` still deferred in `env-gen-packs.md`.

## [0.1.0] - 2026-08-01

First public spine release: core multi-git Workspace, Progen integration, and Actions behind one `odm` binary.

### Added

- **Core**
  - `odm init` — bootstrap a Workspace (`.odm/`, config, optional git)
  - `odm sync` / `odm pin` / `odm status` / `odm doctor` — multi-git lifecycle on plain clones (no submodules)
  - `odm project` lifecycle — add, list, rm, info, git
  - Workspace discovery, pin file, gitignore manage
- **Progen**
  - In-repo Obsidian-compatible vault engine under `odm progen …` (façade swap-ready for upstream crates later)
  - Lifecycle: add/list/rm/info; store: get/body/ls/tree/backlinks/reindex/doctor
  - Top-level `odm find` and `odm context` with multi-Progen scope (`--progen`, `--progen-group`)
- **Actions**
  - `odm run` — list and dispatch Action bundles (`tasks: [{ run, dir? }]`) from Workspace config
  - Shell-out model; cwd via task dir / `--project` / `--wt`; exit-code passthrough
- **CLI**
  - Globals: `--root`, `--json`, `--project`, `--wt`, `--progen`, `--progen-group`
  - Sketch stubs at release: `generate`, `agent`, `project worktree` (exit 1 not-implemented) — see [Unreleased] for generate + worktree landing after 0.1.0
- **Dogfood**
  - `examples/core-desk` — offline Workspace exercising core, path-only Progen, groups, and shell Actions
- **Ship**
  - `scripts/release-build.sh` — release tarball under `dist/odm-<version>-<target>.tar.gz`
  - Consumer install docs (README, `docs/reference/install.md`)

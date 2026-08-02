# todo Workspace — ODM surface review

Review of ODM against `examples/todo`: a real-network poly-repo desk using public GitHub repos under `jared-hembrow/*`. No commits or pushes were made to those remotes.

- **Binary:** `odm 0.1.0` (`target/debug/odm`)
- **Date:** 2026-08-02
- **Dogfood:** `scripts/dogfood.sh` → **OK** (temp copy, full tour)
- **Probe:** `scripts/probe.sh` → **90 PASS / 0 FAIL** (log: `out/probe-log.txt`, gitignored)
- **Remotes:** tip-top, cheat-key, portfolio, rss-td-game, cheat-key-sheets — all left clean and unpushed

---

## Workspace shape

- **4 Projects** (git-managed, real HTTPS remotes)
  - tip-top, cheat-key, portfolio (`branch: main`)
  - rss-td-game (`branch: master` — default branch is not `main`)
- **2 Progens**
  - `desk` — path-only local vault (`progens/desk`, Todo*Token notes + wikilinks)
  - `sheets` — git-managed clone of cheat-key-sheets
- **Groups:** `default` → desk; `all-docs` → desk+sheets; `personal` → desk
- **Actions / generators / agent-pack** fixtures under `actions/`, `generators/`, `templates/`, `agent-packs/`

---

## Surface matrix (probed)

### Bootstrap / discovery

- **`odm init` on existing Workspace** — refuses (exit 2). Good.
- **`odm init --interactive`** — not implemented (exit 1). Documented sketch.
- **Unknown command** — exit 1; with `--json` yields error envelope (`ok: false` / `error`).
- **`--root` required** for nested example (no upward walk) — correct per architecture.

### `sync`

- Full sync clones all five managed URLs (4 projects + sheets progen), writes `.odm/odm.lock.yaml`.
- Named sync (`odm sync tip-top`, `odm sync sheets`) works; unknown name → exit 1.
- Semantics are clone-if-needed + **fetch only** (no checkout rewrite on present trees).
- Wrong default branch fails hard (rss-td-game initially configured as `main` → git exit 128). Config must match remote default (`master` here).

### `pin`

- After first sync, lock pins all managed entities at cloned HEADs; `pin status` → `in_sync`.
- **`pin apply`** checks out each pin as **detached HEAD** (by design). Trees stay clean.
- Re-attach is a local `git checkout <branch>` — not an ODM verb.
- Unknown pin name → exit **4** (`not_found`), not usage 1.
- Pin file records `rev`, `url`, `branch` per entity; auto-maintained on successful managed ops.

### `status`

- Human + `--json` list projects and progens with managed/path, git, pin, dirty.
- JSON includes `worktree_slots`, `worktree_orphans`, top-level `agent_packs`.
- **Finding — path-only progen nested in a git monorepo:** `desk` reports `is_git: true`, `managed: false`, `dirty: true` because git discovery walks to the ancestor odm repo. Misleading for “path-only vault” UX when the example lives inside another git tree. Standalone Workspace (`git init` at todo root in dogfood) has the same shape: desk is inside the Workspace git root.
- Managed clones report clean when porcelain is empty.

### `doctor`

- Passes config/path/origin/pin checks against real remotes (`origin_match` green).
- Warns `odm_layout` until cache/log/progen dirs exist; **`doctor --fix`** creates them.
- Warns `gitignore_drift` until managed marker block matches desired; **`--fix`** rewrites `# >>> ODM managed` … `# <<< ODM managed`.
- Hand-written ignores outside the markers are preserved; extras like `out/` and lock file should stay outside the block.
- Orphan worktree dirs → warn (not fixable via `--fix`); dirty registered slots → warn.
- Missing agent-pack dest → `pack_missing:<name>` warn.
- Does **not** run store-content doctor (that is `odm progen doctor`).

### `project`

- **list / info** — config + disk/git/pin; info includes type, head, origin, worktree_slots/orphans.
- **git** — requires `--` before git args; read-only probes (`rev-parse`, `log`, `branch -a`, `remote -v`, `status`) work on real clones.
- **`--wt`** on `project git` resolves to `worktrees/<project>/<slot>/`; missing slot → exit 4.
- **add / rm** — roundtrip with local bare fixture: default rm keeps tree; `--delete --force` removes clean tree.
- **worktree add** works from Primary even after pin-detach if branch create is allowed; creates local branch only (never pushed).
- **worktree list** JSON includes `dirty` per slot.
- **worktree prune** removes empty orphan dirs; registered slots untouched; **prune --all** multi-project.
- **worktree rm --force** removes slot.
- **Finding — actions in worktrees:** git worktrees use a **`.git` file**, not directory. Actions that `test -d .git` fail in slots; use `test -e .git` or `git rev-parse`. Documented in `actions/todo.yaml`.

### `progen`

- Lifecycle list/info/add/rm mirror projects; path-only add keeps vault on rm by default.
- **reindex** — desk 4 notes / 5 links; sheets 4 notes / 0 links (real cheatsheet md with frontmatter ids).
- **get / body / ls / tree / backlinks** — single-root; multi-progen without `--progen` → exit 1.
- Wikilinks resolve by note id (`[[desk-readme]]`, `[[projects-map]]`); backlinks to `projects-map` include welcome + desk-readme.
- Missing id → exit 4.
- **progen doctor** — vault_path ok for desk + sheets.
- sheets clone is a usable real-world progen (Neovim/Tmux/macOS/AI cheatsheets searchable after reindex).

### `find`

- Federated FTS across progens; `--progen` / `--progen-group` narrow.
- Empty query lists scoped notes (desk + sheets).
- Tokens like `TodoWelcomeToken` hit; bare CamelCase prefix `TodoWelcome` did **not** hit (tokenizer/literal token behavior — multi-word is AND of whitespace tokens, not substring).
- `--limit` is per-store; `0` rejected (exit 1); unknown progen exit 1; zero hits exit 0.
- Group `all-docs` unions desk + sheets; `default` / `personal` scope to desk.

### `context` / `agent prompt`

- Same neighborhood shape: anchor + outgoing + incoming (one hop, no cross-store walk).
- `name:id` form works (`desk:welcome`); conflict with `--progen` → exit 1.
- Multi-progen bare id → exit 1 (must disambiguate).
- `--json` keys: `anchor`, `incoming`, `outgoing`, `progen`.
- `agent prompt` is a thin alias of context (same exits/shapes).

### `run`

- List human + JSON; unknown action exit 1.
- Exit code passthrough (`fail` → 7).
- Multi-task chain concatenates stdout.
- `--project` sets cwd to Primary; `--wt` sets cwd to slot (missing → 4).
- `--json` captures stdout/stderr into envelope with `exitCode`.
- Task `dir:` relative to Workspace still works (`in-tip-top-dir`, `read-only-log`).
- Unknown `--project` → exit 1.

### `generate`

- List includes local `note` and url-only `remote-deferred`.
- Dry-run validates, writes nothing; JSON includes `dry_run`.
- Real run copies template tree; non-empty dest without `--force` → exit 3; `--force` overwrites.
- Url-only generator → exit 1 (“remote generators deferred”).
- Dest escaping Workspace → exit **2** (workspace/path error), not usage 1.
- No variable substitution (v1 local copy only).

### `agent pack`

- install (copy) / link (symlink) / list / rm against `--home` outside or under Workspace.
- Registry at `.odm/agent-packs.json`; status inventories packs.
- Exists without `--force` → exit 3; missing source → 4; unknown rm → 4.
- Removing dest out from under registry → list `missing` + doctor `pack_missing`.
- rm still succeeds for stale registry cleanup.

### `agent start`

- v1 one-shot: `odm --project <name> [--wt <slot>] agent start -- <program> [args…]`.
- Cwd = Project Primary or worktree slot; human inherit; exit = child exit (`true` → 0, `false` → non-zero).
- Independent of packs/prompt; no runtime matrix.

---

## Real-repo specifics

- **Network required** for first sync; dogfood curls GitHub before cloning.
- **Branch names matter** — portfolio/tip-top/cheat-key/sheets use `main`; rss-td-game uses `master`.
- **Public only** in this example (reproducible without private auth). Private repos from the same account work the same way when credentials are available; not committed here.
- **Read-only discipline** held: probe/dogfood never `commit`/`push`/`reset --hard` on remotes; final porcelain clean; no commits ahead of origin.
- Pin apply detaches HEAD locally — fine for reproducibility; day-to-day coding wants branch checkout again.

---

## Comparison to `core-desk`

| | core-desk | todo |
|--|-----------|------|
| Network | offline bare fixtures | real GitHub HTTPS |
| Projects | alpha, beta | 4 real apps |
| Progens | 2 path-only | 1 path-only + 1 git-managed real vault |
| Intent | CI-safe full tour | human dogfood + surface review |
| Script | `dogfood.sh` only | `dogfood.sh` + exhaustive `probe.sh` |

---

## Issues / product notes (from probing)

1. ~~**Path-only progen `is_git` / dirty inside a git Workspace or monorepo**~~ — **fixed**: `Git::is_repo_root` + observation/rm-delete use own `.git` only.
2. **Worktree + `.git` file** — document for Action authors; core-desk `in-alpha` uses `test -f README.md` (safer). todo actions use `test -e .git`.
3. **Exit code taxonomy quirks** (consistent once known): unknown pin → 4; path escape → 2; unknown entity name on many cmds → 1; missing worktree slot → 4.
4. ~~**FTS is token-literal**~~ — **docs fixed** in `cli.md` (whole FTS5 token; not substring/prefix). Matcher unchanged (intentional).
5. **`gitignore` dual maintenance** — hand rules + ODM markers; run `doctor --fix` once after adding managed entities so drift clears.
6. ~~**`pin apply` UX**~~ — **fixed**: human/JSON call out detached HEAD; docs note `in_sync` ≠ on a branch. Semantics unchanged.
7. **Global `--wt` clap quirk** — help admits execution uses argv scan (`resolve_wt_from_env`); works in practice for `run` and `project git` when a single value is passed.
8. **Sheets without wikilinks** — reindex/list/find still useful; backlinks empty — fine for cheatsheet corpora.

---

## Verdict

ODM’s shipped spine holds up against real multi-repo GitHub materialize + pin + worktree + dual Progen (local vault + remote docs) + actions/generate/packs. Dogfood and probe are green. Main footguns for real use: correct `branch` in config, read-only hygiene on shared remotes, worktree `.git` file semantics, and status “dirty” on path-only progens living inside another git tree.

**Recommended loop for this desk:**

```bash
cargo build -p odm
ODM=target/debug/odm examples/todo/scripts/dogfood.sh   # clean temp proof
# interactive:
cd examples/todo && odm --root . sync && odm --root . doctor --fix \
  && odm --root . progen reindex && odm --root . find TodoWelcomeToken
```

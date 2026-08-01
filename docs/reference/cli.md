# CLI surface v1

Command tree and global flags for the `odm` binary. Domain terms: root `CONTEXT.md`. Discovery and `.odm/`: `architecture.md`. Config: `config.md`. Git lifecycle: `multi-git.md`. Progen scope: `progen.md`. Upstream store verbs: `research/progenitor-surface.md`.

Depth markers:

- **Full** — behavior locked here (enough to implement without reopening).
- **Sketch** — name reserved + one-liner; depth in `worktrees.md`, `graph.md`, `env-gen-packs.md`.

## Binary and non-goals

- Single binary: `odm`. Store engine integrated as crates; ODM owns UX.
- Façade name is **`odm progen`** — never “brain”.
- **Out of scope:** `odm serve`, MCP, deep daemon design.
- Legacy Go top-level action names and `--root-path` are **not** preserved.

## Global flags

Available on commands that need them (parser-global; unknown entity names → hard error).

- **`--root <path>`**: Workspace root. Must contain `.odm/odm.config.yaml`. No upward walk. Discovery without flag: `architecture.md`.
- **`--json`**: Machine-readable stdout; stable per-command shapes. Human diagnostics on stderr.
- **`--project <name>`**: Target **Project** by config name (not a path).
- **`--progen <name>`**: Include/select **Progen** by config name; **repeatable**; union with other scope flags (`progen.md`).
- **`--progen-group <name>`**: Include members of a **Progen group**; **repeatable**; union (`progen.md`).
- **`--wt <slot>`**: **Worktree slot** name → tree at `worktrees/<project>/<slot>/`. Requires a Project context (`--project` or a `project` subcommand). **Sketch** behavior beyond path binding — see worktrees.md.

Rules:

- `--project`, `--progen`, `--progen-group`, `--wt` take **names only**, not filesystem paths.
- Write/mutate store commands need exactly one Progen (`--progen` or sole configured Progen) — `progen.md`.

## Exit codes

- **`0`**: success (including `find` with zero hits → empty list).
- **`1`**: usage / unknown command / bad flags / unknown entity name.
- **`2`**: Workspace/config error (not a Workspace, invalid config, missing bundle path).
- **`3`**: operation failed (git/pin/store error; see `run` for action passthrough).
- **`4`**: not found (e.g. unknown node id when that is distinct from usage error).

`odm run`: if the Action is found and executed, process exit code is the **action’s exit code**. Pre-exec failures use `1` / `2`. `--json` wrapper may still include `"exitCode"`.

## Command tree

```text
odm init
odm sync | pin | status | doctor
odm project …
odm progen …
odm find | context
odm run
odm generate …          # sketch
odm agent …             # sketch
```

---

### `odm init` — **full**

```text
odm init [<path>] [--interactive|-i] [--no-git]
```

- **Default path**: cwd.
- **Headless (default)**: create `.odm/odm.config.yaml` (minimal; optional `name` later via flags if added), ensure `.odm/` layout as needed. Empty `projects` / `progens` maps are valid.
- **`--interactive` / `-i`**: prompt for name, git y/n, optional first Project/Progen.
- **Git**: `git init` at Workspace root **by default**; **`--no-git`** skips.
- **Gitignore**: when git repo and `manage_gitignore` default true, seed ignore rules (`multi-git.md`, `architecture.md`).
- **Pin file**: not created until first managed materialize.
- **Already a Workspace** (config present): **refuse** (no silent overwrite).
- **`--json`**: `{ "root", "git": bool }` (and stable fields as implemented).

Bootstrap is the discovery exception: may run with no existing Workspace (`architecture.md`).

---

### `odm sync` — **full**

```text
odm sync [name…]
```

- No names → all **managed** entries (Project + Progen with `url`), depth-ordered, fail-fast (`multi-git.md`).
- With names → those entities only (must exist in config).
- Semantics: materialize if needed, then **fetch only** — never checkout/reset/merge as part of sync.
- Pin auto-maintain after successful ops when pin file present (`multi-git.md`).

---

### `odm pin` — **full**

```text
odm pin apply [name…]
odm pin status [name…]
```

- **`apply`**: checkout each selected pin’s `rev` as **detached HEAD**. Dirty tree → fail unless force flag (exact force flag name: `--force`). Missing path: named apply → fail; all-apply → fail-fast (`multi-git.md`).
- **`status`**: pin file vs current HEAD for managed entries (present / drift / missing path).
- No names → all pinned managed entries.

---

### `odm status` — **full**

```text
odm status
```

Workspace snapshot: configured Projects/Progens, on-disk presence, git/pin drift summary, dirty hints. Does not fetch. `--json` for agents.

---

### `odm doctor` — **full**

```text
odm doctor [--fix]
```

**ODM-side** checks only: config load, declared paths, gitignore management drift, pin file consistency basics. Not store-content doctor (`odm progen doctor`).

- **`--fix`**: mechanical ODM repairs only (same spirit as upstream progen doctor) — no destructive git rewrites.

---

### `odm project` — **full** (worktree **sketch**)

```text
odm project list
odm project add <name> --path <rel> [--url <url>] [--branch <b>] [--type <t>] [--no-clone]
odm project rm <name> [--delete] [--force]
odm project info <name>
odm project git <name> -- <git-args…> [--wt <slot>]
odm project worktree …          # sketch
```

- **`list` / `info`**: config + disk/git summary; `--json`.
- **`add`**: write Project entry; if `url` set, materialize unless `--no-clone` (`multi-git.md`).
- **`rm`**: un-declare; tree **kept** by default; `--delete` removes tree if clean; dirty → fail unless `--force`.
- **`git`**: run `git -C <primary-or-wt> <git-args…>` (argv after `--`, not a shell string). Exit code = git’s. On success, if pin file exists and **HEAD changed**, **auto-maintain** that entity’s pin (not “sync”). `--wt` selects slot working tree.
- **`worktree`**: sketch — list/add/rm (etc.) one-liners only; full behavior in `worktrees.md`.

No `project sync` — use top-level `odm sync [name]`.

---

### `odm progen` — **full** (lifecycle + store façade)

#### Lifecycle (ODM-owned)

```text
odm progen list
odm progen add <name> --path <rel> [--url <url>] [--branch <b>] [--no-clone]
odm progen rm <name> [--delete] [--force]
odm progen info <name>
```

Same add/rm/materialize semantics as Project (`multi-git.md`). Entity summary verb is **`info`** (not `get`).

#### Store façade

ODM resolves Progen **name → path** via Workspace config, then runs single-root progen crate ops. Scope flags: `progen.md`.

Federated defaults live at **top level** (`find`, `context`) — not under `odm progen`.

Re-exported / mapped store commands (hot set):

```text
odm progen get <id>
odm progen body <id>
odm progen ls | tree
odm progen refs | backlinks …
odm progen task|issue|idea|decision|doc …
odm progen archive | rm …
odm progen log | glossary | plan …
odm progen reindex | doctor | watch …
```

- Node **`get`** keeps upstream meaning (id → node). Entity summary is **`info`**.
- Writes: exactly one Progen (`--progen` or sole configured).
- Reads under `odm progen`: single-root (pass `--progen` when multiple configured), except where a command doc explicitly federates.
- **`scan`**: deferred / sketch later.
- **`serve`**: absent (non-goal).
- **`prompt`**: primary home is `odm agent prompt` (sketch); not duplicated as a second full surface under `progen` in this doc.

Flag parity with upstream: pass through where it does not fight ODM globals; do not require duplicating every upstream flag table here — implement against progenitor surface + ODM scope rules.

---

### `odm find` — **full**

```text
odm find [query] [facet-flags…] [--progen …] [--progen-group …] [--json]
```

- Federated **query** (FTS + facets): fan-out progen `query` per selected store; merge per `progen.md` (tag `"progen"`, identity `(progen, id)`, stable Progen order).
- Default scope: all configured Progens.
- Zero hits → exit `0`, empty list.
- Empty `progens` map → error (no progen scope) — exit `2` or `1` as fits “no scope configured”.
- Not named `query` on the ODM binary.

---

### `odm context` — **full**

```text
odm context <id> [--progen <name>] [upstream scope flags…] [--json]
odm context <progen-name>:<id> …
```

- In-store neighborhood only — **no** cross-store graph walk.
- Disambiguation: require `--progen` when more than one Progen is in play, **or** accept `name:id` prefix. Sole configured Progen → bare `id` OK.
- Thin-pass upstream scope flags (`--depth`, etc.) where applicable.

---

### `odm run` — **full**

```text
odm run                          # list Actions (human or --json)
odm run <action-name> [--project <name>] [--wt <slot>] [--json] [--] [extra-args…]
```

- Resolve `<action-name>` from merged Action bundles (`config.md`); unknown → exit `1`.
- **Cwd**: Action `dir` if set; else Workspace root; `--project` / `--wt` override to that working tree when set.
- Extra args: after `--` as defined by the Action.
- Actions are **only** invoked via `run` — never installed as top-level commands.
- Action names may coincide with builtin verbs (`run sync` is fine); no reserved ban list beyond empty/invalid tokens.
- Exit codes: see above (passthrough when executed).
- `--json` wrapper minimum: `{ "action", "exitCode" }`. With `--json`, task stdio is captured (not interleaved on stdout) so the envelope is a single well-formed JSON object. Without `--json`, task stdio inherits the terminal.

---

### `odm generate` — **sketch**

```text
odm generate <generator-name> [generator-flags…]
```

Resolve Generator by name from bundles; template/package behavior deferred (`config.md`, `env-gen-packs.md`).

---

### `odm agent` — **sketch**

All AI/agent-facing UX lives here (not a top-level `pack` command).

```text
odm agent pack install|link|list …
odm agent start [--project] [--wt] …
odm agent prompt <id> --progen …     # thin wrap of progen prompt when specified
```

- Honors `--project`, `--wt`, `--progen`, `--json` where relevant.
- Full start/pack/prompt flows: `worktrees.md`, `env-gen-packs.md`.
- Not MCP/`serve`.

---

## Full vs sketch matrix

- **Full**: global flags (with `--wt` reserved); `init`; `sync`; `pin apply|status`; `status`; `doctor`; `project list|add|rm|info|git`; `progen` lifecycle + store façade mapping; `find`; `context`; `run`.
- **Sketch**: `project worktree …`; `generate …`; `agent …`; `--wt` deep behavior.
- **Absent**: `serve`, MCP, top-level action verbs, `ops` namespace, path-valued scope flags.

## Related

- Workspace discovery / `.odm/`: `architecture.md`
- Config entities and bundles: `config.md`
- Clone / sync / pin semantics: `multi-git.md`
- Federation and scope: `progen.md`
- Upstream command inventory: `research/progenitor-surface.md`
- Sketch depth: `worktrees.md`, `graph.md`, `env-gen-packs.md`

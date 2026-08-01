# CLI surface v1

Command tree and global flags for the `odm` binary. Domain terms: root `CONTEXT.md`. Discovery and `.odm/`: `architecture.md`. Config: `config.md`. Git lifecycle: `multi-git.md`. Progen scope: `progen.md`. Upstream store verbs: `research/progenitor-surface.md`.

Depth markers:

- **Full** — behavior locked here (enough to implement without reopening).
- **Sketch** — name reserved + one-liner; depth in `graph.md`, `env-gen-packs.md` (worktree v1 is full; deferred items still in `worktrees.md`).

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
- **`--wt <slot>`**: **Worktree slot** name → tree at `worktrees/<project>/<slot>/`. Requires a Project context (`--project` or a `project` subcommand). Path binding implemented for `project git` and `run`; missing slot → exit `4` (no auto-create). See `worktrees.md`.

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
odm generate …          # v1 local template
odm agent pack …        # v1 local install/link/list/rm
odm agent prompt …      # v1 thin (context work-package)
odm agent start         # sketch
```

---

### `odm init` — **full**

```text
odm init [<path>] [--interactive|-i] [--no-git]
```

- **Default path**: cwd.
- **Headless (default)**: create `.odm/odm.config.yaml` (minimal; optional `name` later via flags if added), ensure `.odm/` layout as needed. Empty `projects` / `progens` maps are valid.
- **`--interactive` / `-i`**: **not implemented** (flag reserved; exits `1` with not-implemented until shipped). Headless init above remains full.
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

Workspace snapshot: configured Projects/Progens, on-disk presence, git/pin drift summary, dirty hints, **registered** worktree slots and **orphan** slot dirs per Project, and **registered** agent packs. Does not fetch. `--json` for agents:

- Each project includes `worktree_slots: [ { "name", "path", "dirty" } ]` (`path` is `worktrees/<project>/<slot>`; `dirty` is `true` / `false` / `null` when the cleanliness probe fails; empty array when none / non-git / list soft-fails). Progens omit `worktree_slots`.
- Each project also includes `worktree_orphans: [ { "name", "path" } ]` (same orphan definition as doctor/prune; sorted by name; empty array when none / missing dir / soft-fail). Observation only — no `dirty` on orphans. Progens omit `worktree_orphans`.
- Top-level `agent_packs: [ { "name", "source", "path", "mode", "missing" } ]` from `.odm/agent-packs.json` (always present; empty array when none / missing registry / list soft-fails). `missing` is `true` when the pack path has no path/symlink entry (same rule as doctor `pack_missing`); doctor still owns the warn check.

Human output lists slot names only when non-empty (dirty slots get a ` dirty` suffix, e.g. `feat dirty`) and orphan names when non-empty (`orphans: a, b`). When packs are registered, an **Agent packs:** section lists each pack name, mode (`install` / `link`), and a ` missing` suffix when `missing`. Doctor warn + `worktree prune` remain cleanup — see `worktrees.md`.

---

### `odm doctor` — **full**

```text
odm doctor [--fix]
```

**ODM-side** checks only: config load, declared paths, gitignore management drift, pin file consistency basics, worktree slot orphan **warns** (configured Project dirs under `worktrees/<project>/` that are not registered git worktrees; not fixable), dirty registered worktree slot **warns** (`worktree_dirty:<project>:<slot>`; not fixable), and agent pack missing-path **warns** (`pack_missing:<name>` when a registry entry’s path has no path/symlink on disk; not fixable; dangling symlink present is not missing). Not store-content doctor (`odm progen doctor`).

- **`--fix`**: mechanical ODM repairs only (same spirit as upstream progen doctor) — no destructive git rewrites; does not delete orphan worktree dirs, clean/stash dirty slots, or edit/remove agent pack registry entries or destinations.

---

### `odm project` — **full** (worktree **v1**)

```text
odm project list
odm project add <name> --path <rel> [--url <url>] [--branch <b>] [--type <t>] [--no-clone]
odm project rm <name> [--delete] [--force]
odm project info <name>
odm project git <name> [--wt <slot>] -- <git-args…>
odm project worktree list <project>
odm project worktree add <project> <slot> [--branch <b>]
odm project worktree rm <project> <slot> [--force]
odm project worktree prune <project> [--force]
odm project worktree prune --all [--force]
```

- **`list` / `info`**: config + disk/git summary; `--json`. `info` includes registered `worktree_slots: [ { "name", "path", "dirty" } ]` (`path` is `worktrees/<project>/<slot>`; `dirty` is `true` / `false` / `null` on probe failure; empty array when none / non-git / list soft-fails) and `worktree_orphans: [ { "name", "path" } ]` (always present; empty when none / non-git / soft-fail; same orphan definition as doctor/prune). Human lists slot names when non-empty (`worktrees: a, b dirty`) and orphan names when non-empty (`orphans: a, b`).
- **`add`**: write Project entry; if `url` set, materialize unless `--no-clone` (`multi-git.md`).
- **`rm`**: un-declare; tree **kept** by default; `--delete` removes tree if clean; dirty → fail unless `--force`. Does **not** delete `worktrees/<project>/`.
- **`git`**: run `git -C <primary-or-wt> <git-args…>` (argv after `--`, not a shell string). Exit code = git’s. On success, if pin file exists and **HEAD changed** on **Primary**, **auto-maintain** that entity’s pin (not “sync”). `--wt` selects slot working tree (must exist; no auto-create; pin maintain stays Primary-only).
- **`worktree`**: **v1 implemented** — `list` / `add` / `rm` / `prune` / `prune --all` for slots at `worktrees/<project>/<slot>/`. `list` JSON slots include `dirty` (`true` / `false` / `null`). Primary must be a git repo. Details and deferred items: `worktrees.md`.

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

**Implemented** store façade (today):

```text
odm progen get <id>
odm progen body <id>
odm progen ls | tree
odm progen backlinks …
odm progen reindex | doctor
```

Federated query/context live at **top level** (`odm find`, `odm context`) — not under `odm progen`.

**Reserved / deferred** (not in CLI today; do not treat as shipped):

- `refs`, typed node verbs (`task` | `issue` | `idea` | `decision` | `doc`), `archive`, `log`, `glossary`, `plan`, `watch`, `scan`
- Store-side mutate beyond the current façade

- Node **`get`** keeps upstream meaning (id → node). Entity summary is **`info`**.
- Writes: exactly one Progen (`--progen` or sole configured).
- Reads under `odm progen`: single-root (pass `--progen` when multiple configured), except where a command doc explicitly federates.
- **`serve`**: absent (non-goal).
- **`prompt`**: primary home is `odm agent prompt` (v1 thin context work-package); not duplicated as a second full surface under `progen` in this doc.

Flag parity with upstream: pass through where it does not fight ODM globals; do not require duplicating every upstream flag table here — implement against progenitor surface + ODM scope rules.

---

### `odm find` — **full**

```text
odm find [query] [--limit <n>] [--progen …] [--progen-group …] [--json]
```

- Federated **FTS** query: fan-out per selected store; merge per `progen.md` (tag `"progen"`, identity `(progen, id)`, stable Progen order). No facet flags on the ODM CLI.
- Default scope: all configured Progens (`--progen` / `--progen-group` narrow).
- Empty query → list scoped notes (same path as a free-text query).
- `--limit` max hits **per store**, default **200**; `0` rejected (usage exit `1`).
- Zero hits → exit `0`, empty list.
- Empty `progens` map → error (no progen scope) — exit `2` or `1` as fits “no scope configured”.
- Not named `query` on the ODM binary.

---

### `odm context` — **full**

```text
odm context <id> [--progen <name>] [--json]
odm context <progen-name>:<id> …
```

- In-store neighborhood only — **no** cross-store graph walk.
- Fixed one-hop neighborhood as `ContextHit` (`anchor` / `outgoing` / `incoming`). No `--depth` or other upstream scope flags.
- Disambiguation: require `--progen` when more than one Progen is in play, **or** accept `name:id` prefix. Sole configured Progen → bare `id` OK. At most one `--progen` (or use `name:id`).

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

### `odm generate` — **v1 local template**

```text
odm generate                                    # list Generators
odm generate <name> --dest <rel-path> [--force] [--dry-run]
```

- Resolve `<name>` from merged Generator bundles (`config.md`); unknown → exit `1`.
- **List** (no name): human one name per line (sorted); empty → `(no generators)`. `--json`: `{ "generators": [ { "name", "template", "url" } ] }` (missing fields `null`).
- **Run**: materialize a **local** `template` directory (path relative to Workspace root) into `--dest` (also relative to root; must not escape). Recursive copy; no variable substitution.
- **`--dest`**: required when a name is given; creates parent dirs as needed. Destination is the root of the copied tree.
- **`--force`**: required when dest exists and is non-empty; overwrites files in place (does not delete unrelated extras). Without force → exit `3` (operation).
- **`--dry-run`**: same validation as a real run (template resolve, dest relative/no escape, dest not a file, non-empty dest requires `--force`) but **never** creates dirs or copies files (including under `--force`). `copied` = files that **would** be written.
- **Url-only** generators (no usable `template`): list shows them; run → exit `1` with message that remote generators are deferred (with or without `--dry-run`). If both `template` and `url` are set, prefer `template`.
- **`--json` run**: `{ "generator", "dest", "copied", "dry_run" }` (`copied` = files written or that would be written; `dry_run` is `true` for `--dry-run`, `false` for a real run).
- Human success: real run `generated <name> -> <dest> (<n> files)`; dry-run `would generate <name> -> <dest> (<n> files)`.
- Deferred: remote fetch/cache, `template.toml` / prompts / vars, Nx/schematics — `env-gen-packs.md` (`--dry-run` landed).


---

### `odm agent pack` — **v1 local install/link/list/rm**

All AI/agent-facing UX lives under `odm agent` (not a top-level `pack` command). Pack v1 is local filesystem only.

```text
odm agent pack list
odm agent pack install <source> --home <path> [--force]
odm agent pack link <source> --home <path> [--force]
odm agent pack rm <name>
```

- **Workspace required** (same discovery as generate).
- **Source:** local directory path. Relative paths resolve under Workspace root (must not escape). Absolute paths allowed. Pack **name** = directory basename. No remote/marketplace; no pack manifest required in v1.
- **`--home`:** required agent-native root (may be outside Workspace). Pack materializes at `<home>/<name>/`.
- **`install`:** recursive copy of source into `<home>/<name>/`. Dest exists without `--force` → exit `3`. With `--force`, replace then copy. Missing source → exit `4`.
- **`link`:** symlink `<home>/<name>` → absolute resolved source. Same exists/`--force` policy. Platforms without symlink support → clear operation error (no silent copy fallback).
- **`list`:** registry-backed (`.odm/agent-packs.json`). Human: one name per line sorted; empty → `(no agent packs)`; ` missing` suffix when dest has no path/symlink entry. `--json`: `{ "packs": [ { "name", "source", "path", "mode", "missing" } ] }` (`mode` = `"install"` | `"link"`). `missing` is `true` when the pack path has no path/symlink entry (same rule as status `agent_packs` / doctor `pack_missing`); dangling symlink present is not missing. Doctor still owns the warn check.
- **`rm`:** drop registry entry and best-effort delete destination (install tree or link symlink). Missing dest still succeeds (stale-registry cleanup). Unknown name → exit `4`. Human: `removed <name> -> <path>`. `--json`: single entry object (same fields as list items, including `missing`) for the removed pack.
- **install/link/rm `--json`:** single entry object (same fields as list items, including `missing`), not wrapped.
- Human success: `installed <name> -> <path>` / `linked <name> -> <path>` / `removed <name> -> <path>`.
- Deferred: pack manifest, marketplace, config-declared packs (`env-gen-packs.md`); status pack inventory and pack list/entry `missing` observation landed (`agent_packs` / `agent pack list`; doctor `pack_missing` warn separate — see doctor); `agent start` runtime.

### `odm agent prompt` — **v1 thin** (context work-package)

```text
odm agent prompt <id> [--progen <name>] [--json]
odm agent prompt <progen-name>:<id> …
```

- Thin alias of `odm context`: same Progen scope rules, same human markdown neighborhood, same `--json` shape (`ContextHit` with `anchor` / `outgoing` / `incoming`).
- Packages one note’s in-store context to stdout for agents — **not** a second prompt engine, task planner, or graph walk.
- Disambiguation: require `--progen` (global) when more than one Progen is in play, **or** accept `name:id`. Sole configured Progen → bare `id` OK. At most one `--progen`.
- Unknown id → exit `4`. Success → exit `0`.
- Details / deferred depth: `env-gen-packs.md`.

### `odm agent start` — **sketch**

```text
odm agent start [--project] [--wt] …
```

- Still **not implemented** (exit `1`).
- Intent: shell-out to an agent runtime against a Project Primary or `--wt` slot — `worktrees.md`, `env-gen-packs.md`. Not MCP/`serve`.

---

## Full vs sketch matrix

- **Full**: global flags (including `--wt` path binding); `init` (headless; `--interactive` not implemented); `sync`; `pin apply|status`; `status`; `doctor` (includes pack_missing warn); `project list|add|rm|info|git|worktree` (v1); `progen` lifecycle + **partial** store façade (implemented verbs above); `find`; `context`; `run`; `generate` (v1 local template + `--dry-run`); `agent pack` (v1 local install/link/list/rm); `agent prompt` (v1 thin context work-package).
- **Sketch / deferred**: `init --interactive`; reserved progen store verbs; `agent start`; deferred worktree features (config slots, pin↔slot, auto-prune on doctor, branch templates, global `--wt` depth — `worktrees.md`; doctor orphan/dirty **warn**, explicit `worktree prune` / `prune --all`, registered slot `dirty` on list/status/info, and status/info `worktree_orphans` observation landed); generate remote/templating (`--dry-run` landed); pack marketplace/manifest/config declarations (`env-gen-packs.md`; status `agent_packs` inventory, pack list/entry `missing`, and doctor `pack_missing` warn landed).

- **Absent**: `serve`, MCP, top-level action verbs, `ops` namespace, path-valued scope flags.

## Related

- Workspace discovery / `.odm/`: `architecture.md`
- Config entities and bundles: `config.md`
- Clone / sync / pin semantics: `multi-git.md`
- Federation and scope: `progen.md`
- Upstream command inventory: `research/progenitor-surface.md`
- Worktrees (v1 + deferred): `worktrees.md`
- Sketch depth: `graph.md`, `env-gen-packs.md`

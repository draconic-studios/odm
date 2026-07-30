---
title: "Progenitor surface (crates, store, CLI, APIs)"
date: 2026-07-31
sources:
  - /Users/jaredhembrow/Projects/draconic/progenitor
status: complete
---

# Progenitor surface — facts for ODM integrate vs façade

Primary sources only: progenitor workspace source and first-party docs under
`/Users/jaredhembrow/Projects/draconic/progenitor` (checked 2026-07-31).

**Product one-liner:** `progen` is a project-memory tool. Durable memory is plain
Markdown on disk; a SQLite/FTS index is a disposable cache rebuildable anytime.
Canonical store name in docs is `progem/`; CLI default root is `./memory`.
([README.md](file:///Users/jaredhembrow/Projects/draconic/progenitor/README.md);
`apps/cli/src/dispatch.rs`)

---

## 1. Cargo workspace / crates

Workspace members (`Cargo.toml`):

| Crate path | Package name | Role (from crate docs / Cargo.toml) |
|---|---|---|
| `packages/progem-data` | `progem-data` | Pure typed model: `Node` enum, Task/Issue/Idea/Doc/Decision, refs, placement fields, `ModelError`. No other progen deps. |
| `packages/progen-parser` | `progen-parser` | Markdown text↔model: document split, YAML (de)serialize, body `[[links]]`, changelog day-file parse. |
| `packages/progen-core` | `progen-core` | Placement, config registry, edges derivation, glossary, idgen, fsio, lock; re-exports model+parser. |
| `packages/progen-index` | `progen-index` | Disposable SQLite index (open/reindex/ensure_fresh + query read API). |
| `packages/progen-ops` | `progen-ops` | Verbs: set/capture/mutate, get, query/ls/tree/context/refs/backlinks, prompt, plan, log, doctor, scan, watch daemon. |
| `packages/progen-serve` | `progen-serve` | Loopback WebSocket JSON-RPC daemon + in-process watch. |
| `apps/cli` | `progen-cli` (bin `progen`) | Clap CLI + dispatch assembling the libs. |

Not in workspace today:

- **`packages/progen-client`** — removed (changelog 2026-06-30; ADR-16 / plan-8).
  README still lists it; that line is stale.
- **`apps/desktop`** — Electron app; not a Cargo member. Talks only to `progen serve`.
  ([Cargo.toml](file:///Users/jaredhembrow/Projects/draconic/progenitor/Cargo.toml);
  [README.md](file:///Users/jaredhembrow/Projects/draconic/progenitor/README.md);
  [adr16](file:///Users/jaredhembrow/Projects/draconic/progenitor/docs/reference/system/decisions/adr/adr16-d3-desktop-ui-is-an-electron-app-over-the-progen-s.md))

**Dependency stack (bottom → top):**

```
progem-data
    ↑
progen-parser
    ↑
progen-core
    ↑
progen-index
    ↑
progen-ops
    ↑
progen-serve / progen-cli
```

(`progen-core/src/lib.rs`, package Cargo.tomls)

---

## 2. Store format

### 2.1 Memory root

- One directory = one store (“memory root”).
- Docs call it `progem/`; engine code and CLI default use path `memory`
  (`PROGEN_MEMORY_DIR` or `--root` override).
  ([README.md](file:///Users/jaredhembrow/Projects/draconic/progenitor/README.md);
  `dispatch.rs` `resolve_memory_root`; tests in `apps/cli/tests/root_flag.rs`)
- Progenitor’s own vault is `docs/` (opened as Obsidian + engine store).
  (`docs/config.toml`, tree under `docs/planning|reference|knowledge|logs|archive`)

Root-level artifacts:

| Path | Purpose |
|---|---|
| `config.toml` | Kind registry overrides (required-sections, etc.) |
| `glossary.json` | Canonical glossary (not a `.md` node file) |
| `.index/index.db` | Disposable SQLite index (gitignored) |
| `.lock` | Directory lock during writes |
| `planning/`, `reference/`, `knowledge/`, `logs/`, `devops/`, `archive/` | Node tree |

(`progen-core` config/glossary/lock; `progen-index` freshness; memory-tree doc)

### 2.2 Directory conventions (“location encodes meaning”)

Authoritative placement model:
`docs/reference/system/architecture/architecture-progem-memory-tree.md`
and `progen-core/src/placement.rs`.

Shape:

```text
[archive/]<area>/<domain>/<category?>/<filename>
```

Areas (from memory-tree + placement):

- `planning/<domain>/{tasks,issues,ideas,plans}/`
- `reference/<domain>/{overview,architecture,decisions/adr,decisions/rfc,non-functional,api,api/schema,code,style,standards,specs,reporting,marketing,guides}/`
- `knowledge/<domain>/`
- `logs/<YYYY>/<MM>/YYYY-MM-DD-changelog.md` (engine-managed; not a placeable `Node`)
- `devops/<domain>/`
- `archive/` mirrors full tree

**Rules (`placement.rs`):**

- **area + category** derived from node **kind** via `Registry::kind_placement` (not free-authored path).
- **domain** authored, required for domain-bearing areas; open vocabulary.
- **filename** per `FilenameRule` (e.g. `tasks142-oauth-refresh.md`, `idea-3-….md`, `spec-….md`, fixed `glossary.md`, legacy `id-slug`).
- `archived: true` → path under `archive/…` (soft delete).

### 2.3 File shape

Every node file = YAML frontmatter + opaque Markdown body.
Body is stored verbatim; edges also come from inline `[[…]]` body links.
(`progem-data/src/node.rs`; `progen-parser` bodylinks)

### 2.4 Node types and frontmatter

Closed `NodeType` set (`progem-data/src/node.rs`):

| Type | Id shape | Status enum | Notable fields |
|---|---|---|---|
| `task` | `t<N>` | `hold \| ready \| active \| complete` (default `ready`) | `slug`, `tags?`, `complexity?` (1–10), `acceptance?`, `constraints?`, `refs?`, placement |
| `issue` | `i<N>` | `open \| reviewing \| promoted \| closed \| wontfix` | `slug`, `issue-type?` (`bug\|feature-request\|observation`), `severity?` (`critical\|high\|medium\|low`), `tags?`, `refs?`, placement |
| `idea` | `idea-<N>` | `draft \| reviewing \| promoted \| closed` | `slug`, `tags?`, `refs?`, placement |
| `decision` | `d<N>` | — | `task` (originating task id), `slug?`, `refs?`, placement |
| `doc` | `<kind>-<N>` | — | `kind` (open string), `title`, `tags?`, `source-path?`, `media-type?`, `generated`, `source-hash?`, `refs?`, placement |

Shared placement (flattened top-level): `area?`, `domain?`, `archived` (bool, omit if false).

Timestamps: canonical `created_at` / `updated_at` (`NaiveDate`); legacy `created`/`updated` accepted on read.

**Typed refs** (`refs:` list), ADR 0002 closed vocab:

- Kinds: `references`, `supersedes`, `superseded-by`, `derives-from`, `defines`, `relates`, `depends-on`, `implements`, `source-file`
- Target types: `node`, `file`, `url`, `gh-issue`, `gh-pr`, `commit`, `symbol`
- Optional `locator`: `line`, `span: [start,end]`, `anchor`, `symbol`

### 2.5 Unified set payload (create/update)

`progen-ops/src/set.rs` / CLI `progen <type> set [<id>] '<json>'`:

- No id → create; id → update.
- Payload = authored frontmatter only; body is separate (`progen body`).
- Engine keys rejected (`id`, `type`, `created`/`updated` family).
- Unknown keys rejected.
- List fields: default replace; `--append` / `--remove`.
- Ref sugar keys (`derives-from`, `implements`, …) and/or full `refs` array.
- Create requires `title` (tasks etc.) and domain via placement for domain-bearing kinds.

### 2.6 Glossary

- Canonical: `<root>/glossary.json` (`progen-core/src/glossary.rs`).
- Terms: `{ id: g-<slug>, term, definition, links[] }`.
- Reindex compiles `links` → `references` edges.
- `glossary export` derives Markdown on demand; JSON stays source of truth.

### 2.7 Changelog

- Day files: `logs/<YYYY>/<MM>/<YYYY-MM-DD>-changelog.md`.
- `progen log "<title>" …` appends; bare `log` reads faceted history.
- Task author/commit attribution derived live from changelog entries that `[[…]]`-reference the task.
  ([README.md](file:///Users/jaredhembrow/Projects/draconic/progenitor/README.md); `progen-ops/src/log.rs`)

---

## 3. Index (SQLite + FTS5)

Implementation: `packages/progen-index`.

| Fact | Source |
|---|---|
| Path | `<memory_root>/.index/index.db` |
| Nature | Disposable cache; files authoritative |
| Full rebuild | `reindex` DROPs tables and rebuilds from disk |
| Lazy freshness | `ensure_fresh`: fingerprint path+size+mtime (ns); rebuild if stale or schema mismatch |
| Schema version | `"6"` in `meta` |
| Tables | `nodes`, `edges`, FTS5 `nodes_fts` (id UNINDEXED, slug, body), `meta` |
| Node columns | id, type, slug, status, created_at, updated_at, sha, repo, author, kind, complexity, body, path, archived, action_types, area, domain, issue_type, severity |
| Edge columns | src, dst, kind, dst_type, dst_ref, locator, provenance (`frontmatter`\|`body`\|`both`) |
| Walk rules | All `.md` under root; skip dotdirs/dotfiles; `archive/` included (flags archived); `glossary.json` ingested separately |
| Corrupt files | Skipped with warning; do not abort reindex |
| Public API | `open`, `reindex`, `ensure_fresh`, `index_db_path`, `get_node(s)`, `list_nodes*`, `search`/`search_with_snippets`, `refs_of`, `backlinks_of`, filters, counts |

Watch path: `progen watch` / in-serve daemon (`progen-ops/src/daemon.rs`) — notify reindex + periodic `ensure_fresh`.

---

## 4. CLI commands

Binary: `progen` (`apps/cli`, package `progen-cli`).
Global: `--root <path>` (beats `$PROGEN_MEMORY_DIR` > `./memory`).

| Command | Role |
|---|---|
| `task set [<id>] '<json>'` | Create/update task |
| `task prompt <id>` | Compile agent work-package markdown to stdout |
| `issue set` / `issue promote <id>` | Issue CRUD; promote → task with `derives-from` |
| `idea set` / `idea promote <id>` | Idea CRUD; promote → issue |
| `decision set` | Decision CRUD |
| `doc set` | Doc CRUD (scaffold body from kind required-sections on create) |
| `plan apply <file>` | Materialize versioned plan JSON into tasks |
| `glossary add\|edit\|export` | Glossary JSON ops |
| `get <id>` | Frontmatter+body (`--body-only` / `--json`) |
| `body <id>` | Replace body only (`--body-file` / `--body` / stdin) |
| `archive` / `archive --restore` | Soft delete / restore |
| `rm --hard` | Hard delete (refuses without `--hard`) |
| `query [text]` | FTS + facets (`--type/--status/--area/--domain/--scope/--issue-type/--severity/--min-complexity/--include-archived/--group-by/--limit/--with-body/--format`) |
| `ls` / `tree` | Facet listing; tree groups by area→domain |
| `refs` / `backlinks` | Outgoing / incoming typed edges |
| `context <id>` | Bounded neighborhood (`--scope/--depth/--direction/--include-archived`) |
| `log` | Append changelog entry or faceted history read |
| `scan --domain` | Generate `code` reference nodes from Rust public API (`--watch`) |
| `reindex` | Full index rebuild |
| `watch` | Index daemon (`--once`, `--for-secs`, `--reconcile-secs`) |
| `serve` | Loopback WS RPC daemon (`--port`, `--token-file`, `--for-secs`) |
| `doctor` | Health report; `--fix` mechanical repairs only |

(`apps/cli/src/cli.rs`, README Commands section)

---

## 5. Multi-root / multi-store

**What exists:**

- Single memory root per process/invocation.
- Selection: `--root` > `$PROGEN_MEMORY_DIR` > `./memory`.
- `progen serve` / RPC `info` reports one `root`.
- Tests prove flag vs env precedence (`apps/cli/tests/root_flag.rs`).

**What does not exist (gaps):**

- No multi-store registry, federation, or cross-root query.
- No simultaneous open of multiple roots in one CLI process or one serve session.
- No built-in discovery of “nearest progem/ up the tree” (default is cwd-relative `./memory` only).
- ODM wanting multi-project memory must **orchestrate multiple roots** (multiple serve instances, multiple `--root` calls, or a façade) — not a progenitor API today.

---

## 6. Task / context / prompt / query APIs

Library entry points ODM can call **in-process** (Rust) or via **CLI / serve RPC**:

### 6.1 Set / mutate / capture (`progen-ops`)

- `set::set(root, today, node_type, id?, json, ListMode)` — unified CRUD
- `capture::{task_add, issue_add, idea_add, decision_add, doc_add, issue_promote, idea_promote, locate, …}`
- `mutate` — body / archive / rm / patch+move
- `plan` — plan apply
- `get::run_get`
- `prompt::run_prompt(root, task_id) -> markdown`

### 6.2 Query / graph (`progen-ops::query`)

- `run_query(root, text, Format, &QueryOptions)`
- `run_ls(root, Format, &QueryOptions, tree: bool)`
- `run_refs` / `run_backlinks`
- `run_context(root, id, Format, &ContextOptions)`  
  ContextOptions: `scope`, `include_archived`, `depth`, `direction` (`Out|In|Both`)  
  Anchor always included; scope/archive act as walls; cycles bounded.

### 6.3 Task prompt contents (`prompt.rs`)

Deterministic markdown package:

1. Header: id, slug, status, optional complexity  
2. Task body  
3. Acceptance criteria  
4. Constraints  
5. Context/references — node refs inlined (title+body); external/unresolved listed without body  

No model/network calls.

### 6.4 Serve JSON-RPC methods (`progen-serve/src/rpc/mod.rs`)

```
info, query, ls, tree, get, body, set_body, set,
archive, rm, refs, backlinks, context, log, glossary,
reindex, doctor, promote
```

- Wire: `{"id", "method", "params"}` → `{"id","result"}` or `{"id","error":{"message"}}`
- Events: `{"event":"store-changed","ids":[…]|null}`
- Auth: loopback bind + per-session token query param; non-loopback Origin rejected
- Ready line on stdout: `{"event":"listening","port", "token"}`
- Handlers call **same** ops functions as CLI (no shell-out)
- **Not exposed over RPC (CLI-only today):** `task prompt`, `plan apply`, `scan`, `watch` (watch runs inside serve for index warmth)

---

## 7. Config registry

`<root>/config.toml` optional; missing/malformed → built-in defaults
(`progen-core/src/config.rs`).

- `[kinds.<name>]` + `required-sections` (soft/advisory for doctor)
- Areas/scopes/placement maps for query + kind→path derivation
- Doc `kind` is open; unknown kinds legal

Example (progenitor docs store): registers extra kinds `architecture`, `rfc`,
`overview`, `research`, `report`, `non-functional`.

---

## 8. ODM: call vs reimplement

### Prefer **calling** progenitor (do not reimplement)

| Capability | How |
|---|---|
| Node model, frontmatter parse/serialize, placement, id allocation | `progem-data` + `progen-parser` + `progen-core` |
| CRUD, promote, archive, body | `progen-ops` set/mutate/capture **or** CLI **or** serve `set`/`set_body`/`archive`/`rm`/`promote` |
| Query, FTS, ls/tree, refs/backlinks, context | `progen-ops::query` / serve methods / CLI |
| Index lifecycle | `progen-index` / `reindex` / `watch` / serve’s embedded watch |
| Task agent package | `progen-ops::prompt::run_prompt` or `progen task prompt` (not on serve yet — thin façade if needed) |
| Doctor / heal placement | `progen doctor` / ops doctor |
| Changelog + glossary | ops log + core glossary |
| Desktop-style live UI | `progen serve` WebSocket protocol (Electron already does this) |

### Façade / ODM-owned (progenitor does not provide)

| Gap | Implication |
|---|---|
| Multi-root / multi-project | ODM maps projects → roots; spawn N CLIs or N serves |
| Root discovery (`progem` vs `memory` name) | ODM config; note default `./memory` vs docs `progem/` naming split |
| `progen-client` crate | **Gone**; use serve RPC or link ops crates or shell CLI |
| Non-Rust consumers | CLI JSON or serve WS only first-class surfaces |
| Semantic / vector search | Idea only (`idea-5`), not shipped |
| Issue tracker foreign to progem (GitHub sync etc.) | Deferred/out of core |
| ODM-specific triage labels / wayfinder maps / agent skills | Outside progen model — store as docs/tasks with tags or separate layer |
| Serve missing `prompt` / `plan apply` / `scan` | Façade can shell those CLI verbs or add RPC later upstream |

### Integrate-as-store pattern (factual fit)

ODM can treat a directory as a progem root:

1. Point `--root` / `PROGEN_MEMORY_DIR` / serve `Config.root` at it.
2. Use progen placement + set contract so files stay doctor-clean.
3. Rely on index for query/context; never treat SQLite as SoT.
4. For agents: `task prompt` + `context` + `query --format json`.

Reimplementing parser/placement/index would fork the contract (frontmatter field
names, path projection, edge provenance) and break Obsidian/doctor compatibility.

---

## 9. Stale docs to discount

- README still lists `progen-client` and “placeholder” parser/data crates — both
  crates are real and layered; client is deleted (ADR-16 / 2026-06-30 changelog).
- Some comments still say `memory/tasks/tN.md` flat paths; live placement is
  `planning/<domain>/tasks/tasksN-slug.md` (sample: `docs/planning/tasks/tasks/tasks1-….md` with `id: t1`).

---

## 10. Source index (primary)

| Topic | Path |
|---|---|
| Workspace members | `Cargo.toml` |
| Product overview / CLI list | `README.md` |
| Memory tree | `docs/reference/system/architecture/architecture-progem-memory-tree.md` |
| Node model | `packages/progem-data/src/node.rs` |
| Placement | `packages/progen-core/src/placement.rs` |
| Config registry | `packages/progen-core/src/config.rs` |
| Index schema/API | `packages/progen-index/src/lib.rs` |
| Ops surface | `packages/progen-ops/src/lib.rs` |
| Set contract | `packages/progen-ops/src/set.rs` |
| Context | `packages/progen-ops/src/query.rs` (`run_context`) |
| Task prompt | `packages/progen-ops/src/prompt.rs` |
| CLI defs | `apps/cli/src/cli.rs` |
| Root resolution | `apps/cli/src/dispatch.rs` |
| Serve RPC | `packages/progen-serve/src/{lib,rpc/mod,rpc/handlers}.rs` |
| Client removal / Electron | `docs/reference/system/decisions/adr/adr16-….md`, plan-8 |
| Sample task | `docs/planning/tasks/tasks/tasks1-implement-type-and-severity-fields-on-issue-nodes.md` |

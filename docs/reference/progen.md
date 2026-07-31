# Progen and federation

How ODM treats **Progen** stores and multi-store query scope. Domain terms: root `CONTEXT.md`. Config wiring: `docs/reference/config.md`. Upstream facts: `docs/reference/research/progenitor-surface.md`.

## What a Progen is (ODM view)

A **Progen** is a named docs/memory store: Markdown on disk plus a disposable index. ODM declares each store under `progens` in Workspace config (`path`, optional `url`). The store is not owned by the ODM state directory.

Often the store is its own git repo. Path may nest under a Project without making it a Project.

## Ownership split

| Layer | Owns |
|-------|------|
| **Progen (upstream crates / single root)** | Node model, placement, index, in-store query/context, in-store graph, doctor, single-root CLI/serve semantics |
| **ODM** | Registry of named roots, **Progen groups**, scope flags, fan-out reads, result merge, clone/sync of progen paths when `url` is set, `odm progen …` UX façade |

ODM v1 does **not** require upstream multi-root APIs. Federation is orchestration over repeated single-root operations.

## One store, one graph

Inside a single Progen:

- Wikilinks / backlinks / graph edges resolve **only within that store** (progenitor single-root model).
- There is **no** cross-store `[[wikilink]]` and no ODM-invented federated id graph in v1.

### External references (explicit)

A note **may** point outside its store using normal Markdown links, not wikilinks:

```markdown
[title text](https://example.com/doc)
[title text](../other-place/file.md)
[title text](file:///absolute/or/repo/path)
```

These are **external links**: explicit URL or file path syntax so the store engine treats them as non-graph, non-backlink edges. They do not create in-store backlinks and do not participate in federated graph merge.

## Scope model

### Default (reads)

Federating read commands (`context`, `find` / query, `ls`-style listing across memory, etc.) with **no** scope flags use:

**all Progens declared in Workspace config**

- Not cwd-inferred “nearest store.”
- Not a hidden default group.
- If `progens` is empty, there is no progen scope (command-specific error or empty result — defined per CLI command later).

### Narrowing (reads)

| Flag | Meaning |
|------|---------|
| `--progen <name>` | Include this Progen (repeatable) |
| `--progen-group <name>` | Include all members of this **Progen group** (repeatable) |

- Names must exist in Workspace config; unknown → **hard error**.
- Combined flags → **union** of all named members (deduped).
- v1 accepts **config names only**, not raw filesystem paths on these flags.

### Writes / mutates

Any command that creates or changes store content requires **exactly one** Progen:

- Pass `--progen <name>`, or
- If the Workspace has **exactly one** configured Progen, that name is implied.

Never write to a group or to “all.” Multi-progen workspace without `--progen` on a mutating command → **error**.

## Progen groups

**Progen groups** exist only in Workspace config (`progen_groups`). They are named lists of Progen names for query scope. They are not stores, not directories, and not written into any Progen.

Typical use: unrelated products stay in separate Progens; within one product, impl vs marketing vs support may be separate stores combined only via a group at query time.

No `default_progen_group` (or equivalent) in v1 — default remains all Progens.

## Federated read merge

When ODM fans out a read across N roots:

1. Run the single-root operation on each selected Progen path.
2. Tag every hit with the **Progen name**.
3. Treat identity as **`(progen, id)`** — ids are not globally unique across stores; collisions are allowed and must not be merged into one node.
4. Human output: prefix or group by Progen name.
5. `--json`: each record includes `"progen": "<name>"` (and local `id`).
6. Ordering: within each store by that store’s relevance/date rules; across stores, stable order of Progen names (config order, or CLI list order when the user narrowed scope).

No cross-store rank fusion beyond simple concatenation/grouping in v1 unless a command doc specifies otherwise.

## CLI façade

- Store-oriented commands live under **`odm progen …`** (never “brain”).
- Global scope flags (`--progen`, `--progen-group`) apply on ODM commands that federate; exact flag surface is locked in `docs/reference/cli.md`.
- ODM resolves name → path via Workspace config, then calls into progen crates (in-process) or equivalent single-root operations.

## Explicit non-goals (v1)

- Upstream multi-root federation inside progen crates (may be revisited later)
- Cross-store wikilinks or unified global id space
- Path-based `--progen` overrides bypassing config
- Default scope group field in config
- Serve/MCP multi-root daemon design (out of map scope)

## Related

- Config entries: `progens`, `progen_groups` — `docs/reference/config.md`
- Glossary — `CONTEXT.md`
- Research baseline — `docs/reference/research/progenitor-surface.md`

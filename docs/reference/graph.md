# Code↔doc index (sketch)

**Sketch** — not a Ship gate. Depth bar: intent, placement/ownership, CLI names reserved, explicit deferred. Informal name only — **no** `CONTEXT.md` term until productized. Domain terms: root `CONTEXT.md`. In-store graph: `progen.md`. Federation: `progen.md`.

## Intent

Optional **workspace code↔doc index**: tie Project code (symbols/paths) to Progen nodes (and simple tags) so agents can navigate the desk when this feature is built.

This is **not** a second knowledge-graph product and **not** a merge of Progen store graphs.

## Placement and ownership

- **Progen** keeps the **in-store** graph (wikilinks, backlinks, neighborhood). `odm context` stays in-store only — no cross-store walk (`progen.md`, `cli.md`).
- **ODM** would own any workspace-level code↔doc index material (placement under `.odm/cache/` or similar when implemented).
- **No** federated id graph and **no** cross-store `[[wikilink]]` in v1.

## CLI names reserved

- **None** in this design package. No `odm graph …` command tree.
- Future hooks may appear via existing surfaces (`doctor` / `status` mentions) only if an implement slice adds them — not required here.

## Deferred

- Ingest engine choice (tree-sitter vs graphify vs other)
- Tag taxonomy and query UX
- Incremental indexing, invalidation, and storage schema
- Any serve-time or MCP graph API
- Glossary promotion to `CONTEXT.md`

## Non-goals

- Cross-store graph merge or federated wikilinks
- Replacing Progen in-store graph/query
- `odm serve` / MCP / long-running graph daemon
- Blocking Implement core or Ship on this sketch

## Related

- Progen federation and in-store rules: `progen.md`
- CLI `find` / `context`: `cli.md`
- Architecture caches: `architecture.md`
- Phased delivery: `phased-delivery.md`

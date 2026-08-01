# Phased delivery

How ODM moves from design docs to a shipped binary. Domain terms: root `CONTEXT.md`. Product framing: `vision.md`. System shape: `architecture.md`.

## Greenfield / legacy

This product is a **Rust-first greenfield**, not a port of the former Go CLI.

- **Product home** — this repository (`hembrow-innovations/odm`) is the permanent Rust monorepo home for design and implementation.
- **Go** — removed from the tree; recoverable only as history (git tag `legacy-go-archive`). Treat as optional inspiration, not a checklist or compatibility baseline.
- **Research** — `research/legacy-go-odm.md` records what Go did (replace / drop / map hints). Implementers are not required to match it.
- **No legacy config path** — v1 does not read root `odm.config.yaml` / `.json` or submodule-era layouts. Consumers start with `odm init` and Workspace config under `.odm/`.
- **No `migration.md`** — discontinuity is intentional; this file plus README/research carry the story.

## Boundary: design map vs implement maps

| Phase band | Allowed | Forbidden |
|------------|---------|-----------|
| **Design package** (this wayfinder map) | `CONTEXT.md`, `docs/reference/*`, ADRs, planning issues | Cargo workspace, product Rust crates, shipping a binary, treating spikes as the product |
| **Implement maps** (later) | Code, tests, dogfood builds, Releases when Ship criteria met | Reopening design fundamentals without an explicit decision change |

Closing the design map means the docs package is review-complete. It does **not** start implementation in the same map. Implementation is one or more **later wayfinder maps** sliced along the spine below (at least **Implement core** first).

## Phase spine

Post-0.1.0 **landed outside the original Ship spine** (not required before v0.1.0, now available):

- **Worktree slots v1** — `odm project worktree` add/list/rm/**prune** and `--wt` path binding (`worktrees.md`; deferred items still listed there)
- **Generate local template v1** — `odm generate` list + materialize from local bundles, including `--dry-run` no-write preview (`env-gen-packs.md`; remote/`template.toml` depth deferred)

- **Agent pack local v1** — `odm agent pack` install/link/list/rm with list/entry `missing` observation (`env-gen-packs.md` / `cli.md`; marketplace/manifest deferred)
- **Agent `prompt` v1 thin** — context work-package (`env-gen-packs.md`; `agent start` still sketch)
- **Doctor worktree orphan warn** — configured-project slot dirs that are not registered git worktrees (`worktrees.md`; not fixable)
- **Doctor worktree dirty-slot warn** — registered dirty slots `worktree_dirty:<project>:<slot>` (`worktrees.md`; not fixable)
- **Doctor pack missing-path warn** — registry packs whose path is absent on disk `pack_missing:<name>` (`env-gen-packs.md` / `cli.md`; not fixable; `--fix` does not edit registry)
- **Status `agent_packs`** — top-level registry inventory on `odm status` (`name` + `source` + `path` + `mode` + `missing`); empty array when none / soft-fail; doctor still owns `pack_missing` warn (`env-gen-packs.md` / `cli.md`)
- **`odm find --limit`** — max hits per Progen store (default 200)
- **Status + project info `worktree_slots`** — registered slots (`name` + `path` + `dirty`) on `odm status` projects and `odm project info` (same shape on `worktree list`); empty when none / non-git / soft-fail
- **Status + project info `worktree_orphans`** — orphan slot dirs (`name` + `path`) on `odm status` projects and `odm project info` (same definition as doctor/prune; observation only; empty when none / soft-fail); doctor warn + prune remain cleanup (`worktrees.md`)
- **`odm project worktree prune --all`** — multi-project orphan GC across every configured Project (`worktrees.md`; same empty/`--force` rules as per-project prune)

Still deferred / sketch (not Ship gates unless pulled in deliberately):

- Graph/tags, env productization
- Generate remote and full `template.toml` depth
- Agent `start` (prompt is v1 thin — see Phase spine landed)
- Worktree deferred items (config slots, pin↔slot, auto-prune on doctor, branch templates, global `--wt` depth — `worktrees.md`; per-project prune, prune `--all`, doctor orphan/dirty warns, registered slot dirty on list/status/info, and status/info `worktree_orphans` observation landed)

### 1. Design package

**Done means** (acceptance checklist; full record on planning issue design-package-acceptance):

- **Files present** — root `CONTEXT.md`; full-spec refs: `vision`, `architecture`, `config`, `cli`, `progen`, `multi-git`, `phased-delivery`; design-time sketch refs (later mixed depth): `worktrees`, `graph`, `env-gen-packs`. Not required: `concepts.md`, ADRs, research notes
- **Depth** — full-spec enough to start Implement core without reopening fundamentals; design-time sketches were not Ship gates (post-0.1.0 some landed as v1 — see Phase spine); CONTEXT = product nouns only, no “brain”
- **No unresolved conflicts** — map Decisions match cited files; required files don’t contradict locked choices; CONTEXT vocabulary used in refs; Out-of-scope / Not-yet-specified not silently promoted
- **Open questions** — map **Not yet specified** is the register (may be non-empty); no unnamed design blockers
- **Ready** — safe to chart a later **Implement core** map only; this phase does not start implementation
- **Close** — checklist green → close acceptance ticket → close design map. Human gate = closing that ticket

**Out of this phase:**

- Any product implementation or binary
- OS/arch release matrix, brew/other channels
- Full flag tables and implement-only detail deferred in reference docs

### 2. Implement core

**Done means:**

- Rust workspace in this repo matching crate intent in `architecture.md` (at least bin + core + git boundaries as real code)
- Workspace discovery, `.odm/` layout, Workspace config load, pin file basics
- Multi-git lifecycle for declared Projects (plain clones; no submodules): sync/pin/status/doctor as in `cli.md` / `multi-git.md`
- CLI spine: `init`, `sync` / `pin` / `status` / `doctor`, `project` lifecycle; globals (`--root`, `--json`, exit codes) per `cli.md`
- Dogfood Workspace `examples/core-desk` (local bare fixtures) + integration harness (init → add → sync → pin → status → doctor)
- Full v1 config schema loads (including non-core maps); no progen façade / `run` / sketch CLIs in this phase

**Slice order** (vertical; unbuilt core verbs exit `1` “not implemented”):

1. Cargo skeleton (`odm` / `odm-core` / `odm-git`)
2. Config, discovery, `init`
3. Git materialize/sync + `project add` / `list`
4. Pin status/apply + auto-maintain
5. `status` + gitignore manage
6. `doctor` (+ mechanical `--fix`)
7. `project` rm / info / git
8. `examples/core-desk` + integration harness gate

Full acceptance checklist: planning issue vertical-slice-order-and-core-acceptance (Implement core map).

**Out of this phase:**

- Progen façade / federation
- Actions / generators
- Public “ODM v1” GitHub Release (dogfood builds OK)
- Sketch features as product commitments

### 3. Progen integration

**Done means:**

- Progen crates integrated behind `odm-progen` façade; store verbs under `odm progen …`
- Federation and scope per `progen.md` (default-all, `--progen`, `--progen-group`; single-Progen writes)
- ODM-side index/cache only under `.odm/progen/<name>/` as in `architecture.md`
- Top-level `find` / `context` (or equivalent locked in CLI) honor scope rules

**Out of this phase:**

- Actions pipelines
- Cross-store wikilinks inside a Progen store (non-goal)
- Upstreaming multi-root into progen crates (unless a separate decision says so)

### 4. Actions

**Done means:**

- Action bundles loadable from Workspace config pointers; `odm run` (or locked CLI name) dispatches them
- Shell-out model for command bodies; Nx/user scripts remain outside ODM
- Enough action support that the “one desk” story is usable without ad-hoc wrappers for common tasks

**Out of this phase** (historical Actions-slice boundary; post-0.1.0 status differs — see Phase spine):

- HashiCorp go-plugin / npm plugin installers (dropped with Go)
- Generator/`template.toml` full depth (local generate v1 landed later; remote/templating still deferred — `env-gen-packs.md`)
- Agent-pack and worktree productization (both landed as local/v1 later; `agent prompt` v1 thin landed; `agent start` and worktree deferred items still open — `env-gen-packs.md`, `worktrees.md`)

### 5. Ship

**Done means:**

- Single **static `odm` binary** as the distributeable
- Primary channel: **GitHub Releases**
- **v1** requires Implement core + Progen integration + Actions far enough to match the vision one-liner (poly-repo desk, multi-Progen, one CLI). Core-only is not v1 Ship.
- Install/build docs updated for consumers

**Out of this phase (defer unless decided otherwise):**

- Concrete OS/arch matrix as a design-time lock (choose at ship time)
- Homebrew or other secondary channels
- `serve` / MCP
- Legacy Go config readers or submodule migration tools

## Intermediate builds

Dogfood or CI artifacts may exist during phases 2–4. They are not “ODM v1 shipped.” Only phase 5 with the criteria above is a real v1 release claim.

## Related

- Vision: `vision.md`
- Architecture (crates, ownership): `architecture.md`
- CLI: `cli.md`
- Multi-git: `multi-git.md`
- Progen: `progen.md`
- Worktrees (v1 + deferred): `worktrees.md`
- Env / generate / packs (mixed depth): `env-gen-packs.md`
- Still mostly sketch: `graph.md`
- Legacy facts (optional): `research/legacy-go-odm.md`

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

Sketch-only areas (worktrees, graph/tags, env, generators depth, agent packs) are **not** required phases before Ship. They enter an implement slice only when pulled in deliberately.

### 1. Design package

**Done means** (acceptance checklist; full record on planning issue design-package-acceptance):

- **Files present** — root `CONTEXT.md`; full-spec refs: `vision`, `architecture`, `config`, `cli`, `progen`, `multi-git`, `phased-delivery`; sketches: `worktrees`, `graph`, `env-gen-packs`. Not required: `concepts.md`, ADRs, research notes
- **Depth** — full-spec enough to start Implement core without reopening fundamentals; sketches at the locked sketch bar (not Ship gates); CONTEXT = product nouns only, no “brain”
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
- CLI spine: `init` and core verbs above; globals (`--root`, exit codes) per `cli.md`

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

**Out of this phase:**

- HashiCorp go-plugin / npm plugin installers (dropped with Go)
- Generator/`template.toml` full depth unless explicitly pulled from sketch into this slice
- Agent-pack and worktree productization unless explicitly pulled in

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
- Sketch sections: `worktrees.md`, `graph.md`, `env-gen-packs.md`
- Legacy facts (optional): `research/legacy-go-odm.md`

# ODM (Orchestrated Development Management)

Rust poly-repo workspace manager for humans and AI agents: declarative layout, plain git clones (no submodules), multi-**progen** docs/memory stores, worktrees, and a single `odm` CLI.

> **Status:** Design phase. Implementation removed; the product is being redesigned and will be rebuilt as a Rust workspace in this repo.

## Docs

- Design wayfinder map: [`docs/planning/issues/issues-1-odm-design-docs-map.md`](docs/planning/issues/issues-1-odm-design-docs-map.md)
- Legacy Go research (pre-removal): [`docs/reference/research/legacy-go-odm.md`](docs/reference/research/legacy-go-odm.md)
- Progenitor integration research: [`docs/reference/research/progenitor-surface.md`](docs/reference/research/progenitor-surface.md)

Reference design docs land under `docs/reference/` as the map is worked.

## Legacy Go

The previous Go CLI (`src/`, build scripts) was removed. Full tree: git tag `legacy-go-archive` or history before this change. Do not treat old README/submodule behavior as the product contract.

## Install / build

Not available yet. Target: single static binary via GitHub Releases once the Rust workspace ships.

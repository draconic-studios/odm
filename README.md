# ODM (Orchestrated Development Management)

Poly-repo workspace OS for humans and AI agents: one config, one CLI, and orchestrated Projects + Progens without submodules or a second brain product.

**Website:** [hembrow-innovations.github.io/odm](https://hembrow-innovations.github.io/odm/) (source: [`website/`](website/) on `main`; Pages via GitHub Actions)

**Status:** v0.1.0 spine (multi-git, Progen, Actions) plus post-0.1.0 **worktree slots** (add/list/rm/prune; doctor orphan/dirty warns) and local **`odm generate`**. Agent pack install/link/list/rm is local v1 (`odm status` `agent_packs` inventory; doctor `pack_missing` warn); **`odm agent prompt`** is v1 thin (context work-package); **`odm agent start`** is v1 one-shot exec (runtime matrix / pack auto-apply / serve deferred).

## Install

### Build from source (primary)

Requirements: **Rust 1.70+**, **git** on `PATH`. Actions (`odm run`) need a Unix shell.

```bash
git clone https://github.com/hembrow-innovations/odm.git
cd odm
cargo build -p odm --release
# binary: target/release/odm

# or install into cargo's bin dir:
cargo install --path crates/odm
odm --version
```

See [docs/reference/install.md](docs/reference/install.md) for details.

### GitHub Releases (when published)

When a release is published, download the platform tarball from [GitHub Releases](https://github.com/hembrow-innovations/odm/releases), extract `odm` onto your `PATH`, and verify with `odm --version`. Until then, build from source above.

Local packaging (tarball under `dist/`, optional `gh release create`):

```bash
./scripts/release-build.sh
```

Local test coverage (optional; not CI): `./scripts/coverage.sh` (needs `cargo-llvm-cov`; writes under `target/coverage/`).

## Quickstart

```bash
odm init
odm project add alpha --path projects/alpha --url <git-url>
odm sync
odm pin status
odm status
odm doctor
```

Progen (docs/memory stores) and Actions:

```bash
odm progen list
odm find <query>                 # default --limit 200
odm find <query> --limit 5
odm context <id>
odm run            # list actions
odm run <name>
```

`odm status` and `odm project info` report registered worktree slots and orphan slot dirs; `odm status` also lists registered agent packs.

Generators (local template) and worktree slots:

```bash
odm generate                              # list Generators
odm generate <name> --dest <rel-path> [--dry-run] [--force]  # materialize local template (or preview)
odm project worktree list <project>
odm project worktree add <project> <slot> [--branch <b>]
odm project worktree rm <project> <slot>
odm project worktree prune <project> [--force]
odm project worktree prune --all [--force]
```

See [docs/reference/cli.md](docs/reference/cli.md) for full surfaces (including `agent pack`), [examples/core-desk/README.md](examples/core-desk/README.md) for offline dogfood, and [examples/todo/README.md](examples/todo/README.md) for real-GitHub dogfood + [REVIEW.md](examples/todo/REVIEW.md).

Dogfood Workspace (offline fixtures):

```bash
cargo build -p odm
# full tour: ODM=target/debug/odm examples/core-desk/scripts/dogfood.sh
cd examples/core-desk
# see examples/core-desk/README.md
odm --root . sync
odm progen reindex
odm find DeskUniqueToken
odm run hello
odm generate                              # sample generators/ + hello template
odm generate hello --dest out/hello
```

Network dogfood (real public repos; read-only on remotes):

```bash
cargo build -p odm
ODM=target/debug/odm examples/todo/scripts/dogfood.sh
# TEMP=1 ODM=target/debug/odm examples/todo/scripts/probe.sh
```

## Docs

- **Website** (guides + quickstart): https://hembrow-innovations.github.io/odm/
- **Install**: [docs/reference/install.md](docs/reference/install.md) · [site install](https://hembrow-innovations.github.io/odm/install.html)
- **Vision**: [docs/reference/vision.md](docs/reference/vision.md)
- **CLI**: [docs/reference/cli.md](docs/reference/cli.md)
- **Architecture**: [docs/reference/architecture.md](docs/reference/architecture.md)
- **Config**: [docs/reference/config.md](docs/reference/config.md)
- **Multi-git**: [docs/reference/multi-git.md](docs/reference/multi-git.md)
- **Progen**: [docs/reference/progen.md](docs/reference/progen.md)
- **Worktrees**: [docs/reference/worktrees.md](docs/reference/worktrees.md)
- **Env / generate / packs**: [docs/reference/env-gen-packs.md](docs/reference/env-gen-packs.md)
- **Graph** (sketch): [docs/reference/graph.md](docs/reference/graph.md)
- **Phased delivery**: [docs/reference/phased-delivery.md](docs/reference/phased-delivery.md)
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)
- **Domain terms**: [CONTEXT.md](CONTEXT.md)

## Development

```bash
cargo test
```

Website browser smoke tests (Playwright): see [`website/README.md`](website/README.md).

## Legacy Go

The previous Go CLI was removed. Recoverable as git tag `legacy-go-archive` (or history before that change). Not a compatibility baseline — see [docs/reference/research/legacy-go-odm.md](docs/reference/research/legacy-go-odm.md).

## License

MIT

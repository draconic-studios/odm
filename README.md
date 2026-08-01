# ODM (Orchestrated Development Management)

Poly-repo workspace OS for humans and AI agents: one config, one CLI, and orchestrated Projects + Progens without submodules or a second brain product.

**Website:** [hembrow-innovations.github.io/odm](https://hembrow-innovations.github.io/odm/) (source: [`website/`](website/); publish: `./scripts/pages-publish.sh`)

**Status:** v0.1.0 spine (multi-git, Progen, Actions) plus post-0.1.0 **worktree slots** (add/list/prune; doctor orphan/dirty warns) and local **`odm generate`**. Agent pack install/link/list/rm is local v1 (`odm status` `agent_packs` inventory; doctor `pack_missing` warn); **`odm agent prompt`** is v1 thin (context work-package); `agent start` remains sketch.

## Install

### GitHub Releases (primary)

Download the tarball for your platform from [GitHub Releases](https://github.com/hembrow-innovations/odm/releases), extract `odm` onto your `PATH`, and verify:

```bash
tar xzf odm-0.1.0-<target>.tar.gz
# move ./odm somewhere on PATH
odm --version
```

See [docs/reference/install.md](docs/reference/install.md) for details.

### Build from source

Requirements: **Rust 1.70+**, **git** on `PATH`. Actions (`odm run`) need a Unix shell.

```bash
git clone https://github.com/hembrow-innovations/odm.git
cd odm
cargo build -p odm --release
# binary: target/release/odm

# or install into cargo's bin dir:
cargo install --path crates/odm
```

Release packaging (local tarball under `dist/`):

```bash
./scripts/release-build.sh
```

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
odm generate <name> --dest <rel-path> [--dry-run]  # materialize local template (or preview)
odm project worktree list <project>
odm project worktree add <project> <slot> [--branch <b>]
odm project worktree prune <project> [--force]
odm project worktree prune --all [--force]
```

See [docs/reference/cli.md](docs/reference/cli.md) for full surfaces (including `agent pack`) and [examples/core-desk/README.md](examples/core-desk/README.md) for dogfood depth.

Dogfood Workspace (offline fixtures):

```bash
cargo build -p odm
cd examples/core-desk
# see examples/core-desk/README.md
odm --root . sync
odm progen reindex
odm find DeskUniqueToken
odm run hello
odm generate                              # sample generators/ + hello template
odm generate hello --dest out/hello
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
- **Phased delivery**: [docs/reference/phased-delivery.md](docs/reference/phased-delivery.md)
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)
- **Domain terms**: [CONTEXT.md](CONTEXT.md)

## Development

```bash
cargo test
```

## Legacy Go

The previous Go CLI was removed. Recoverable as git tag `legacy-go-archive` (or history before that change). Not a compatibility baseline — see [docs/reference/research/legacy-go-odm.md](docs/reference/research/legacy-go-odm.md).

## License

MIT

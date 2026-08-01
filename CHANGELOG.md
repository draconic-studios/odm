# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-08-01

First public spine release: core multi-git Workspace, Progen integration, and Actions behind one `odm` binary.

### Added

- **Core**
  - `odm init` — bootstrap a Workspace (`.odm/`, config, optional git)
  - `odm sync` / `odm pin` / `odm status` / `odm doctor` — multi-git lifecycle on plain clones (no submodules)
  - `odm project` lifecycle — add, list, rm, info, git
  - Workspace discovery, pin file, gitignore manage
- **Progen**
  - Vault engine façade under `odm progen …`
  - Top-level `odm find` and `odm context` with multi-Progen scope
  - Progen lifecycle (add/list/rm/info) and store verbs
- **Actions**
  - `odm run` — list and dispatch Action bundles from Workspace config
  - Shell-out model for action command bodies
- **Dogfood**
  - `examples/core-desk` — offline Workspace exercising core, path-only Progen, and shell Actions
- **Ship**
  - `scripts/release-build.sh` — release tarball under `dist/odm-<version>-<target>.tar.gz`
  - Consumer install docs (README, `docs/reference/install.md`)

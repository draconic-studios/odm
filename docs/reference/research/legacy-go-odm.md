# Research: Legacy Go ODM capabilities

**Question:** What does this repo's Go ODM implement today (CLI surface, config shape, submodules, plugins, actions) that migration docs must call out as replace, drop, or optionally map?

**Scope:** Facts from this codebase only (`src/`, `README.md`, `scripts/`). No external product docs.

**Date:** 2026-07-31

---

## 1. Layout and build

| Item | Fact | Source |
|------|------|--------|
| Language / module | Go module `odm`, `go 1.24.5` | `src/go.mod` |
| Entrypoint | `src/main.go` → `package main` | `src/main.go` |
| Local build | `cd src && go build -o ../bin/odm main.go` | `scripts/build.sh` |
| Install script | builds to `src/odm`, moves to `/usr/local/bin/` | `scripts/install.sh` |
| Cross-compile | darwin arm64/amd64, linux amd64, windows amd64 → `bin/odm-*` | `scripts/os-arch-build.sh` |
| README build note | `go build -o odm main.go` (implies cwd = source tree; scripts use `src/`) | `README.md` |
| Dependencies | `gopkg.in/yaml.v3`, `github.com/hashicorp/go-plugin`, `github.com/hembrow-innovations/odm-plugin`, gRPC/protobuf transitively | `src/go.mod` |

Packages under `src/`:

- `main` — CLI coordinator
- `orchestrator` — config load/write, core actions, project CRUD in memory
- `git` — submodule add/remove via shell `git`
- `plugin` — HashiCorp go-plugin manager + declaration discovery
- `core-plugins` — in-process `cmd`, `copy`, `env`
- `installer` — plugin install (npm path; pip stub)
- `utils` — arg parse, fs, run command, nested map access
- `messages` — stale help strings (not wired to live CLI)

---

## 2. CLI surface and entrypoints

### 2.1 Startup flow

1. `main()` → `HandleInput()` parses `os.Args[1:]` into `utils.Command`.
2. Default `--root-path` to `os.Getwd()` if unset.
3. `Coordinator.initCoordinator` requires `root-path`, loads config.
4. `runCoordinator` dispatches help → core action → named config action.

Sources: `src/main.go` (`HandleInput`, `initCoordinator`, `runCoordinator`).

### 2.2 Argument parsing

Custom parser (not cobra/flag):

- First token = command `Name`
- Non-flag tokens before first `-`/`--` = positional `Args`
- Token `help` among positionals sets `Help`
- Flags: `--name value`, `--name=value`, or trailing bool flags
- Short `-x` treated like long (strip 1 or 2 dashes)

Source: `src/utils/parse-args.go`.

### 2.3 Built-in (core) commands

Hard-coded list only:

```go
actionsList := []string{"add", "remove", "install"}
```

Source: `src/orchestrator/actions.go` (`IsCoreAction`).

| Command | Args | Flags used in code | Behavior |
|---------|------|--------------------|----------|
| `add` | `<repo-url> <destination-path>` | `--type` (default `"project"`), `--root-path` | `git submodule add` then in-memory `UpdateProject` | `actions.go` `AddProject`, `git/sub-modules.go` |
| `remove` | `<projectName>` (config key, not path) | `--root-path` | look up project by name → `RemoveGitSubmodule` by `project.Path` → then calls `RemoveProject` again (recursive bug; see §8) | `actions.go` |
| `install` | `<installType> <packageName>` | `--root-path` | `installer.InstallPlugin` with `PluginFolder: ".odm/plugins"`, `Type`/`Package` from args | `actions.go` |

README documents `odm add` / `odm remove` with path semantics and `odm build-docs`. Code: remove uses **project name** from config map; **`build-docs` is not implemented** as a core action (only a comment in `main.go` that it should move to a plugin).

Sources: `README.md`; `src/orchestrator/actions.go`; `src/main.go` line comment about `build-docs`.

### 2.4 Config-defined actions as commands

Any key under `actions` in config becomes a top-level command:

```text
odm <action-name>
```

Resolved after core actions fail the core list. Source: `src/main.go` (`isDefinedAction`, `executeDefinedAction`); `README.md` “Custom Actions”.

### 2.5 Help

- `command.Help` true → `runHelp` prints `"Help"` and returns; **no real help text**.
- `src/messages/help.go` defines `GlobalUsage`, `BuildUsage`, `RunUsage` for commands `build`, `run`, `clean`, `docker-build`, `docker-run` — **not referenced** by `main` or core action list. Stale / prior design.

### 2.6 Exit codes

Constants defined (`ExitSuccess`, `ExitGenericError`, `ExitInvalidInput`, `ExitNetworkError`) but `main` always `os.Exit(2)` on error. Source: `src/main.go`.

### 2.7 Plugin manager on the happy path

`initPluginManager` exists and would default plugin dir to `.plugins` under root, but **`main` never calls `initPluginManager`**. Config-defined actions that need external plugins therefore cannot load plugins on the current entry path unless something else initializes `PluginManager` (nothing does).

Source: `src/main.go` (`initPluginManager` vs `main` / `runCoordinator`).

---

## 3. Config file shape

### 3.1 Discovery

- Must live at **project root** (`RootPath`).
- Exact basenames: `odm.config.yaml` **or** `odm.config.json` (first match in directory listing order: yaml checked before json in the loop, but whichever is found first when iterating dir entries wins only if that name matches; loop breaks on first of either).
- No support for `.odm.config.*` despite README first line saying “`.odm.config.json` or `.odm.config.yaml`” — actual code and later README sections use **`odm.config.yaml` / `odm.config.json` without leading dot**.

Sources: `src/main.go` `GetOdmConfigFile`; `README.md` (inconsistent naming).

### 3.2 Schema (structs)

From `src/orchestrator/orchestrator-config.go`:

```text
OrchestratorConfig
  name: string
  documentaton: Documentaton          # typo in field + yaml/json tag "documentaton"
    docs-path: string
    doc-type: string
    output: string
  projects: map[string]Project        # keyed map, not array
    name, path, repo, type: string
  actions: map[string]Action
    args: map[string]string
    tasks: []Task
      executer: string
      options: map[string]any
      input: map[string]any           # on Task struct; execution path does not wire Task.Input
  plugins: map[string]Plugin
    name, path, type: string          # yaml tags on path/type mistyped as "ymal" in struct tags
  plugin-config: PluginConfig
    location: string
    plugin-suffix: string
```

**Not in struct (README-only or absent):**

- Top-level `description` (README example) — ignored by unmarshal
- README `projects` as a **list** — code expects a **map**
- README `documentation` key — code tag is **`documentaton`**
- No worktrees, pins, multi-root, env profiles, or `.odm/` layout in config schema

### 3.3 Read / write

- `ReadOdmConfig` — json or yaml into `Orchestrator.Config`. Source: `src/orchestrator/file.go`.
- `WriteOdmConfig` — exists; **no caller** in the repo greps (add/update project never persists). Source: `src/orchestrator/file.go`; `project.go` only mutates memory.

### 3.4 Example action shape (README + execution)

```yaml
actions:
  my-custom-action:
    tasks:
      - executer: cmd
        options:
          command: echo "Hello..."
          path: .
```

Execution builds `odmplugin.ExecutionRequestBody{ Args: action.Args, Options: task.Options, Input: currentOutput }` and chains string `currentOutput` across tasks. Source: `src/main.go` `executeDefinedAction`.

---

## 4. Git submodule usage

Only multi-repo mechanism in code: **git submodules**.

| Operation | Implementation | Source |
|-----------|----------------|--------|
| Add | `git submodule add <url> <path>` in parent repo cwd | `src/git/sub-modules.go` `AddGitSubmodule` |
| Remove | 5 steps: `submodule deinit -f`, remove `.gitmodules` section (name = `filepath.Base(path)`), `git rm --cached`, `rm -rf .git/modules/<path>`, `rm -rf <path>` | `RemoveGitSubmodule` |
| Config linkage | `Project{Name, Path, Repo, Type}`; name from last path segment on add | `actions.go` `AddProject` |
| Types | free string; flag default `"project"`; README example uses `library` | `actions.go`, `README.md` |

No: submodule update/init/sync CLI, pin SHAs in odm config, sparse checkout, or git worktrees.

---

## 5. Plugin system

### 5.1 Two layers

1. **Core plugins (in-process)** — map in `core-plugins.CorePluginList`: `cmd`, `copy`, `env`. Source: `src/core-plugins/core-plugin-list.go`.
2. **External plugins** — HashiCorp `go-plugin` over NetRPC; handshake/map from `github.com/hembrow-innovations/odm-plugin`; dispense name `"executer"`; 5s timeout. Source: `src/plugin/plugin-manager.go`.

### 5.2 Discovery (external)

- Plugin dir = `RootPath` + `plugin-config.location` (default intended `.plugins` in `initPluginManager`).
- Reads `pluginDir/declarations/*.json` into `PluginDeclaration`: `name`, `version`, `language`, `source`, `type`, `package`.
- `source` must be path to executable binary.

Note: installer lays out **`.odm/plugins`** with `definitions/`, `plugins.json`, npm `node_modules` — **not the same path or layout** as discovery’s `declarations/` under `.plugins`. Migration must treat this as unfinished dual design.

Sources: `plugin-manager.go` `discoverPlugins`; `installer/pre-install.go`; `main.go` `initPluginManager`; `actions.go` install opts.

### 5.3 Config `plugins` map

Struct fields exist (`name`, `path`, `type` with broken `ymal` tags) but runtime execution resolves executers via **core map** or **PluginManager.Plugins** by name — not clearly driven by the config `plugins` map in the execute path.

### 5.4 Install pipeline

- CLI: `odm install <type> <package>`
- Creates `.odm/plugins/definitions`, `.odm/plugins/plugins.json`, `.odm/plugins/package.json` if missing
- Type switch: only `"npm"` implemented (`NpmInstall`); unknown type errors
- npm: `npm install` in plugin folder; expects `plugin.json` with `source`; copies declaration under `definitions/`
- Pip installer file exists with comment “Stopped til odm works…” and is **not** wired in `Installation.start`

Sources: `src/installer/*.go`.

### 5.5 Core plugin contracts

| Executer | Options | Behavior | Source |
|----------|---------|----------|--------|
| `cmd` | `command` (required string), `path` (optional cwd) | `strings.Fields` split; `utils.RunCommand` | `core-plugins/command.go` |
| `copy` | `source`, `destination`, `type` (`folder` vs default file) | paths joined with `root-path` from `body.Args` or cwd | `core-plugins/copy.go` |
| `env` | `output`, `items[]` with file types env/json/yaml, key mappings | merges into one `.env` write | `core-plugins/env.go`, `README.md` |

Task chaining: previous task stdout/result string → next task `Input`.

---

## 6. Actions / scripts (product vs repo)

### 6.1 Product “actions”

User-defined pipelines in config (`actions` → ordered `tasks` with `executer` + `options` + optional action-level `args`). This is the primary extensibility surface. Source: `orchestrator-config.go`, `main.go`.

### 6.2 Repo maintenance scripts

Not product CLI — build/install only:

- `scripts/build.sh`
- `scripts/install.sh`
- `scripts/os-arch-build.sh`

No Makefile, no CI config in tree for odm itself (beyond these scripts).

---

## 7. Worktree / multi-project support

| Capability | Present? | Notes |
|------------|----------|-------|
| Git worktrees | **No** | No references in `src/` |
| Multiple projects | **Yes, limited** | `projects` map of submodule-backed entries |
| Nested odm configs | **No** | Single root config file |
| Federation / monorepo graph | **No** | Flat project list only |
| Per-project actions | **No** | Actions are root-level only |

---

## 8. Implementation gaps / bugs migration should not silently preserve

Facts that affect “what works today” vs README:

1. **`build-docs` documented, not implemented** as core command; documentation struct unused in execution path. (`README.md`, `actions.go`, `main.go` comment)
2. **`initPluginManager` never called** — external plugins dead on main path. (`main.go`)
3. **`WriteOdmConfig` never called** after add/update — projects not persisted. (`file.go`, `project.go`, `actions.go`)
4. **`RemoveProject` recursive self-call** instead of `DeleteProject` — broken remove after submodule step. (`actions.go` vs `project.go`)
5. **Plugin path inconsistency**: default `.plugins` + `declarations/` vs install `.odm/plugins` + `definitions/`. (`main.go`, `installer/`, `plugin-manager.go`)
6. **Stale help** for build/run/docker. (`messages/help.go`)
7. **README vs schema**: dotted config names, `documentation` vs `documentaton`, projects array vs map, `description` field.
8. **Env core plugin** type-asserts `items` to `[]BuildItem` from `map[string]any` options — likely broken when loaded from YAML/JSON without custom unmarshal (options are `map[string]any`). (`env.go`, `Task.Options`)
9. **Second default for `PluginSuffix`** is nested under `if Location == ""` twice — suffix default never applies as written. (`main.go` `initPluginManager`)

---

## 9. Migration guidance (replace / drop / optional map)

Based only on what this codebase implements or claims.

### 9.1 Replace (capability still needed in some form)

| Legacy | Why |
|--------|-----|
| Root config file (`odm.config.yaml` / `.json`) driving named actions | Central product concept |
| Named actions as ordered task pipelines | Core orchestration model |
| Shell task executer (`cmd`) | Primary workhorse |
| File/folder copy task (`copy`) | Documented core plugin |
| Env file synthesis (`env`) from env/json/yaml | Documented core plugin |
| Multi-component registry (`projects` with path/repo/type) | Models monorepo/sub-repos |
| Add/remove component linked to VCS checkout | User-facing lifecycle |
| `--root-path` / run-from-project-root | Invocation model |
| Optional external plugin executers | Extensibility (even if unfinished) |
| Cross-platform single binary distribution | `scripts/os-arch-build.sh` intent |

Prefer **clean contracts** in Rust redesign rather than byte-compatible YAML if tags/typos (`documentaton`, `ymal`) and dual layouts are fixed deliberately — call out breaking rename in migration.md.

### 9.2 Drop (do not port as-is)

| Legacy | Why drop |
|--------|----------|
| HashiCorp go-plugin + `odm-plugin` Go RPC stack | Go-specific; redesign plugin ABI |
| npm/pip plugin installers and `.odm/plugins` npm workspace | Incomplete, path-incoherent; revisit packaging later |
| Stale CLI surface in `messages/help.go` (build/run/clean/docker-*) | Not implemented |
| `build-docs` as currently “documented behavior” without code | Spec afresh or omit until designed |
| Infinite-recursion / non-persisting project mutations | Bugs, not features |
| Dual plugin dirs (`.plugins` vs `.odm/plugins`) and dual declaration folders | Collapse to one design |
| Typo-stable config keys (`documentaton`) as compatibility target unless users already depend on them | Prefer correct names + migration note |
| Unused exit-code constants / always exit 2 | Redesign process UX |
| Pip installer stub | Explicitly unfinished (`pip-installer.go` comment) |

### 9.3 Optionally map

| Legacy | Optional mapping notes |
|--------|------------------------|
| Git **submodules** specifically | Map to “nested git dependency” concept; redesign may use submodules, subtrees, worktrees, or pin files — do not assume submodule is the forever VCS mechanism |
| Project `type` free string (`project` / `library` / …) | Preserve as metadata enum if useful |
| Action-level `args` map passed into every task | Useful for shared parameters (`root-path`, etc.) |
| Task output chaining (string pipe) | Keep as simple composition or replace with typed artifacts |
| JSON **or** YAML config | Convenience; one canonical format is enough if migration converts |
| Plugin declaration JSON (`name`, `version`, `language`, `source`, `type`, `package`) | Shape may inform new plugin manifest |
| `plugin-config.location` / suffix | If plugins remain filesystem-discovered |
| Documentation block (`docs-path`, `doc-type`, `output`) | Only if docs aggregation returns as a first-class feature |
| Config `plugins` map vs filesystem discovery | Pick one discovery model; map users if any |

### 9.4 Not present (do not invent as “legacy parity”)

No worktree CLI, no pin file, no progen/federation, no nested odm roots, no service graph, no docker orchestration in live code (help text only).

---

## 10. Source index

| Path | Role |
|------|------|
| `README.md` | User-facing claims (partially ahead of/behind code) |
| `src/main.go` | CLI coordinator, config discovery, action execution |
| `src/orchestrator/orchestrator-config.go` | Config schema |
| `src/orchestrator/file.go` | Read/write config |
| `src/orchestrator/actions.go` | Core commands add/remove/install |
| `src/orchestrator/project.go` | In-memory project CRUD |
| `src/orchestrator/config.go` | Orchestrator shell type |
| `src/git/sub-modules.go` | Submodule add/remove |
| `src/plugin/plugin-manager.go` | External plugin load/run |
| `src/core-plugins/*` | cmd, copy, env |
| `src/installer/*` | Plugin install FS + npm |
| `src/utils/parse-args.go` | CLI parsing |
| `src/messages/help.go` | Dead help copy |
| `src/go.mod` | Module and deps |
| `scripts/*` | Build/install |

---

## 11. One-line summary for migration.md

Legacy Go ODM is a single-binary CLI that loads root `odm.config.{yaml,json}`, runs hard-coded `add`/`remove`/`install` plus named action pipelines of `cmd`/`copy`/`env` (and unfinished HashiCorp plugins), and treats multi-repo as git submodules — with docs features and plugin install paths incomplete; Rust redesign should replace the config+actions+component model, drop go-plugin/npm installers/stale docker help, and only optionally map submodules and env/copy task shapes.
)

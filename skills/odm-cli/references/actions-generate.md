# Actions and generators

## Actions (`odm run`)

Actions are named tasks from **Action bundle** files pointed to by Workspace
config. They are **never** installed as top-level CLI verbs.

### Config

```yaml
# .odm/odm.config.yaml
actions:
  core: actions/core.yaml
```

```yaml
# actions/core.yaml
hello:
  tasks:
    - run: echo hello-desk
in-api:
  tasks:
    - run: npm test
      dir: apps/api          # optional; default = Workspace root
```

- Merge all bundles into one Action namespace.
- Duplicate Action names across bundles → config error (exit 2).
- Bundle map keys are organizational only (not CLI selectors).
- v1 task spine: `run` (shell string) + optional `dir`.

### CLI

```bash
odm run --json                              # list
odm run <action-name>
odm run <action-name> --project <name>
odm --project <name> --wt <slot> run <action-name>
odm run <action-name> -- -- extra args
odm --json run <action-name>                # capture wrapper
```

### Behavior

- Unknown action → exit **1**.
- **Cwd resolution:** Action `dir` if set; else Workspace root; `--project` /
  `--wt` override to that working tree when set.
- Without `--json`: task stdio inherits the terminal.
- With `--json`:

```json
{ "action": "hello", "exitCode": 0, "stdout": "…", "stderr": "…" }
```

- After spawn, process exit code = **action’s** exit code (may be outside 0–4).
- Pre-exec failures use 1 / 2.

### Requirements

- Unix shell available for `run` tasks.
- `git` still required for Workspace git ops, but Actions are plain shell.

## Generators (`odm generate`)

Local template scaffolds. Not Actions.

### Config

```yaml
# .odm/odm.config.yaml
generators:
  core: generators/core.yaml
```

```yaml
# generators/core.yaml
hello:
  template: templates/hello        # local path relative to Workspace root
# remote:
#   url: https://github.com/acme/gen.git   # list OK; run deferred
```

### CLI

```bash
odm generate --json
# { "generators": [ { "name", "template", "url" } ] }

odm generate <name> --dest <rel-path> [--force] [--dry-run] [--json]
```

### Behavior

- **List** (no name): sorted names; empty → `(no generators)`.
- **Run**: recursive copy of local `template` dir into `--dest` (both relative
  to Workspace root; must not escape). **No** variable substitution in v1.
- `--dest` required when a name is given; creates parent dirs as needed.
- Non-empty dest without `--force` → exit **3**.
- `--force`: overwrite files in place (does not delete unrelated extras).
- `--dry-run`: full validation, no filesystem writes; `copied` = would-write count.
- Url-only generators: appear in list; run → exit **1** (remote deferred).
- If both `template` and `url` set, prefer `template`.

### JSON run shape

```json
{ "generator": "hello", "dest": "out/hello", "copied": 3, "dry_run": false }
```

### Agent pattern

```bash
odm generate --json
odm generate hello --dest out/hello --dry-run --json
odm generate hello --dest out/hello --json
# existing non-empty:
odm generate hello --dest out/hello --force --json
```

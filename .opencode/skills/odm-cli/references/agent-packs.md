# Agent packs, prompt, and start

All AI/agent-facing UX lives under `odm agent` (not a top-level `pack` command).

## Agent packs (v1 local)

Portable directories of agent assets (skills, prompts, conventions). ODM
copies or symlinks them into an **agent home** and records them in
`.odm/agent-packs.json`.

```bash
odm agent pack list --json
odm agent pack install <source> --home <path> [--force]
odm agent pack link <source> --home <path> [--force]
odm agent pack rm <name>
```

### Rules

- **Workspace required** (same discovery as other commands).
- **Source:** local directory. Relative → under Workspace root (must not escape).
  Absolute paths allowed. Pack **name** = directory basename.
- **`--home`:** required agent-native root (may be outside Workspace).
  Materializes at `<home>/<name>/`.
- **`install`:** recursive copy. Dest exists without `--force` → exit 3.
  Missing source → exit 4.
- **`link`:** symlink `<home>/<name>` → absolute resolved source. Same
  exists/`--force` policy. No silent copy fallback if symlinks unsupported.
- **`list`:** registry-backed. `missing: true` when dest has no path/symlink
  (dangling symlink present is **not** missing). Doctor warns `pack_missing:<name>`.
- **`rm`:** drop registry entry + best-effort delete dest. Unknown name → 4.
  Missing dest still succeeds (stale cleanup).
- No remote/marketplace; no pack manifest required in v1.

### Common homes

```bash
# opencode project skills
odm agent pack install ./agent-packs/desk --home .opencode/skills --force

# Claude Code
odm agent pack install ./agent-packs/desk --home ~/.claude/skills --force

# generic agents
odm agent pack link ./agent-packs/desk --home ~/.agents/skills --force
```

### JSON shapes

List:

```json
{ "packs": [ { "name": "desk", "source": "…", "path": "…", "mode": "install|link", "missing": false } ] }
```

install / link / rm `--json`: single entry object (same fields), not wrapped.

### status inventory

`odm status --json` includes top-level:

```json
"agent_packs": [ { "name", "source", "path", "mode", "missing" } ]
```

Always present (empty array when none).

## agent prompt (v1 thin)

```bash
odm agent prompt <id> [--progen <name>] [--json]
odm agent prompt <progen-name>:<id> [--json]
```

Thin alias of `odm context`: same scope rules, same human markdown, same
`--json` `ContextHit` (`anchor` / `outgoing` / `incoming`).

- Packages one note’s in-store neighborhood to stdout for agents.
- **Not** a second prompt engine, task planner, or cross-store graph walk.
- Unknown id → exit 4.

## agent start (v1 one-shot)

```bash
odm --project <name> [--wt <slot>] [--json] agent start -- <program> [args…]
odm --project <name> agent start <program> [args…]
```

### Behavior

- Direct exec of caller-supplied argv with cwd = Project Primary or `--wt` slot.
- **`--project` required** (missing → usage 1).
- Optional `--wt` (existing slot only; missing → 4; no auto-create).
- At least one program token; empty → usage 1.
- **Independent of packs and prompt** — does not read registry, install packs,
  set pack env, or call `prompt`/`context`.
- **No runtime matrix** — no default agent binary; no claude/cursor detection.
- Human: inherit child stdio; exit = child exit.
- `--json`:

```json
{ "cwd": "…", "program": "npm", "args": ["test"], "exitCode": 0, "stdout": "…", "stderr": "…" }
```

Then exit with child code. Pre-exec failures use standard 1–4 envelope.

### Examples

```bash
odm --project api agent start -- true
odm --project api --wt feat agent start -- npm test
odm --project api --json agent start -- git status
```

## Deferred (do not invent)

- Pack marketplace / manifests / config-declared packs
- Pack auto-apply on `agent start`
- Runtime brand matrix / default agent binary
- Prompt-on-start composition
- Long-running sessions, MCP, `odm serve`

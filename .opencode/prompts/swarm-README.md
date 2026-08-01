# Swarm loop (AFK)

Build-out loop: seed backlog from real gaps, then one ticket per fresh session through explore → TDD implement → test → review → commit → close.

## Layout

- **Agents**: `.opencode/agent/swarm-*.md`
- **Prompts**: `swarm-iteration.md`, `swarm-seed.md`
- **Loop**: `.opencode/scripts/swarm-loop.sh`
- **TUI one-shot**: `/swarm`
- **Runtime**: `.opencode/swarm/state.json`, `log/`, touch `STOP` to halt

## Run

```bash
# AFK multi-iteration (default 20)
.opencode/scripts/swarm-loop.sh

# Cap iterations
.opencode/scripts/swarm-loop.sh 5

# Extra opencode flags after max_iters
.opencode/scripts/swarm-loop.sh 10 -m opencode/gpt-5.1-codex

# Stop between iterations
touch .opencode/swarm/STOP
```

Env:

- **SWARM_MAX_FAILS** — consecutive non-success before loop exits 2 (default 3)
- **SWARM_AGENT** — agent name (default `swarm-orchestrator`)
- **OPENCODE_BIN** — opencode binary

Single iteration without the shell loop:

```bash
opencode run --agent swarm-orchestrator --auto \
  -f .opencode/prompts/swarm-iteration.md \
  -- "Execute exactly one swarm pipeline cycle."
```

Or in TUI: `/swarm` (use with care; AFK wants `--auto`).

## Exit codes (loop)

- **0** — finished max iters or clean DONE
- **2** — too many consecutive stuck/fail iterations
- **130** — `STOP` file
- **127** — opencode missing

Agent lines `SWARM_EXIT:0|2|3` are parsed from stdout and override bare process codes when present.

## Safety

- Commits land on the **current branch** (no push, no worktrees, no new branches).
- Prefer a dedicated clone if you do not want main rewritten unattended.
- Review the first seeded map before long unattended runs.
- Token cost scales with iterations × full pipeline.

## First useful human check

After iteration 1 (usually a seed): skim new issues under `docs/planning/issues/`, retag anything that needs a human, then clear `STOP` and continue.

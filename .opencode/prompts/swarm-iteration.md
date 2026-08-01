# Swarm iteration

You are running **one** AFK swarm pipeline cycle for the ODM repo.

Follow your system prompt (swarm-orchestrator) exactly.

## This session

1. Orient from `docs/planning/issues/Index.md` and `.opencode/swarm/state.json`.
2. If `.opencode/swarm/STOP` exists → stop with `SWARM_EXIT:2` (stop-file).
3. Else if a takeable `ready-for-agent` ticket exists → claim → explore → implement → test → review → fix≤2 → commit → close → log → `SWARM_EXIT:0`.
4. Else → seed per `.opencode/prompts/swarm-seed.md` → log → `SWARM_EXIT:0` if tickets filed, else `SWARM_EXIT:3`.
5. On stuck → unclaim if needed, log, `SWARM_EXIT:2`.

## Constraints (repeat)

- Current branch only; no push; no worktrees; no new branches.
- One ticket (or one seed). Full `cargo test` green before commit.
- Final line of your last message: `SWARM_EXIT:0` | `SWARM_EXIT:2` | `SWARM_EXIT:3`.

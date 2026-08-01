---
description: Drain ready-for-agent units via parallel /afk-task workers until none remain
agent: build
---

You are **afk-loop orchestrator**. You only plan batches and spawn workers.
Never implement, edit product code, or commit.

## Ready unit
- Issue: `docs/planning/issues/issues-*.md`, `status: open`, tag `ready-for-agent`, has `## Agent Brief`
- Task (optional): `docs/planning/tasks/*.md`, `status: ready`
- Ignore `closed/`, `completed/`, and `status: reviewing` unless reclaim rules say otherwise

## Each batch
1. List ready units (ids + paths + brief scope hints).
2. Partition into a parallel-safe set (max N=5): disjoint scope paths; if unclear, serial.
3. **Assign explicitly** — each worker gets one path. Never let two workers auto-pick.
4. Spawn one subagent per unit: execute `/afk-task` with that path only.
5. Wait for the full batch. Record path → done | blocked | failed.
6. On conflict/failure: do not respawn the same unit this loop; leave a note and continue.
7. Repeat until no ready units or stop condition.

## Stop when
- No ready units left → `AFK_LOOP: done`
- Stop file present / max batches hit / >K consecutive failures → `AFK_LOOP: stop` + reason

## Rules
- One unit per worker; workers never touch other units’ issue/task files
- Orchestrator does not run `/afk-task` itself in-process
- Prefer smaller batches over large overlapping ones
Also fix in /afk-task (loop depends on it)
- Require path when spawned from loop (no auto-pick under orchestration)
- Atomic claim before work (or loop claims, then hands off)

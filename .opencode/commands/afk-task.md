---
description: Autonomously complete exactly one ready unit — a ready issue or a ready task from docs/planning (TDD, in-scope only), then exit
agent: build
---

You are an autonomous agent completing exactly ONE ready unit from `docs/planning`, then exiting.
A **ready unit** is either a **`ready-for-agent` issue** under `docs/planning/issues`
(`status: open` + tag `ready-for-agent`, carrying a `## Agent Brief` with scope/verification/
acceptance) or a **`status: ready` task** under `docs/planning/tasks`. Both are agent contracts;
treat them the same — the Agent Brief plays the role of the task's Scope/Steps/Acceptance.

Arguments: $ARGUMENTS
- **Path required under afk-loop:** when spawned by the loop orchestrator, `$ARGUMENTS` MUST be exactly one issue/task path. Do not auto-pick. If missing/empty, exit immediately with `AFK_TASK: failed — path required`.
- Solo use (human invokes `/afk-task` with no args): pick the lowest-numbered ready unit across BOTH `docs/planning/issues/*.md` (`status: open` + tag `ready-for-agent`) and `docs/planning/tasks/*.md` (`status: ready`), none in `closed/`/`completed/` — one shared id sequence, lowest id wins.

Workflow (follow AGENTS.md and project conventions throughout):
1. Resolve the unit path (from args, or solo auto-pick). Read the unit file, then any relevant plan under `docs/planning/plans` and related docs. Delegate broad exploration to a subagent; consume its summary.
2. **Atomic claim before any product work** (fail closed):
   - Re-read the unit file. Abort if not still ready (`status: open` + `ready-for-agent` for issues; `status: ready` for tasks) or if already `reviewing`/`active`/`closed`/`complete`.
   - Claim in the same edit: issue → `status: reviewing`; task → `status: active`. Write the file immediately.
   - Optional: `git add` only that unit file and commit `chore(planning): claim <id>` so parallel workers cannot double-claim. If the file changed under you (claim lost), exit `AFK_TASK: failed — claim lost`.
3. Implement the unit 100% — TDD, no stubs, no skipped scope. Respect its scope (task "Scope (may touch)" or issue `## Agent Brief`); no repo-wide changes. Never touch other units' issue/task files.
4. Verify: run the checks and tests until green.
5. Spawn ONE subagent to adversarially review your diff for defects and convention violations; fix what it confirms.
6. Update docs: tick the matching checkboxes; add change to changelog. Then close by kind, git commit work:
   - **ready issue** → set `status: closed`, move to `docs/planning/issues/closed/`.
   - **task** → set `status: complete`, move to `docs/planning/tasks/completed/`, and close its source issue (`status: closed` → `issues/closed/`).
7. If genuinely blocked (missing decision, broken precondition), append a "## Blocked" section explaining exactly why and what's needed, revert the claim (task → `status: hold`; issue → `status: open`, keep the `ready-for-agent` tag until a human drops it), commit that, and exit — do NOT mark it complete/closed. Report `AFK_TASK: blocked`.

Exit status line (last line of your final message): `AFK_TASK: done` | `AFK_TASK: blocked` | `AFK_TASK: failed — <reason>`.

Rules: never touch other issue/task files except this unit's own (and, for a task, its source issue); never run vault-wide or store-wide fixes. Any issues arise, create an issue for reviewing. Do not implement a different unit than the assigned path.

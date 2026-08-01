# Swarm seed — backlog from real gaps

AFK. No user quiz. File only **concrete, verifiable** work.

## Sources (scan in order)

1. **Hardening spine** — flaky/missing tests, clippy debt, error/exit honesty, `examples/core-desk` dogfood gaps, docs drift vs code (`README`, `CHANGELOG`, `docs/reference/*`).
2. **Worktree slots** — `docs/reference/worktrees.md` + CLI stubs (`project worktree`, `--wt`).
3. **Generators** — `docs/reference/env-gen-packs.md` (Generators) + `odm generate` stub.
4. **Agent packs** — same sketch + `odm agent` stubs.
5. **Graph** — `docs/reference/graph.md` (lowest priority; most deferred).
6. **Other improvements** — only if you can state acceptance criteria an agent can check without a human.

## Do not

- Reopen closed phase 1–5 design decisions without a dedicated decision ticket tagged `ready-for-human`.
- One mega-ticket for an entire sketch — split tracer bullets.
- Speculative product invention not grounded in docs or code stubs.
- Duplicate open/closed issues (search `docs/planning/issues/` first).

## Priority

1. Hardening / correctness of existing core + progen + actions  
2. Worktrees  
3. Generators  
4. Agent packs  
5. Graph  

## Output artifacts

Follow `docs/agents/issue-tracker.md` and `docs/agents/triage-labels.md`.

1. Allocate next ids (high-water across live + `closed/`).
2. Optional **map** issue: tags `wayfinder-map`, `planning` — Destination / Notes / Decisions so far / Not yet specified / Out of scope.
3. **Tickets**: each `status: open`, tags include `ready-for-agent` and when wayfinder: `wayfinder`, `wayfinder-task` (or research/grilling if truly needed — prefer `task` for AFK build).
4. Each ticket must have:
   - `## Description`
   - `## Agent Brief` (enough to implement with zero questions)
   - `## Acceptance` checklist
   - `## Blocked by` (`None` or wikilinks)
5. Refresh `docs/planning/issues/Index.md` (Maps, Frontier, Blocked).
6. Prefer **≤8** new tickets per seed; quality over volume. Size each for one agent session.

## Agent Brief bar

An implementer must not need to ask:

- What user-visible behavior changes  
- Which docs/specs bind  
- How to verify (`cargo test`, CLI invocation, example path)  
- Out of scope for this ticket  

## When design is foggy

File `ready-for-human` or `wayfinder-grilling` instead of `ready-for-agent`. Do not fake certainty.

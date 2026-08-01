# Thermo-Nuclear Code Quality Review

- **Date**: 2026-08-02
- **Scope**: Current `main` tree (clean working tree; no feature branch delta). Focus: Unreleased growth — worktrees, agent packs, generate, status/doctor observation, CLI presentation, membership path safety.
- **Bar**: Structural quality, not “does it work.” Behavior-preserving restructures preferred. Approval is blocked by clear structural regressions and missed code-judo when a simpler model is visible.
- **Verdict**: **Do not approve as-is for long-term maintainability.** Product contracts are strong; sampling and CLI presentation were extended by bolt-on rather than by deepening existing seams. Highest leverage is inventory/sample unification + CLI `Present`/`Ctx` spine — not more local polish.

---

## Executive summary

Recent landing is **behaviorally disciplined** (soft-fail empties, not-fixable doctor warns, shared orphan definition, dry-run parity, path-token uniqueness, JSON exit matrices). The structural disease is different:

1. **Observation stopped being the single sample plane.** Primaries are observed once; worktrees and packs are re-sampled in status, doctor (twice), and CLI `project_info`.
2. **`main.rs` is still a god dispatcher** (~807 LOC, ~38 dual-mode print branches) while `commands/` mostly holds DTOs/formatters — half-migrated issues-38.
3. **`membership.rs` sits at 993 LOC** — one more feature push crosses the 1k target without a split.
4. **Triplicated pack-missing rule** and **twin `copy_tree` engines** are pure duplicated code shapes waiting to diverge.

None of this is “it doesn’t work.” It is **incidental complexity that will tax every subsequent verb**.

---

## Priority 1 — Structural regressions

### 1. Observation half-layer: worktrees & packs bolted on after the fact

- **Smell**: Shotgun Surgery + Divergent Change on “what is true about the workspace right now.”
- **Evidence**: `crates/odm-core/src/status.rs:86–113` re-lists worktrees and packs after `observe_workspace`. `status_from_observation` is documented incomplete (`status.rs:117–120`). Same attach loop cloned in `crates/odm/src/commands/project.rs:146–156`. Doctor pays **two** full `worktree_list` passes (`doctor.rs:106–108` → `doctor_worktree.rs` orphan loop + dirty loop).
- **Why it fails the bar**: New surfaces fork sampling instead of extending the canonical sample. Soft-fail policy, dirty probes, and orphan definition will keep drifting.
- **Code judo**: One inventory API, e.g. `observe_project_worktrees` + `observe_packs` (or extend `WorkspaceObservation`). Status, doctor, prune-name-set, and `project_info` all consume it. Doctor maps one sample → orphan warns + dirty warns. **Deletes** dual doctor loops, CLI clone, and the “pure projection then mutate” lie.
- **Bonus**: Prune currently calls full `worktree_list` (dirty probes on every slot) only to take names (`worktree.rs` prune path). Inventory should offer **names-only** when dirty is unused.

### 2. CLI presentation monoculture in `main.rs`

- **Smell**: Duplicated Code (presentation ritual), Middle Man (`commands/` that don’t command), Mysterious Name (`commands` ≠ handlers).
- **Evidence**: `crates/odm/src/main.rs` (~807 LOC) owns discover/load/Git/print/exit per arm. ~38 `if out.json` branches. Success contracts split: typed DTOs in `commands/*` vs ad-hoc `serde_json::json!` still in main (sync, pin, init, many progen store arms). `commands/status.rs` is a pure alias of `build_status` (12 LOC). Human project list bypasses DTO and re-joins config ⨝ snap (`main.rs` project list arm + `project.rs` human formatter).
- **Why it fails the bar**: Every new verb multiplies dual-mode ritual. File is under 1k but **complexity-dense** — next 2–3 features hit the target with zero architecture change. Lib doc claims “thin adapter”; reality is a command runtime.
- **Code judo**:
  - `Ctx::open` once (root, ws, git, globals).
  - `Present` + `finish(out, value)` — human + JSON + exit_code on the value.
  - Family `dispatch(&Ctx, Cmd)` — handlers leave main.
  - Humans format **DTOs only** (delete dual joins).
  - Delete pure aliases (`status_snapshot`, free `pack_entry_dto` over `From`).
- **Target**: `main.rs` ~80–120 LOC. Do **not** churn error envelope, clap schema, or domain crate calls while doing this.

### 3. `membership.rs` at the 1k wall (993 LOC)

- **Smell**: Divergent Change — add/rm, path remap, delete-with-restore, gitignore/pin hooks, `project_git`, CLI `path_buf_to_rel`, and ~600 LOC tests in one file.
- **Evidence**: `crates/odm-core/src/membership.rs` total 993; AGENTS.md target ≤1000, hard 1250.
- **Code judo**: Split now, before the next membership feature:
  - `membership.rs` — add/rm only
  - `project_git.rs` — passthrough + pin maintain
  - CLI path helper → bin or `paths`
  - tests → `membership_tests.rs` + shared `ScriptedRunner` testsupport
- **Bar**: Do not land another feature that grows this file past 1000 without the split.

---

## Priority 2 — Missed code-judo / duplicated concepts

### 4. Pack “missing” rule triplicated

- **Smell**: Duplicated Code + Primitive Obsession (raw `symlink_metadata` as domain rule).
- **Sites**:
  - `status.rs:109` — `missing: e.path.symlink_metadata().is_err()`
  - `doctor_pack.rs` — skip when `symlink_metadata().is_ok()`
  - `odm/src/commands/agent_pack.rs` — DTO `missing` probe
- **Judo**: `PackEntry::is_missing()` (or `PackObs { entry, missing }` from inventory). One rule, three call sites become one method. Docs already promise one rule; types should own it.

### 5. Twin filesystem engines: generate vs agent_pack

- **Smell**: Duplicated Code.
- **Evidence**: `generate.rs` `copy_tree` / type-conflict remove / symlink copy; `agent_pack.rs` parallel `copy_tree` + nested `prepare_dest_for_install` / `prepare_dest_for_link` (highest cyclomatic density in recent growth).
- **Judo**: `odm-core` fsutil — `copy_tree`, `is_dir_empty`, `remove_path`, dest-prep skeleton. Generate and packs become thin policies over shared FS. Collapse install/link prep to one “exists × force × terminal action” table.

### 6. Path-policy errors remapped by English `contains`

- **Smell**: Primitive Obsession / brittle contract.
- **Evidence**: `membership.rs` and `config.rs` (and actions cwd) map `resolve_under_root` failures via `msg.contains("relative")` vs escape wording.
- **Judo**: Typed `PathResolveError { Absolute, Escape }` (or structured `OdmError` detail). Match enums. Message edits must not change exit codes.

### 7. Prune rows abuse `WorktreeSlotInfo { dirty: None }`

- **Smell**: Type lie after JSON already dropped `dirty` on prune rows.
- **Evidence**: Core prune still builds `WorktreeSlotInfo` with `dirty: None`.
- **Judo**: `WorktreeNamePath { name, path }` for prune outcomes. SlotInfo keeps ternary dirty only for list/status/info.

### 8. CLI twinning: project ↔ progen membership envelopes

- **Smell**: Duplicated Code at presentation layer (domain split is correct).
- **Evidence**: Parallel add/rm JSON `json!`, list human formatters, materialize labels already unified in `materialize.rs` but add/rm envelopes did not follow.
- **Judo**: Shared `NamedOk` / `NamedMaterialize` DTOs; one entity-list human template.

---

## Priority 3 — Spaghetti / branching / special cases

### 9. Doctor double-list encoded into tests

- Dirty path comments “no second `is_clean`” within one list call, but doctor still runs two list+probe passes. Tests that script two porcelain queues **lock in** the waste. Fix the structure; then simplify test scripts.

### 10. `--wt` truth lives as an argv scan in the dispatcher

- Clap global `cli.wt` is help-oriented; execution re-parses argv (`main.rs` `collect_wt_from_argv` / `resolve_wt_flags`). Workaround is understandable; **home is wrong**.
- **Judo**: Own collection next to `cli.rs` / flags; resolve once into `Ctx.wt`. One source of truth. Do not add a third path.

### 11. Soft-fail swallows all `Err(_)` into empty

- Product spirit is right (empty arrays, soft observation). Policy is copy-pasted. Centralize `soft(obs) -> T` so “none” vs “could not sample” stays one decision (optional later: surface soft warnings).

### 12. Library crates still speak CLI flag names

- `odm-actions` `CwdTarget::from_flags` messages mention `--wt` / `--project`.
- `odm-progen` `one_progen_flag` / `reindex_for_cli` — CLI cardinality in the store façade.
- Prefer domain errors (“worktree requires project”) mapped at the bin edge — or accept flag names only at a thin CLI helper module, not deep in store.

---

## Priority 4 — Boundaries, types, file size

### File sizes (signal)

- **membership.rs**: 993 — **split before grow**
- **main.rs**: 807 — **complexity wall, not LOC wall yet**
- **agent_pack.rs**: 781 — extract FS + registry when touching packs
- **generate.rs**: 758 — prod lean; tests heavy (OK)
- **config.rs**: 742 — OK; escape remap cleanup when touching
- **doctor_worktree.rs**: 600 total / ~80 prod — **tests dominate**; externalize + shrink after inventory judo
- **No hard-limit (1250) breach today.** membership is the only near-target emergency.

### Boundary notes

- **Keep**: core/git/actions/progen crate split; progen does not leak into membership vault scaffold; doctor ODM-side vs `doctor_progens`; generate remote deferred cleanly; `CwdTarget` semantics in actions; orphan definition shared at disk helpers (`orphan_slot_names` / `worktree_orphan_infos`).
- **Migrate over time**: `format_status_human` / `format_doctor_human` out of core toward bin DTOs (don’t big-bang).
- **Question**: `exit_code` living in core while comments say bin owns exit — works and is tested; pick one story and document it.

### Thin wrappers that do not earn keep

- `commands/status.rs` alias
- Free `pack_entry_dto` over existing `From`
- Field-copy worktree DTOs when core outcomes could serialize (keep DTOs that change policy: `ActionRunDto` streams, pack `missing`, soft-fail info)

---

## What is healthy — do not churn

- Path token policy centralized (`paths::parse_path_token`) for entity names and slots; membership escape rejects before config write.
- Orphan definition shared for prune/doctor/status at the disk helper layer — **finish** the job for registered+dirty+packs sampling.
- Pin state classifier pure and single-sourced; doctor entity path checks consume observation.
- Doctor fix allowlist discipline (`fixable: false` for orphan/dirty/pack_missing) with tests that `--fix` does not delete/clean/edit registry.
- Generate dry-run: same validation path, count mirrors copy, strong tests.
- Error taxonomy (`usage` / `workspace` / `operation` / `not_found`) + stable `code()`; JSON error envelope in `output.rs`.
- `run_context_prompt` already shares context ≡ agent prompt — pattern to copy.
- `materialize.rs` single label map — issues-38 win; finish the same for remaining success payloads.
- Integration tests as JSON/exit contract locks — make structural refactors safe.
- Agent pack registry atomic write + dangling-symlink-not-missing correctly specified.
- Actions cwd priority explicit and tested; task `dir` escape at resolve time.

---

## Preferred remedy sequence (ambitious, ordered)

1. **Inventory sample API** — `observe_project_worktrees` (+ names-only) and pack observation with `is_missing()`. Rewrite status attach, doctor worktree/pack checks, `project_info`. Delete dual loops and triplication.
2. **CLI spine** — `Ctx` + `Present`/`finish` + family dispatch; DTO-only humans; kill success `json!` in main; delete `status.rs` alias.
3. **Split membership** off the 1k cliff; shared `ScriptedRunner` testsupport.
4. **fsutil** shared `copy_tree` / dest prep; collapse pack install/link ladders.
5. **Typed path resolve errors** — delete `contains("relative")`.
6. **Prune types** — `WorktreeNamePath`; no dirty probes on prune/orphan-only paths.
7. **Opportunistic**: move remaining human formatters toward bin; pull CLI flag strings out of deep library APIs when touched.

Do not: rename JSON fields during spine work; redesign domain crates; extract vanity modules that don’t delete concepts.

---

## Approval checklist

| Criterion | Status |
|-----------|--------|
| No clear structural regression | **FAIL** — observation half-layer; CLI god-dispatch |
| No obvious missed dramatic simplification | **FAIL** — inventory + Present/Ctx are visible and high leverage |
| No unjustified file-size explosion | **WARN** — membership 993; main complexity-dense |
| No spaghetti-growth from special-case branching | **WARN** — bolt-on status/doctor/info; doctor double-list |
| No hacky/magical abstraction | **PASS** with notes — stringly path remap is brittle, not magical |
| No unnecessary wrapper/cast churn | **WARN** — half-migrated DTO layer + thin aliases |
| No architecture-boundary leak / helper duplication | **FAIL** — pack missing ×3; copy_tree ×2; sampling ×N |
| No missed obvious decomposition | **FAIL** — membership split overdue |

**Approval: blocked** until at least items **1–3** of the remedy sequence have a committed path (landed or explicitly scheduled with ownership). Local UX fixes and more integration tests alone do not clear this bar.

---

## Fowler smell index (this review)

- **Duplicated Code**: pack missing ×3; copy_tree ×2; status/project_info attach; doctor dual list; CLI dual print ritual; project/progen envelopes
- **Shotgun Surgery**: “add workspace fact” touches status + doctor_* + CLI info + sometimes prune
- **Divergent Change**: membership.rs; main.rs match arms
- **Middle Man**: commands modules that only map DTOs while main still orchestrates
- **Primitive Obsession**: path errors as English strings; dirty stuffed into prune rows
- **Speculative Generality**: not the main problem — under-extraction is
- **Feature Envy**: CLI project_info envies core worktree sampling

---

## Scope note

Working tree was clean on `main` (`3ed674a`). This is a **codebase health audit of current Unreleased implementation**, not a PR diff review. Re-run against a feature branch with `git diff main...HEAD` when reviewing a specific change set; the structural bar and remedy sequence still apply.

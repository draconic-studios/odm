# Issues

Live issues in this folder. Closed/wontfix notes go under `closed/`.

See `docs/agents/issue-tracker.md`.

## Maps

- [[issues-119-swarm-audit-hardening-map]] — Swarm audit bugs/improvements (path safety, find, CLI honesty)
- [[issues-120-test-coverage-map]] — Behavior-seam test suite toward full coverage
- [[issues-121-full-capability-demo-map]] — core-desk full ODM capability demo + verification

## Frontier (open, unblocked)

### Pre-existing

- [[issues-118-website-playwright-review-improve]] — website Playwright review/improve (`ready-for-agent`; unblocked by 117)

### Swarm audit — critical / high (prefer first)

- [[issues-122-find-snippet-unicode-panic]] — find snippet UTF-8 panic (`ready-for-agent`)
- [[issues-123-fts-query-escaping]] — FTS safe queries (`ready-for-agent`)
- [[issues-124-bundle-path-escape]] — action/generator bundle path escape (`ready-for-agent`)
- [[issues-125-membership-path-escape]] — membership add path escape (`ready-for-agent`)
- [[issues-126-action-dir-escape]] — action task dir escape (`ready-for-agent`)
- [[issues-127-wt-missing-exit-code]] — missing wt/path exit 4 (`ready-for-agent`)
- [[issues-128-run-json-stdio]] — run --json stdout/stderr (`ready-for-agent`)
- [[issues-129-clap-usage-exit-json]] — clap exit 1 + JSON (`ready-for-agent`)
- [[issues-131-progen-index-freshness-dup-ids]] — index stale + dup ids (`ready-for-agent`)

### Swarm audit — medium

- [[issues-130-entity-name-uniqueness]] — unique path-safe entity names (`ready-for-agent`)
- [[issues-132-cli-json-ux-hardening]] — prune JSON / progen messages / dual wt (`ready-for-agent`)
- [[issues-133-git-noninteractive]] — git no auth hang (`ready-for-agent`)
- [[issues-134-wikilink-fence-frontmatter]] — fences + bad FM (`ready-for-agent`)
- [[issues-135-generate-force-type-conflict]] — generate force type clash (`ready-for-agent`)
- [[issues-136-docs-honesty-release-policy]] — Releases / AGENTS / progen.md (`ready-for-agent`)

### Test coverage (unblocked)

- [[issues-137-coverage-tooling]] — local llvm-cov script (`ready-for-agent`)
- [[issues-138-error-io-exit-unit-matrix]] — error/io unit tests (`ready-for-agent`)
- [[issues-139-cli-pin-sync-rm-integration]] — pin/sync/rm CLI tests (`ready-for-agent`)
- [[issues-141-progen-unit-edges-ops]] — progen ops/unit edges (`ready-for-agent`)
- [[issues-142-progen-group-cli-integration]] — --progen-group CLI (`ready-for-agent`)
- [[issues-143-odm-git-worktree-real-git]] — real-git worktree test (`ready-for-agent`)
- [[issues-144-core-desk-assets-full-surface]] — core-desk asset expand (`ready-for-agent`)

## Blocked

### Coverage / demo chain

- [[issues-140-cli-exit-code-matrix]] — prefer after 127+129
- [[issues-145-core-desk-dogfood-script]] — blocked by 144
- [[issues-146-core-desk-full-tour-gate]] — blocked by 144
- [[issues-147-demo-gap-followups]] — blocked by 145+146

## Architecture deepen (post-0.1.0)

Remaining from architecture review 2026-08-01; tagged `ready-for-agent` with Agent Briefs. Work frontier first.

_(none remaining)_

## Closed maps (delivery spine)

- [[issues-1-odm-design-docs-map]] — Design package (phase 1)
- [[issues-14-implement-core-map]] — Implement core (phase 2)
- [[issues-25-progen-integration-map]] — Progen integration (phase 3)
- [[issues-26-actions-map]] — Actions (phase 4)
- [[issues-27-ship-map]] — Ship (phase 5)
- [[issues-40-worktree-slots-map]] — Worktree slots (post-0.1.0)
- [[issues-45-generators-map]] — Generators local template v1 (post-0.1.0)
- [[issues-50-agent-packs-map]] — Agent packs local install/link/list (post-0.1.0)
- [[issues-55-post-v1-hardening-map]] — Post-v1 hardening + agent prompt thin
- [[issues-60-post-v1-polish-map]] — Post-v1 polish (docs honesty, doctor split, find limit, status slots)
- [[issues-66-post-v1-dogfood-slot-depth-map]] — Post-v1 dogfood + worktree slot depth (CHANGELOG, core-desk, info slots, dirty doctor, orphan prune)
- [[issues-72-post-v1-honesty-dogfood-map]] — Post-v1 honesty + dogfood after slot depth (phased-delivery/CHANGELOG, core-desk prune/dirty, clippy, README)
- [[issues-77-post-v1-worktree-observation-map]] — Post-v1 worktree multi-prune + slot dirty observation
- [[issues-82-post-v1-pack-lifecycle-hardening-map]] — Post-v1 pack rm + doctor pack_missing + worktree module split
- [[issues-89-post-v1-status-packs-map]] — Post-v1 status packs + pack observation dogfood
- [[issues-94-post-v1-status-orphans-map]] — Post-v1 status/info worktree orphans + dogfood
- [[issues-99-post-v1-generate-dry-run-map]] — Post-v1 generate `--dry-run` (local template preview)
- [[issues-104-post-v1-pack-list-missing-map]] — Post-v1 pack list `missing` observation (status/doctor parity)
- [[issues-109-project-website-github-pages-map]] — Project website (`website/` on main, Actions Pages)

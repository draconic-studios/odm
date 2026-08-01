---
id: issues-119
title: "Swarm audit hardening map"
description: "Bugs and improvements found by agent swarm walkover 2026-08-01; path safety, progen find crashes, CLI exit/JSON honesty."
status: closed
issue-type: observation
severity: high
tags:
  - planning
  - issue
  - wayfinder-map
---

# Swarm audit hardening map

## Destination

Ship-safe ODM after swarm audit: no process panics on normal vault content, path policy consistent across config/actions/membership, CLI exit codes and JSON shapes match `docs/reference/cli.md`, and obvious agent footguns closed.

## Notes

- Swarm: up to 10 explore agents across odm-core, CLI, git/actions/progen, tests, core-desk, docs.
- Do **not** refile pack-list `missing` (104–108) or website Playwright (116–118).
- Prefer TDD; one ticket = one AFK session where possible.

## Decisions so far

- File critical/high first; medium UX/docs as separate tickets under this map.
- Path-escape family split into focused tickets (bundles vs membership vs action dir) so reviews stay small.

## Fog / tickets

### Critical / high

- [[issues-122-find-snippet-unicode-panic]]
- [[issues-123-fts-query-escaping]]
- [[issues-124-bundle-path-escape]]
- [[issues-125-membership-path-escape]]
- [[issues-126-action-dir-escape]]
- [[issues-127-wt-missing-exit-code]]
- [[issues-128-run-json-stdio]]
- [[issues-129-clap-usage-exit-json]]
- [[issues-130-entity-name-uniqueness]]
- [[issues-131-progen-index-freshness-dup-ids]]

### Medium

- [[issues-132-cli-json-ux-hardening]]
- [[issues-133-git-noninteractive]]
- [[issues-134-wikilink-fence-frontmatter]]
- [[issues-135-generate-force-type-conflict]]
- [[issues-136-docs-honesty-release-policy]]

## Related maps

- [[issues-120-test-coverage-map]] — coverage suite for regressions
- [[issues-121-full-capability-demo-map]] — dogfood that surfaces remaining gaps

## Answer

Destination met 2026-08-02. All fog tickets 122–136 closed under `issues/closed/`. Verified children present closed before map close.

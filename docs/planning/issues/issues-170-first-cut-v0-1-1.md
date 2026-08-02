---
id: issues-170
title: "First cut v0.1.1 (prepare; human-gated publish)"
description: "Bump crate version to 0.1.1, prepare CHANGELOG notes for first multi-platform binary release. Tag and gh release publish are ready-for-human."
status: open
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
  - release
  - ready-for-agent
---

# First cut v0.1.1 (prepare; human-gated publish)

## Description

Prepare the first multi-platform binary release **`v0.1.1`**: version bump + CHANGELOG section from Unreleased. **Do not retag `v0.1.0`.** Tag creation and GitHub Release publish are **human-gated**.

## Affected

- Workspace / crate `Cargo.toml` version fields (and lockfile if required)
- `CHANGELOG.md` — cut `0.1.1` section from Unreleased; note multi-platform assets + install path
- Release notes body (inline in CHANGELOG or draft for `gh release`)

## Impact

Without a versioned cut, the release workflow has nothing coherent to tag; install docs cannot point at a real multi-platform release.

## Proposed Fix

See Agent Brief.

## Blocked by

Soft dependencies (agent may prepare version/CHANGELOG before workflow lands; **must not tag/publish** until workflow + assets path exist):

- [[issues-167-multi-target-release-workflow]] — needed before human publish produces assets
- [[issues-166-release-ci-policy]] — policy before merge of workflow

Install script + docs may land in parallel ([[issues-168-install-sh]], [[issues-169-install-docs-website]]).

## Agent Brief

**Category:** chore  
**Summary:** Prepare `0.1.1` version + CHANGELOG. Leave tag/`gh release` to a human (`ready-for-human` publish section).

**Bindings:**

- Parent map: [[issues-165-multiplatform-github-releases-curl-install]] Decisions
- Overrides rolling Unreleased ([[issues-157-release-hygiene-rolling-unreleased]]) **only** for this binary release cut
- Four triples + SHA256 assets once workflow runs on tag `v0.1.1`
- Install default `~/.local/bin`; curl via `scripts/install.sh`

**Map Decisions (v1 lock — do not reopen):**

- **Version:** `0.1.1` / tag `v0.1.1` — do **not** retag `v0.1.0`
- **Agent does:** Cargo version bump(s), CHANGELOG prepare, any in-repo release notes draft
- **Human does:** `git tag v0.1.1`, push tag, confirm GitHub Release + four tarballs + SHA256, smoke `curl|sh` / `odm --version`
- **Publish gate:** default human-gated even if credentials exist — do not agent-push release tags unless maintainer explicitly overrides in-session

**Desired behavior:**

### Agent-ready (this ticket's implementable part)

1. Bump crate/workspace version to `0.1.1` consistently.
2. Move/prepare `CHANGELOG.md` section for `0.1.1` describing first multi-platform GitHub Release assets (four triples), SHA256, and curl install path; leave appropriate Unreleased remainder if any.
3. Ensure version strings referenced for release packaging stay consistent with tag `v0.1.1`.
4. Commit preparation only — **no tag, no `gh release create`, no force-publish**.

### Human-gated publish (do not automate)

After workflow + install path are ready:

1. Review CHANGELOG + version commit(s).
2. Tag `v0.1.1` and push; confirm Actions release workflow succeeds.
3. Verify Release assets: four `odm-0.1.1-<triple>.tar.gz` + SHA256.
4. Smoke install.sh and `odm --version`.
5. Close this ticket / parent map Answer when publish is confirmed.

**Acceptance criteria:**

- [ ] Workspace/crate version is `0.1.1`
- [ ] CHANGELOG has a `0.1.1` section prepared for multi-platform release + install
- [ ] No `v0.1.0` retag
- [ ] Agent session does **not** push release tag or publish without explicit human gate
- [ ] Publish checklist left clear for human (below or Comments)
- [ ] Human publish (later): tag + Release assets verified — then close

**Out of scope:**

- Implementing the workflow or install.sh (other children)
- Windows/musl/signing
- crates.io publish
- Agent-initiated tag push by default

## Human publish checklist

- [ ] Policy + release workflow merged
- [ ] install.sh + docs at least usable
- [ ] Version/CHANGELOG commit on main
- [ ] `git tag v0.1.1 && git push origin v0.1.1`
- [ ] Actions release job green; four tarballs + SHA256 on Release
- [ ] `curl … | sh` smoke + `odm --version`

## Comments

Minted from [[issues-165-multiplatform-github-releases-curl-install]] 2026-08-02.

> Publish steps are `ready-for-human`. Agent prepares tree only.

---
id: "0003"
title: "Meta-Repository Purpose"
status: "Accepted"
date: "2026-04-06"
affects_repos: ["provena-workspace"]
---

# ADR 0003: Meta-Repository Purpose

## Context

Local development tooling and workspace state need a home that does not distort the code architecture. At the same time, all architectural decisions that need to survive must live in `provena` - not in a working repo.

## Decision

`provena-workspace` is a private ephemeral working repository. It is not a Rust crate and must not become a dumping ground for shared code or architectural decisions.

It may contain:

- local VS Code workspace files
- developer setup notes
- local orchestration scripts
- scratch notes and working context

It must not contain:

- shared Rust code
- public platform contracts that belong in `provena`
- commercial product implementation that belongs in `provenancai`
- ADRs or architectural decisions - those belong in `provena`

## Consequences

- Repository boundaries remain explicit.
- `provena` remains the single authoritative source for all architectural decisions.
- `provena-workspace` can be discarded or recreated without loss of anything important.

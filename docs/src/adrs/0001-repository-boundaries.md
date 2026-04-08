---
id: "0001"
title: "Repository Boundaries"
status: "Accepted"
date: "2026-04-06"
affects_repos: ["provena", "provenancai", "provena-workspace"]
---

# ADR 0001: Repository Boundaries

## Context

The Provena ecosystem needs a clean separation between the public open source platform and the private commercial product built on top of it.

## Decision

- `provena` is the public OSS repository. It is self-contained and authoritative - any architectural decision affecting the kernel belongs here.
- `provenancai` is a private repository for commercial product code built on top of Provena.
- `provena-workspace` is a private ephemeral working repo for local workspace state and developer context. It is not authoritative and nothing in it needs to survive independently.

## Consequences

- The public repository stays focused, publishable, and self-explanatory to any external contributor.
- Private product logic is not mixed into the OSS platform repository.
- `provena-workspace` is treated as disposable - cross-repo architectural decisions that need to survive belong in `provena`, not the workspace.

# ADR 0003: Meta-Repository Purpose

## Status
Accepted

## Context
Cross-repo decisions, workspace state, and local development tooling need a durable home that does not distort the code architecture.

## Decision
Use `provena-workspace` as a private meta-repository. It is not a Rust crate and must not become a dumping ground for shared code.

It may contain:
- ADRs spanning multiple repositories
- local VS Code workspace files
- developer setup notes
- local orchestration scripts
- release coordination notes

It must not contain:
- shared Rust code unless a deliberate new artifact is being created
- public platform contracts that belong in `provena`
- commercial product implementation that belongs in `provenancai`

## Consequences
- Workspace context is preserved without polluting the public repo.
- The name does not imply a reusable code dependency.
- Repository boundaries remain explicit.

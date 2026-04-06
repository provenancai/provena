# ADR 0001: Repository Boundaries

## Status
Accepted

## Context
The Provena ecosystem needs a clean separation between the public open source platform and the private commercial product built on top of it.

## Decision
- `provena` is the public OSS repository.
- `provena-examples` is a separate public repository for examples and reference integrations.
- `provenancai` is a private repository for commercial product code.
- `provena-workspace` is a private meta-repository for local workspace state and cross-repo context.

## Consequences
- The public repository stays focused and publishable.
- Private product logic is not mixed into the OSS platform repository.
- Cross-repo context survives independently of chat history or editor state.

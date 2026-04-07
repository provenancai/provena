<!-- TODO: Add clear "What is Provena?" thesis statement to landing page -->
<!-- TODO: Add problem statement / motivation section — why provenance matters -->
<!-- TODO: Add high-level architecture diagram -->
<!-- TODO: Document core kernel primitives — Artifact, Capability, Ledger Event, Derivation, Authorization -->
<!-- TODO: Improve ADR discoverability — index page, summaries, links from concept pages -->
<!-- TODO: Add conceptual artifact derivation flow example -->
<!-- TODO: Clarify kernel vs product boundary — keep minimal, OSS-appropriate framing -->
<!-- TODO: Verify docs/book is in .gitignore -->

# Provena

Provena is an open source provenance microkernel written in Rust.

It provides a capability-based plugin registry and router, an append-only provenance ledger, and a plugin SDK for building first- and third-party extensions. The REST API is the universal interface surface. The CLI is `prv`.

## Relationship to ProvenancAI

Provena is the engine. ProvenancAI is the commercial platform built on top of it — the same relationship as Git to GitHub. This repository contains only the public platform surface: domain types, the plugin SDK, the kernel, ledger interfaces, and the API entrypoint. Commercial product logic, policy engines, and enterprise integrations live in separate private repositories and are never required to run or extend Provena.

## Quick start

```bash
cargo run -p prv
```

The minimal public API starts on `127.0.0.1:3000` with a `/health` endpoint.

## What is in this book

- **Architecture** — how the system is structured, how capabilities and plugins work, how storage migrations are handled, and how the ledger drives authorization
- **ADRs** — the architectural decisions that shaped the design
- **Contributing** — how to contribute to Provena

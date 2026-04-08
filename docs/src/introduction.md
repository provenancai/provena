<!-- TODO: Add high-level architecture diagram -->
<!-- TODO: Document core kernel primitives - Artifact, Capability, Ledger Event, Derivation, Authorization -->
<!-- TODO: Improve ADR discoverability - index page, summaries, links from concept pages -->
<!-- TODO: Add conceptual artifact derivation flow example -->
<!-- TODO: Verify docs/book is in .gitignore -->

# What is Provena?

Provena is a provenance engine for business artifacts - invoices, emails, contracts, approvals, reports, and operational records.

These artifacts already exist in every organisation. The problem is that they are scattered across systems, disconnected from each other, and difficult to trace. When something needs to be verified - a financial number, a decision, a compliance record - the evidence has to be reconstructed manually, from memory and fragments.

Provena links those artifacts into verifiable chains of evidence: a connected graph where every node is a real business record and every edge is a deterministic, auditable relationship.

## The problem

Organisations generate enormous volumes of artifacts, but those artifacts are:

- scattered across storage systems with no shared model
- disconnected from the context that produced them
- difficult to trace back to their sources
- impossible to verify without manual reconstruction

This produces predictable failures: unclear decision lineage, financial numbers that cannot be traced, compliance gaps, and the inability to reconstruct what happened and why.

## The conceptual model

Provena treats every artifact as a node in a provenance graph. Edges between nodes represent derived relationships - invoices derived from timesheets, reports derived from datasets, approvals derived from communications, payments derived from invoices.

This allows an organisation to answer questions that are currently unanswerable:

- Where did this number come from?
- Who approved this decision, and what information did they have?
- What source material was used to produce this report?

Think of it as Git for operational business records - the same append-only, integrity-verified, fully traceable model, applied to the artifacts an organisation actually runs on.

---

The Provena kernel defines the protocol primitives required to represent these relationships in a deterministic and verifiable way.

## Provena

Provena is an open source provenance microkernel written in Rust.

It provides a capability-based plugin registry and router, an append-only provenance ledger, and a plugin SDK for building first- and third-party extensions. The REST API is the universal interface surface. The CLI is `prv`.

## Relationship to ProvenancAI

Provena is the engine. ProvenancAI is the commercial platform built on top of it - the same relationship as Git to GitHub. This repository contains only the public platform surface: domain types, the plugin SDK, the kernel, ledger interfaces, and the API entrypoint. Commercial product logic, policy engines, and enterprise integrations live in separate private repositories and are never required to run or extend Provena.

## Quick start

```bash
cargo run -p prv
```

The minimal public API starts on `127.0.0.1:3000` with a `/health` endpoint.

## What is in this book

- **Architecture** - how the system is structured, how capabilities and plugins work, how storage migrations are handled, and how the ledger drives authorization
- **ADRs** - the architectural decisions that shaped the design
- **Contributing** - how to contribute to Provena

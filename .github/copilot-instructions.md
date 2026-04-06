# Provena — GitHub Copilot Instructions

Provena is an open source provenance microkernel written in Rust. CLI: `prv`. Commercial platform built on top: ProvenancAI (`pai`).

## What this codebase is

A capability-based plugin microkernel with an append-only provenance ledger. Plugins are out-of-process HTTP services. The kernel routes requests by capability, not by plugin name.

## Rust conventions

- Edition 2021
- Async: Tokio
- HTTP: Axum
- Database: sqlx with PostgreSQL (compile-time checked queries only)
- Errors: `thiserror` in library crates, `anyhow` in binaries
- Serialization: serde + serde_json
- **No `unwrap()` or `expect()` in library crates** — handle all errors explicitly
- Newtypes for all domain identifiers — never pass raw strings or UUIDs as IDs
- Static binaries only — musl target for release builds

## Architecture rules Copilot must respect

- The kernel contains zero business logic — only registry, routing, and health
- Plugins are out-of-process — never in-process dynamic libs
- Capability names encode semantic role — `storage.azure.blob.sensitive` is not interchangeable with `storage.azure.blob.standard`
- Singleton capabilities have no fallback — connectivity loss is a hard 503
- The ledger is append-only — never suggest UPDATE or DELETE on ledger tables
- Every ledger entry must carry a provenance class: Deterministic, Machine, or Human
- Human provenance class events require explicit named authorization — never trigger automatically

## Crate layout

```
provena-core    # Domain types, no IO
provena-sdk     # Plugin traits and manifest
provena-kernel  # Registry, router, health monitor
provena-ledger  # Append-only ledger (PostgreSQL)
provena-api     # Axum REST surface
prv             # CLI binary
```

## Suggestions Copilot should avoid

- Do not suggest `unwrap()`, `expect()`, or `todo!()` in library code
- Do not suggest in-process plugin loading
- Do not suggest fallback routing between storage backends
- Do not suggest mutable ledger operations
- Do not suggest stringly-typed identifiers where newtypes exist
# Crates

The workspace is split into focused crates with strict dependency ordering. No crate depends on a crate below it in the hierarchy.

```text
provena/
├── provena-core/      # Domain identifiers and provenance types - no IO, no async
├── provena-sdk/       # Plugin manifest and trait surface
├── provena-kernel/    # Capability registry, routing, and health
├── provena-ledger/    # Append-only ledger interfaces and in-memory scaffold
├── provena-api/       # Axum router for the OSS surface
└── prv/               # CLI entrypoint
```

## provena-core

Domain newtypes only. `PluginId`, `CapabilityName`, `ProvenanceClass`, `CoreError`. No async, no IO. This crate is a dependency of everything else; keep it small.

## provena-sdk

The plugin contract surface. Defines the `Plugin` trait, `PluginManifest`, and `CapabilityDescriptor`. Third-party plugin authors depend on this crate and nothing else from the kernel.

## provena-kernel

The microkernel. Owns the capability registry, priority-ordered routing, and plugin health tracking. Does not contain business logic. Does not know what capabilities mean - only that they exist and where they route.

## provena-ledger

The append-only ledger. Defines `LedgerStore` as a trait, `LedgerEntry`, and `LedgerEntryId`. The default implementation is an in-memory store. PostgreSQL-backed persistence is provided via a separate implementation using `sqlx`.

## provena-api

Axum HTTP router. Exposes the kernel and ledger over REST. This is the universal interface surface for all clients.

## prv

CLI binary. Wires the kernel and API together and provides developer tooling.

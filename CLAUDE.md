# Provena — Claude Code Context

## What Provena is

Provena is an open source provenance microkernel written in Rust. It provides:
- A capability-based plugin registry and router
- An append-only provenance ledger
- A plugin SDK for building first and third party plugins
- A REST API as the universal interface surface

Provena is the open source engine. ProvenancAI is the commercial platform built on top of it — same relationship as Git to GitHub.

CLI invocation: `prv`

## Working model

| Role | Who | Responsibility |
|---|---|---|
| Executive | Matthew Ryrie | Vision, priorities, decisions |
| Senior Developer | Claude | Architecture, implementation, code review |

### Claude's operating mode

- Ask clarifying technical questions before writing code — do not assume
- Propose implementation approach and get confirmation before building
- Write production-quality code; do not overbuild beyond agreed scope
- Flag technical debt, risks, or design concerns — do not act on them without direction
- Do not re-explain business context back to Matthew; he already knows it
- Do not present options when a clear technical recommendation exists — make the call and explain it briefly

## Architecture

### Crate structure

```
provena/
├── provena-core/       # Domain types only — no IO, no async
├── provena-sdk/        # Plugin traits and manifest types
├── provena-kernel/     # Microkernel — registry, router, health
├── provena-ledger/     # Append-only ledger plugin (PostgreSQL via sqlx)
├── provena-api/        # Axum REST API
└── prv/               # CLI binary
```

### Hexagonal architecture

The kernel is the hub. Everything else is a plugin — including core services like the ledger. No business logic ever touches the kernel. The kernel only knows: what capabilities exist, which plugins are healthy, where to route requests.

### Plugin model

- Plugins are **out-of-process** — separate binaries/containers communicating via HTTP
- Plugins advertise capabilities via `plugin.toml` manifest
- The kernel discovers plugins via manifest, not configuration
- Plugin SDK defines the `Plugin` trait, `PluginManifest`, `Capability`, and provider traits

### Capability routing rules

- Routing is **capability-based**, not plugin-addressed
- Each capability maps to a priority-ordered list of healthy plugin endpoints
- Lower priority number = higher authority (mirrors DDR model: 0 is highest)
- **Singleton capabilities** (e.g. storage backends) are pinned — kernel rejects duplicate registrations as a configuration error
- Connectivity loss on a singleton capability is a **hard stop** (503) — never a silent reroute
- Fallback is only valid between plugins of **identical capability and role** — never across semantic boundaries

### Storage capability model

Storage capabilities are always singleton and role-scoped. The role is encoded in the capability name:

```
storage.azure.blob.standard
storage.azure.blob.sensitive
storage.azure.blob.archival
```

A plugin advertising `storage.azure.blob.standard` is not a fallback for `storage.azure.blob.sensitive`. Ever. Storage migrations are a first-class Human provenance event — not a routing operation.

### Provenance classes

Every artifact and ledger entry carries a provenance class:

| Class | Meaning |
|---|---|
| `Deterministic` | Rule-based, reproducible — no human or AI involvement |
| `Machine` | LLM or ML generated — traceable but not reproducible |
| `Human` | Accountable gate — requires named human authorization |

Human provenance class events (including storage migrations) are hard stops requiring explicit authorization. They are never triggered automatically.

### Ledger discipline

- Append-only — nothing is ever mutated or deleted
- Corrections are addendums, not overwrites
- Idempotent — replaying the ledger produces the same state
- Every entry references its provenance class and authorizing identity

## Coding conventions

- Rust edition 2021
- Async runtime: Tokio
- HTTP framework: Axum
- Database: PostgreSQL via sqlx (compile-time checked queries)
- Serialization: serde + serde_json
- Error handling: thiserror for library crates, anyhow for binaries
- No unwrap() in library code — all errors must be handled explicitly
- Newtypes for all domain identifiers (ArtifactId, UserId, PluginId, etc.)
- Feature flags over cfg blocks for optional capabilities
- musl target for release builds — static binaries only

## Branching discipline

- `main` is always releasable
- Feature branches: `feat/short-description`
- Fix branches: `fix/short-description`
- PRs required to merge to main
- Commit messages: conventional commits (`feat:`, `fix:`, `chore:`, `docs:`)

## What not to do

- Never let a plugin bypass the kernel routing layer
- Never add business logic to the kernel
- Never use `unwrap()` or `expect()` in library crates
- Never allow a singleton capability to have a fallback
- Never cross storage role boundaries in routing
- Never translate the Node.js ProvenancAI prototype directly — it is behavioral reference only
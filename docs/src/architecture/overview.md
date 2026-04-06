# Architecture Overview

Provena follows a hexagonal architecture with the kernel at the hub.

The kernel knows three things: what capabilities exist, which plugins are healthy, and where to route requests. No business logic ever lives in the kernel. Everything that does work — including core services like the ledger — is a plugin.

```mermaid
flowchart TD
    Client["Client (Postman / prv CLI)"] --> API[provena-api]
    API --> Kernel[provena-kernel]
    Kernel --> Ledger[Ledger plugin]
    Kernel --> StorageA["Storage plugin A — Active"]
    Kernel --> StorageB["Storage plugin B — Standby"]

    subgraph platform[Platform crates]
        SDK[provena-sdk]
        Core[provena-core]
        LedgerCrate[provena-ledger]
    end
```

## Provenance classes

Every artifact and ledger entry carries a provenance class:

| Class | Meaning |
|---|---|
| `Deterministic` | Rule-based, reproducible — no human or AI involvement |
| `Machine` | LLM or ML generated — traceable but not reproducible |
| `Human` | Accountable gate — requires named human authorization |

Human-class events are hard stops. They are never triggered automatically.

## Ledger discipline

The ledger is append-only. Nothing is ever mutated or deleted. Corrections are addendums, not overwrites. The ledger is idempotent — replaying it produces the same state. Every entry references its provenance class and the authorizing identity.

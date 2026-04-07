# Authorization Model

Provena's authorization model is built on a simple but important insight: **whether an artifact can be used as a source for downstream derivation depends on its provenance history, not just on who is asking**.

Standard access control models fall short here:

- **RBAC** (role-based) — a user has a role, a role permits an action. Too coarse. Does not capture the state of the artifact being acted on.
- **ABAC** (attribute-based) — policy evaluated against user and resource attributes. Flexible, but policy complexity grows quickly and is hard to audit in a provenance context.
- **ReBAC** (relationship-based, the Zanzibar model) — authorization derived from the relationship graph between entities, with contextual tuples that include resource state. Closest to what is needed, but still not quite right.

What Provena needs is a **provenance depth check** — not a permissions lookup, but a question answered by reading the artifact's ledger history.

## The rule

An artifact produced by a `Machine` provenance event cannot be used as a derivation source until a `Human` provenance event has been recorded against it in the ledger.

| Provenance class | Derivation eligible |
|---|---|
| `Deterministic` | Always — rule-based, fully reproducible |
| `Machine` | Only after a `Human` authorization event in the ledger |
| `Human` | Yes — the Human event is the clearance record |

This means the ledger is not just an audit trail. It is the live authorization state. A `[REVIEW]` flag on an AI-generated output is not cosmetic — it means the artifact has not yet received the `Human` provenance event that would make it eligible as a derivation source.

## How the kernel enforces this

The kernel's `activate_capability` method is the first expression of this pattern at the infrastructure layer. It requires a `LedgerEntry` carrying the `Human` provenance class before it will transition a capability from `Standby` to `Active`. The kernel will not accept any other provenance class for this operation.

The same gate applies to artifact derivation in the ProvenancAI commercial layer — before any governed output (SOW, evidence package, financial report) can be produced, the source artifacts must have sufficient provenance depth recorded in the ledger.

## Why not a separate authorization service?

A Zanzibar-compatible system (SpiceDB, OpenFGA) running alongside the kernel would introduce a second source of truth for state that is already captured in the ledger. Keeping authorization state in the ledger preserves the single append-only record, eliminates synchronization risk, and means every authorization decision is itself a traceable ledger event.

The full design rationale is documented in [ADR 0004](../adrs/0004-ledger-as-authorization-state-store.md).

---
id: "0004"
title: "Ledger as Authorization State Store"
status: "Accepted"
date: "2026-04-07"
affects_repos: ["provena", "provenancai"]
---

# ADR 0004: Ledger as Authorization State Store

## Context

ProvenancAI needs an authorization model that can answer a question more complex than standard role-based or attribute-based access control can handle:

> Can artifact Y be used as a source to derive artifact Z?

The answer depends not just on who is asking, but on the **current provenance state of the artifact itself**. An artifact that has not been sufficiently reviewed, verified, or authorized cannot be a valid derivation source — regardless of the requesting user's permissions.

Three access control models were considered:

- **RBAC (Role-Based)** — user has a role, role permits an action. Too coarse. Does not capture artifact state.
- **ABAC (Attribute-Based)** — policy evaluated against user and resource attributes. Flexible but policy complexity grows quickly and is difficult to audit.
- **ReBAC (Relationship-Based, Zanzibar model)** — authorization is derived from the relationship graph between entities. Google's Zanzibar paper extends this with contextual tuples — the authorization answer depends on the state of the resource at evaluation time.

ReBAC with contextual tuples is the closest model to what is needed, but ProvenancAI's requirements go further: the authorization predicate is not just a relationship lookup, it is a **provenance depth check** against the artifact's ledger history.

## Decision

The append-only ledger is the authorization state store. Authorization decisions that depend on artifact state are resolved by reading the artifact's ledger history, not a separate permissions table.

The three provenance classes define the authorization semantics:

| Class | Meaning | Derivation eligibility |
|---|---|---|
| `Deterministic` | Rule-based, fully reproducible | Always eligible |
| `Machine` | LLM or ML generated — traceable but not reproducible | Eligible only after a `Human` authorization event |
| `Human` | Explicit named authorization | Marks the artifact as cleared for downstream use |

An artifact produced by a `Machine` provenance event is not a valid derivation source until a `Human` provenance event has been recorded against it in the ledger. This is not a permissions check — it is a provenance depth check. The ledger entry is the proof.

This model means:

- The ledger is not just an audit trail. It is the live authorization state.
- The kernel's `activate_capability` requiring a `Human` provenance event is the first expression of this pattern at the infrastructure layer.
- The ProvenancAI commercial layer will extend this pattern to artifact derivation — before any governed output (SOW, evidence package, financial report) can be produced, the source artifacts must have sufficient provenance depth recorded in the ledger.

## Alternatives Considered

**Separate authorization service (e.g. SpiceDB, OpenFGA)** — a Zanzibar-compatible system running alongside the kernel. Rejected because it introduces a second source of truth for state that is already captured in the ledger. Keeping authorization state in the ledger preserves the single append-only record and eliminates synchronization risk.

**ABAC policy engine (e.g. OPA)** — evaluates policy against artifact attributes at request time. Rejected because artifact provenance state is not an attribute — it is a history. OPA would require the ledger to be projected into an attribute store, which duplicates state and weakens the append-only guarantee.

## Consequences

- Authorization logic that depends on artifact state must read the ledger — it cannot be resolved from a snapshot or a permissions table alone.
- The ledger schema must support efficient provenance depth queries, not just append and replay.
- `Human` provenance events are hard gates — they are never triggered automatically and always require a named authorizing identity recorded in the ledger.
- Any plugin or commercial layer component that produces derived artifacts must check source artifact provenance depth before proceeding. This check is enforced by the kernel routing layer, not left to individual plugins.
- The model is auditable by design — every authorization decision leaves a ledger entry, and the ledger is append-only.

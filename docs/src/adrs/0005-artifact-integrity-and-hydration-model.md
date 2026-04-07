---
id: "0005"
title: "Artifact Integrity and Hydration Model"
status: "Accepted"
date: "2026-04-07"
affects_repos: ["provena", "provenancai"]
---

# ADR 0005: Artifact Integrity and Hydration Model

## Context

ProvenancAI ingests source artifacts (documents, emails, invoices, contracts) and uses them as inputs to produce governed outputs. For those outputs to be defensible, the integrity of every source artifact must be verifiable at any point in time — not just at the moment of ingest.

Two concerns needed to be resolved:

**Hydration strategy** — when an artifact is registered, how much of its content is brought into the system? Options range from fully eager (store everything on registration) to fully lazy (store nothing, fetch on demand).

**Integrity model** — how does the system detect and record when a source artifact has changed since it was first ingested?

These two concerns are related but separable. The hydration strategy determines where content lives; the integrity model determines whether the content is still what it was when the ledger entry was created.

## Decision

### v1 — Full hydration on explicit upload

In v1, artifacts enter the system through an explicit user action (a file picker upload). There are no ambient connectors, no background crawlers, and no live links to external systems.

When a file is uploaded:

- The full content is stored as a blob
- A SHA-256 hash of the content is computed at ingest time
- The hash is recorded in the ledger entry alongside the artifact metadata

The SHA-256 hash is the integrity anchor. It is what makes the artifact defensible — a downstream output can always cite the exact content that was used to produce it, verifiable by hash.

### v2 — Connector-based live links with integrity snapshots

In v2, connector plugins (e.g. `connector.microsoft.sharepoint`, `connector.microsoft.email`) register capabilities that allow the system to reach live external sources. For these artifacts:

- The ledger entry stores a content reference (a pointer to the external source) plus the SHA-256 hash of the content at the time of first ingest
- A background process periodically re-fetches the source and re-hashes it
- If the hash has drifted since the last verification, a new ledger entry is recorded: the source artifact has mutated
- Mutation of a source artifact is a ledger event — it is traceable, timestamped, and carries a provenance class

Deduplication (avoiding storing the same content twice when multiple artifacts reference the same source) is a v2 concern and is not addressed in v1.

### Integrity model is consistent across versions

The SHA-256 integrity model is identical in v1 and v2. A fully hydrated blob and a live-linked document are verified the same way. The connector is the fetch mechanism; the ledger entry structure does not change between versions.

This means the integrity verification infrastructure built for v1 does not need to be redesigned for v2 — only extended with a background polling mechanism and a mutation event type.

## Alternatives Considered

**Fully lazy hydration from the start** — store only references, fetch content on demand. Rejected for v1 because it introduces connector complexity before the core artifact model is proven, and because some source types (uploaded files) have no meaningful "live link" to refer back to.

**Content-addressed storage (deduplicate by hash)** — store each unique piece of content once, reference by hash. Desirable eventually but deferred to v2. For MVP the added complexity is not justified.

**No integrity checking** — store artifacts without hashing. Rejected outright. Without a hash, downstream outputs cannot be defensibly traced to their source content. The hash is not optional.

## Consequences

- Every artifact ingest must compute and store a SHA-256 hash — this is a hard requirement, not optional behaviour
- The ledger entry schema must include a `content_hash` field from day one
- v1 artifact storage is simple blob storage — the abstraction boundary should be designed so the storage provider can be swapped without changing the ledger entry structure
- In v2, source artifact mutation is a first-class ledger event — it must be modelled as a new provenance record, not a mutation of the original entry (the ledger is append-only)
- A mutated source artifact's effect on downstream outputs derived from it is a policy question deferred to the ProvenancAI commercial layer — Provena records the mutation, ProvenancAI decides what to do about it
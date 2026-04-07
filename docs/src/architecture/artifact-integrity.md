---
id: "arch-artifact-integrity"
title: "Artifact Integrity and Hydration Model"
---

# Artifact Integrity and Hydration Model

Every artifact that enters Provena is given an integrity anchor at ingest time. That anchor is a SHA-256 hash of the content. It is what makes downstream outputs defensible — a governed output can always cite the exact content that was used to produce it, verifiable by hash.

## v1 — Full hydration on explicit upload

In v1, artifacts enter the system through an explicit user action. There are no ambient connectors and no live links to external systems. The user picks a file; it comes in fully.

On ingest:

1. The full content is stored as a blob
2. A SHA-256 hash is computed over the content
3. The hash is recorded in the ledger entry alongside artifact metadata

That is the complete ingest path for v1. Simple, no moving parts.

## v2 — Live links via connectors

In v2, connector plugins (e.g. `connector.microsoft.sharepoint`, `connector.microsoft.email`) register capabilities that allow the system to reach live external sources.

For connector-sourced artifacts:

- The ledger entry stores a content reference (pointer to the external source) plus the SHA-256 hash of the content at first ingest
- A background process periodically re-fetches the source and re-hashes it
- If the hash has drifted, a new ledger event is recorded: **source artifact mutated**

Mutation of a source artifact is a first-class ledger event — traceable, timestamped, carrying a provenance class. The ledger is append-only, so the original entry is never changed. The mutation is an addendum.

## The integrity model is consistent across versions

A fully hydrated blob and a live-linked document are verified the same way — by SHA-256 hash. The connector is the fetch mechanism; the ledger entry structure does not change between v1 and v2. The integrity infrastructure built for v1 extends naturally into v2 without redesign.

## What Provena does vs what ProvenancAI decides

Provena records that a source artifact mutated. What that means for downstream outputs — whether a derived SOW needs re-review, whether a report is invalidated — is a policy decision for the ProvenancAI commercial layer. The kernel's job is to notice and record; the platform's job is to act.

The full design rationale is in [ADR 0005](../adrs/0005-artifact-integrity-and-hydration-model.md).

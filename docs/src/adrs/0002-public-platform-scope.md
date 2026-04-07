---
id: "0002"
title: "Public Platform Scope"
status: "Accepted"
date: "2026-04-06"
affects_repos: ["provena"]
---

# ADR 0002: Public Platform Scope

## Context

The public `provena` repository should be meaningful on its own while remaining small enough to maintain as a clean OSS platform.

## Decision

The public repository contains the platform contracts and the bare minimum implementation required to demonstrate the model end to end.

This includes:

- core domain types
- plugin SDK and manifests
- kernel routing and health behavior
- minimal reference plugins needed to make the system functional
- the public API and CLI

This excludes:

- ProvenancAI-specific product logic
- commercial-only plugins and infrastructure adapters
- internal policy engines and enterprise integrations

## Consequences

- The OSS repository is runnable and technically credible to any external contributor.
- Commercial extensions remain layered on top rather than embedded in the platform repo.

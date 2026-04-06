# ADR 0002: Public Platform Scope

## Status
Accepted

## Context
The public `provena` repository should be meaningful on its own while remaining small enough to maintain as a clean OSS platform.

## Decision
The public repository contains the platform contracts and the bare minimum implementation required to demonstrate the model end to end.

This includes:
- core domain types
- plugin SDK and manifests
- kernel routing and health behavior
- minimal reference plugins needed to make the system functional
- optionally the public API and CLI if they are part of the OSS surface

This excludes:
- ProvenancAI-specific product logic
- commercial-only plugins and infrastructure adapters
- internal policy engines and enterprise integrations

## Consequences
- The OSS repository is runnable and technically credible.
- Commercial extensions remain layered on top rather than embedded in the platform repo.

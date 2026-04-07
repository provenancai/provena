# Provena

Provena is the public open source provenance microkernel.

This repository intentionally contains only the public platform surface:

- core domain types
- plugin SDK and manifests
- kernel registry, routing, and health behavior
- ledger-facing append-only interfaces
- API surface and CLI entrypoint

Commercial product code, policy logic, and enterprise integrations belong in sibling private repositories, not here.

## Workspace Layout

```text
provena/
├── provena-core/      # Domain identifiers and provenance types
├── provena-sdk/       # Plugin manifest and trait surface
├── provena-kernel/    # Capability registry and routing
├── provena-ledger/    # Append-only ledger interfaces and scaffold
├── provena-api/       # Axum router for the OSS surface
└── prv/               # CLI entrypoint
```

## Status

This is the initial public-only scaffold. It establishes repository boundaries and crate ownership without pulling in private product behavior.

## Running

```bash
cargo run -p prv
```

That starts the minimal public API on `127.0.0.1:3000` with a `/health` endpoint.

## License

Provena is licensed under the Apache License 2.0.

See the [LICENSE](./LICENSE) file for details.
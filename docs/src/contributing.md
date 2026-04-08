# Contributing

Contributions to Provena are welcome.

## Before you start

- Open an issue or discussion before writing code for anything non-trivial.
- The scope of this repository is the public platform surface only - domain types, the plugin SDK, the kernel, ledger interfaces, and the API entrypoint. Commercial product logic, enterprise integrations, and private policy engines are out of scope.

## Development setup

```bash
# Build everything
cargo build

# Run tests
cargo test

# Start the development server
cargo run -p prv
```

## Branching

- `main` is always releasable.
- Feature branches: `feat/short-description`
- Fix branches: `fix/short-description`
- Pull requests are required to merge to `main`.
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/): `feat:`, `fix:`, `chore:`, `docs:`.

## Code conventions

- Rust edition 2021, async runtime Tokio, HTTP framework Axum.
- `thiserror` for library crates, `anyhow` for binaries.
- No `unwrap()` or `expect()` in library code - all errors must be handled explicitly.
- Newtypes for all domain identifiers.
- Static binaries only - musl target for release builds.

## What not to do

- Do not add business logic to the kernel.
- Do not allow a singleton capability to have a fallback.
- Do not cross storage role boundaries in routing.
- Do not let a plugin bypass the kernel routing layer.

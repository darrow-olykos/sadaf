# Example codebase structure for extensible CLI program

- Entry point is `src/main.rs`, which delegates to matched subcommand.
- A subcommand is a single "feature" and lives in its own crate in `/crates/*`.
- The crates in `crates/*` are workspace members meaning that everything shares same `Cargo.lock` and output directory.
- Cargo does not assume that crates in a workspace depend on each other so path dependencies to each crate in `crate/*` must be specified in root `Cargo.toml`
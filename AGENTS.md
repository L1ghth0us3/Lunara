# Repository Guidelines

## Project Structure & Module Organization
- Cargo.toml: Workspace or crate manifest at the repo root.
- src/: Main crate sources; binaries in `src/bin/` (one file per binary).
- crates/: Additional workspace crates (if we split components).
- tests/: Integration tests (`tests/*.rs`), unit tests inline in modules.
- benches/ and examples/: Optional performance and usage examples.
- docs/: Supplemental docs; top-level `README.md` and `CHANGELOG.md`. Kanban board lives under `docs/lunara_v_*.md`.
- .githooks/: Local git hooks (repo may set `core.hooksPath` to `.githooks`).

## Build, Test, and Development Commands
- cargo check: Fast type-check for quick iteration.
- cargo build: Build default targets; add `--release` for optimized builds.
- cargo test --all --workspace: Run unit and integration tests.
- cargo fmt --all -- --check: Verify formatting (use `cargo fmt --all` to apply).
- cargo clippy --all-targets -- -D warnings: Lint; treat warnings as errors.

## Coding Style & Naming Conventions
- Formatting: rustfmt defaults (4‑space indent, standard imports/order).
- Linting: Clippy; fix or justify warnings; keep CI green.
- Naming: snake_case for functions/modules, UpperCamelCase for types/traits,
  SCREAMING_SNAKE_CASE for constants, kebab-case for crate/binary names.
- Commits: Conventional Commits (e.g., `feat(cli): add subcommands`).

## Testing Guidelines
- Unit tests: `#[cfg(test)]` modules alongside code in `src/`.
- Integration tests: `tests/*.rs` using public API; prefer black‑box style.
- Coverage: Prioritize critical paths and edge cases; add regression tests on bugs.
- Run: `cargo test --all --workspace` before opening PRs.

## Commit & Pull Request Guidelines
- One focused change per commit; clear, Conventional Commit messages.
- Branching: Use feature branches (e.g., `feat/workflow-gates`); avoid committing to `main`.
- PRs: Provide summary, rationale, linked issues, and any CLI output/screenshots.
  Update `README.md`/`CHANGELOG.md` for user‑facing or workflow‑impacting changes.

## Agent‑Specific Instructions
- Stage intentionally (`git add -p`); keep diffs small and scoped.
- Until the Rust Lunara CLI is implemented, run the cargo commands above as the gate.
- On failures: fix → stage → re‑run checks; only squash/amend after green.

## Security & Configuration Tips
- Do not commit secrets; prefer local `.env` files excluded by `.gitignore`.
- Pin toolchains in `rust-toolchain.toml`; prefer reproducible builds.

# Lunara

Lunara is a Rust-first, language-agnostic workflow tool that enforces a safe, repeatable, and concise development loop. It orchestrates build → lint → test gates, tracks commit intent, and applies policy checks (e.g., protected branches, docs-required) so agents and humans can ship confidently.

## Status
- Active development toward v0.1.0. Scope and Kanban: `docs/lunara_v_0_1.md`.
- Main development branch: `0.1-dev` (avoid direct changes on `main`).

## Quickstart
- Requirements: Rust (stable), rustfmt, clippy. The repository includes `rust-toolchain.toml` to pin toolchain and components.
- Build: `cargo build --workspace`
- Run CLI: `cargo run -p lunara-cli`
- Tests: `cargo test --all --workspace`
- Lint/format: `cargo fmt --all -- --check` and `cargo clippy --all-targets -- -D warnings`

## Workspace Layout
- `crates/lunara-core`: Core library (shared types/utilities).
- `crates/lunara-cli`: Command-line binary (entry point).
- `docs/`: Documentation (v0.1.0 plan, future guides).

## Contributing
- Read `AGENTS.md` for contributor workflow, coding style, and commit conventions.
- Use Conventional Commits (e.g., `feat(cli): add subcommand`).
- Work on feature branches (e.g., `feat/workflow-gates`) and keep diffs focused.

## Vision (v0.1)
- Core: Git adapter, local runner, intent cache, policy engine, commit composer, clear CLI UX.
- Plugins: JS and Rust pipelines (auto-detect; run build/lint/test) + docs policy module.

## License
Dual-licensed under MIT or Apache-2.0 (see crate manifests). License files will be added before the v0.1.0 release.


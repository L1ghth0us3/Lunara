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

## CLI

The current CLI is a lightweight skeleton to validate UX and gating:

- Usage: `lunara <command> [options]`
- Commands:
  - `help`: Show help and usage
  - `version`: Print version information
  - `init`: Initialize a Lunara config in this repo (stub)
  - `status`: Show current repo status and gates (stub)
  - `check`: Run configured checks (stub)
  - `run`: Execute the configured pipeline (stub)
- Exit codes: 0=ok, 1=error, 2=usage, 10=checks failed

`lunara status` prints read-only repo info (LUN-6):

- repo root and current branch (or detached short SHA)
- upstream tracking info if available (ahead/behind)
- change summary: counts of staged, unstaged, and untracked files
- diff summary: shortstat for unstaged and staged changes (files/+/–)

## Config (`lunara.yml`)

Minimal v1 schema (LUN-5):

```
version: "1"
pipeline:
  languages: [rust, js]
  gates: [build, lint, test]
  # timeouts (optional)
  # timeouts:
  #   per_step_secs: 600
  #   overall_secs: 1800
policies:
  docs_required: false
  protected_branches: ["main", "release/*"]
intent:
  path: ".lunara/intent.json"
```

Notes:
- `languages`: one or more of `rust`, `js`.
- `gates`: any of `build`, `lint`, `test` (non-empty).
- If `lunara.yml` is absent, the CLI may fall back to sensible defaults in a future version.

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

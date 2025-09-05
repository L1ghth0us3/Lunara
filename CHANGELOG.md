# Changelog

All notable changes to this project are documented here.

## [Unreleased]
### Added
- Initial Rust workspace scaffold: `lunara-core` (lib) and `lunara-cli` (bin).
- Toolchain pinning via `rust-toolchain.toml` (stable + rustfmt, clippy).
- `.gitignore` with Rust targets.
- Contributor guide `AGENTS.md` (Rust-first workflow) with Kanban pointer under `docs/`.
- Repository Kanban: `docs/lunara_v_0_1.md` outlining v0.1.0 plan.
- Project `README.md` with overview, quickstart, and layout.
- CLI: skeleton commands (`help`, `version`, `init`, `status`, `check`, `run`) with exit codes.

### Changed
- Repository initialized and pushed to GitHub;
- Created and pushed development branch `0.1-dev` (primary branch for ongoing work).

### Notes
- Main branch (`main`) holds initial scaffolding; active development occurs on `0.1-dev`.
- Future releases will follow semantic versioning; entries will be moved under a tagged section on release.

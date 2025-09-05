# Lunara v0.1.0 — Kanban Board

**Columns:** Backlog → Ready → InReview → Done  
**Priority labels:** `[prio:low]`, `[prio:mid]`, `[prio:high]`

---

## Backlog
- [ ] LUN-1: Define MVP scope and non-goals `[prio:high]`
- [ ] LUN-2: Architecture blueprint (core, runners, plugins, policies) `[prio:high]`
- [ ] LUN-3: Choose implementation language for core (Go or Rust) and justify `[prio:mid]`
- [ ] LUN-4: Specify plugin protocol (JSON-RPC over stdio) draft `[prio:high]`
// moved to Ready as MVP and actively being implemented
- [ ] LUN-7: Local runner (shell-agnostic, cross-OS) `[prio:high]`
- [ ] LUN-8: Intent cache service (.lunara/intent.json) & rebind rules `[prio:high]`
- [ ] LUN-9: Policy engine (protected branches, docs required) `[prio:high]`
- [ ] LUN-10: Commit composer (conventional commits, amend-after-green) `[prio:mid]`
- [ ] LUN-12: JS plugin (@lunara/js): detect, build, lint, test `[prio:high]`
- [ ] LUN-13: Rust plugin (@lunara/rust): detect toolchain/workspaces, lint (clippy), fmt (rustfmt), test (cargo test) `[prio:high]`
- [ ] LUN-14: Docs policy module (globs + suggestions) `[prio:mid]`
- [ ] LUN-15: Config-driven pipelines with conditions/timeouts `[prio:mid]`
- [ ] LUN-16: Example repos (JS & Rust) to validate end-to-end `[prio:mid]`
- [ ] LUN-17: Packaging & distribution (single binary + plugin packages) `[prio:mid]`
- [ ] LUN-18: CI for Lunara repo (build, unit tests, lint) `[prio:mid]`
- [ ] LUN-19: Windows/macOS/Linux parity checks `[prio:mid]`
- [ ] LUN-20: Telemetry/analytics toggle (off by default) `[prio:low]`
- [ ] LUN-21: Logging and error taxonomy `[prio:mid]`
- [ ] LUN-22: Security review (runner safety, env/secret handling) `[prio:mid]`
- [ ] LUN-23: README, quickstart, and getting-started tutorial `[prio:high]`
- [ ] LUN-24: CHANGELOG and release process `[prio:mid]`
- [ ] LUN-25: Version v0.1.0 tagging and GitHub Release notes `[prio:high]`
- [ ] LUN-26: License & CONTRIBUTING.md `[prio:low]`
- [ ] LUN-27: Issue templates and PR checklist `[prio:low]`

---

## Ready
*(Move items here when design is stable and work can start immediately.)*
- [ ] LUN-4: Plugin protocol (JSON-RPC over stdio) — finalize message shapes `[prio:high]`
- [ ] LUN-7: Local runner MVP (execute, timeout, retry) `[prio:high]`
 
 

---

## InReview
*(Move items here when a PR is open and needs review.)*
- [ ] LUN-11: CLI skeleton with rich sections & exit codes `[prio:mid]`
- [ ] LUN-5: Config schema `lunara.yml` (fields + validation) `[prio:high]`

---

## Done
*(Move items here once merged to main.)*
- [ ] LUN-6: Git adapter MVP (read-only ops) `[prio:high]`

---

## Milestone: v0.1.0 Definition of Done (DoD)
To consider Lunara **v0.1.0** complete, the following must be **Done**:

**Core**
- [ ] LUN-6 Git adapter (read/write), with safety rails on protected branches `[prio:high]`
- [ ] LUN-7 Local runner (timeouts, retries, cross-OS) `[prio:high]`
- [ ] LUN-8 Intent cache (create, verify staged tree, rebind) `[prio:high]`
- [ ] LUN-9 Policy engine (protected branch + docs-required) `[prio:high]`
- [ ] LUN-10 Commit composer (conventional commits, amend-after-green) `[prio:mid]`
- [ ] LUN-11 UX/CLI with diffstat & clear guidance `[prio:mid]`

**Plugins**
- [ ] LUN-12 JS plugin with auto-detect (scripts or sensible defaults) `[prio:high]`
- [ ] LUN-13 Rust plugin (clippy, rustfmt, cargo test; workspace-aware) `[prio:high]`
- [ ] LUN-14 Docs policy module (configurable globs & suggestions) `[prio:mid]``

**Config & Examples**
- [ ] LUN-5 `lunara.yml` schema + validation `[prio:high]`
- [ ] LUN-16 Example repos (JS & Rust) pass `build→lint→test→commit` gates `[prio:mid]`

**Quality**
- [ ] LUN-18 Basic CI (lint, unit tests) `[prio:mid]`
- [ ] LUN-19 Cross-platform smoke tests (Windows/macOS/Linux) incl. Rust toolchains (MSVC/GNU) and targets `[prio:mid]`
- [ ] LUN-22 Security review checklist complete `[prio:mid]`

**Release**
- [ ] LUN-23 README + Quickstart finished `[prio:high]`
- [ ] LUN-24 CHANGELOG has v0.1.0 entries `[prio:mid]`
- [ ] LUN-25 Create tag `v0.1.0` & GitHub Release with binaries `[prio:high]`
- [ ] LUN-26 LICENSE and CONTRIBUTING.md present `[prio:low]`

---

## Workflow Rules
1. **Backlog → Ready** when design is stable and acceptance criteria are defined.
2. **Ready → InReview** when a PR is opened and CI is green.
3. **InReview → Done** when approved and merged to `main`.
4. Every task must include a **priority label** `[prio:low|mid|high]`.
5. Tasks that impact user-facing behavior must check the **docs policy** before merging.

---

## Acceptance Criteria Templates
**Feature task:**
- Definition: What user outcome does this enable?
- Acceptance tests: Given/When/Then …
- Rollback plan: …

**Policy task:**
- Rules enumerated and examples provided.
- Failing/Passing cases documented.

**Plugin task:**
- Detect logic, supported tools, command plan.
- Minimal repo used for validation.

---

## Notes
- Keep scope tight: JS & Rust plugins only for v0.1.0.
- Runners: local only in v0.1.0 (containers are post-0.1 stretch).
- Keep telemetry off by default; no network calls without user action.

---

## Rust-focused additions
- [ ] LUN-28: Rust toolchain detection (`rustup`, `cargo`, version) `[prio:high]`
- [ ] LUN-29: Workspace & features support (`--workspace`, `--all-features`, `--features`) `[prio:mid]`
- [ ] LUN-30: Clippy config discovery (`clippy.toml`) & category gates `[prio:mid]`
- [ ] LUN-31: rustfmt config discovery (`rustfmt.toml`) & format check `[prio:mid]`
- [ ] LUN-32: Test strategy (unit/integration/examples, `-- --nocapture`) `[prio:mid]`
- [ ] LUN-33: Target matrix hooks (e.g., `wasm32`, `x86_64-pc-windows-msvc`) `[prio:low]`
- [ ] LUN-34: Caching guidance (`CARGO_HOME`, `RUSTC_WRAPPER`, sccache) `[prio:low]`
- [ ] LUN-35: Example Rust repo (workspace with 2 crates) `[prio:mid]`

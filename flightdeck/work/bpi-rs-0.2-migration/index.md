# Index — bpi-rs 0.2 migration

## State

Design approved at conversation level: bpi-rs should become a high-quality, idiomatic Rust SDK, with breaking changes allowed. The public API should move toward module clients such as `client.video().info(...)`, not a flat 300+ method surface.

This topic owns the staged migration plan and should keep all active design and planning artifacts under this folder.

The current migration execution protocol is `api-upgrade-protocol.md`. Agents must follow module-batch migration, real Probe evidence, local-only raw Probe output under `target/...`, and low commit count rules from that protocol.

The accepted promoted contract shape is now one endpoint contract plus response fixtures:

```text
tests/contracts/<domain>/<endpoint>/contract.json
tests/contracts/<domain>/<endpoint>/responses/<case>.json
```

Do not resume the old committed shape `tests/contracts/<domain>/<endpoint>/<profile>.request.json` for promoted endpoint contracts.

`migration-status.md` is a local temporary status board for context recovery. It tracks pinned execution order, current batch, module status, evidence, and next action. It must not be committed. This workspace excludes it through `.git/info/exclude`; if another workspace needs the board, create the local file again instead of adding it to version control.

## Next

- Use `goal.md`, `api-upgrade-protocol.md`, and `migration-status.md` to choose the next module batch.
- Default to a real endpoint contract batch with Probe evidence unless a non-Probe shared-core/domain-client bridge is explicitly selected and recorded as such.
- For each batch, follow `api-upgrade-protocol.md`.
- Update `migration-status.md` after each batch, but do not commit it.

## Read now

- goal.md
- api-upgrade-protocol.md
- migration-status.md
- design.md
- plan.md
- knowledge/rust/sdk-quality.md

## Read if

- plans/ — when implementation planning begins.

## Progress

Done:
- Created active Flightdeck topic package.
- Wrote the 0.2 migration design spec.
- Installed and reviewed Rust best-practices, async-patterns, and rust-testing skills as reference material for future sessions.
- Researched primary Rust/Tokio/reqwest/tracing docs and distilled the SDK quality bar into `knowledge/rust/sdk-quality.md`.
- Added logging/observability and SDK embedding customization requirements to `design.md` and `knowledge/rust/sdk-quality.md`.
- Added type, fixture, and documentation discipline requirements to address weak models, random test parameters, and stale examples.
- Read all root `src/*.rs` files and sampled key submodules for errors, signing, login, video, dynamic, user, tests, README, Cargo, and Taskfile.
- Wrote `plan.md` with code-research findings and a staged migration plan.
- Wrote `api-upgrade-protocol.md` with Probe-first module-batch migration rules.
- Wrote local-only `migration-status.md` for pinned execution order and resumable batch state.
- Completed pilot `clientinfo/ip` request-contract validation in commit `c261e4f`.
- Migrated existing pilot contracts to endpoint `contract.json` plus `responses/*.json` fixtures in commit `6383119`.
- Covered `clientinfo/ip`, `login/vip-info`, `login/read-info`, and `login/qr` in the accepted contract shape.

Current:
- Shared-core/domain bridge work has a completed `clientinfo/module-client-bridge` example after endpoint contract batches.
- Goal-mode continuation now defaults back to Probe-backed endpoint contract batches unless a non-Probe bridge batch is explicitly selected and recorded. Do not repeat completed examples such as `video/info-read`, `login/read-info`, or `clientinfo/ip`.
- Use the local status board to select the next incomplete module or cohesive submodule batch.

Verified:
- After commit `6383119`, `cargo fmt --check`, `cargo check --all-features`, and `cargo test --all-features --lib` passed.
- `cargo test --all-features --lib`: 358 passed, 0 failed, 302 ignored.
- For `shared-core/contract-error-label-validation`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.
- For `shared-core/payload-request-helpers`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.
- For `clientinfo/module-client-bridge`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.

## Local-only Constraints

- Do not commit `migration-status.md`.
- Do not commit raw Probe outputs, account-specific response data, cookies, `SESSDATA`, `bili_jct`, `buvid`, or local account notes.
- Keep raw Probe output under `target/bpi-probe-runs/...`.
- Keep request drafts under `target/bpi-contract-drafts/...`.
- Commit only reviewed endpoint contracts and sanitized response fixtures under `tests/contracts/...`.
- Prefer one commit per completed module batch; do not create endpoint-sized commit spam.

## Open questions

- None blocking the design review.

# Index — bpi-rs 0.2 migration

## State

Design approved at conversation level: bpi-rs should become a high-quality, idiomatic Rust SDK, with breaking changes allowed. The public API should move toward module clients such as `client.video().info(...)`, not a flat 300+ method surface.

This topic owns the staged migration plan and should keep all active design and planning artifacts under this folder.

The current migration execution protocol is `api-upgrade-protocol.md`. Agents must follow module-batch migration, real Probe evidence, local-only raw Probe output under `target/...`, and low commit count rules from that protocol.

`migration-status.md` is a local temporary status board for context recovery. It tracks pinned execution order, current batch, module status, evidence, and next action. It must not be committed. This workspace excludes it through `.git/info/exclude`; if another workspace needs the board, create the local file again instead of adding it to version control.

## Next

- Use `migration-status.md` to choose the next module batch.
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

Current:
- Use the local status board to select and execute the next module batch.

Verified:
- `task check_all` passed after the pilot batch: 337 passed, 0 failed, 302 ignored.

## Local-only Constraints

- Do not commit `migration-status.md`.
- Do not commit raw Probe outputs, account-specific response data, cookies, `SESSDATA`, `bili_jct`, `buvid`, or local account notes.
- Keep raw Probe output under `target/bpi-probe-runs/...`.
- Keep request drafts under `target/bpi-contract-drafts/...`.
- Commit only reviewed contracts under `tests/contracts/...` and reviewed sanitized fixtures under `tests/fixtures/...`.
- Prefer one commit per completed module batch; do not create endpoint-sized commit spam.

## Open questions

- None blocking the design review.

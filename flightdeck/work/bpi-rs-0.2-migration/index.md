# Index — bpi-rs 0.2 migration

## State

Design approved at conversation level: bpi-rs should become a high-quality, idiomatic Rust SDK, with breaking changes allowed. The public API should move toward module clients such as `client.video().info(...)`, not a flat 300+ method surface.

This topic owns the staged migration plan and should keep all active design and planning artifacts under this folder.

## Next

- User reviews `plan.md`.
- After approval, write `plans/00-baseline-inventory.md` and begin Stage 0.

## Read now

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

Current:
- Waiting for user review of `plan.md`.

Verified:
- Current crate baseline previously passed `cargo check --all-features`.

## Open questions

- None blocking the design review.

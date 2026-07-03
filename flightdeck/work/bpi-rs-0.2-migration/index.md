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
- The latest endpoint-candidate audit found no new safe read batch to promote. Before running more Probe work, identify either a newly implemented/read-safe endpoint not covered by existing contracts, a valid current `manga/download-read` flow/chapter/handshake, or an explicitly enabled gated/mutating batch.
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
- plans/manga-download-read-probe-block.md — before retrying manga download/read contracts.
- plans/remaining-endpoint-contract-audit.md — before claiming another safe Probe read batch remains or before selecting non-Probe bridge work.

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
- Completed `electric/private-read-remark-detail-flow`: anonymous direct Probe preserves `requires_login`, normal/vip Probe flows extract a private `remark_id` from `remark-list` at runtime, and promoted contracts/fixtures avoid committing literal private ids or message content.
- Rechecked `manga/download-read` with additional normal/vip `ComicDetail`, `epId`, and alternate public sample `GetImageIndex` Probe trials; all returned HTTP 200/API `code = 99`, so the batch remains Probe-blocked with no promoted contract. Details are in `plans/manga-download-read-probe-block.md`.
- Audited remaining endpoint candidates against source request calls, promoted `tests/contracts/**/contract.json`, and `migration-status.md`; no new safe Probe read batch was found. Details are in `plans/remaining-endpoint-contract-audit.md`.
- Completed explicit non-Probe `web_widget/module-client-bridge`: `BpiClient::web_widget()` now exposes a `WebWidgetClient` with payload-returning `region_banner`, `header_page`, and `online` methods backed by already promoted `web_widget` contracts. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `activity/module-client-bridge`: `BpiClient::activity()` now exposes an `ActivityClient` with payload-returning `info`, `list`, and `list_default` methods backed by already promoted `activity` contracts. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `article/module-client-bridge`: `BpiClient::article()` now exposes an `ArticleClient` with payload-returning `info`, `view`, `cards`, and `articles` methods backed by already promoted article read contracts. WBI-signed `view` and `cards` reuse the existing client signing helper; mutating `article/action` endpoints remain excluded. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `fav/module-client-bridge`: `BpiClient::fav()` now exposes a `FavClient` with payload-returning `folder_info`, `created_list`, `collected_list`, `resource_infos`, `list_detail`, and `resource_ids` methods backed by already promoted fav read contracts. Mutating `fav/action.rs` endpoints remain excluded. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `cheese/module-client-bridge`: `BpiClient::cheese()` now exposes a `CheeseClient` with payload-returning `info`, `info_by_season_id`, `info_by_ep_id`, `ep_list`, and `video_stream` methods backed by already promoted cheese info, ep-list, and playurl contracts. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `bangumi/module-client-bridge`: `BpiClient::bangumi()` now exposes a `BangumiClient` with payload-returning `info`, `detail`, `detail_by_season_id`, `detail_by_ep_id`, `sections`, `timeline`, and `video_stream` methods backed by already promoted bangumi info, timeline, and playurl contracts. Mutating `bangumi/follow.rs` endpoints remain excluded. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `wallet/module-client-bridge`: `BpiClient::wallet()` now exposes a `WalletClient` with payload-returning `info` over the already promoted authenticated wallet read contract. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `opus/module-client-bridge`: `BpiClient::opus()` now exposes an `OpusClient` with payload-returning `space_feed` over the already promoted public opus space-feed contract. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `misc/module-client-bridge`: `BpiClient::misc()` now exposes a `MiscClient` with payload-returning `buvid3`, `buvid`, `b23_short_link`, `bili_ticket`, and `bili_ticket_string` methods backed by already promoted misc contracts. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `message/module-client-bridge`: `BpiClient::message()` now exposes a `MessageClient` with payload-returning `unread_count`, `reply_feed`, and `single_unread` methods backed by already promoted message read contracts. Mutating `message_send` remains excluded. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `vip/module-client-bridge`: `BpiClient::vip()` now exposes a `VipClient` with payload-returning `center_info` over the already promoted VIP center-info read contract. Mutating VIP action endpoints and disabled clock-in remain excluded. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `comment/module-client-bridge`: `BpiClient::comment()` now exposes a `CommentClient` with payload-returning `list`, `replies`, `hot`, and `count` methods backed by already promoted comment read contracts. Mutating `comment/action` endpoints remain excluded, and `hot` returns an optional payload because promoted success fixtures record `data: null`. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `historytoview/module-client-bridge`: `BpiClient::historytoview()` now exposes a `HistoryToViewClient` with payload-returning `history_list`, `history_shadow`, and `toview_list` methods backed by already promoted history/to-view read contracts. Mutating history delete/clear/shadow-set and to-view add/delete/clear endpoints remain excluded. No Probe run or contract promotion was expected for this bridge batch.
- Completed explicit non-Probe `note/module-client-bridge`: `BpiClient::note()` now exposes a `NoteClient` with payload-returning `is_forbid`, `private_info`, `public_info`, `archive_list`, `user_private_list`, `public_archive_list`, and `user_public_list` methods backed by already promoted note read contracts. Mutating `note/action` add/delete endpoints remain excluded. No Probe run or contract promotion was expected for this bridge batch.

Current:
- Shared-core/domain bridge work has completed `clientinfo/module-client-bridge`, `web_widget/module-client-bridge`, `activity/module-client-bridge`, `article/module-client-bridge`, `fav/module-client-bridge`, `cheese/module-client-bridge`, `bangumi/module-client-bridge`, `wallet/module-client-bridge`, `opus/module-client-bridge`, `misc/module-client-bridge`, `message/module-client-bridge`, `vip/module-client-bridge`, `comment/module-client-bridge`, `historytoview/module-client-bridge`, and `note/module-client-bridge` examples after endpoint contract batches.
- Goal-mode continuation defaults to Probe-backed endpoint contract batches unless a non-Probe bridge batch is explicitly selected and recorded. Do not repeat completed examples such as `video/info-read`, `login/read-info`, or `clientinfo/ip`.
- Remaining normal endpoint work is currently gated, mutating/flow-sensitive, deprecated/documented exception, wrapper-only, static/local helper, or Probe-blocked. `manga/download-read` remains blocked by repeated `code = 99` Probe results. Do not force a non-Probe bridge unless it is explicitly selected and recorded as such.

Verified:
- After commit `6383119`, `cargo fmt --check`, `cargo check --all-features`, and `cargo test --all-features --lib` passed.
- `cargo test --all-features --lib`: 358 passed, 0 failed, 302 ignored.
- For `shared-core/contract-error-label-validation`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.
- For `shared-core/payload-request-helpers`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.
- For `clientinfo/module-client-bridge`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.
- For `electric/private-read-remark-detail-flow`, RED `cargo test --all-features --lib electric --quiet` failed on missing promoted contract/fixture files; after promotion, `cargo test --all-features --lib probe --quiet`, `cargo test --all-features --lib electric --quiet`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.
- For `manga/download-read-probe-block`, additional real Probe attempts were run under `target/bpi-probe-runs/manga/download-read/...`; all returned API `code = 99`, no contracts were promoted, and `git diff --check` passed for the tracked notes.
- For `remaining-endpoint-contract-audit`, source/request-call inventory plus promoted contract and status-board review found no new safe read endpoint batch to run. `git diff --check` passed for the tracked docs update; cargo gates were not run because no Rust source or contract files changed.
- For `web_widget/module-client-bridge`, RED `cargo test --all-features --lib web_widget_domain_client_can_be_created --quiet` first failed because `BpiClient::web_widget()` did not exist. After implementation, `cargo test --all-features --lib web_widget --quiet`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, and `cargo test --all-features --lib --quiet` passed.
- For `activity/module-client-bridge`, RED `cargo test --all-features --lib activity_domain_client_can_be_created --quiet` first failed because `BpiClient::activity()` did not exist; RED `cargo test --all-features --lib activity_client --quiet` then failed because the `ActivityClient` endpoint and payload methods did not exist. After implementation, `cargo test --all-features --lib activity --quiet`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.
- For `article/module-client-bridge`, RED `cargo test --all-features --lib article_domain_client_can_be_created --quiet` first failed because `BpiClient::article()` did not exist; RED `cargo test --all-features --lib article::client --quiet` then failed because the `ArticleClient` endpoint and payload methods did not exist. After implementation, `cargo test --all-features --lib article::client --quiet`, `cargo test --all-features --lib article --quiet`, `cargo fmt --check`, `git diff --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, and `cargo test --all-features --lib --quiet` passed.
- For `fav/module-client-bridge`, RED `cargo test --all-features --lib fav_domain_client_can_be_created --quiet` first failed because `BpiClient::fav()` did not exist; RED `cargo test --all-features --lib fav::client --quiet` then failed because the `FavClient` endpoint and payload methods did not exist. After implementation, `cargo test --all-features --lib fav::client --quiet`, `cargo test --all-features --lib fav --quiet`, `cargo fmt --check`, `git diff --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, and `cargo test --all-features --lib --quiet` passed.
- For `cheese/module-client-bridge`, RED `cargo test --all-features --lib cheese_domain_client_can_be_created --quiet` first failed because `BpiClient::cheese()` did not exist; RED `cargo test --all-features --lib cheese::client --quiet` then failed because the `CheeseClient` endpoint and payload methods did not exist. After implementation, `cargo test --all-features --lib cheese::client --quiet`, `cargo test --all-features --lib cheese --quiet`, `cargo fmt --check`, `git diff --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, and `cargo test --all-features --lib --quiet` passed.
- For `bangumi/module-client-bridge`, RED `cargo test --all-features --lib bangumi_domain_client_can_be_created --quiet` first failed because `BpiClient::bangumi()` did not exist; RED `cargo test --all-features --lib bangumi::client --quiet` then failed because the `BangumiClient` endpoint and payload methods did not exist. After implementation, `cargo test --all-features --lib bangumi::client --quiet`, `cargo test --all-features --lib bangumi --quiet`, `cargo fmt --check`, `git diff --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, and `cargo test --all-features --lib --quiet` passed.
- For `wallet/module-client-bridge`, RED `cargo test --all-features --lib wallet_domain_client_can_be_created --quiet` first failed because `BpiClient::wallet()` did not exist; RED `cargo test --all-features --lib wallet_client --quiet` then failed because the `WalletClient` endpoint and payload methods did not exist. After implementation, `cargo test --all-features --lib wallet --quiet`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.
- For `opus/module-client-bridge`, RED `cargo test --all-features --lib opus_domain_client_can_be_created --quiet` first failed because `BpiClient::opus()` did not exist; RED `cargo test --all-features --lib opus_client --quiet` then failed because the `OpusClient` endpoint and payload methods did not exist. After implementation, `cargo test --all-features --lib opus --quiet`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.
- For `misc/module-client-bridge`, RED `cargo test --all-features --lib misc_domain_client_can_be_created --quiet` first failed because `BpiClient::misc()` did not exist; RED `cargo test --all-features --lib misc_client --quiet` then failed because the `MiscClient` endpoint and payload methods did not exist. After implementation, `cargo test --all-features --lib misc --quiet`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.
- For `message/module-client-bridge`, RED `cargo test --all-features --lib message_domain_client_can_be_created --quiet` first failed because `BpiClient::message()` did not exist; RED `cargo test --all-features --lib message_client --quiet` then failed because the `MessageClient` endpoint and payload methods did not exist. After implementation, `cargo test --all-features --lib message --quiet`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.
- For `vip/module-client-bridge`, RED `cargo test --all-features --lib vip_domain_client_can_be_created --quiet` first failed because `BpiClient::vip()` did not exist; RED `cargo test --all-features --lib vip_client --quiet` then failed because the `VipClient` endpoint and payload method did not exist. After implementation, `cargo test --all-features --lib vip --quiet`, `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, `cargo test --all-features --lib --quiet`, and `git diff --check` passed.
- For `comment/module-client-bridge`, RED `cargo test --all-features --lib comment_domain_client_can_be_created --quiet` first failed because `BpiClient::comment()` did not exist; RED `cargo test --all-features --lib comment_client --quiet` then failed because the `CommentClient` endpoint and payload methods did not exist. After implementation, `cargo test --all-features --lib comment::client --quiet`, `cargo test --all-features --lib comment --quiet`, `cargo fmt --check`, `git diff --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, and `cargo test --all-features --lib --quiet` passed.
- For `historytoview/module-client-bridge`, RED `cargo test --all-features --lib historytoview_domain_client_can_be_created --quiet` first failed because `BpiClient::historytoview()` did not exist; RED `cargo test --all-features --lib historytoview::client --quiet` then failed because the `HistoryToViewClient` endpoint and payload methods did not exist. After implementation, `cargo test --all-features --lib historytoview::client --quiet`, `cargo test --all-features --lib historytoview --quiet`, `cargo fmt --check`, `git diff --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, and `cargo test --all-features --lib --quiet` passed.
- For `note/module-client-bridge`, RED `cargo test --all-features --lib note_domain_client_can_be_created --quiet` first failed because `BpiClient::note()` did not exist; RED `cargo test --all-features --lib note::client --quiet` then failed because the `NoteClient` endpoint and payload methods did not exist. After implementation, `cargo test --all-features --lib note::client --quiet`, `cargo test --all-features --lib note --quiet`, `cargo fmt --check`, `git diff --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, and `cargo test --all-features --lib --quiet` passed.

## Local-only Constraints

- Do not commit `migration-status.md`.
- Do not commit raw Probe outputs, account-specific response data, cookies, `SESSDATA`, `bili_jct`, `buvid`, or local account notes.
- Keep raw Probe output under `target/bpi-probe-runs/...`.
- Keep request drafts under `target/bpi-contract-drafts/...`.
- Commit only reviewed endpoint contracts and sanitized response fixtures under `tests/contracts/...`.
- Prefer one commit per completed module batch; do not create endpoint-sized commit spam.

## Open questions

- None blocking the design review.

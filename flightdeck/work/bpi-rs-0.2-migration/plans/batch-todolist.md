# Batch TODO — bpi-rs 0.2 migration

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans when executing this list inline. Keep TDD inside each batch, but commit only after the whole batch passes verification.

**Goal:** Continue the 0.2 migration in larger reviewable batches instead of committing every few methods.

**Architecture:** Default work remains Probe-backed endpoint contract batches. When no safe Probe batch exists, choose one explicit non-Probe bridge batch that reuses already promoted contracts, record it in `migration-status.md` before code, then implement all methods in that batch before one commit.

**Tech Stack:** Rust 2024, reqwest, serde, thiserror, tracing, existing `BilibiliRequest` payload helpers, `tests/contracts/**/contract.json`, Flightdeck topic state.

---

## Batch Rules

- Do not repeat completed endpoint contract batches: `video/info-read`, `login/read-info`, `clientinfo/ip`, or any batch already marked complete in `migration-status.md`.
- Do not start guessed Probe work. A Probe-backed batch needs a real unpromoted safe endpoint, request drafts under `target/bpi-contract-drafts/...`, raw Probe output under `target/bpi-probe-runs/...`, reviewed contracts under `tests/contracts/...`, and sanitized fixtures.
- Keep `flightdeck/cockpit.md` as a stable task index only. Per-batch progress belongs in this file, the topic `index.md`, and the local-only `migration-status.md`.
- A non-Probe bridge batch must be recorded first in `migration-status.md` with:
  - batch name
  - `Type = Explicit non-Probe ...`
  - no Probe raw output expected
  - exact promoted contracts reused
  - excluded mutating/gated endpoints
- TDD still applies inside the batch:
  - write one or more RED tests for the whole public surface
  - run the RED command and confirm the expected failure
  - implement all methods in the batch
  - run focused tests
  - run full gates
- Commit only once per completed batch after deck/status updates.
- In this workspace, do not commit until the human explicitly approves the completed batch commit.

## Current Working State

- Branch: `feat/bpi-rs-0.2-migration`
- Last committed batch: `48cd13c feat(api): move legacy write surface to module clients`
- Current intended batch: `release/post-write-surface-doc-sync`
- Current batch type: Non-Probe release documentation/status sync batch.
- Current commit policy: commit after verification; no endpoint Probe or live mutating execution expected.

## Batch 33: `release/post-write-surface-doc-sync`

**Type:** Non-Probe release documentation/status sync batch.

**Status:** Implemented and verified in the working tree; commit pending.

**Why this batch:** Batch 32 has been committed in `48cd13c`, direct `BpiClient` flat async
inventory is now `COUNT=0`, and `manga/download-read` is marked not implemented for this
migration. The topic docs still contain some pre-commit and superseded policy language. This
batch makes the continuation map match the committed state before more 0.2 release cleanup.

**Scope:**

- Update current batch/commit status in topic docs.
- Mark the old `release/gated-flat-api-policy` as superseded by the breaking flat cleanup
  plus module-client write-surface commit.
- Keep `manga/download-read` as not implemented and out of default continuation.
- Do not edit `flightdeck/cockpit.md`.
- Do not run Probe or live mutating endpoints.

**Verification plan:**

```powershell
rg -n "commit pending human approval|COUNT=107|valid current Probe path|manga/download-read still lacks|Current batch is `write-api/module-client-legacy-write-surface`" flightdeck\work\bpi-rs-0.2-migration\index.md flightdeck\work\bpi-rs-0.2-migration\plans\batch-todolist.md flightdeck\work\bpi-rs-0.2-migration\plans\gated-flat-api-release-policy.md
rg -n "COUNT=0|48cd13c|not implemented|post-write-surface-doc-sync" flightdeck\work\bpi-rs-0.2-migration\index.md flightdeck\work\bpi-rs-0.2-migration\plans\batch-todolist.md flightdeck\work\bpi-rs-0.2-migration\plans\gated-flat-api-release-policy.md
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
stale active-state search:
  no matches for pending write-surface approval, missing manga Probe path, or old current-batch wording.

current-state search:
  found COUNT=0, 48cd13c, not implemented, and post-write-surface-doc-sync markers.

git diff --check:
  passed with only Windows LF/CRLF conversion warnings.

git diff -- flightdeck/cockpit.md:
  no diff.
```

## Batch 32: `write-api/module-client-legacy-write-surface`

**Type:** Explicit non-Probe module-client write compatibility source batch.

**Status:** Implemented, verified, and committed in `48cd13c feat(api): move legacy write surface to module clients`.

**Why this batch:** The old direct `BpiClient` flat methods were removed in Batch 31, but several of those methods were the only public SDK entry points for write/session/mutating flows. Before deleting or reshaping more old code, the new module-client API needs to preserve that capability surface. This batch moves the old method bodies onto domain clients while keeping direct `impl BpiClient` public async inventory at `COUNT=0`.

**Scope:**

- Reintroduce the old write/session/mutating capability on module clients only, preserving old method names and signatures for compatibility parity, for example `client.video().video_like(...)` and `client.fav().fav_folder_add(...)`.
- Do not restore direct `BpiClient::...` flat methods.
- Make domain-client internals accessible inside sibling modules where needed.
- Keep helper functions required by the migrated write methods.
- Do not execute mutating endpoints or Probe requests.
- Keep `account.toml` local-only; it contains normal/vip credentials and must not be displayed or committed.
- Keep `flightdeck/cockpit.md` unchanged.

**Excluded:**

- No live mutating execution.
- No Probe run or contract promotion.
- No rename pass to idiomatic short names such as `client.video().like(...)`; that should be a separate alias/design batch.
- No `flightdeck/cockpit.md` update.

**TDD evidence:**

```text
RED: cargo test --all-features --lib module_clients_expose_legacy_write_capability_futures --quiet
  failed because `ArticleClient`, `VideoClient`, and `FavClient` did not expose representative old write methods.

GREEN: cargo test --all-features --lib module_clients_expose_legacy_write_capability_futures --quiet
  passed after moving the old write capability to module clients.
```

**Inventory evidence:**

```text
direct_bpi_client_public_async_count=0
old module write/session/mutating public async count from 45400bb^ target files = 117
new module write/session/mutating public async count in target files = 117
```

**Files:**

- Modify: `src/**/client.rs` where module clients need crate-visible root client access.
- Modify: write/session/mutating source modules under article, audio, bangumi, comment, creativecenter, danmaku, dynamic, electric, fav, historytoview, live, login, manga, message, note, user, video, and vip.
- Modify: `src/client.rs` for the module-client capability RED/GREEN test.
- Modify: `flightdeck/work/bpi-rs-0.2-migration/index.md`.
- Modify: `flightdeck/work/bpi-rs-0.2-migration/plans/batch-todolist.md`.
- Modify: `flightdeck/work/bpi-rs-0.2-migration/migration-status.md` local only, do not commit.

**Verification plan:**

```powershell
cargo test --all-features --lib module_clients_expose_legacy_write_capability_futures --quiet
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo fmt --check
cargo test --all-features --lib --quiet
cargo check --all-features --examples
cargo check --all-features --bins
cargo test --doc
git diff --check
git diff -- flightdeck/cockpit.md
PowerShell direct BpiClient public async inventory count
PowerShell old/new target-file public async inventory count
```

**Observed verification:**

```text
cargo test --all-features --lib module_clients_expose_legacy_write_capability_futures --quiet: pass
cargo check --all-features: pass
cargo clippy --all-targets --all-features --locked -- -D warnings: pass
cargo fmt --check: pass
cargo test --all-features --lib --quiet: pass (915 passed, 0 failed, 196 ignored)
cargo check --all-features --examples: pass
cargo check --all-features --bins: pass
cargo test --doc: pass
git diff --check: pass with only Windows LF/CRLF conversion warnings
direct BpiClient public async inventory: COUNT=0
old/new target-file public async inventory: 117/117
```

**Notes:**

- This is a capability-preserving module-client migration, not a Probe-backed endpoint validation batch.
- The method names intentionally remain old-style in this batch to reduce behavioral and review risk. A later alias batch can add cleaner idiomatic names and deprecate old-style module method names if desired.
- `account.toml` exists locally with normal/vip profile keys, remains ignored, and was not displayed or committed.

## Batch 31: `flat-api/remove-remaining-legacy-flat-methods`

**Type:** Explicit breaking API compatibility source batch.

**Status:** Implemented and verified; commit pending.

**Why this batch:** The human explicitly approved removing old code and accepting breaking changes. Batch 30 documented that the remaining direct `impl BpiClient` public async inventory is `COUNT=107` and that default source cleanup was blocked only by the compatibility decision. That decision is now made: remove the remaining legacy flat `BpiClient` methods rather than keeping them compatibility-only.

**Scope:**

- Remove every remaining direct `pub async fn` declared inside `impl BpiClient`.
- Keep the module-client APIs and shared request/transport helpers.
- Do not run mutating endpoints or Probe requests.
- Keep `account.toml` local-only; it contains normal/vip credentials and must not be displayed or committed.
- Keep `flightdeck/cockpit.md` unchanged.

**Pre-check evidence:**

```text
COUNT=107
article=4
audio=4
bangumi=2
comment=6
creativecenter=11
danmaku=8
dynamic=7
electric=3
fav=7
historytoview=6
live=12
login=4
manga=8
message=1
note=3
user=9
video=9
vip=3
```

**Files:**

- Modify: remaining `src/**` files that still declare flat `impl BpiClient` async methods.
- Modify: affected ignored tests that referenced those flat methods.
- Modify: `flightdeck/work/bpi-rs-0.2-migration/index.md`.
- Modify: `flightdeck/work/bpi-rs-0.2-migration/plans/batch-todolist.md`.
- Modify: `flightdeck/work/bpi-rs-0.2-migration/migration-status.md` local only, do not commit.

**Verification plan:**

```powershell
rg -n "impl BpiClient|pub async fn" src
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
cargo check --all-features --examples
cargo check --all-features --bins
cargo test --doc
git diff --check
git diff -- flightdeck/cockpit.md
git status --short --ignored=matching account.toml flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
```

**Observed verification:**

```text
direct BpiClient public async inventory: COUNT=0
cargo fmt --check: pass
cargo check --all-features: pass
cargo clippy --all-targets --all-features --locked -- -D warnings: pass
cargo test --all-features --lib --quiet: pass (914 passed, 196 ignored)
cargo check --all-features --examples: pass
cargo check --all-features --bins: pass
cargo test --doc: pass
git diff --check: pass
git diff -- flightdeck/cockpit.md: empty
```

**Notes:**

- This is a breaking source/API cleanup batch. Remaining direct `BpiClient` flat async methods were removed instead of retained as compatibility shims.
- Ignored legacy live tests that only exercised removed flat methods were removed or trimmed; promoted contract/model tests were retained where they still cover module-client-supported read behavior.
- `account.toml` exists locally with normal/vip profile keys, remains ignored, and was not displayed or committed.

## Batch 30: `release/gated-flat-api-policy`

**Type:** Non-Probe release policy documentation batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** After `flat-api/remove-dynamic-deprecated-404-shims` and `shared-core/wbi-helper-api-boundary`, the current direct `impl BpiClient` public async inventory is `COUNT=107`. All remaining methods are gated/mutating/session surfaces or Probe-blocked read work; the user has not enabled mutating/gated execution, and `manga/download-read` still lacks a valid current Probe path.

**Scope:**

- Record the release policy for the remaining 107 legacy flat methods.
- Make default goal-mode continuation stop selecting these methods for source cleanup unless an explicit mutating/session gate or manga Probe path is provided.
- Document the safety requirements for any future `BPI_MUTATING_TEST=1` batch.
- Keep `flightdeck/cockpit.md` unchanged.

**Excluded:**

- No Rust source changes.
- No Probe run.
- No contract promotion.
- No mutating/session endpoint execution.
- No `flightdeck/cockpit.md` update.
- No commit until explicit human approval.

**Evidence inventory:**

```text
COUNT=107
article=4
audio=4
bangumi=2
comment=6
creativecenter=11
danmaku=8
dynamic=7
electric=3
fav=7
historytoview=6
live=12
login=4
manga=8
message=1
note=3
user=9
video=9
vip=3
```

**Files:**

- Add: `flightdeck/work/bpi-rs-0.2-migration/plans/gated-flat-api-release-policy.md`
- Modify: `flightdeck/work/bpi-rs-0.2-migration/plans/batch-todolist.md`
- Modify: `flightdeck/work/bpi-rs-0.2-migration/index.md`
- Modify: `flightdeck/work/bpi-rs-0.2-migration/migration-status.md` local only, do not commit

**Verification plan:**

```powershell
rg -n "release/gated-flat-api-policy|COUNT=107|BPI_MUTATING_TEST|manga/download-read|compatibility-only|No default source cleanup" flightdeck\work\bpi-rs-0.2-migration\plans\gated-flat-api-release-policy.md flightdeck\work\bpi-rs-0.2-migration\plans\batch-todolist.md flightdeck\work\bpi-rs-0.2-migration\index.md flightdeck\work\bpi-rs-0.2-migration\migration-status.md
git diff --check
git diff -- flightdeck/cockpit.md
git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
```

Cargo gates are not required unless Rust source, tests, contracts, or compiled rustdoc examples change.

**Observed verification:**

```text
rg -n "release/gated-flat-api-policy|COUNT=107|BPI_MUTATING_TEST|manga/download-read|compatibility-only|No default source cleanup" flightdeck\work\bpi-rs-0.2-migration\plans\gated-flat-api-release-policy.md flightdeck\work\bpi-rs-0.2-migration\plans\batch-todolist.md flightdeck\work\bpi-rs-0.2-migration\index.md flightdeck\work\bpi-rs-0.2-migration\migration-status.md
  found the policy batch marker, COUNT=107 inventory, BPI_MUTATING_TEST gate, manga/download-read block, compatibility-only policy, and No default source cleanup language.

git diff --check
  exited 0 with only existing LF/CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 1: `video/collection-player-client-bridge`

**Type:** Explicit non-Probe domain-client bridge batch.

**Why this batch:** `remaining-endpoint-contract-audit.md` says no new safe Probe read batch remains. Video collection/player read contracts already exist, but their public read surface still lives behind flat `BpiClient` methods rather than `VideoClient`.

**Promoted contracts reused:**

```text
tests/contracts/video/collection-read/seasons-archives-list/contract.json
tests/contracts/video/collection-read/home-seasons-series/contract.json
tests/contracts/video/collection-read/seasons-series-list/contract.json
tests/contracts/video/collection-read/series-info/contract.json
tests/contracts/video/collection-read/series-archives/contract.json
tests/contracts/video/player-read/online-total/contract.json
tests/contracts/video/player-read/player-info-v2/contract.json
tests/contracts/video/player-read/related-videos/contract.json
tests/contracts/video/player-read/homepage-recommendations/contract.json
tests/contracts/video/player-read/ai-summary/contract.json
tests/contracts/video/player-read/tags/contract.json
tests/contracts/video/player-read/interactive-info/contract.json
```

**Excluded:**

```text
src/video/action.rs
src/video/report.rs
src/video/collection/action.rs
src/video/appeal.rs
src/video/pbp.rs
src/video/snapshot.rs
src/video/video_zone*.rs
```

### Files

- Modify: `src/video/client.rs`
- Modify: `src/video/collection/info.rs`
- Modify: `src/video/online.rs`
- Modify: `src/video/player.rs`
- Modify: `src/video/recommend.rs`
- Modify: `src/video/summary.rs`
- Modify: `src/video/tags.rs`
- Modify: `src/video/interact_video.rs`
- Modify: `src/video/mod.rs` only if re-exports are missing
- Modify: `flightdeck/work/bpi-rs-0.2-migration/migration-status.md` local only, do not commit
- Modify after verification: `flightdeck/work/bpi-rs-0.2-migration/index.md`
- Modify after verification: `flightdeck/cockpit.md`

### Public methods to add to `VideoClient`

```rust
pub async fn seasons_archives_list(
    &self,
    params: VideoCollectionSeasonsArchivesParams,
) -> BpiResult<GetSeasonsArchivesData>;

pub async fn home_seasons_series(
    &self,
    params: VideoCollectionHomeSeasonsSeriesParams,
) -> BpiResult<GetSeasonsSeriesData>;

pub async fn seasons_series_list(
    &self,
    params: VideoCollectionSeasonsSeriesParams,
) -> BpiResult<GetSeasonsSeriesData>;

pub async fn series_info(
    &self,
    params: VideoCollectionSeriesInfoParams,
) -> BpiResult<GetSeriesData>;

pub async fn series_archives(
    &self,
    params: VideoCollectionSeriesArchivesParams,
) -> BpiResult<GetSeriesArchivesData>;

pub async fn online_total(
    &self,
    params: VideoOnlineTotalParams,
) -> BpiResult<OnlineTotalResponseData>;

pub async fn player_info_v2(
    &self,
    params: VideoPlayerInfoParams,
) -> BpiResult<PlayerInfoResponseData>;

pub async fn related_videos(
    &self,
    params: VideoRelatedParams,
) -> BpiResult<Vec<RelatedVideo>>;

pub async fn homepage_recommendations(
    &self,
    params: VideoHomepageRecommendationsParams,
) -> BpiResult<RcmdFeedResponseData>;

pub async fn ai_summary(
    &self,
    params: VideoAiSummaryParams,
) -> BpiResult<AiSummaryResponseData>;

pub async fn tags(
    &self,
    params: VideoTagsParams,
) -> BpiResult<Vec<VideoTag>>;

pub async fn interactive_video_info(
    &self,
    params: InteractiveVideoInfoParams,
) -> BpiResult<InteractiveVideoInfoResponseData>;
```

### Step 1: Record the non-Probe batch

Update local-only `migration-status.md` Current Batch:

```text
Batch = video/collection-player-client-bridge
Type = Explicit non-Probe domain-client bridge batch
Profiles = Not applicable; no new Probe run expected
Probe raw output = None for this bridge batch
Promoted contracts = collection-read and player-read contracts listed above
Commit = Pending
Verification = RED/GREEN pending
```

Expected status after this step:

```powershell
git status --short
```

Only source/committed docs should appear. `migration-status.md` should remain ignored.

### Step 2: RED test for the whole surface

In `src/video/client.rs`, keep/add the test:

```rust
#[test]
fn video_client_exposes_collection_and_player_read_methods() -> BpiResult<()> {
    let client = BpiClient::new()?;
    let video = client.video();

    std::mem::drop(
        video.seasons_archives_list(VideoCollectionSeasonsArchivesParams::new(
            Mid::new(4279370)?,
            SeasonId::new(4294056)?,
        )),
    );
    std::mem::drop(
        video.home_seasons_series(VideoCollectionHomeSeasonsSeriesParams::new(Mid::new(
            4279370,
        )?)),
    );
    std::mem::drop(
        video.seasons_series_list(VideoCollectionSeasonsSeriesParams::new(Mid::new(4279370)?)),
    );
    std::mem::drop(video.series_info(VideoCollectionSeriesInfoParams::new(250285)?));
    std::mem::drop(
        video.series_archives(VideoCollectionSeriesArchivesParams::new(
            Mid::new(4279370)?,
            250285,
        )?),
    );
    std::mem::drop(video.online_total(VideoOnlineTotalParams::from_bvid(
        "BV1xx411c7mD".parse()?,
        Cid::new(62131)?,
    )));
    std::mem::drop(video.player_info_v2(VideoPlayerInfoParams::from_bvid(
        "BV1xx411c7mD".parse()?,
        Cid::new(62131)?,
    )));
    std::mem::drop(video.related_videos(VideoRelatedParams::from_bvid(
        "BV1xx411c7mD".parse()?,
    )));
    std::mem::drop(video.homepage_recommendations(VideoHomepageRecommendationsParams::new()));
    std::mem::drop(video.ai_summary(VideoAiSummaryParams::from_bvid(
        "BV1xx411c7mD".parse()?,
        Cid::new(62131)?,
        928123,
    )?));
    std::mem::drop(
        video.tags(VideoTagsParams::from_bvid("BV1xx411c7mD".parse()?).cid(Cid::new(62131)?)),
    );
    std::mem::drop(
        video.interactive_video_info(InteractiveVideoInfoParams::from_aid(
            Aid::new(114347430905959)?,
            1273647,
        )?),
    );

    let source = include_str!("client.rs");
    let payload_helper = concat!(".send_", "bpi_payload");

    assert!(
        source.matches(payload_helper).count() >= 17,
        "VideoClient should use payload helpers for info, playurl, collection, and player read methods"
    );
    Ok(())
}
```

Run:

```powershell
cargo test --all-features --lib video::client::tests::video_client_exposes_collection_and_player_read_methods --quiet
```

Expected RED: compile error or test failure because the new `VideoClient` methods do not exist yet or payload helper count is too low.

### Step 3: Expose endpoint constants for reuse

Change the endpoint constants used by the new `VideoClient` methods from private to crate-visible:

```rust
pub(crate) const SEASONS_ARCHIVES_LIST_ENDPOINT: &str = "...";
pub(crate) const HOME_SEASONS_SERIES_ENDPOINT: &str = "...";
pub(crate) const SEASONS_SERIES_LIST_ENDPOINT: &str = "...";
pub(crate) const SERIES_INFO_ENDPOINT: &str = "...";
pub(crate) const SERIES_ARCHIVES_ENDPOINT: &str = "...";
pub(crate) const ONLINE_TOTAL_ENDPOINT: &str = "...";
pub(crate) const PLAYER_INFO_V2_ENDPOINT: &str = "...";
pub(crate) const RELATED_VIDEOS_ENDPOINT: &str = "...";
pub(crate) const HOMEPAGE_RECOMMENDATIONS_ENDPOINT: &str = "...";
pub(crate) const AI_SUMMARY_ENDPOINT: &str = "...";
pub(crate) const TAGS_ENDPOINT: &str = "...";
pub(crate) const INTERACTIVE_INFO_ENDPOINT: &str = "...";
```

Do not change endpoint strings.

### Step 4: Implement collection read methods in `VideoClient`

Use existing request construction exactly:

```rust
let params = self.client.get_wbi_sign2(params.query_pairs()).await?;
self.client
    .get(SEASONS_ARCHIVES_LIST_ENDPOINT)
    .with_bilibili_headers()
    .query(&params)
    .send_bpi_payload("video.collection.seasons_archives_list")
    .await
```

Use WBI for:

```text
seasons_archives_list
home_seasons_series
seasons_series_list
```

No WBI for:

```text
series_info
series_archives
```

### Step 5: Implement player read methods in `VideoClient`

Use WBI for:

```text
player_info_v2
homepage_recommendations
ai_summary
```

No WBI for:

```text
online_total
related_videos
tags
interactive_video_info
```

Each method must return `BpiResult<T>` payload, not `BpiResponse<T>`.

### Step 6: Focused verification

Run:

```powershell
cargo test --all-features --lib video::client::tests::video_client_exposes_collection_and_player_read_methods --quiet
cargo test --all-features --lib video::client --quiet
cargo test --all-features --lib video::collection --quiet
cargo test --all-features --lib video::online --quiet
cargo test --all-features --lib video::player --quiet
cargo test --all-features --lib video::recommend --quiet
cargo test --all-features --lib video::summary --quiet
cargo test --all-features --lib video::tags --quiet
cargo test --all-features --lib video::interact_video --quiet
cargo test --all-features --lib video --quiet
```

Expected: all pass, with only intentionally ignored legacy live tests skipped.

### Step 7: Full verification

Run:

```powershell
cargo fmt --check
git diff --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features --lib --quiet
```

Expected: all exit 0.

### Step 8: Update deck and local board once

Update:

```text
flightdeck/work/bpi-rs-0.2-migration/index.md
flightdeck/cockpit.md
flightdeck/work/bpi-rs-0.2-migration/migration-status.md
```

Record:

```text
Completed explicit non-Probe video/collection-player-client-bridge
No Probe run or contract promotion expected
Contracts reused from collection-read and player-read
Mutating action/report/collection-action endpoints excluded
Verification commands and results
```

Do not stage `migration-status.md`.

### Step 9: One commit for the whole batch

Run:

```powershell
git add src/video/client.rs src/video/collection/info.rs src/video/online.rs src/video/player.rs src/video/recommend.rs src/video/summary.rs src/video/tags.rs src/video/interact_video.rs flightdeck/cockpit.md flightdeck/work/bpi-rs-0.2-migration/index.md flightdeck/work/bpi-rs-0.2-migration/plans/batch-todolist.md
git commit -m "feat(video): add collection player client bridge"
```

After commit, update local-only `migration-status.md` commit hash.

Final checks:

```powershell
git status --short
git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
git show --stat --oneline --decorate --no-renames HEAD
```

## Later Candidate Batches

### Candidate: `manga/download-read`

**Type:** Not implemented in the current migration.

**Current state:** Deferred. Browser/Playwright proved the reader can fetch both the
free and VIP-owned sample chapters, but the SDK cannot generate the required current
reader proof fields (`m2` for `GetImageIndex`, `m1` for `ImageToken`). The old private
helper implementation has been removed from `src/manga/download.rs`; that module now
keeps response models only and documents the feature as not implemented.

Do not select this as normal continuation work. Reopen it only as a dedicated
proof-provider/API-design batch.

### Candidate: gated mutating batches

**Type:** Probe-backed only with explicit user selection and safety controls.

**Examples:**

```text
comment/action
fav/action
video/action
video/report
user/relation/action
live mutations
login SMS/password/cookie refresh/logout
```

These require explicit gating such as `BPI_MUTATING_TEST=1` and must not be selected by default.

## Self-Review

- The plan preserves the active goal's default: Probe-backed first, non-Probe only when explicitly recorded.
- The chosen immediate batch is non-Probe because the endpoint audit found no new safe Probe read batch.
- The plan avoids repeating completed `video/info-read`; it targets collection/player read contracts that are already promoted but not yet bridged through `VideoClient`.
- The plan uses one commit for the whole batch.
- `migration-status.md` stays local-only and ignored.

## Batch 2: `live/remaining-read-client-bridge`

**Type:** Explicit non-Probe domain-client bridge batch.

**Status:** Completed and committed in `95b56d5 feat(live): add remaining read client bridge`.

**Why this batch:** `shared-core/module-client-coverage-audit` inventory found that `LiveClient` exposes only the already completed public-core live contracts, while promoted live read contracts also exist for gift, room interaction, account-private, guard, moderation-private, and telemetry read batches. These are already proven by promoted contracts and fixture/model tests; no new Probe run is expected.

**Promoted contracts reused:**

```text
tests/contracts/live/gift-read/gift-types/contract.json
tests/contracts/live/gift-read/room-gift-list/contract.json
tests/contracts/live/gift-read/blind-gift-info/contract.json
tests/contracts/live/room-interaction-read/danmu-info/contract.json
tests/contracts/live/room-interaction-read/emoticons/contract.json
tests/contracts/live/room-interaction-read/lottery-info/contract.json
tests/contracts/live/account-private-read/my-medals/contract.json
tests/contracts/live/account-private-read/follow-up-list/contract.json
tests/contracts/live/account-private-read/follow-up-web-list/contract.json
tests/contracts/live/account-private-read/replay-list/contract.json
tests/contracts/live/guard-read/guard-list/contract.json
tests/contracts/live/moderation-private-read/silent-users/contract.json
tests/contracts/live/moderation-private-read/banned-users/contract.json
tests/contracts/live/moderation-private-read/shield-keywords/contract.json
tests/contracts/live/telemetry-read/heartbeat/contract.json
```

**Excluded:**

```text
src/live/manage.rs create/update/start/stop/update-room-news mutations
src/live/danmaku.rs live_send_danmu
src/live/silent_user_manage.rs add/delete silent-user, banned-user, and shield-keyword mutations
src/live/report.rs live_web_heart_beat remains read/telemetry only; no report mutation is included
src/live/message_stream.rs and other non-contracted helper/stub files
```

### Public methods to add to `LiveClient`

```rust
pub async fn gift_types(&self) -> BpiResult<Vec<GiftTypeItem>>;
pub async fn room_gift_list(
    &self,
    room_id: i64,
    area_parent_id: Option<i32>,
    area_id: Option<i32>,
) -> BpiResult<RoomGiftData>;
pub async fn blind_gift_info(&self, gift_id: i64) -> BpiResult<BlindGiftData>;
pub async fn danmu_info(&self, room_id: u64, info_type: u8) -> BpiResult<LiveDanmuInfoData>;
pub async fn emoticons(&self, room_id: i64, platform: &str) -> BpiResult<EmoticonData>;
pub async fn lottery_info(&self, room_id: i64) -> BpiResult<LotteryInfoData>;
pub async fn my_medals(&self, page: i32, page_size: i32) -> BpiResult<MyMedalsData>;
pub async fn follow_up_list(
    &self,
    page: Option<i32>,
    page_size: Option<i32>,
    ignore_record: Option<i32>,
    hit_ab: Option<bool>,
) -> BpiResult<FollowUpLiveData>;
pub async fn follow_up_web_list(&self, hit_ab: Option<bool>) -> BpiResult<LiveWebListData>;
pub async fn replay_list(&self, page: Option<i32>, page_size: Option<i32>) -> BpiResult<ReplayListData>;
pub async fn guard_list(
    &self,
    room_id: i64,
    ruid: i64,
    page: Option<i32>,
    page_size: Option<i32>,
    typ: Option<i32>,
) -> BpiResult<GuardListData>;
pub async fn silent_users(&self, params: LiveSilentUserListParams) -> BpiResult<SilentUserListData>;
pub async fn banned_users(&self, params: LiveBannedUserListParams) -> BpiResult<BannedUserListData>;
pub async fn shield_keywords(&self, params: LiveShieldKeywordListParams) -> BpiResult<ShieldKeywordListData>;
pub async fn web_heart_beat(&self, params: LiveWebHeartBeatParams) -> BpiResult<HeartBeatData>;
```

### Verification Plan

Run RED before implementation:

```powershell
cargo test --all-features --lib live::client::tests::live_client_exposes_remaining_read_methods --quiet
```

Focused GREEN checks:

```powershell
cargo test --all-features --lib live::client --quiet
cargo test --all-features --lib live --quiet
```

Full gates:

```powershell
cargo fmt --check
git diff --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features --lib --quiet
```

Observed verification:

```text
RED: cargo test --all-features --lib live::client::tests::live_client_exposes_remaining_read_methods --quiet
     failed with 15 missing LiveClient methods.
GREEN: cargo test --all-features --lib live::client::tests::live_client_exposes_remaining_read_methods --quiet
       passed: 1 passed.
Focused: cargo test --all-features --lib live::client --quiet
         passed: 3 passed.
Focused: cargo test --all-features --lib live --quiet
         passed: 62 passed, 0 failed, 35 ignored.
Full: cargo fmt --check; git diff --check; cargo clippy --all-targets --all-features --locked -- -D warnings; cargo check --all-features; cargo test --all-features --lib --quiet
      all exited 0; lib tests passed with 929 passed, 0 failed, 292 ignored.
```

## Batch 3: `shared-core/module-client-coverage-audit`

**Type:** Non-Probe docs/code audit.

**Status:** Recorded in `plans/module-client-coverage-audit.md`; commit pending human approval.

**Why this batch:** After the video collection/player and live remaining-read bridge batches, the next useful reviewable batch is to confirm whether any promoted contracts still lack module-client coverage before selecting more bridge work. This avoids continuing function-sized edits without a known gap.

**Evidence:**

```text
Total promoted contracts: 206
Only promoted contract name absent from src/**/client.rs labels: login.qr.flow
```

**Conclusion:** `login.qr.flow` is a Probe flow contract, not a single endpoint method. It composes `login.qr_generate` and `login.qr_poll` with runtime `qrcode_key` extraction. `LoginClient` already exposes `qr_generate()` and `qr_poll(LoginQrPollParams)`, and the flow contract remains covered by Probe flow tests. No new safe non-Probe module-client bridge batch was discovered.

**Files:**

- Add: `flightdeck/work/bpi-rs-0.2-migration/plans/module-client-coverage-audit.md`
- Modify: `flightdeck/work/bpi-rs-0.2-migration/plans/batch-todolist.md`
- Modify: `flightdeck/work/bpi-rs-0.2-migration/index.md`
- Modify local only: `flightdeck/work/bpi-rs-0.2-migration/migration-status.md`

**Verification plan:**

```powershell
git diff --check
git status --short
```

No cargo gate is required for this batch because no Rust source, tests, or contracts are changed. If Rust files become dirty, run the full gates from `api-upgrade-protocol.md`.

## Batch 4: `docs/readme-module-api-refresh`

**Type:** Non-Probe documentation batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** Endpoint promotion and module-client bridge coverage are complete for safe read surfaces, but `README.md` still describes the old 0.1 flat API, `BpiClient::new()` without `Result`, envelope-first `BpiResponse<T>` examples, and library-owned logging behavior. The 0.2 migration design requires README/examples to match the new module-client API.

**Scope:**

- Rewrite README quick start around `BpiClient::builder()` and module clients.
- Show authenticated setup through `.cookie(...)`, `.account(...)`, and `set_account_from_cookie_str(...)`.
- Show QR login through `client.login().qr_generate()` and `client.login().qr_poll(...)`.
- Explain ordinary module-client methods return `BpiResult<T>` payloads while legacy/custom request helpers may still return `BpiResponse<T>`.
- Update local development/test guidance so default checks are offline and Probe/live credentials remain local.

**Excluded:**

- No Rust source changes.
- No Probe run.
- No API behavior changes.
- No committed account data or raw Probe output.

**Verification plan:**

```powershell
git diff --check
rg "BpiClient::new\\(\\);|\\.data\\.unwrap\\(|login_send_qrcode|login_check_qrcode_status|bangumi_info\\(" README.md
```

Cargo gates are not required for this docs-only batch unless Rust source or examples in compiled docs are changed.

**Observed verification:**

```text
git diff --check
  passed with only CRLF conversion warnings.

rg "BpiClient::new\(\);|\.data\.unwrap\(|login_send_qrcode|login_check_qrcode_status|bangumi_info\(" README.md
  no matches.

rg -n "BpiClient::new\(\);|\.data\.unwrap\(|login_send_qrcode|login_check_qrcode_status|bangumi_info\(|开始从账号信息加载cookies|统一结构体" README.md
  no matches.
```

Cargo gates were not run for this docs-only batch because no Rust source, tests, contracts, or compiled rustdoc examples changed.

## Batch 6: `examples/module-client-quickstart`

**Type:** Non-Probe example/documentation batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** README and `docs/migration-0.2.md` now document the 0.2 module-client API, but the repository did not have a compile-checked runnable example. A small `examples/` quickstart keeps the public usage path honest without introducing another endpoint or function-sized code batch.

**Scope:**

- Add `examples/module_clients.rs`.
- Keep the example offline by default; it constructs the client and typed params but only calls live APIs when `BPI_RUN_EXAMPLE=1`.
- Support optional authenticated setup through `BPI_COOKIE` without printing or storing secrets.
- Link the example from README and document the check/run commands.

**Excluded:**

- No Probe run.
- No contract promotion.
- No API behavior changes.
- No committed credentials, raw Probe output, or account-specific data.
- No `flightdeck/cockpit.md` progress update; cockpit `Next` stays a stable task index.

**Verification plan:**

```powershell
cargo fmt --check
cargo check --all-features --examples
git diff --check
rg "BpiClient::new\(\);|\.data\.unwrap\(|login_send_qrcode|login_check_qrcode_status|bangumi_info\(" README.md docs\migration-0.2.md examples\module_clients.rs
git diff -- flightdeck/cockpit.md
```

Full library gates are not required unless this batch changes `src/`, `tests/contracts/`, or library behavior. The example gate compiles all example code with all feature-gated module clients enabled.

**Observed verification:**

```text
cargo fmt --check
  passed.

cargo check --all-features --examples
  passed.

git diff --check
  passed with only CRLF conversion warnings.

rg "BpiClient::new\(\);|\.data\.unwrap\(|login_send_qrcode|login_check_qrcode_status|bangumi_info\(" README.md docs\migration-0.2.md examples\module_clients.rs
  no matches.

git diff -- flightdeck/cockpit.md
  no diff.
```

Full library gates were not run because this batch only adds a compiled example plus docs/topic metadata and does not change `src/`, `tests/contracts/`, or library behavior.

## Batch 7: `examples/stale-binary-quickstart-refresh`

**Type:** Non-Probe release-cleanup source/example batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** A stale API audit after Batch 6 found `src/main.rs` still demonstrates the old flat `bangumi_info(...)` method and envelope-style `result.data` handling. This is executable source, so it should be refreshed to the 0.2 module-client quickstart style instead of leaving a compiled old-API example in the crate.

**Pre-check evidence:**

```text
rg -n "bangumi_info\(|login_send_qrcode|login_check_qrcode_status|video_info\(|video_playurl\(" src\main.rs README.md docs\migration-0.2.md examples\module_clients.rs
  src\main.rs:9:        .bangumi_info(BangumiInfoParams::new(MediaId::new(28220978)?))
```

**Scope:**

- Update `src/main.rs` to use `client.bangumi().info(...)`.
- Return `BpiResult<()>` from the binary entry point.
- Keep the binary offline by default; only call live APIs when `BPI_RUN_EXAMPLE=1`.
- Keep README/docs/example stale-API checks clean.

**Excluded:**

- No Probe run.
- No contract promotion.
- No domain API behavior change.
- No `flightdeck/cockpit.md` update.

**Verification plan:**

```powershell
cargo fmt --check
cargo check --all-features --bins
rg -n "bangumi_info\(|login_send_qrcode|login_check_qrcode_status|video_info\(|video_playurl\(" src\main.rs README.md docs\migration-0.2.md examples\module_clients.rs
git diff --check
git diff -- flightdeck/cockpit.md
```

Full library gates are not required unless this batch changes library modules, contracts, or tests. The binary check compiles `src/main.rs` and `src/bin/bpi-probe.rs` with all features enabled.

**Observed verification:**

```text
cargo fmt --check
  passed.

cargo check --all-features --bins
  passed.

rg -n "bangumi_info\(|login_send_qrcode|login_check_qrcode_status|video_info\(|video_playurl\(" src\main.rs README.md docs\migration-0.2.md examples\module_clients.rs
  no matches.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.
```

Full library gates were not run because this batch changes only the executable quickstart in `src/main.rs` and topic/docs metadata, with no library module, contract, or test behavior change.

## Batch 8: `release/rustdoc-example-audit`

**Type:** Non-Probe release verification batch.

**Status:** Verified; commit pending human approval.

**Why this batch:** `design.md` acceptance criteria and `plan.md` Stage 6 require README/rustdoc examples to compile where possible or be explicitly marked `no_run` when they require network/credentials. The previous example batches covered README, `examples/`, and `src/main.rs`; this batch records the rustdoc gate itself.

**Scope:**

- Run `cargo test --doc`.
- If rustdoc examples fail because they use stale 0.1 APIs or execute network/credential flows, update the relevant doc examples in the same batch.
- Keep the batch as release verification/source-doc cleanup, not a domain endpoint migration.

**Excluded:**

- No Probe run.
- No contract promotion.
- No library behavior change unless a failing rustdoc example exposes stale public documentation that must be corrected.
- No `flightdeck/cockpit.md` update.

**Verification plan:**

```powershell
cargo test --doc
cargo fmt --check
git diff --check
git diff -- flightdeck/cockpit.md
```

If doc examples require source-doc edits, also run the focused compile gate that covers the edited target.

**Observed verification:**

```text
cargo test --doc
  passed; rustdoc reported 0 doc tests.

cargo fmt --check
  passed.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.
```

No source-doc edits were needed because the rustdoc gate passed. The 0-test result means the current public rustdoc has no executable doc examples; README and `examples/` coverage remain the user-facing example checks for now.

## Batch 9: `release/current-stack-full-gate`

**Type:** Non-Probe release verification batch.

**Status:** Verified; commit pending human approval.

**Why this batch:** The current working tree contains multiple completed non-Probe docs/example/source cleanup batches waiting for human-approved commit. Running the project default full gate once over the accumulated stack gives stronger review evidence than relying only on per-batch focused checks.

**Scope:**

- Run `task check_all` if the Task runner is available.
- If `task` is unavailable, run the equivalent direct commands from `Taskfile.yml`.
- Also keep the example/binary/doc gates from the recent release-cleanup batches covered.
- Confirm raw Probe/status artifacts remain untracked/ignored and `flightdeck/cockpit.md` has no diff.

**Excluded:**

- No Probe run.
- No contract promotion.
- No API behavior change.
- No `flightdeck/cockpit.md` update.

**Verification plan:**

```powershell
task check_all
cargo check --all-features --examples
cargo check --all-features --bins
cargo test --doc
git diff --check
git diff -- flightdeck/cockpit.md
git status --short
git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
```

**Observed verification:**

```text
task check_all
  passed.
  cargo fmt --check passed.
  cargo clippy --all-targets --all-features --locked -- -D warnings passed.
  cargo check --all-features passed.
  cargo test --all-features --lib passed with 929 passed, 0 failed, 292 ignored.

cargo check --all-features --examples
  passed.

cargo check --all-features --bins
  passed.

cargo test --doc
  passed; rustdoc reported 0 doc tests.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

rg stale old quickstart/API patterns across README, docs/migration-0.2.md, examples/module_clients.rs, and src/main.rs
  no matches.

git status --short
  tracked/untracked working-tree changes remain uncommitted for human review.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 10: `shared-core/legacy-flat-api-surface-audit`

**Type:** Non-Probe release cleanup audit batch.

**Status:** Verified; commit pending human approval.

**Why this batch:** Stage 6 still names "Remove obsolete flat API surface" as a release-cleanup requirement. The current module-client bridges and docs now point users to the 0.2 API, but old `impl BpiClient` endpoint methods still exist. Before deleting or deprecating hundreds of methods, record a reproducible inventory and split strategy so follow-up work can be batched instead of reverting to method-sized churn.

**Scope:**

- Count public async methods still declared directly on `impl BpiClient`.
- Aggregate the count by source module.
- Separate low-risk read/helper compatibility from mutating/gated/flow-sensitive areas using a conservative heuristic.
- Record why this audit does not run Probe and why it should not remove methods by itself.
- Recommend follow-up batch boundaries.

**Excluded:**

- No Rust source changes.
- No Probe run.
- No contract promotion.
- No flat API removal in this audit batch.
- No `flightdeck/cockpit.md` update.

**Files:**

- Add: `flightdeck/work/bpi-rs-0.2-migration/plans/legacy-flat-api-surface-audit.md`
- Modify: `flightdeck/work/bpi-rs-0.2-migration/plans/batch-todolist.md`
- Modify: `flightdeck/work/bpi-rs-0.2-migration/index.md`
- Modify local only: `flightdeck/work/bpi-rs-0.2-migration/migration-status.md`

**Verification plan:**

```powershell
git diff --check
git diff -- flightdeck/cockpit.md
git status --short
```

Cargo gates are not required unless this batch changes Rust source, tests, contracts, or compiled examples.

**Observed verification:**

```text
git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short
  tracked/untracked working-tree changes remain uncommitted for human review.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

Cargo gates were not run for this batch because it only adds the audit note and updates topic metadata; no Rust source, tests, contracts, or compiled examples changed.

## Batch 11: `flat-api/remove-small-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `shared-core/legacy-flat-api-surface-audit` showed that the old flat `impl BpiClient` surface remains after module-client migration. Removing the whole surface at once would be too large to review, so start with the smallest low-risk read modules whose module-client replacements already exist and whose remaining old call sites are only legacy ignored live tests.

**Scope:**

- Remove `BpiClient::clientinfo_ip`.
- Remove `BpiClient::wallet_info`.
- Remove `BpiClient::opus_space_feed`.
- Remove `BpiClient::web_widget_region_banner`.
- Remove `BpiClient::web_widget_header_page`.
- Remove `BpiClient::web_widget_online`.
- Rewrite the corresponding ignored live tests to use `client.clientinfo().ip(...)`, `client.wallet().info(...)`, `client.opus().space_feed(...)`, and `client.web_widget().{region_banner,header_page,online}(...)`.

**Evidence reused:**

```text
tests/contracts/clientinfo/ip/contract.json
tests/contracts/wallet/read/info/contract.json
tests/contracts/opus/space-read/space-feed/contract.json
tests/contracts/web_widget/read/region-banner/contract.json
tests/contracts/web_widget/read/header-page/contract.json
tests/contracts/web_widget/read/online/contract.json
```

**Excluded:**

- No mutating or gated endpoints.
- No medium/large flat API removals.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and existing module-client bridge batches.
- No `flightdeck/cockpit.md` update; cockpit `Next` remains a stable task index.

**Pre-check evidence:**

```text
rg -n "pub async fn (clientinfo_ip|wallet_info|opus_space_feed|web_widget_region_banner|web_widget_header_page|web_widget_online)\b|\.clientinfo_ip\(|\.wallet_info\(|\.opus_space_feed\(|\.web_widget_region_banner\(|\.web_widget_header_page\(|\.web_widget_online\(" src tests examples README.md docs
  found only method definitions and their legacy ignored live-test call sites in src/clientinfo, src/wallet, src/opus, and src/web_widget.
```

**Verification plan:**

```powershell
rg -n "pub async fn (clientinfo_ip|wallet_info|opus_space_feed|web_widget_region_banner|web_widget_header_page|web_widget_online)\b|\.clientinfo_ip\(|\.wallet_info\(|\.opus_space_feed\(|\.web_widget_region_banner\(|\.web_widget_header_page\(|\.web_widget_online\(" src tests examples README.md docs
cargo test --all-features --lib clientinfo --quiet
cargo test --all-features --lib wallet --quiet
cargo test --all-features --lib opus --quiet
cargo test --all-features --lib web_widget --quiet
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "pub async fn (clientinfo_ip|wallet_info|opus_space_feed|web_widget_region_banner|web_widget_header_page|web_widget_online)\b|\.clientinfo_ip\(|\.wallet_info\(|\.opus_space_feed\(|\.web_widget_region_banner\(|\.web_widget_header_page\(|\.web_widget_online\(" src tests examples README.md docs
  no matches.

cargo test --all-features --lib clientinfo --quiet
  passed: 10 passed, 0 failed, 1 ignored.

cargo test --all-features --lib wallet --quiet
  passed: 14 passed, 0 failed, 1 ignored.

cargo test --all-features --lib opus --quiet
  passed: 11 passed, 0 failed, 3 ignored.

cargo test --all-features --lib web_widget --quiet
  passed: 17 passed, 0 failed, 3 ignored.

cargo fmt --check
  passed.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.
```

## Batch 12: `flat-api/remove-activity-message-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** Batch 11 proved the first low-risk removal slice. The remaining modules named in the audit's smallest removal group are `activity` and read-only `message` methods. Both already have payload-returning module-client replacements and promoted contracts; the remaining old call sites are legacy ignored live tests.

**Scope:**

- Remove `BpiClient::activity_info`.
- Remove `BpiClient::activity_list`.
- Remove `BpiClient::activity_list_default`.
- Remove `BpiClient::message_unread_count`.
- Remove `BpiClient::message_reply_feed`.
- Remove `BpiClient::message_single_unread`.
- Rewrite the corresponding ignored live tests to use `client.activity().{info,list,list_default}(...)` and `client.message().{unread_count,reply_feed,single_unread}(...)`.

**Evidence reused:**

```text
tests/contracts/activity/info/contract.json
tests/contracts/activity/list/contract.json
tests/contracts/message/read/unread-count/contract.json
tests/contracts/message/read/reply-feed/contract.json
tests/contracts/message/read/single-unread/contract.json
```

**Excluded:**

- `BpiClient::message_send`; this is mutating and remains gated.
- No other message, dynamic upload, login/session, or mutating surface.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and existing module-client bridge batches.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "activity_info\(|activity_list\(|activity_list_default\(|message_unread_count\(|message_reply_feed\(|message_single_unread\(" src tests examples README.md docs
  found only method definitions, an internal `activity_list_default` delegation, and legacy ignored live-test call sites.
```

**Verification plan:**

```powershell
rg -n "pub async fn (activity_info|activity_list|activity_list_default|message_unread_count|message_reply_feed|message_single_unread)\b|\.activity_info\(|\.activity_list\(|\.activity_list_default\(|\.message_unread_count\(|\.message_reply_feed\(|\.message_single_unread\(" src tests examples README.md docs
cargo test --all-features --lib activity --quiet
cargo test --all-features --lib message --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "pub async fn (activity_info|activity_list|activity_list_default|message_unread_count|message_reply_feed|message_single_unread)\b|\.activity_info\(|\.activity_list\(|\.activity_list_default\(|\.message_unread_count\(|\.message_reply_feed\(|\.message_single_unread\(" src tests examples README.md docs
  no matches.

cargo test --all-features --lib activity --quiet
  passed: 17 passed, 0 failed, 6 ignored.

cargo test --all-features --lib message --quiet
  passed: 25 passed, 0 failed, 5 ignored.

cargo fmt --check
  passed.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.
```

## Batch 13: `flat-api/remove-bangumi-cheese-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** The next medium read-shim cleanup can remove cohesive PGC course/video read surfaces that already have payload-returning module-client replacements. `bangumi` and `cheese` share the same pattern: info/detail, timeline or episode list, and playurl read methods backed by promoted contracts.

**Scope:**

- Remove `BpiClient::bangumi_info`.
- Remove `BpiClient::bangumi_detail`.
- Remove `BpiClient::bangumi_detail_by_season_id`.
- Remove `BpiClient::bangumi_detail_by_epid`.
- Remove `BpiClient::bangumi_sections_by_season_id`.
- Remove `BpiClient::bangumi_timeline`.
- Remove `BpiClient::bangumi_video_stream`.
- Remove `BpiClient::bangumi_video_stream_by_epid`.
- Remove `BpiClient::bangumi_video_stream_by_cid`.
- Remove `BpiClient::cheese_info`.
- Remove `BpiClient::cheese_info_by_season_id`.
- Remove `BpiClient::cheese_info_by_ep_id`.
- Remove `BpiClient::cheese_ep_list`.
- Remove `BpiClient::cheese_video_stream`.
- Rewrite the corresponding ignored live tests and shim-specific validation tests to use `client.bangumi()` and `client.cheese()` module clients or the underlying typed params.

**Evidence reused:**

```text
tests/contracts/bangumi/info/review-user/contract.json
tests/contracts/bangumi/info/season-detail-season/contract.json
tests/contracts/bangumi/info/season-detail-episode/contract.json
tests/contracts/bangumi/info/season-section/contract.json
tests/contracts/bangumi/timeline/contract.json
tests/contracts/bangumi/playurl/contract.json
tests/contracts/cheese/info/season-detail-season/contract.json
tests/contracts/cheese/info/season-detail-episode/contract.json
tests/contracts/cheese/info/ep-list/contract.json
tests/contracts/cheese/playurl/contract.json
```

**Excluded:**

- `BpiClient::bangumi_follow` and `BpiClient::bangumi_unfollow`; these are mutating and remain gated.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and existing module-client bridge batches.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "bangumi_info\(|bangumi_detail\(|bangumi_detail_by_season_id\(|bangumi_detail_by_epid\(|bangumi_sections_by_season_id\(|bangumi_timeline\(|bangumi_video_stream\(|bangumi_video_stream_by_epid\(|bangumi_video_stream_by_cid\(|cheese_info\(|cheese_info_by_season_id\(|cheese_info_by_ep_id\(|cheese_ep_list\(|cheese_video_stream\(" src tests examples README.md docs
  found only method definitions, internal convenience delegations, shim-specific validation, and legacy ignored live-test call sites.
```

**Verification plan:**

```powershell
rg -n "pub async fn (bangumi_info|bangumi_detail|bangumi_detail_by_season_id|bangumi_detail_by_epid|bangumi_sections_by_season_id|bangumi_timeline|bangumi_video_stream|bangumi_video_stream_by_epid|bangumi_video_stream_by_cid|cheese_info|cheese_info_by_season_id|cheese_info_by_ep_id|cheese_ep_list|cheese_video_stream)\b|\.(bangumi_info|bangumi_detail|bangumi_detail_by_season_id|bangumi_detail_by_epid|bangumi_sections_by_season_id|bangumi_timeline|bangumi_video_stream|bangumi_video_stream_by_epid|bangumi_video_stream_by_cid|cheese_info|cheese_info_by_season_id|cheese_info_by_ep_id|cheese_ep_list|cheese_video_stream)\(" src tests examples README.md docs
cargo test --all-features --lib bangumi --quiet
cargo test --all-features --lib cheese --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "pub async fn (bangumi_info|bangumi_detail|bangumi_detail_by_season_id|bangumi_detail_by_epid|bangumi_sections_by_season_id|bangumi_timeline|bangumi_video_stream|bangumi_video_stream_by_epid|bangumi_video_stream_by_cid|cheese_info|cheese_info_by_season_id|cheese_info_by_ep_id|cheese_ep_list|cheese_video_stream)\b|\.(bangumi_info|bangumi_detail|bangumi_detail_by_season_id|bangumi_detail_by_epid|bangumi_sections_by_season_id|bangumi_timeline|bangumi_video_stream|bangumi_video_stream_by_epid|bangumi_video_stream_by_cid|cheese_info|cheese_info_by_season_id|cheese_info_by_ep_id|cheese_ep_list|cheese_video_stream)\(" src tests examples README.md docs
  no matches.

cargo test --all-features --lib bangumi --quiet
  passed: 38 passed, 0 failed, 11 ignored.

cargo test --all-features --lib cheese --quiet
  passed: 19 passed, 0 failed, 4 ignored.

cargo fmt --check
  passed.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.
```

## Batch 14: `flat-api/remove-search-video-ranking-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `search` and `video_ranking` are cohesive public read modules whose promoted contracts and module-client bridges are already complete. Their remaining old flat `impl BpiClient` methods are release-cleanup shims, and current call-site search found only method definitions plus legacy ignored live-test calls.

**Scope:**

- Remove `BpiClient::search_article`.
- Remove `BpiClient::search_bangumi`.
- Remove `BpiClient::search_bili_user`.
- Remove `BpiClient::search_live`.
- Remove `BpiClient::search_live_room`.
- Remove `BpiClient::search_live_user`.
- Remove `BpiClient::search_movie`.
- Remove `BpiClient::search_video`.
- Remove `BpiClient::search_default`.
- Remove `BpiClient::search_suggest`.
- Remove `BpiClient::search_hotwords`.
- Remove `BpiClient::video_popular_list`.
- Remove `BpiClient::video_popular_series_list`.
- Remove `BpiClient::video_popular_series_one`.
- Remove `BpiClient::video_popular_precious`.
- Remove `BpiClient::video_ranking_list`.
- Remove `BpiClient::video_region_dynamic`.
- Remove `BpiClient::video_region_tag_dynamic`.
- Remove `BpiClient::video_region_newlist`.
- Remove `BpiClient::video_region_newlist_rank`.
- Rewrite corresponding ignored live tests to use `client.search()` and `client.video_ranking()` module clients.

**Evidence reused:**

```text
tests/contracts/search/read/article/contract.json
tests/contracts/search/read/bangumi/contract.json
tests/contracts/search/read/bili-user/contract.json
tests/contracts/search/read/live/contract.json
tests/contracts/search/read/live-room/contract.json
tests/contracts/search/read/live-user/contract.json
tests/contracts/search/read/movie/contract.json
tests/contracts/search/read/video/contract.json
tests/contracts/search/read/default/contract.json
tests/contracts/search/read/suggest/contract.json
tests/contracts/search/read/hotwords/contract.json
tests/contracts/video_ranking/read/popular-list/contract.json
tests/contracts/video_ranking/read/popular-series-list/contract.json
tests/contracts/video_ranking/read/popular-series-one/contract.json
tests/contracts/video_ranking/read/popular-precious/contract.json
tests/contracts/video_ranking/read/ranking-list/contract.json
tests/contracts/video_ranking/read/region-dynamic/contract.json
tests/contracts/video_ranking/read/region-tag-dynamic/contract.json
tests/contracts/video_ranking/read/region-newlist/contract.json
tests/contracts/video_ranking/read/region-newlist-rank/contract.json
```

**Excluded:**

- No mutating or gated endpoints.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and existing module-client bridge batches.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "search_article\(|search_bangumi\(|search_bili_user\(|search_live\(|search_live_room\(|search_live_user\(|search_movie\(|search_video\(|search_default\(|search_suggest\(|search_hotwords\(|video_popular_list\(|video_popular_series_list\(|video_popular_series_one\(|video_popular_precious\(|video_ranking_list\(|video_region_dynamic\(|video_region_tag_dynamic\(|video_region_newlist\(|video_region_newlist_rank\(" src tests examples README.md docs
  found only method definitions and legacy ignored live-test call sites in `src/search` and `src/video_ranking`.
```

**Verification plan:**

```powershell
rg -n "pub async fn (search_article|search_bangumi|search_bili_user|search_live|search_live_room|search_live_user|search_movie|search_video|search_default|search_suggest|search_hotwords|video_popular_list|video_popular_series_list|video_popular_series_one|video_popular_precious|video_ranking_list|video_region_dynamic|video_region_tag_dynamic|video_region_newlist|video_region_newlist_rank)\b|\.(search_article|search_bangumi|search_bili_user|search_live|search_live_room|search_live_user|search_movie|search_video|search_default|search_suggest|search_hotwords|video_popular_list|video_popular_series_list|video_popular_series_one|video_popular_precious|video_ranking_list|video_region_dynamic|video_region_tag_dynamic|video_region_newlist|video_region_newlist_rank)\(" src tests examples README.md docs
cargo test --all-features --lib search --quiet
cargo test --all-features --lib video_ranking --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "pub async fn (search_article|search_bangumi|search_bili_user|search_live|search_live_room|search_live_user|search_movie|search_video|search_default|search_suggest|search_hotwords|video_popular_list|video_popular_series_list|video_popular_series_one|video_popular_precious|video_ranking_list|video_region_dynamic|video_region_tag_dynamic|video_region_newlist|video_region_newlist_rank)\b|\.(search_article|search_bangumi|search_bili_user|search_live|search_live_room|search_live_user|search_movie|search_video|search_default|search_suggest|search_hotwords|video_popular_list|video_popular_series_list|video_popular_series_one|video_popular_precious|video_ranking_list|video_region_dynamic|video_region_tag_dynamic|video_region_newlist|video_region_newlist_rank)\(" src tests examples README.md docs
  no matches.

cargo test --all-features --lib search --quiet
  passed: 35 passed, 0 failed, 12 ignored.

cargo test --all-features --lib video_ranking --quiet
  passed: 17 passed, 0 failed, 10 ignored.

cargo fmt --check
  passed.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 15: `flat-api/remove-fav-note-history-comment-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `fav`, `note`, `historytoview`, and `comment` are medium read surfaces whose promoted contracts and module-client bridges are already complete. The remaining flat `impl BpiClient` read methods are release-cleanup shims; pre-check found only method definitions and legacy ignored live-test calls. Mutating history/to-view, favorite action, note action, and comment action methods remain excluded.

**Scope:**

- Remove `BpiClient::fav_folder_info`.
- Remove `BpiClient::fav_created_list`.
- Remove `BpiClient::fav_collected_list`.
- Remove `BpiClient::fav_resource_infos`.
- Remove `BpiClient::fav_list_detail`.
- Remove `BpiClient::fav_resource_ids`.
- Remove `BpiClient::note_is_forbid`.
- Remove `BpiClient::note_get_private_info`.
- Remove `BpiClient::note_get_public_info`.
- Remove `BpiClient::note_list_archive`.
- Remove `BpiClient::note_list_user_private`.
- Remove `BpiClient::note_list_public_archive`.
- Remove `BpiClient::note_list_public_user`.
- Remove `BpiClient::history_list`.
- Remove `BpiClient::history_shadow_get`.
- Remove `BpiClient::toview_list`.
- Remove `BpiClient::comment_list`.
- Remove `BpiClient::comment_replies`.
- Remove `BpiClient::comment_hot`.
- Remove `BpiClient::comment_count`.
- Rewrite corresponding ignored live tests to use `client.fav()`, `client.note()`, `client.historytoview()`, and `client.comment()` module clients.

**Evidence reused:**

```text
tests/contracts/fav/read/folder-info/contract.json
tests/contracts/fav/read/created-list/contract.json
tests/contracts/fav/read/collected-list/contract.json
tests/contracts/fav/read/resource-infos/contract.json
tests/contracts/fav/read/list-detail/contract.json
tests/contracts/fav/read/resource-ids/contract.json
tests/contracts/note/read/is-forbid/contract.json
tests/contracts/note/read/private-info/contract.json
tests/contracts/note/read/public-info/contract.json
tests/contracts/note/read/archive-list/contract.json
tests/contracts/note/read/user-private-list/contract.json
tests/contracts/note/read/public-archive-list/contract.json
tests/contracts/note/read/user-public-list/contract.json
tests/contracts/historytoview/read/history-list/contract.json
tests/contracts/historytoview/read/history-shadow/contract.json
tests/contracts/historytoview/read/toview-list/contract.json
tests/contracts/comment/read/list/contract.json
tests/contracts/comment/read/replies/contract.json
tests/contracts/comment/read/hot/contract.json
tests/contracts/comment/read/count/contract.json
```

**Excluded:**

- Favorite action methods in `src/fav/action.rs`.
- Note action methods in `src/note/action.rs`.
- History/to-view mutating methods: `history_delete`, `history_clear`, `history_shadow_set`, `toview_add_video`, `toview_delete`, and `toview_clear`.
- Comment action methods in `src/comment/action.rs`.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and existing module-client bridge batches.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "fav_folder_info\(|fav_created_list\(|fav_collected_list\(|fav_resource_infos\(|fav_list_detail\(|fav_resource_ids\(|note_is_forbid\(|note_get_private_info\(|note_get_public_info\(|note_list_archive\(|note_list_user_private\(|note_list_public_archive\(|note_list_public_user\(|history_list\(|history_shadow_get\(|toview_list\(|comment_list\(|comment_replies\(|comment_hot\(|comment_count\(" src tests examples README.md docs
  found only method definitions, module-client methods with different receivers, and legacy ignored live-test call sites in `src/fav`, `src/note`, `src/historytoview`, and `src/comment`.
```

**Verification plan:**

```powershell
rg -n "pub async fn (fav_folder_info|fav_created_list|fav_collected_list|fav_resource_infos|fav_list_detail|fav_resource_ids|note_is_forbid|note_get_private_info|note_get_public_info|note_list_archive|note_list_user_private|note_list_public_archive|note_list_public_user|history_list|history_shadow_get|toview_list|comment_list|comment_replies|comment_hot|comment_count)\b|\.(fav_folder_info|fav_created_list|fav_collected_list|fav_resource_infos|fav_list_detail|fav_resource_ids|note_is_forbid|note_get_private_info|note_get_public_info|note_list_archive|note_list_user_private|note_list_public_archive|note_list_public_user|history_shadow_get|toview_list|comment_list|comment_replies|comment_hot|comment_count)\(" src tests examples README.md docs
cargo test --all-features --lib fav --quiet
cargo test --all-features --lib note --quiet
cargo test --all-features --lib historytoview --quiet
cargo test --all-features --lib comment --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
Focused stale flat definition scan inside `impl BpiClient` for the 20 target methods
  no matches.

rg -n "\.(fav_folder_info|fav_created_list|fav_collected_list|fav_resource_infos|fav_list_detail|fav_resource_ids|note_is_forbid|note_get_private_info|note_get_public_info|note_list_archive|note_list_user_private|note_list_public_archive|note_list_public_user|history_shadow_get|comment_list|comment_replies|comment_hot|comment_count)\(|bpi\.history_list\(|bpi\.toview_list\(" src tests examples README.md docs
  no matches.

cargo test --all-features --lib fav --quiet
  passed: 27 passed, 0 failed, 12 ignored.

cargo test --all-features --lib note --quiet
  passed: 22 passed, 0 failed, 8 ignored.

cargo test --all-features --lib historytoview --quiet
  passed: 19 passed, 0 failed, 3 ignored.

cargo test --all-features --lib comment --quiet
  passed: 13 passed, 0 failed, 6 ignored.

cargo fmt --check
  passed.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 16: `flat-api/remove-misc-vip-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `misc` and `vip.center_info` are small, already promoted read/helper surfaces with module-client replacements. Current call-site search found only flat method definitions and legacy ignored live-test calls for the selected methods. `vip_sign` and VIP action methods remain excluded because they are disabled/mutating/gated surfaces.

**Scope:**

- Remove `BpiClient::misc_buvid3`.
- Remove `BpiClient::misc_buvid`.
- Remove `BpiClient::misc_b23_short_link`.
- Remove `BpiClient::misc_sign_bili_ticket`.
- Remove `BpiClient::misc_sign_bili_ticket_string`.
- Remove `BpiClient::vip_center_info`.
- Rewrite corresponding ignored live tests to use `client.misc()` and `client.vip()` module clients.

**Evidence reused:**

```text
tests/contracts/misc/buvid3/contract.json
tests/contracts/misc/buvid/contract.json
tests/contracts/misc/b23tv/short-link/contract.json
tests/contracts/misc/sign/bili-ticket/contract.json
tests/contracts/vip/read/center-info/contract.json
```

**Excluded:**

- `BpiClient::vip_sign` in `src/vip/clockin.rs`.
- VIP action methods in `src/vip/action.rs`.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and existing module-client bridge batches.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "misc_buvid3\(|misc_buvid\(|misc_b23_short_link\(|misc_sign_bili_ticket\(|misc_sign_bili_ticket_string\(|vip_center_info\(" src tests examples README.md docs
  found only method definitions, an internal legacy delegation from `misc_sign_bili_ticket_string` to `misc_sign_bili_ticket`, and legacy ignored live-test call sites in `src/misc` and `src/vip`.
```

**Verification plan:**

```powershell
rg -n "pub async fn (misc_buvid3|misc_buvid|misc_b23_short_link|misc_sign_bili_ticket|misc_sign_bili_ticket_string|vip_center_info)\b|\.(misc_buvid3|misc_buvid|misc_b23_short_link|misc_sign_bili_ticket|misc_sign_bili_ticket_string|vip_center_info)\(" src tests examples README.md docs
cargo test --all-features --lib misc --quiet
cargo test --all-features --lib vip --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "pub async fn (misc_buvid3|misc_buvid|misc_b23_short_link|misc_sign_bili_ticket|misc_sign_bili_ticket_string|vip_center_info)\b|\.(misc_buvid3|misc_buvid|misc_b23_short_link|misc_sign_bili_ticket|misc_sign_bili_ticket_string|vip_center_info)\(" src tests examples README.md docs
  no matches.

cargo test --all-features --lib misc --quiet
  passed: 22 passed, 0 failed, 5 ignored.

cargo test --all-features --lib vip --quiet
  passed: 24 passed, 0 failed, 4 ignored.

cargo fmt --check
  passed.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 17: `flat-api/remove-article-manga-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `article` read APIs and `manga` read-core APIs already have promoted contracts and module-client replacements. Current call-site search found only flat method definitions and legacy ignored live-test calls for the selected methods. Mutating article actions, manga buy/clock-in/share/exchange actions, and Probe-blocked manga download/read remain excluded.

**Scope:**

- Remove `BpiClient::article_info`.
- Remove `BpiClient::article_view`.
- Remove `BpiClient::article_cards`.
- Remove `BpiClient::article_articles_info`.
- Remove `BpiClient::manga_season_info`.
- Remove `BpiClient::manga_clock_in_info`.
- Remove `BpiClient::manga_user_point`.
- Remove `BpiClient::manga_point_products`.
- Remove `BpiClient::manga_coupons`.
- Rewrite corresponding ignored live tests to use `client.article()` and `client.manga()` module clients.

**Evidence reused:**

```text
tests/contracts/article/info/contract.json
tests/contracts/article/view/contract.json
tests/contracts/article/cards/contract.json
tests/contracts/article/articles/contract.json
tests/contracts/manga/read-core/season-info/contract.json
tests/contracts/manga/read-core/clock-in-info/contract.json
tests/contracts/manga/read-core/user-point/contract.json
tests/contracts/manga/read-core/point-products/contract.json
tests/contracts/manga/read-core/coupons/contract.json
```

**Excluded:**

- Article action methods in `src/article/action.rs`.
- Manga buy, clock-in mutation, point exchange, and share methods.
- Probe-blocked `manga/download-read`.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and existing module-client bridge batches.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "article_info\(|article_view\(|article_cards\(|article_articles_info\(|manga_season_info\(|manga_clock_in_info\(|manga_user_point\(|manga_point_products\(|manga_coupons\(" src tests examples README.md docs
  found only method definitions and legacy ignored live-test call sites in `src/article` and `src/manga`.
```

**Verification plan:**

```powershell
rg -n "pub async fn (article_info|article_view|article_cards|article_articles_info|manga_season_info|manga_clock_in_info|manga_user_point|manga_point_products|manga_coupons)\b|\.(article_info|article_view|article_cards|article_articles_info|manga_season_info|manga_clock_in_info|manga_user_point|manga_point_products|manga_coupons)\(" src tests examples README.md docs
cargo test --all-features --lib article --quiet
cargo test --all-features --lib manga --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "pub async fn (article_info|article_view|article_cards|article_articles_info|manga_season_info|manga_clock_in_info|manga_user_point|manga_point_products|manga_coupons)\b|\.(article_info|article_view|article_cards|article_articles_info|manga_season_info|manga_clock_in_info|manga_user_point|manga_point_products|manga_coupons)\(" src tests examples README.md docs
  no matches.

cargo test --all-features --lib article --quiet
  passed: 30 passed, 0 failed, 12 ignored.

cargo test --all-features --lib manga --quiet
  passed: 18 passed, 0 failed, 9 ignored.

cargo fmt --check
  passed after cargo fmt normalized import ordering/blank lines.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 18: `flat-api/remove-electric-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `electric` public/private read APIs already have promoted contracts and module-client replacements. Current call-site search found only flat method definitions and legacy ignored live-test call sites for the selected methods. B coin, message send, and remark reply mutations remain excluded.

**Scope:**

- Remove `BpiClient::electric_month_up_list`.
- Remove `BpiClient::electric_video_show`.
- Remove `BpiClient::electric_recharge_list`.
- Remove `BpiClient::electric_rank_recent`.
- Remove `BpiClient::electric_charge_record`.
- Remove `BpiClient::electric_upower_item_detail`.
- Remove `BpiClient::electric_charge_follow_info`.
- Remove `BpiClient::electric_upower_member_rank`.
- Remove `BpiClient::electric_remark_list`.
- Remove `BpiClient::electric_remark_detail`.
- Rewrite corresponding ignored live tests to use `client.electric()`.

**Evidence reused:**

```text
tests/contracts/electric/public-read/month-up-list/contract.json
tests/contracts/electric/public-read/video-show/contract.json
tests/contracts/electric/private-read/recharge-list/contract.json
tests/contracts/electric/private-read/rank-recent/contract.json
tests/contracts/electric/private-read/charge-record/contract.json
tests/contracts/electric/public-read/upower-item-detail/contract.json
tests/contracts/electric/private-read/charge-follow-info/contract.json
tests/contracts/electric/public-read/upower-member-rank/contract.json
tests/contracts/electric/private-read/remark-list/contract.json
tests/contracts/electric/private-read/remark-detail/contract.json
tests/contracts/electric/private-read/remark-detail/flow/normal.contract.json
tests/contracts/electric/private-read/remark-detail/flow/vip.contract.json
```

**Excluded:**

- `src/electric/bcoin.rs`.
- `BpiClient::electric_message_send` and `BpiClient::electric_remark_reply` in `src/electric/charge_msg.rs`.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and existing module-client bridge batches.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "electric_month_up_list\(|electric_video_show\(|electric_recharge_list\(|electric_rank_recent\(|electric_charge_record\(|electric_upower_item_detail\(|electric_charge_follow_info\(|electric_upower_member_rank\(|electric_remark_list\(|electric_remark_detail\(" src tests examples README.md docs
  found only method definitions and legacy ignored live-test call sites in `src/electric`.
```

**Verification plan:**

```powershell
rg -n "pub async fn (electric_month_up_list|electric_video_show|electric_recharge_list|electric_rank_recent|electric_charge_record|electric_upower_item_detail|electric_charge_follow_info|electric_upower_member_rank|electric_remark_list|electric_remark_detail)\b|\.(electric_month_up_list|electric_video_show|electric_recharge_list|electric_rank_recent|electric_charge_record|electric_upower_item_detail|electric_charge_follow_info|electric_upower_member_rank|electric_remark_list|electric_remark_detail)\(" src tests examples README.md docs
cargo test --all-features --lib electric --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "pub async fn (electric_month_up_list|electric_video_show|electric_recharge_list|electric_rank_recent|electric_charge_record|electric_upower_item_detail|electric_charge_follow_info|electric_upower_member_rank|electric_remark_list|electric_remark_detail)\b|\.(electric_month_up_list|electric_video_show|electric_recharge_list|electric_rank_recent|electric_charge_record|electric_upower_item_detail|electric_charge_follow_info|electric_upower_member_rank|electric_remark_list|electric_remark_detail)\(" src tests examples README.md docs
  no matches.

cargo test --all-features --lib electric --quiet
  passed: 28 passed, 0 failed, 14 ignored.

cargo fmt --check
  passed after cargo fmt normalized one chain formatting.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 19: `flat-api/remove-audio-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `audio` info/status/stream/music-list/rank read APIs already have promoted contracts and module-client replacements. Current call-site search found only flat method definitions and legacy ignored live-test call sites for the selected methods. Collection mutation, coin mutation, and rank subscription methods remain excluded.

**Scope:**

- Remove `BpiClient::audio_info`.
- Remove `BpiClient::audio_tags`.
- Remove `BpiClient::audio_members`.
- Remove `BpiClient::audio_lyric`.
- Remove `BpiClient::audio_status_number`.
- Remove `BpiClient::audio_collection_status`.
- Remove `BpiClient::audio_coin_count`.
- Remove `BpiClient::audio_stream_url_web`.
- Remove `BpiClient::audio_stream_url`.
- Remove `BpiClient::audio_collections_list`.
- Remove `BpiClient::audio_collection_info`.
- Remove `BpiClient::audio_hot_menu`.
- Remove `BpiClient::audio_rank_menu`.
- Remove `BpiClient::audio_rank_period`.
- Remove `BpiClient::audio_rank_detail`.
- Remove `BpiClient::audio_rank_music_list`.
- Rewrite corresponding ignored live tests to use `client.audio()`.

**Evidence reused:**

```text
tests/contracts/audio/info/contract.json
tests/contracts/audio/tags/contract.json
tests/contracts/audio/members/contract.json
tests/contracts/audio/lyric/contract.json
tests/contracts/audio/status-number/contract.json
tests/contracts/audio/collection-status/contract.json
tests/contracts/audio/coin-count/contract.json
tests/contracts/audio/stream-url-web/contract.json
tests/contracts/audio/stream-url/contract.json
tests/contracts/audio/collections-list/contract.json
tests/contracts/audio/collection-info/contract.json
tests/contracts/audio/hot-menu/contract.json
tests/contracts/audio/rank-menu/contract.json
tests/contracts/audio/rank-period/contract.json
tests/contracts/audio/rank-detail/contract.json
tests/contracts/audio/rank-music-list/contract.json
```

**Excluded:**

- `BpiClient::audio_collection_to_fav`.
- `BpiClient::audio_collection_to`.
- `BpiClient::audio_coin`.
- `BpiClient::audio_rank_subscribe`.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and existing module-client bridge batches.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "audio_info\(|audio_tags\(|audio_members\(|audio_lyric\(|audio_status_number\(|audio_collection_status\(|audio_coin_count\(|audio_stream_url_web\(|audio_stream_url\(|audio_collections_list\(|audio_collection_info\(|audio_hot_menu\(|audio_rank_menu\(|audio_rank_period\(|audio_rank_detail\(|audio_rank_music_list\(" src tests examples README.md docs
  found only method definitions and legacy ignored live-test call sites in `src/audio`.
```

**Verification plan:**

```powershell
rg -n "pub async fn (audio_info|audio_tags|audio_members|audio_lyric|audio_status_number|audio_collection_status|audio_coin_count|audio_stream_url_web|audio_stream_url|audio_collections_list|audio_collection_info|audio_hot_menu|audio_rank_menu|audio_rank_period|audio_rank_detail|audio_rank_music_list)\b|\.(audio_info|audio_tags|audio_members|audio_lyric|audio_status_number|audio_collection_status|audio_coin_count|audio_stream_url_web|audio_stream_url|audio_collections_list|audio_collection_info|audio_hot_menu|audio_rank_menu|audio_rank_period|audio_rank_detail|audio_rank_music_list)\(" src tests examples README.md docs
cargo test --all-features --lib audio --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "pub async fn (audio_info|audio_tags|audio_members|audio_lyric|audio_status_number|audio_collection_status|audio_coin_count|audio_stream_url_web|audio_stream_url|audio_collections_list|audio_collection_info|audio_hot_menu|audio_rank_menu|audio_rank_period|audio_rank_detail|audio_rank_music_list)\b|\.(audio_info|audio_tags|audio_members|audio_lyric|audio_status_number|audio_collection_status|audio_coin_count|audio_stream_url_web|audio_stream_url|audio_collections_list|audio_collection_info|audio_hot_menu|audio_rank_menu|audio_rank_period|audio_rank_detail|audio_rank_music_list)\(" src tests examples README.md docs
  no matches.

cargo test --all-features --lib audio --quiet
  passed: 59 passed, 0 failed, 19 ignored.

cargo fmt --check
  passed after cargo fmt normalized one test call chain.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 20: `flat-api/remove-video-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `video` info, playurl, collection-read, and player-read APIs already have promoted contracts and module-client replacements. Current call-site search found only flat method definitions and legacy ignored live-test call sites for the selected methods. Mutating video action/report/collection-action endpoints remain excluded.

**Scope:**

- Remove `BpiClient::video_info`.
- Remove `BpiClient::video_detail`.
- Remove `BpiClient::video_pagelist`.
- Remove `BpiClient::video_desc`.
- Remove `BpiClient::video_playurl`.
- Remove `BpiClient::video_seasons_list`.
- Remove `BpiClient::video_series_list`.
- Remove `BpiClient::video_seasons_series_list`.
- Remove `BpiClient::video_series_info`.
- Remove `BpiClient::video_series_archives`.
- Remove `BpiClient::video_online_total`.
- Remove `BpiClient::video_player_info_v2`.
- Remove `BpiClient::video_related_videos`.
- Remove `BpiClient::video_homepage_recommendations`.
- Remove `BpiClient::video_ai_summary`.
- Remove `BpiClient::video_tags`.
- Remove `BpiClient::video_interactive_video_info`.
- Rewrite corresponding ignored live tests to use `client.video()`.

**Evidence reused:**

```text
tests/contracts/video/info-read/view/contract.json
tests/contracts/video/info-read/detail/contract.json
tests/contracts/video/info-read/pagelist/contract.json
tests/contracts/video/info-read/desc/contract.json
tests/contracts/video/playurl/play-url/contract.json
tests/contracts/video/collection-read/seasons-archives-list/contract.json
tests/contracts/video/collection-read/home-seasons-series/contract.json
tests/contracts/video/collection-read/seasons-series-list/contract.json
tests/contracts/video/collection-read/series-info/contract.json
tests/contracts/video/collection-read/series-archives/contract.json
tests/contracts/video/player-read/online-total/contract.json
tests/contracts/video/player-read/player-info-v2/contract.json
tests/contracts/video/player-read/related-videos/contract.json
tests/contracts/video/player-read/homepage-recommendations/contract.json
tests/contracts/video/player-read/ai-summary/contract.json
tests/contracts/video/player-read/tags/contract.json
tests/contracts/video/player-read/interactive-info/contract.json
```

**Excluded:**

- `src/video/action.rs`.
- `src/video/report.rs`.
- `src/video/collection/action.rs`.
- `src/video/appeal.rs`, `src/video/pbp.rs`, `src/video/snapshot.rs`, and `src/video/video_zone*.rs`.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and existing module-client bridge batches.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "video_info\(|video_detail\(|video_pagelist\(|video_desc\(|video_playurl\(|video_seasons_list\(|video_series_list\(|video_seasons_series_list\(|video_series_info\(|video_series_archives\(|video_online_total\(|video_player_info_v2\(|video_related_videos\(|video_homepage_recommendations\(|video_ai_summary\(|video_tags\(|video_interactive_video_info\(" src tests examples README.md docs
  found only method definitions and legacy ignored live-test call sites in `src/video`.
```

**Verification plan:**

```powershell
rg -n "pub async fn (video_info|video_detail|video_pagelist|video_desc|video_playurl|video_seasons_list|video_series_list|video_seasons_series_list|video_series_info|video_series_archives|video_online_total|video_player_info_v2|video_related_videos|video_homepage_recommendations|video_ai_summary|video_tags|video_interactive_video_info)\b|\.(video_info|video_detail|video_pagelist|video_desc|video_playurl|video_seasons_list|video_series_list|video_seasons_series_list|video_series_info|video_series_archives|video_online_total|video_player_info_v2|video_related_videos|video_homepage_recommendations|video_ai_summary|video_tags|video_interactive_video_info)\(" src tests examples README.md docs
cargo test --all-features --lib video --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "pub async fn (video_info|video_detail|video_pagelist|video_desc|video_playurl|video_seasons_list|video_series_list|video_seasons_series_list|video_series_info|video_series_archives|video_online_total|video_player_info_v2|video_related_videos|video_homepage_recommendations|video_ai_summary|video_tags|video_interactive_video_info)\b|\.(video_info|video_detail|video_pagelist|video_desc|video_playurl|video_seasons_list|video_series_list|video_seasons_series_list|video_series_info|video_series_archives|video_online_total|video_player_info_v2|video_related_videos|video_homepage_recommendations|video_ai_summary|video_tags|video_interactive_video_info)\(" src tests examples README.md docs
  no matches.

cargo test --all-features --lib video --quiet
  passed: 107 passed, 0 failed, 50 ignored.

cargo fmt --check
  passed after cargo fmt normalized import ordering.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 21: `flat-api/remove-creativecenter-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `creativecenter` season, videos, statistics, and railgun read APIs already have promoted contracts and module-client replacements. Current call-site search found only flat method definitions, legacy ignored live-test call sites, and one ignored action live test that uses `season_info` as read setup. Mutating upload, opus delete, and season action/edit APIs remain excluded.

**Scope:**

- Remove `BpiClient::season_list`.
- Remove `BpiClient::season_info`.
- Remove `BpiClient::season_by_aid`.
- Remove `BpiClient::season_section_episodes`.
- Remove `BpiClient::up_archives_list`.
- Remove `BpiClient::up_archive_videos`.
- Remove `BpiClient::up_stat`.
- Remove `BpiClient::up_archive_compare`.
- Remove `BpiClient::up_article_stat`.
- Remove `BpiClient::up_video_trend`.
- Remove `BpiClient::up_article_trend`.
- Remove `BpiClient::up_play_source`.
- Remove `BpiClient::up_viewer_data`.
- Remove `BpiClient::up_electromagnetic_info`.
- Rewrite corresponding ignored live tests to use `client.creativecenter()`.

**Evidence reused:**

```text
tests/contracts/creativecenter/season/list/contract.json
tests/contracts/creativecenter/season/info/contract.json
tests/contracts/creativecenter/season/aid/contract.json
tests/contracts/creativecenter/season/section/contract.json
tests/contracts/creativecenter/videos/archives-list/contract.json
tests/contracts/creativecenter/videos/archive-videos/contract.json
tests/contracts/creativecenter/statistics/up-stat/contract.json
tests/contracts/creativecenter/statistics/archive-compare/contract.json
tests/contracts/creativecenter/statistics/article-stat/contract.json
tests/contracts/creativecenter/statistics/video-trend/contract.json
tests/contracts/creativecenter/statistics/article-trend/contract.json
tests/contracts/creativecenter/statistics/play-source/contract.json
tests/contracts/creativecenter/statistics/viewer-data/contract.json
tests/contracts/creativecenter/railgun-read/electromagnetic-info/contract.json
```

**Excluded:**

- `src/creativecenter/upload.rs`.
- `src/creativecenter/opus.rs`.
- `src/creativecenter/season/action.rs` mutating methods.
- `src/creativecenter/season/edit.rs`.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and existing module-client bridge batches.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "up_archives_list\(|up_archive_videos\(|up_electromagnetic_info\(|up_stat\(|up_archive_compare\(|up_article_stat\(|up_video_trend\(|up_article_trend\(|up_play_source\(|up_viewer_data\(|season_list\(|season_info\(|season_by_aid\(|season_section_episodes\(" src tests examples README.md docs
  found only method definitions, legacy ignored live-test call sites in `src/creativecenter`, and one ignored mutating action live test that uses `season_info` as setup.
```

**Verification plan:**

```powershell
rg -n "\bbpi\.(season_list|season_info|season_by_aid|season_section_episodes|up_archives_list|up_archive_videos|up_stat|up_archive_compare|up_article_stat|up_video_trend|up_article_trend|up_play_source|up_viewer_data|up_electromagnetic_info)\(|BpiClient::(season_list|season_info|season_by_aid|season_section_episodes|up_archives_list|up_archive_videos|up_stat|up_archive_compare|up_article_stat|up_video_trend|up_article_trend|up_play_source|up_viewer_data|up_electromagnetic_info)\b" src tests examples README.md docs
cargo test --all-features --lib creativecenter --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "\bbpi\.(season_list|season_info|season_by_aid|season_section_episodes|up_archives_list|up_archive_videos|up_stat|up_archive_compare|up_article_stat|up_video_trend|up_article_trend|up_play_source|up_viewer_data|up_electromagnetic_info)\(|BpiClient::(season_list|season_info|season_by_aid|season_section_episodes|up_archives_list|up_archive_videos|up_stat|up_archive_compare|up_article_stat|up_video_trend|up_article_trend|up_play_source|up_viewer_data|up_electromagnetic_info)\b" src tests examples README.md docs
  no matches.

cargo test --all-features --lib creativecenter --quiet
  passed: 36 passed, 0 failed, 22 ignored.

cargo fmt --check
  passed after cargo fmt removed blank lines left by import deletion.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 22: `flat-api/remove-danmaku-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `danmaku` JSON, protobuf/raw bytes, history XML, and XML read APIs already have promoted contracts and module-client replacements. Current call-site search found flat method definitions and legacy ignored live-test call sites for the selected methods. Mutating send/recall/buy/thumbup/report/edit endpoints remain excluded.

**Scope:**

- Remove `BpiClient::danmaku_history_dates`.
- Remove `BpiClient::danmaku_snapshot`.
- Remove `BpiClient::danmaku_thumbup_stats`.
- Remove `BpiClient::danmaku_adv_state`.
- Remove `BpiClient::danmaku_web_seg_proto`.
- Remove `BpiClient::danmaku_web_seg_wbi_proto`.
- Remove `BpiClient::danmaku_web_view_proto`.
- Remove `BpiClient::danmaku_mobile_seg_proto`.
- Remove `BpiClient::danmaku_web_history_seg_proto`.
- Remove `BpiClient::danmaku_history_xml_bytes`.
- Remove `BpiClient::danmaku_xml_list_so`.
- Remove `BpiClient::danmaku_xml_list`.
- Rewrite corresponding ignored live tests to use `client.danmaku()`.

**Evidence reused:**

```text
tests/contracts/danmaku/json-read/history-dates/contract.json
tests/contracts/danmaku/json-read/snapshot/contract.json
tests/contracts/danmaku/json-read/thumbup-stats/contract.json
tests/contracts/danmaku/json-read/adv-state/contract.json
tests/contracts/danmaku/non-json-read/web-seg/contract.json
tests/contracts/danmaku/non-json-read/web-seg-wbi/contract.json
tests/contracts/danmaku/non-json-read/web-view/contract.json
tests/contracts/danmaku/non-json-read/mobile-seg/contract.json
tests/contracts/danmaku/non-json-read/web-history-seg/contract.json
tests/contracts/danmaku/history-xml/contract.json
tests/contracts/danmaku/xml-read/list-so/contract.json
tests/contracts/danmaku/xml-read/comment-xml/contract.json
```

**Excluded:**

- `BpiClient::danmaku_send`.
- `BpiClient::danmaku_send_default`.
- `BpiClient::danmaku_recall`.
- `BpiClient::danmaku_buy_adv`.
- `BpiClient::danmaku_thumbup`.
- `BpiClient::danmaku_report`.
- `BpiClient::danmaku_edit_state`.
- `BpiClient::danmaku_edit_pool`.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and existing module-client bridge batches.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "danmaku_history_dates\(|danmaku_snapshot\(|danmaku_thumbup_stats\(|danmaku_adv_state\(|danmaku_web_seg_proto\(|danmaku_web_seg_wbi_proto\(|danmaku_web_view_proto\(|danmaku_mobile_seg_proto\(|danmaku_web_history_seg_proto\(|danmaku_history_xml_bytes\(|danmaku_xml_list_so\(|danmaku_xml_list\(" src tests examples README.md docs
  found only flat method definitions and legacy ignored live-test call sites in `src/danmaku`.
```

**Verification plan:**

```powershell
rg -n "\bbpi\.(danmaku_history_dates|danmaku_snapshot|danmaku_thumbup_stats|danmaku_adv_state|danmaku_web_seg_proto|danmaku_web_seg_wbi_proto|danmaku_web_view_proto|danmaku_mobile_seg_proto|danmaku_web_history_seg_proto|danmaku_history_xml_bytes|danmaku_xml_list_so|danmaku_xml_list)\(|BpiClient::(danmaku_history_dates|danmaku_snapshot|danmaku_thumbup_stats|danmaku_adv_state|danmaku_web_seg_proto|danmaku_web_seg_wbi_proto|danmaku_web_view_proto|danmaku_mobile_seg_proto|danmaku_web_history_seg_proto|danmaku_history_xml_bytes|danmaku_xml_list_so|danmaku_xml_list)\b" src tests examples README.md docs
cargo test --all-features --lib danmaku --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "\bbpi\.(danmaku_history_dates|danmaku_snapshot|danmaku_thumbup_stats|danmaku_adv_state|danmaku_web_seg_proto|danmaku_web_seg_wbi_proto|danmaku_web_view_proto|danmaku_mobile_seg_proto|danmaku_web_history_seg_proto|danmaku_history_xml_bytes|danmaku_xml_list_so|danmaku_xml_list)\(|BpiClient::(danmaku_history_dates|danmaku_snapshot|danmaku_thumbup_stats|danmaku_adv_state|danmaku_web_seg_proto|danmaku_web_seg_wbi_proto|danmaku_web_view_proto|danmaku_mobile_seg_proto|danmaku_web_history_seg_proto|danmaku_history_xml_bytes|danmaku_xml_list_so|danmaku_xml_list)\b" src tests examples README.md docs
  no matches.

cargo test --all-features --lib danmaku --quiet
  passed: 41 passed, 0 failed, 17 ignored.

cargo fmt --check
  passed after cargo fmt normalized import ordering.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 29: `shared-core/wbi-helper-api-boundary`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `shared-core/remaining-flat-api-boundary-audit` classified `get_wbi_sign` and `get_wbi_sign2` as helper/API decision surfaces rather than endpoint methods. Current usage is internal to module clients, legacy gated methods, Probe runner, and tests; external README/docs/examples do not call them. For the 0.2 public API, WBI signing should remain an internal request-construction concern instead of a flat public `BpiClient` API.

**Scope:**

- Remove unused `BpiClient::get_wbi_sign`, which had no production call sites after the public API boundary decision.
- Change `BpiClient::get_wbi_sign2` from public to crate-visible.
- Keep the existing signing implementation and cache behavior unchanged.
- Keep internal module-client, Probe runner, and tests compiling.

**Excluded:**

- No WBI algorithm change.
- No request contract or Probe run.
- No mutating/gated endpoint enablement.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "get_wbi_sign2?\(|BpiClient::get_wbi_sign|\.get_wbi_sign" src tests examples README.md docs
  found only internal crate usage and tests; README/docs/examples had no public call-site.
```

**Files:**

- Modify: `src/utils/wbi.rs`
- Modify: `flightdeck/work/bpi-rs-0.2-migration/plans/batch-todolist.md`
- Modify after verification: `flightdeck/work/bpi-rs-0.2-migration/index.md`
- Modify local-only: `flightdeck/work/bpi-rs-0.2-migration/migration-status.md`

**Verification plan:**

```powershell
rg -n "^\s*pub\s+async\s+fn\s+get_wbi_sign2?\(|get_wbi_sign\(" src README.md docs examples
cargo test --all-features --lib utils::wbi --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "^\s*pub\s+async\s+fn\s+get_wbi_sign2?\(|get_wbi_sign\(" src README.md docs examples
  no matches.

cargo test --all-features --lib utils::wbi --quiet
  passed: 0 passed, 0 failed, 1 ignored.

cargo fmt --check
  passed.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 924 passed, 0 failed, 291 ignored.

Remaining direct impl BpiClient public async inventory
  COUNT=107; utils has no remaining public async flat methods.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 28: `flat-api/remove-dynamic-deprecated-404-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `shared-core/remaining-flat-api-boundary-audit` found no remaining low-risk read shims, but it did identify three deprecated legacy dynamic flat methods that are non-mutating documented exceptions. They are already marked deprecated in source after observed HTTP 404 text/html responses from `vc.bilibili.com`, and the current module-client dynamic read API is already available for supported dynamic detail/feed/content paths.

**Scope:**

- Remove `BpiClient::dynamic_card_detail`.
- Remove `BpiClient::dynamic_repost_detail`.
- Remove `BpiClient::dynamic_spec_item_likes`.
- Remove tests whose only purpose is to preserve those deprecated 404 legacy flat methods.
- Keep dynamic module-client read APIs and promoted contracts unchanged.

**Excluded:**

- `dynamic_upload_pic`.
- `dynamic_create_text`.
- `dynamic_create_complex`.
- `dynamic_like`.
- `dynamic_remove_draft`.
- `dynamic_set_top`.
- `dynamic_remove_top`.
- No Probe run or contract promotion; the legacy endpoints are already documented as deprecated 404 surfaces.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "pub async fn dynamic_(card_detail|repost_detail|spec_item_likes)|dynamic_card_detail\(|dynamic_repost_detail\(|dynamic_spec_item_likes\(" src\dynamic tests examples README.md docs
  found only the three selected deprecated flat method definitions in src\dynamic.
```

**Files:**

- Modify: `src/dynamic/get_dynamic_detail.rs`
- Modify: `src/dynamic/basic_info.rs`
- Modify: `flightdeck/work/bpi-rs-0.2-migration/plans/batch-todolist.md`
- Modify after verification: `flightdeck/work/bpi-rs-0.2-migration/index.md`
- Modify local-only: `flightdeck/work/bpi-rs-0.2-migration/migration-status.md`

**Verification plan:**

```powershell
rg -n "\bbpi\.(dynamic_card_detail|dynamic_repost_detail|dynamic_spec_item_likes)\(|BpiClient::(dynamic_card_detail|dynamic_repost_detail|dynamic_spec_item_likes)\b|pub async fn dynamic_(card_detail|repost_detail|spec_item_likes)" src tests examples README.md docs
cargo test --all-features --lib dynamic --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "\bbpi\.(dynamic_card_detail|dynamic_repost_detail|dynamic_spec_item_likes)\(|BpiClient::(dynamic_card_detail|dynamic_repost_detail|dynamic_spec_item_likes)\b|pub async fn dynamic_(card_detail|repost_detail|spec_item_likes)|DynamicCardDetailParams" src tests examples README.md docs
  no matches.

cargo test --all-features --lib dynamic --quiet
  passed: 48 passed, 0 failed, 23 ignored.

cargo fmt --check
  passed after cargo fmt removed a blank line left by deleting the legacy params test.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 924 passed, 0 failed, 292 ignored.

Remaining direct impl BpiClient public async inventory
  COUNT=109; dynamic=7, all remaining dynamic methods are action/publish write surfaces.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 27: `shared-core/remaining-flat-api-boundary-audit`

**Type:** Non-Probe release cleanup audit batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** After the flat read cleanup batches through `flat-api/remove-login-safe-read-flow-shims`, the old flat API surface audit is stale. Current inventory must prove whether any remaining direct `impl BpiClient` public async methods are still low-risk read shims, or whether they are gated/mutating/deprecated/Probe-blocked/helper surfaces that need explicit product/API decisions.

**Scope:**

- Recompute public async methods declared directly inside `impl BpiClient` blocks.
- Record remaining method count and module distribution.
- Classify remaining methods into gated/mutating/session, deprecated/documented exception, Probe-blocked read, and helper categories.
- Confirm no new Probe run, contract promotion, or Rust behavior change is expected.
- Keep `flightdeck/cockpit.md` unchanged.

**Evidence commands:**

```powershell
$files = rg --files src | Where-Object { $_ -like '*.rs' }
$rows = foreach ($file in $files) {
  $lines = Get-Content $file
  $inImpl = $false
  $depth = 0
  for ($i = 0; $i -lt $lines.Count; $i++) {
    $line = $lines[$i]
    if (-not $inImpl -and $line -match '^impl BpiClient\s*\{') {
      $inImpl = $true
      $depth = 0
    }
    if ($inImpl) {
      if ($line -match '^\s*pub\s+async\s+fn\s+([A-Za-z0-9_]+)') {
        $parts = $file -split '[\\/]'
        [pscustomobject]@{ Domain = $parts[1]; File = $file; Line = $i + 1; Method = $matches[1] }
      }
      $depth += ([regex]::Matches($line, '\{')).Count
      $depth -= ([regex]::Matches($line, '\}')).Count
      if ($depth -le 0) { $inImpl = $false }
    }
  }
}
$rows.Count
$rows | Group-Object Domain | Sort-Object Name
$rows | Sort-Object File,Line
```

**Observed pre-check:**

```text
Total remaining direct flat async methods: 112.
All remaining methods are in action/mutating/session/deprecated/Probe-blocked/helper files:
article action, audio action/rank subscribe, bangumi follow, comment action,
creativecenter opus/season edit/upload/action, danmaku action, dynamic action/publish plus deprecated 404 legacy detail APIs,
electric B coin/message/reply, fav action, history/to-view mutations, live danmaku/manage/moderation writes,
login SMS/logout/sign update, manga buy/clock-in/share/exchange plus Probe-blocked download-read,
message send, note add/delete, user relation/group/space notice mutations,
utils WBI helper methods, video action/report/collection action, and VIP action/clockin.
```

**Files:**

- Add: `flightdeck/work/bpi-rs-0.2-migration/plans/remaining-flat-api-boundary-audit.md`
- Modify: `flightdeck/work/bpi-rs-0.2-migration/index.md`
- Modify: `flightdeck/work/bpi-rs-0.2-migration/plans/batch-todolist.md`
- Modify local-only: `flightdeck/work/bpi-rs-0.2-migration/migration-status.md`

**Verification plan:**

```powershell
rg -n "Total remaining direct flat async methods: 112|gated/mutating/session|deprecated/documented exception|Probe-blocked read|helper" flightdeck\work\bpi-rs-0.2-migration\plans\remaining-flat-api-boundary-audit.md
git diff --check
git diff -- flightdeck/cockpit.md
git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
```

Cargo gates are not required for this docs-only audit because no Rust source, tests, contracts, or compiled rustdoc examples change.

**Observed verification:**

```text
rg -n "Total remaining direct flat async methods: 112|gated/mutating/session|deprecated/documented exception|Probe-blocked read|helper" flightdeck\work\bpi-rs-0.2-migration\plans\remaining-flat-api-boundary-audit.md
  found the expected count and boundary classifications.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 26: `flat-api/remove-login-safe-read-flow-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** Login safe read and QR/captcha flow APIs already have promoted contracts and `LoginClient` replacements. Current call-site search found selected flat read/flow method definitions and legacy ignored live-test call sites in `src/login`. SMS login, logout, signature update, password login, and cookie refresh remain flow-sensitive or mutating and stay excluded.

**Scope:**

- Remove `BpiClient::login_info_user_stat`.
- Remove `BpiClient::login_info_nav_info`.
- Remove `BpiClient::login_info_user_info`.
- Remove `BpiClient::login_info_coin`.
- Remove `BpiClient::member_center_account_info`.
- Remove `BpiClient::member_center_today_coin_exp`.
- Remove `BpiClient::member_center_daily_reward`.
- Remove `BpiClient::member_center_vip_info`.
- Remove `BpiClient::login_notice`.
- Remove `BpiClient::login_log`.
- Remove `BpiClient::login_generate_captcha`.
- Remove `BpiClient::login_send_qrcode`.
- Remove `BpiClient::login_check_qrcode_status`.
- Remove legacy flat helpers `BpiClient::is_logged_in` and `BpiClient::is_vip`.
- Rewrite corresponding ignored login tests to use `client.login()`.

**Evidence reused:**

```text
tests/contracts/login/account-info/contract.json
tests/contracts/login/captcha/generate/contract.json
tests/contracts/login/coin/contract.json
tests/contracts/login/daily-reward/contract.json
tests/contracts/login/nav/contract.json
tests/contracts/login/notice/login-log/contract.json
tests/contracts/login/notice/login-notice/contract.json
tests/contracts/login/qr/flow/contract.json
tests/contracts/login/qr/generate/contract.json
tests/contracts/login/qr/poll/contract.json
tests/contracts/login/stat/contract.json
tests/contracts/login/today-coin-exp/contract.json
tests/contracts/login/vip-info/contract.json
```

**Excluded:**

- `BpiClient::login_send_sms_code`.
- `BpiClient::login_with_sms`.
- `BpiClient::logout_web`.
- `BpiClient::member_center_update_user_sign`.
- Password login, cookie refresh, and other flow-sensitive login paths.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and the existing `login/safe-flow-client-bridge`.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "impl BpiClient|pub async fn (login_|member_center_|is_logged_in|is_vip)" src\login
  found selected flat safe read/flow definitions plus excluded SMS, logout, and sign-update methods.

rg -n "\b(login_info_user_stat|login_info_nav_info|login_info_user_info|login_info_coin|member_center_account_info|member_center_today_coin_exp|member_center_daily_reward|member_center_vip_info|login_notice|login_log|login_generate_captcha|login_send_qrcode|login_check_qrcode_status|is_logged_in|is_vip)\b|\.((login_info_user_stat|login_info_nav_info|login_info_user_info|login_info_coin|member_center_account_info|member_center_today_coin_exp|member_center_daily_reward|member_center_vip_info|login_notice|login_log|login_generate_captcha|login_send_qrcode|login_check_qrcode_status|is_logged_in|is_vip))\(" src tests examples README.md docs flightdeck\work\bpi-rs-0.2-migration\plans
  found selected flat method definitions and legacy ignored login-test call sites in `src/login`; docs references are historical migration examples.
```

**Verification plan:**

```powershell
rg -n "\bbpi\.(login_info_user_stat|login_info_nav_info|login_info_user_info|login_info_coin|member_center_account_info|member_center_today_coin_exp|member_center_daily_reward|member_center_vip_info|login_notice|login_log|login_generate_captcha|login_send_qrcode|login_check_qrcode_status|is_logged_in|is_vip)\(|BpiClient::(login_info_user_stat|login_info_nav_info|login_info_user_info|login_info_coin|member_center_account_info|member_center_today_coin_exp|member_center_daily_reward|member_center_vip_info|login_notice|login_log|login_generate_captcha|login_send_qrcode|login_check_qrcode_status|is_logged_in|is_vip)\b" src tests examples README.md docs
rg -n "pub async fn (login_info_user_stat|login_info_nav_info|login_info_user_info|login_info_coin|member_center_account_info|member_center_today_coin_exp|member_center_daily_reward|member_center_vip_info|login_notice|login_log|login_generate_captcha|login_send_qrcode|login_check_qrcode_status|is_logged_in|is_vip)\(" src\login
cargo test --all-features --lib login --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "\bbpi\.(login_info_user_stat|login_info_nav_info|login_info_user_info|login_info_coin|member_center_account_info|member_center_today_coin_exp|member_center_daily_reward|member_center_vip_info|login_notice|login_log|login_generate_captcha|login_send_qrcode|login_check_qrcode_status|is_logged_in|is_vip)\(|BpiClient::(login_info_user_stat|login_info_nav_info|login_info_user_info|login_info_coin|member_center_account_info|member_center_today_coin_exp|member_center_daily_reward|member_center_vip_info|login_notice|login_log|login_generate_captcha|login_send_qrcode|login_check_qrcode_status|is_logged_in|is_vip)\b" src tests examples README.md docs
  no matches.

rg -n "pub async fn (login_info_user_stat|login_info_nav_info|login_info_user_info|login_info_coin|member_center_account_info|member_center_today_coin_exp|member_center_daily_reward|member_center_vip_info|login_notice|login_log|login_generate_captcha|login_send_qrcode|login_check_qrcode_status|is_logged_in|is_vip)\(" src\login
  no matches.

cargo test --all-features --lib login --quiet
  passed: 77 passed, 0 failed, 11 ignored.

cargo fmt --check
  passed after cargo fmt normalized import ordering.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.
```

## Batch 25: `flat-api/remove-live-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `live` read, account-private read, moderation-private read, guard, and telemetry APIs already have promoted contracts and `LiveClient` replacements. Current call-site search found selected flat read method definitions and legacy ignored live-test call sites in `src/live`. Room management, send danmu, and moderation mutation methods remain excluded.

**Scope:**

- Remove `BpiClient::live_area_list`.
- Remove `BpiClient::live_room_info`.
- Remove `BpiClient::live_stream`.
- Remove `BpiClient::live_recommend`.
- Remove `BpiClient::live_version`.
- Remove `BpiClient::live_gift_types`.
- Remove `BpiClient::live_room_gift_list`.
- Remove `BpiClient::live_blind_gift_info`.
- Remove `BpiClient::live_get_danmu_info`.
- Remove `BpiClient::live_emoticons`.
- Remove `BpiClient::live_lottery_info`.
- Remove `BpiClient::live_my_medals`.
- Remove `BpiClient::live_follow_up_list`.
- Remove `BpiClient::live_follow_up_web_list`.
- Remove `BpiClient::live_replay_list`.
- Remove `BpiClient::live_guard_list`.
- Remove `BpiClient::live_list_silent_users`.
- Remove `BpiClient::live_list_banned_users`.
- Remove `BpiClient::live_list_shield_keyword`.
- Remove `BpiClient::live_web_heart_beat`.
- Rewrite corresponding ignored live tests to use `client.live()`.

**Evidence reused:**

```text
tests/contracts/live/public-core/area-list/contract.json
tests/contracts/live/public-core/room-info/contract.json
tests/contracts/live/public-core/stream/contract.json
tests/contracts/live/public-core/recommend/contract.json
tests/contracts/live/public-core/version/contract.json
tests/contracts/live/gift-read/gift-types/contract.json
tests/contracts/live/gift-read/room-gift-list/contract.json
tests/contracts/live/gift-read/blind-gift-info/contract.json
tests/contracts/live/room-interaction-read/danmu-info/contract.json
tests/contracts/live/room-interaction-read/emoticons/contract.json
tests/contracts/live/room-interaction-read/lottery-info/contract.json
tests/contracts/live/account-private-read/my-medals/contract.json
tests/contracts/live/account-private-read/follow-up-list/contract.json
tests/contracts/live/account-private-read/follow-up-web-list/contract.json
tests/contracts/live/account-private-read/replay-list/contract.json
tests/contracts/live/guard-read/guard-list/contract.json
tests/contracts/live/moderation-private-read/silent-users/contract.json
tests/contracts/live/moderation-private-read/banned-users/contract.json
tests/contracts/live/moderation-private-read/shield-keywords/contract.json
tests/contracts/live/telemetry-read/heartbeat/contract.json
```

**Excluded:**

- `BpiClient::live_create_room`.
- `BpiClient::live_update_room_info`.
- `BpiClient::live_stop`.
- `BpiClient::live_update_pre_live_info`.
- `BpiClient::live_update_room_news`.
- `BpiClient::live_send_danmu`.
- `BpiClient::live_add_silent_user`.
- `BpiClient::live_del_block_user`.
- `BpiClient::live_add_banned_user`.
- `BpiClient::live_del_banned_user`.
- `BpiClient::live_add_shield_keyword`.
- `BpiClient::live_del_shield_keyword`.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and the existing live client bridge batches.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "pub async fn live_|live_(area_list|room_info|stream|recommend|version|gift_types|room_gift_list|blind_gift_info|get_danmu_info|emoticons|lottery_info|my_medals|follow_up_list|follow_up_web_list|replay_list|guard_list|list_silent_users|list_banned_users|list_shield_keyword|web_heart_beat)\(" src\live tests examples README.md docs
  found selected flat read method definitions and legacy ignored live-test call sites in `src/live`; also found excluded live mutation methods.
```

**Verification plan:**

```powershell
rg -n "\bbpi\.(live_area_list|live_room_info|live_stream|live_recommend|live_version|live_gift_types|live_room_gift_list|live_blind_gift_info|live_get_danmu_info|live_emoticons|live_lottery_info|live_my_medals|live_follow_up_list|live_follow_up_web_list|live_replay_list|live_guard_list|live_list_silent_users|live_list_banned_users|live_list_shield_keyword|live_web_heart_beat)\(|BpiClient::(live_area_list|live_room_info|live_stream|live_recommend|live_version|live_gift_types|live_room_gift_list|live_blind_gift_info|live_get_danmu_info|live_emoticons|live_lottery_info|live_my_medals|live_follow_up_list|live_follow_up_web_list|live_replay_list|live_guard_list|live_list_silent_users|live_list_banned_users|live_list_shield_keyword|live_web_heart_beat)\b" src tests examples README.md docs
cargo test --all-features --lib live --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "\bbpi\.(live_area_list|live_room_info|live_stream|live_recommend|live_version|live_gift_types|live_room_gift_list|live_blind_gift_info|live_get_danmu_info|live_emoticons|live_lottery_info|live_my_medals|live_follow_up_list|live_follow_up_web_list|live_replay_list|live_guard_list|live_list_silent_users|live_list_banned_users|live_list_shield_keyword|live_web_heart_beat)\(|BpiClient::(live_area_list|live_room_info|live_stream|live_recommend|live_version|live_gift_types|live_room_gift_list|live_blind_gift_info|live_get_danmu_info|live_emoticons|live_lottery_info|live_my_medals|live_follow_up_list|live_follow_up_web_list|live_replay_list|live_guard_list|live_list_silent_users|live_list_banned_users|live_list_shield_keyword|live_web_heart_beat)\b" src tests examples README.md docs
  no matches.

rg -n "pub async fn live_(area_list|room_info|stream|recommend|version|gift_types|room_gift_list|blind_gift_info|get_danmu_info|emoticons|lottery_info|my_medals|follow_up_list|follow_up_web_list|replay_list|guard_list|list_silent_users|list_banned_users|list_shield_keyword|web_heart_beat)\(" src\live
  no matches.

cargo test --all-features --lib live --quiet
  passed: 62 passed, 0 failed, 35 ignored.

cargo fmt --check
  passed after cargo fmt removed a blank line left by live_version deletion.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.
```

## Batch 24: `flat-api/remove-user-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `user` public-read and relation-read APIs already have promoted contracts and module-client replacements. Current call-site search found selected flat read method definitions and legacy ignored live-test call sites in `src/user`. Relation group/action mutations and space notice mutation remain excluded.

**Scope:**

- Remove `BpiClient::user_relation_stat`.
- Remove `BpiClient::user_up_stat`.
- Remove `BpiClient::user_navnum`.
- Remove `BpiClient::user_album_count`.
- Remove `BpiClient::user_space_info`.
- Remove `BpiClient::user_card_info`.
- Remove `BpiClient::user_card_info_with_photo`.
- Remove `BpiClient::user_card_info_without_photo`.
- Remove `BpiClient::user_cards`.
- Remove `BpiClient::user_infos`.
- Remove `BpiClient::user_space_notice`.
- Remove `BpiClient::user_bangumi_follow_list`.
- Remove `BpiClient::user_contributed_videos`.
- Remove `BpiClient::user_medal_wall`.
- Remove `BpiClient::user_name_to_uid`.
- Remove `BpiClient::user_followings`.
- Remove `BpiClient::user_followers`.
- Remove `BpiClient::user_follow_tags`.
- Rewrite corresponding ignored live tests to use `client.user()`.

**Evidence reused:**

```text
tests/contracts/user/public-read/album-count/contract.json
tests/contracts/user/public-read/bangumi-follow-list/contract.json
tests/contracts/user/public-read/card/contract.json
tests/contracts/user/public-read/cards/contract.json
tests/contracts/user/public-read/infos/contract.json
tests/contracts/user/public-read/medal-wall/contract.json
tests/contracts/user/public-read/name-to-uid/contract.json
tests/contracts/user/public-read/nav-stat/contract.json
tests/contracts/user/public-read/relation-stat/contract.json
tests/contracts/user/public-read/space-info/contract.json
tests/contracts/user/public-read/space-notice/contract.json
tests/contracts/user/public-read/up-stat/contract.json
tests/contracts/user/public-read/uploaded-videos/contract.json
tests/contracts/user/relation-read/followings/contract.json
tests/contracts/user/relation-read/followers/contract.json
tests/contracts/user/relation-read/follow-tags/contract.json
```

**Excluded:**

- `BpiClient::user_space_notice_set`.
- `BpiClient::user_modify_relation`.
- `BpiClient::user_group_create_tag`.
- `BpiClient::user_group_update_tag`.
- `BpiClient::user_group_delete_tag`.
- `BpiClient::user_group_add_users_to_tags`.
- `BpiClient::user_group_remove_users_`.
- `BpiClient::user_group_copy_users_to_tags`.
- `BpiClient::user_group_move_users_to_tags`.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and the existing `user/module-client-bridge`.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "user_relation_stat\(|user_up_stat\(|user_navnum\(|user_album_count\(|user_space_info\(|user_card_info\(|user_card_info_with_photo\(|user_card_info_without_photo\(|user_cards\(|user_infos\(|user_space_notice\(|user_bangumi_follow_list\(|user_contributed_videos\(|user_medal_wall\(|user_name_to_uid\(|user_followings\(|user_followers\(|user_follow_tags\(" src\user tests examples README.md docs
  found selected flat read method definitions and legacy ignored live-test call sites in `src/user`.

PowerShell impl scan under src\user
  found selected 18 read methods plus excluded mutation methods.
```

**Verification plan:**

```powershell
rg -n "\bbpi\.(user_relation_stat|user_up_stat|user_navnum|user_album_count|user_space_info|user_card_info|user_card_info_with_photo|user_card_info_without_photo|user_cards|user_infos|user_space_notice|user_bangumi_follow_list|user_contributed_videos|user_medal_wall|user_name_to_uid|user_followings|user_followers|user_follow_tags)\(|BpiClient::(user_relation_stat|user_up_stat|user_navnum|user_album_count|user_space_info|user_card_info|user_card_info_with_photo|user_card_info_without_photo|user_cards|user_infos|user_space_notice|user_bangumi_follow_list|user_contributed_videos|user_medal_wall|user_name_to_uid|user_followings|user_followers|user_follow_tags)\b" src tests examples README.md docs
cargo test --all-features --lib user --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "\bbpi\.(user_relation_stat|user_up_stat|user_navnum|user_album_count|user_space_info|user_card_info|user_card_info_with_photo|user_card_info_without_photo|user_cards|user_infos|user_space_notice|user_bangumi_follow_list|user_contributed_videos|user_medal_wall|user_name_to_uid|user_followings|user_followers|user_follow_tags)\(|BpiClient::(user_relation_stat|user_up_stat|user_navnum|user_album_count|user_space_info|user_card_info|user_card_info_with_photo|user_card_info_without_photo|user_cards|user_infos|user_space_notice|user_bangumi_follow_list|user_contributed_videos|user_medal_wall|user_name_to_uid|user_followings|user_followers|user_follow_tags)\b" src tests examples README.md docs
  no matches.

PowerShell impl scan under src\user
  only excluded mutating/gated methods remain: `user_space_notice_set`, `user_group_create_tag`, `user_group_update_tag`, `user_group_delete_tag`, `user_group_add_users_to_tags`, `user_group_remove_users_`, `user_group_copy_users_to_tags`, `user_group_move_users_to_tags`, and `user_modify_relation`.

cargo test --all-features --lib user --quiet
  passed: 98 passed, 0 failed, 48 ignored.

cargo fmt --check
  passed after cargo fmt normalized import ordering and spacing.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 23: `flat-api/remove-dynamic-read-shims`

**Type:** Non-Probe release cleanup source batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `dynamic` feed/detail/content/lottery read APIs already have promoted contracts and module-client replacements. Current call-site search found flat method definitions and legacy ignored live-test call sites for the selected methods. Deprecated legacy 404/basic_info APIs and mutating action/publish endpoints remain excluded.

**Scope:**

- Remove `BpiClient::dynamic_all`.
- Remove `BpiClient::dynamic_check_new`.
- Remove `BpiClient::dynamic_nav_feed`.
- Remove `BpiClient::dynamic_feed_banner`.
- Remove `BpiClient::dynamic_detail`.
- Remove `BpiClient::dynamic_reactions`.
- Remove `BpiClient::dynamic_lottery_notice`.
- Remove `BpiClient::dynamic_forwards`.
- Remove `BpiClient::dynamic_pics`.
- Remove `BpiClient::dynamic_forward_item`.
- Remove `BpiClient::dynamic_live_users`.
- Remove `BpiClient::dynamic_up_users`.
- Remove `BpiClient::dynamic_recent_up_list`.
- Rewrite corresponding ignored live tests to use `client.dynamic()`.

**Evidence reused:**

```text
tests/contracts/dynamic/feed/all/contract.json
tests/contracts/dynamic/feed/check-new/contract.json
tests/contracts/dynamic/feed/nav/contract.json
tests/contracts/dynamic/feed/banner/contract.json
tests/contracts/dynamic/detail/detail/contract.json
tests/contracts/dynamic/detail/reactions/contract.json
tests/contracts/dynamic/detail/forwards/contract.json
tests/contracts/dynamic/detail/pics/contract.json
tests/contracts/dynamic/detail/forward-item/contract.json
tests/contracts/dynamic/content/live-users/contract.json
tests/contracts/dynamic/content/up-users/contract.json
tests/contracts/dynamic/content/recent-up/contract.json
tests/contracts/dynamic/lottery-notice-read/lottery-notice/contract.json
```

**Excluded:**

- `BpiClient::dynamic_card_detail`.
- `BpiClient::dynamic_repost_detail`.
- `BpiClient::dynamic_spec_item_likes`.
- `BpiClient::dynamic_upload_pic`.
- `BpiClient::dynamic_create_text`.
- `BpiClient::dynamic_create_complex`.
- `BpiClient::dynamic_like`.
- `BpiClient::dynamic_remove_draft`.
- `BpiClient::dynamic_set_top`.
- `BpiClient::dynamic_remove_top`.
- No Probe run or contract promotion; request/response behavior is already proven by promoted contracts and the existing `dynamic/module-client-bridge`.
- No `flightdeck/cockpit.md` update.

**Pre-check evidence:**

```text
rg -n "pub async fn dynamic_|dynamic_all\(|dynamic_check_new\(|dynamic_nav_feed\(|dynamic_feed_banner\(|dynamic_detail\(|dynamic_reactions\(|dynamic_lottery_notice\(|dynamic_forwards\(|dynamic_pics\(|dynamic_forward_item\(|dynamic_live_users\(|dynamic_up_users\(|dynamic_recent_up_list\(" src\dynamic tests examples README.md docs
  found selected flat read method definitions and legacy ignored live-test call sites in `src/dynamic`; also found excluded action/publish/deprecated definitions.
```

**Verification plan:**

```powershell
rg -n "\bbpi\.(dynamic_all|dynamic_check_new|dynamic_nav_feed|dynamic_feed_banner|dynamic_detail|dynamic_reactions|dynamic_lottery_notice|dynamic_forwards|dynamic_pics|dynamic_forward_item|dynamic_live_users|dynamic_up_users|dynamic_recent_up_list)\(|BpiClient::(dynamic_all|dynamic_check_new|dynamic_nav_feed|dynamic_feed_banner|dynamic_detail|dynamic_reactions|dynamic_lottery_notice|dynamic_forwards|dynamic_pics|dynamic_forward_item|dynamic_live_users|dynamic_up_users|dynamic_recent_up_list)\b" src tests examples README.md docs
cargo test --all-features --lib dynamic --quiet
cargo fmt --check
cargo check --all-features
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --lib --quiet
git diff --check
git diff -- flightdeck/cockpit.md
```

**Observed verification:**

```text
rg -n "\bbpi\.(dynamic_all|dynamic_check_new|dynamic_nav_feed|dynamic_feed_banner|dynamic_detail|dynamic_reactions|dynamic_lottery_notice|dynamic_forwards|dynamic_pics|dynamic_forward_item|dynamic_live_users|dynamic_up_users|dynamic_recent_up_list)\(|BpiClient::(dynamic_all|dynamic_check_new|dynamic_nav_feed|dynamic_feed_banner|dynamic_detail|dynamic_reactions|dynamic_lottery_notice|dynamic_forwards|dynamic_pics|dynamic_forward_item|dynamic_live_users|dynamic_up_users|dynamic_recent_up_list)\b" src tests examples README.md docs
  no matches.

rg -n "pub async fn dynamic_" src\dynamic
  only excluded deprecated/action/publish methods remain: `dynamic_card_detail`, `dynamic_repost_detail`, `dynamic_spec_item_likes`, `dynamic_upload_pic`, `dynamic_create_text`, `dynamic_create_complex`, `dynamic_like`, `dynamic_remove_draft`, `dynamic_set_top`, and `dynamic_remove_top`.

cargo test --all-features --lib dynamic --quiet
  passed: 53 passed, 0 failed, 23 ignored.

cargo fmt --check
  passed after cargo fmt normalized spacing.

cargo check --all-features
  passed with no warnings.

cargo clippy --all-targets --all-features --locked -- -D warnings
  passed.

cargo test --all-features --lib --quiet
  passed: 929 passed, 0 failed, 292 ignored.

git diff --check
  passed with only CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```

## Batch 5: `docs/migration-guide-0.2`

**Type:** Non-Probe documentation batch.

**Status:** Implemented and verified in the working tree; commit pending human approval.

**Why this batch:** `design.md`/`plan.md` Stage 6 explicitly calls for `docs/migration-0.2.md`. README now describes the new module-client direction, but users still need a focused migration guide from the old flat 0.1 API to the current 0.2 migration surface.

**Pre-check:** `Taskfile.yml` already satisfies the Stage 0 task cleanup requirements from `plan.md`: stale `blackroom`, `emoji`, and `garb` tasks are absent, and `test_offline` plus `check_all` exist. No Taskfile edit is needed in this batch.

**Scope:**

- Create `docs/migration-0.2.md`.
- Cover client construction changes.
- Cover flat methods to module-client methods.
- Cover `BpiResponse<T>` to payload-returning `BpiResult<T>` for migrated methods.
- Cover account/cookie setup changes.
- Cover QR login primitive changes.
- Cover local Probe/live-test credential guidance.

**Excluded:**

- No Rust source changes.
- No Probe run.
- No API behavior changes.
- No Taskfile changes unless verification discovers a real mismatch.

**Verification plan:**

```powershell
git diff --check
rg "BpiClient::new\(\);|\.data\.unwrap\(|login_send_qrcode|login_check_qrcode_status|bangumi_info\(" docs\migration-0.2.md
rg -n "migration-0.2.md|docs/migration-0.2.md" README.md docs\migration-0.2.md flightdeck\work\bpi-rs-0.2-migration\index.md
```

Cargo gates are not required unless Rust source, tests, contracts, or compiled rustdoc examples change.

**Observed verification:**

```text
git diff --check
  passed with only CRLF conversion warnings.

rg "BpiClient::new\(\);|\.data\.unwrap\(|login_send_qrcode|login_check_qrcode_status|bangumi_info\(" docs\migration-0.2.md README.md
  no matches.

rg -n "migration-0\.2\.md|docs/migration-0\.2\.md" README.md docs\migration-0.2.md flightdeck\work\bpi-rs-0.2-migration\index.md
  README link present and topic index references the guide.

rg -n "blackroom|emoji|garb|test_offline|check_all" Taskfile.yml
  found `test_offline` and `check_all`; no stale `blackroom`, `emoji`, or `garb` tasks.
```

Cargo gates were not run for this docs-only batch because no Rust source, tests, contracts, or compiled rustdoc examples changed.

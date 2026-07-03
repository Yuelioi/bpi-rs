# Batch TODO — bpi-rs 0.2 migration

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans when executing this list inline. Keep TDD inside each batch, but commit only after the whole batch passes verification.

**Goal:** Continue the 0.2 migration in larger reviewable batches instead of committing every few methods.

**Architecture:** Default work remains Probe-backed endpoint contract batches. When no safe Probe batch exists, choose one explicit non-Probe bridge batch that reuses already promoted contracts, record it in `migration-status.md` before code, then implement all methods in that batch before one commit.

**Tech Stack:** Rust 2024, reqwest, serde, thiserror, tracing, existing `BilibiliRequest` payload helpers, `tests/contracts/**/contract.json`, Flightdeck topic state.

---

## Batch Rules

- Do not repeat completed endpoint contract batches: `video/info-read`, `login/read-info`, `clientinfo/ip`, or any batch already marked complete in `migration-status.md`.
- Do not start guessed Probe work. A Probe-backed batch needs a real unpromoted safe endpoint, request drafts under `target/bpi-contract-drafts/...`, raw Probe output under `target/bpi-probe-runs/...`, reviewed contracts under `tests/contracts/...`, and sanitized fixtures.
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

## Current Working State

- Branch: `feat/bpi-rs-0.2-migration`
- Last committed batch: `061931f feat(video): add collection player client bridge`
- Dirty files at Batch 2 selection: `flightdeck/cockpit.md`, `flightdeck/work/bpi-rs-0.2-migration/goal.md`, and `flightdeck/work/bpi-rs-0.2-migration/index.md`
- Dirty change purpose: goal-mode/cockpit routing updates requested by the user before starting the next batch.
- Current intended batch: `live/remaining-read-client-bridge`

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

### Candidate: `shared-core/module-client-coverage-audit`

**Type:** Non-Probe docs/code audit unless it discovers a real missing endpoint batch.

**Purpose:** Compare `tests/contracts/**/contract.json` against domain clients to find promoted read contracts not reachable through module clients.

**Do not implement endpoint behavior in this audit commit unless the candidate list is small and cohesive.**

### Candidate: `manga/download-read`

**Type:** Probe-backed only if a valid current chapter/flow/handshake is identified.

**Current state:** Probe-blocked by repeated API `code = 99`. Do not retry blindly.

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

**Status:** Implemented and verified in the working tree; commit pending.

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

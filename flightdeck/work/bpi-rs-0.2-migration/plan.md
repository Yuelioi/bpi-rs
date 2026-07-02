# bpi-rs 0.2 Migration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Migrate bpi-rs from a broad but tightly coupled 0.1 API wrapper into an idiomatic, embeddable, testable Rust SDK with staged breaking changes.

**Architecture:** Do not migrate all endpoint modules at once. First establish the SDK foundation: independent client instances, builder/config, session/cookie handling, transport/response/error layers, deterministic fixtures, and shared signing. Then migrate a small set of core domains to prove the pattern before moving the rest.

**Tech Stack:** Rust 2024, reqwest 0.12, tokio, serde, thiserror, tracing, bitflags, existing optional domain dependencies.

---

## Code Research Summary

Root `src/*.rs` files were all read:

- `src/auth.rs`
- `src/client.rs`
- `src/lib.rs`
- `src/log.rs`
- `src/main.rs`
- `src/request.rs`
- `src/response.rs`

Sampled submodules:

- errors: `src/err/error.rs`, `src/err/from.rs`, `src/err/code.rs`
- signing: `src/utils/wbi.rs`, `src/utils/aid_bvid.rs`, `src/misc/sign/bili_ticket.rs`
- login: `src/login/login_action/qr.rs`, `src/login/login_info/nav.rs`, `src/login/cookie_refresh.rs`
- video: `src/video/info/view.rs`, `src/video/videostream_url.rs`, `src/video/action.rs`
- dynamic: `src/dynamic/detail.rs`, `src/dynamic/publish.rs`
- user: `src/user/info.rs`, `src/user/space.rs`
- project metadata: `Cargo.toml`, `Taskfile.yml`, `README.md`

Important counts from `rg`:

- async API methods: 326
- `send_bpi(...)` calls: 305
- test functions: 306
- `BpiClient::new()` call sites: 308
- `unwrap()` / `expect()` call sites: 147
- `serde_json::Value` uses: 227
- `Option<...>` occurrences: 1132

These counts mean the migration must be staged. The project is too large for a single safe rewrite.

## Current Architecture Findings

The current design has useful coverage but weak boundaries:

- `BpiClient::new()` returns a global singleton through `OnceLock`, so account/cookie state can leak across consumers and tests.
- Debug/test builds auto-call `init_log()` and auto-read `account.toml`, so library construction has process-wide side effects.
- `src/log.rs` initializes a tracing subscriber and formats a custom China-time timestamp. A library should emit tracing events but not install global logging.
- `BpiClient` directly owns `reqwest::Client`, `Arc<Jar>`, and `Mutex<Option<Account>>`; there is no separate builder/config/session layer.
- Cookie clearing is a stub and cannot currently clear the jar.
- `request.rs` couples reqwest extension methods, logging, HTTP status checks, JSON envelope parsing, and API error conversion.
- `request.rs` logs full URLs, which can expose signed query params or credential-bearing URLs.
- `BpiResponse<T>` assumes most endpoints are envelope JSON with optional `data`, but the crate also has XML/protobuf/bytes-like endpoint needs.
- `BpiError` loses source errors and collapses many reqwest failures into stringly `Network`.
- Signing is global: `WBI_KEY_MAP` is a process-level static cache, not scoped to a client/session.
- Endpoint modules attach hundreds of methods directly to `impl BpiClient`, creating a flat public API surface.
- Many endpoint parameters are raw `Option<u64>`, `Option<&str>`, `u8`, or magic integers instead of typed IDs/enums/builders.
- Some action methods use defaults like `aid.unwrap_or(0)` and `bvid.unwrap_or("")`, which can silently build invalid requests.
- Tests are mostly live API calls. Several have side effects: liking, coin, favorite, dynamic publish, upload, space notice mutation, relation/group changes.
- Test data is inconsistent: some canonical IDs exist, but many IDs are random-looking or user-specific.
- README and rustdoc examples describe the old singleton API and `BpiResponse<T>` unwrapping.
- `Taskfile.yml` contains stale feature tasks such as `blackroom`, `emoji`, and `garb` that do not exist in `Cargo.toml`.

## Migration Strategy

Use four tracks, in this order:

1. **Safety track:** baseline inventory, live-test quarantine, fixture layout, docs honesty.
2. **Foundation track:** client builder, config, session, transport, response, error, tracing policy.
3. **Capability track:** account/cookie parsing, csrf, WBI, bili_ticket, buvid, typed IDs.
4. **Domain track:** migrate selected modules first, then batch-migrate the rest.

The first implementation milestone should not touch all endpoint modules. It should make the new foundation coexist with old modules until core domains are migrated.

## Proposed File Structure

Target structure for the foundation:

```text
src/
  client/
    mod.rs          public BpiClient and domain accessors
    builder.rs      BpiClientBuilder and configuration defaults
    inner.rs        ClientInner shared state
    config.rs       BpiClientConfig and default headers/user-agent/timeouts
  transport/
    mod.rs          transport traits/types
    reqwest.rs      reqwest-backed transport
    request.rs      typed request representation
    response.rs     raw response representation
  session/
    mod.rs          Session, Account, cookie parsing, csrf lookup
    cookie.rs       cookie parsing/sanitization helpers
  sign/
    mod.rs
    wbi.rs
    bili_ticket.rs
    buvid.rs
  response.rs       ApiEnvelope<T>, envelope parsing helpers
  error.rs          BpiError, BpiResult<T>, ApiErrorCategory
  ids.rs            Aid, Bvid, Cid, Mid, DynamicId, RoomId, MediaId
  testing/
    mod.rs          test-only helpers behind cfg(test)
```

Compatibility path during migration:

- Keep current `src/client.rs`, `src/request.rs`, `src/auth.rs`, `src/err/`, and `src/utils/` until replacement modules compile.
- Add new modules behind new names first if needed, then switch `lib.rs` exports.
- Only remove old files after core domains compile against the new foundation.

Fixture layout:

```text
tests/
  fixtures/
    envelope/
      success.json
      api-error.json
      missing-data.json
    video/
      view.json
      playurl-dash.json
    login/
      nav.json
      qrcode-poll-success.json
    dynamic/
      detail-forward.json
    user/
      space-info.json
  support/
    mod.rs
    fixture.rs
    mock_transport.rs
```

## Stage 0: Baseline Inventory and Test Quarantine

**Files:**

- Create: `flightdeck/work/bpi-rs-0.2-migration/plans/00-baseline-inventory.md`
- Modify: `Taskfile.yml`
- Modify later: source test modules as live tests are quarantined

- [ ] **Step 1: Record reproducible baseline commands**

Run:

```powershell
cargo fmt --check
cargo check --all-features
cargo test --all-features --lib
```

Expected baseline:

- `cargo check --all-features` should pass based on prior run.
- `cargo test --all-features --lib` may hit live network/account side effects. Record failures rather than fixing them in this step.

- [ ] **Step 2: Create a test inventory**

Use:

```powershell
rg "#\[tokio::test\]|#\[test\]" src -n
rg "post\(|multipart|csrf\?|发布|点赞|投币|收藏|删除|修改|上传|set|create|add|del" src -n
```

Classify tests into:

- offline unit
- live read-only
- live authenticated read-only
- live mutating

- [ ] **Step 3: Define live test gating**

Adopt these environment variables:

```text
BPI_LIVE_TEST=1
BPI_MUTATING_TEST=1
BPI_COOKIE=<cookie string>
BPI_TEST_MID=<optional authenticated user's mid>
```

Default `cargo test` must not perform network or mutating operations.

- [ ] **Step 4: Fix stale task entries**

Remove Taskfile entries for features not present in `Cargo.toml`: `blackroom`, `emoji`, `garb`.

Add explicit tasks:

```yaml
test_offline:
  cmds:
    - cargo test --all-features --lib

check_all:
  cmds:
    - cargo fmt --check
    - cargo clippy --all-targets --all-features --locked -- -D warnings
    - cargo check --all-features
    - cargo test --all-features --lib
```

- [ ] **Step 5: Commit baseline work**

```powershell
git add Taskfile.yml flightdeck/work/bpi-rs-0.2-migration/plans/00-baseline-inventory.md
git commit -m "chore: record 0.2 migration baseline"
```

## Stage 1: Client Foundation

**Files:**

- Create: `src/client/mod.rs`
- Create: `src/client/builder.rs`
- Create: `src/client/config.rs`
- Create: `src/client/inner.rs`
- Create: `src/session/mod.rs`
- Create: `src/session/cookie.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Add tests for independent clients**

Create tests proving two clients can have different session state and do not share cookies.

Required behavior:

```rust
let a = BpiClient::builder().cookie("SESSDATA=a").build()?;
let b = BpiClient::builder().cookie("SESSDATA=b").build()?;
assert_ne!(a.session().cookie_header_for_test(), b.session().cookie_header_for_test());
```

- [ ] **Step 2: Implement `BpiClientBuilder`**

Required builder knobs:

- timeout
- connect timeout
- user agent
- referer/origin defaults
- proxy/no-proxy policy
- cookie string
- `Account`
- base API URL override for tests
- externally supplied `reqwest::Client`

The builder must return `BpiResult<BpiClient>`, not panic.

- [ ] **Step 3: Replace singleton construction**

New public construction:

```rust
let client = BpiClient::new()?;
let client = BpiClient::builder().build()?;
```

No `OnceLock<BpiClient>` in the new client path.

- [ ] **Step 4: Remove constructor side effects**

Construction must not:

- initialize tracing subscriber
- create or read `account.toml`
- auto-load credentials
- log account state

- [ ] **Step 5: Commit client foundation**

```powershell
git add src/client src/session src/lib.rs
git commit -m "feat: add independent client builder and session foundation"
```

## Stage 2: Transport, Response, and Error

**Files:**

- Create: `src/transport/mod.rs`
- Create: `src/transport/reqwest.rs`
- Create: `src/transport/request.rs`
- Create: `src/transport/response.rs`
- Replace or migrate: `src/error.rs`
- Replace or migrate: `src/response.rs`
- Deprecate later: `src/request.rs`, `src/err/`

- [ ] **Step 1: Add envelope fixture tests**

Fixtures required:

```text
tests/fixtures/envelope/success.json
tests/fixtures/envelope/api-error.json
tests/fixtures/envelope/missing-data.json
```

Tests must cover:

- `data`
- `result` alias
- no payload
- API error code/message
- decode failure

- [ ] **Step 2: Implement `ApiEnvelope<T>` and `BpiResult<T>`**

Public alias:

```rust
pub type BpiResult<T> = Result<T, BpiError>;
```

Endpoint helpers should return payload `T` by default. Add explicit envelope-returning helpers only when needed.

- [ ] **Step 3: Implement typed `BpiError`**

Required variants:

- `HttpStatus`
- `Transport`
- `Decode`
- `Api`
- `Auth`
- `InvalidParameter`
- `MissingData`
- `UnsupportedResponse`

Preserve `source` for reqwest and serde failures where possible.

- [ ] **Step 4: Implement sanitized request tracing**

Transport may emit:

- method
- endpoint label
- sanitized URL path
- duration
- status
- API code/category

Transport must not emit:

- `SESSDATA`
- csrf
- raw `Cookie`
- full signed URLs

- [ ] **Step 5: Commit transport foundation**

```powershell
git add src/transport src/error.rs src/response.rs tests/fixtures tests/support
git commit -m "feat: add transport response and error foundation"
```

## Stage 3: Shared Types, Auth, and Signing

**Files:**

- Create: `src/ids.rs`
- Create: `src/sign/mod.rs`
- Create: `src/sign/wbi.rs`
- Create: `src/sign/bili_ticket.rs`
- Create: `src/sign/buvid.rs`
- Modify: `src/session/mod.rs`
- Modify: `src/session/cookie.rs`
- Migrate from: `src/auth.rs`, `src/utils/wbi.rs`, `src/utils/aid_bvid.rs`, `src/misc/sign/bili_ticket.rs`

- [ ] **Step 1: Add ID newtypes**

Minimum first batch:

```rust
pub struct Aid(u64);
pub struct Cid(u64);
pub struct Mid(u64);
pub struct RoomId(u64);
pub struct MediaId(u64);
pub struct DynamicId(String);
pub struct Bvid(String);
```

Each type needs:

- `Debug`, `Clone`, `PartialEq`, `Eq`
- parse/validate where meaningful
- `Display`
- serde support where endpoint models need it

- [ ] **Step 2: Move account and cookie parsing into `session`**

`Account` should support:

- typed construction
- cookie-string parsing
- csrf lookup
- sanitized debug formatting

Do not print cookie values.

- [ ] **Step 3: Move WBI cache under client/session**

Replace global `WBI_KEY_MAP` with client-owned signing state.

Tests:

- deterministic `enc_wbi` with fixed timestamp
- cache hit path
- cache refresh path through mock transport
- malformed nav response returns typed error, not panic

- [ ] **Step 4: Move bili_ticket into `sign`**

Keep HMAC logic deterministic and separately tested.

Live ticket generation must be opt-in because it needs csrf/session.

- [ ] **Step 5: Commit shared capabilities**

```powershell
git add src/ids.rs src/sign src/session src/lib.rs
git commit -m "feat: add typed ids session and signing capabilities"
```

## Stage 4: Core Domain Migration

Migrate only these domains first:

1. `misc/sign`
2. `login`
3. `video`
4. `user`
5. `dynamic`

Each domain migration must follow the same checklist.

**Files per domain:**

```text
src/<domain>/
  mod.rs
  client.rs
  model.rs
  params.rs
```

- [ ] **Step 1: Create domain client handle**

Pattern:

```rust
pub struct VideoClient {
    inner: Arc<ClientInner>,
}

impl BpiClient {
    pub fn video(&self) -> VideoClient {
        VideoClient::new(self.inner.clone())
    }
}
```

- [ ] **Step 2: Move endpoint params into `params.rs`**

Use typed params:

```rust
pub struct VideoViewParams {
    pub id: VideoId,
}

pub enum VideoId {
    Aid(Aid),
    Bvid(Bvid),
}
```

- [ ] **Step 3: Move response models into `model.rs`**

Avoid broad `serde_json::Value` unless payload is genuinely unmodeled. If a field remains `Value`, add a comment explaining the observed variability.

- [ ] **Step 4: Add fixture tests**

Each migrated endpoint needs:

- parameter serialization test
- success fixture deserialization test
- API error fixture test
- missing data fixture test where relevant
- live smoke test only behind env gate

- [ ] **Step 5: Commit each domain separately**

Example:

```powershell
git add src/video tests/fixtures/video
git commit -m "feat(video): migrate view and playurl clients"
```

## Stage 5: Remaining Domain Batches

Batch order should follow risk and dependency:

1. read-only public modules: `clientinfo`, `video_ranking`, `search`, `activity`, `article`
2. authenticated read-only modules: `fav/info`, `historytoview`, `wallet`, `vip/info`
3. media/format modules: `danmaku`, `audio`, `bangumi`, `cheese`
4. mutating modules: `comment/action`, `fav/action`, `user/relation/action`, `dynamic/publish`, `creativecenter`
5. high-risk live modules: `live`, `message`, `manga`, `electric`

Each batch gets its own plan under `flightdeck/work/bpi-rs-0.2-migration/plans/`.

## Stage 6: Release Cleanup

**Files:**

- Modify: `README.md`
- Modify: `Cargo.toml`
- Modify: `Taskfile.yml`
- Modify: `src/lib.rs`
- Remove obsolete old modules after all migrated domains compile

- [ ] **Step 1: Remove obsolete flat API surface**

Delete old `impl BpiClient` endpoint methods only after migrated domain clients compile and tests pass.

- [ ] **Step 2: Rewrite README**

README must show:

```rust
let client = BpiClient::builder()
    .cookie(cookie)
    .build()?;

let view = client.video().view(VideoId::Bvid("BV...".parse()?)).await?;
```

Examples requiring credentials must be marked as such.

- [ ] **Step 3: Add migration guide**

Create:

```text
docs/migration-0.2.md
```

Cover:

- singleton removal
- `BpiResponse<T>` to `BpiResult<T>`
- flat methods to domain clients
- account/cookie setup changes
- live test env variables

- [ ] **Step 4: Release verification**

Run:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features --lib
cargo test --doc
```

- [ ] **Step 5: Commit release cleanup**

```powershell
git add README.md Cargo.toml Taskfile.yml src docs
git commit -m "docs: prepare bpi-rs 0.2 migration guide"
```

## Execution Rule

Do not start Stage 4 until Stages 1-3 have tests and pass `cargo check --all-features`.

Do not migrate mutating endpoints until:

- live test gating exists
- fixture tests exist for the domain
- credentials are only provided by environment variables
- tests cannot mutate a real account unless `BPI_MUTATING_TEST=1`

Do not remove old public APIs until the replacement path is documented and core migrated domains compile.

## Immediate Next Plan

The next file to write should be:

```text
flightdeck/work/bpi-rs-0.2-migration/plans/00-baseline-inventory.md
```

It should turn Stage 0 into exact commands, inventories, and no-source-change cleanup steps. Implementation should begin there.

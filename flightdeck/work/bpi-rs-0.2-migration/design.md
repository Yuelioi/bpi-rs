# Design — bpi-rs 0.2 migration

## Purpose

bpi-rs 0.2 is a breaking redesign of the current Bilibili API SDK. The goal is not to patch old code in place, but to turn the project into a maintainable open-source Rust crate with clear module boundaries, predictable client state, strong error semantics, and a testing strategy that does not depend entirely on live Bilibili endpoints.

The existing crate already compiles with current dependencies and has broad API coverage. The main problems are architectural: `BpiClient::new()` returns a global singleton, account and cookie state can leak across callers, debug/test builds auto-read `account.toml`, request/auth/signing behavior is scattered, and the large flat method surface will get harder to maintain as coverage grows.

## Design Principles

- Favor idiomatic Rust over cross-language portability constraints.
- Prefer explicit state over hidden global state.
- Keep public APIs discoverable by domain.
- Make low-level transport behavior reusable without leaking reqwest details into every endpoint.
- Separate offline unit tests from live API smoke tests.
- Migrate in stages so every checkpoint can compile, be reviewed, and be reverted.

## Research Inputs

This design should be implemented against these references:

- Rust API Guidelines checklist: https://rust-lang.github.io/api-guidelines/checklist.html
- Tokio unit testing guide: https://tokio.rs/tokio/topics/testing
- Tokio tracing guide: https://tokio.rs/tokio/topics/tracing
- reqwest `ClientBuilder` and cookie provider docs: https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html
- reqwest `Jar` docs: https://docs.rs/reqwest/latest/reqwest/cookie/struct.Jar.html
- tracing `#[instrument]` docs: https://docs.rs/tracing/latest/tracing/attr.instrument.html

Installed local skills were also reviewed as reference material for Rust best practices, async patterns, and Rust testing.

## Public API Direction

The new public API should be module-oriented:

```rust
let client = BpiClient::builder()
    .account(account)
    .build()?;

let view = client.video().view(VideoId::Bv("BV...".into())).await?;
let nav = client.login().nav().await?;
```

`BpiClient` remains the root object, but endpoint methods live behind domain clients such as `video()`, `login()`, `user()`, `dynamic()`, and `misc()`. Each domain client is a lightweight borrowed or cloned handle over shared client internals.

The old flat `client.video_info(...)` style is not preserved as a primary API. A temporary compatibility facade may be added only if it stays small and does not compromise the new structure.

## Core Architecture

The redesigned crate should be organized around these internal layers:

- `client`: public root client, builder, shared configuration, domain client accessors.
- `transport`: reqwest wrapper, default headers, timeout, redirect/cookie behavior, raw send, JSON decoding.
- `session`: account, cookies, csrf, login state, and authenticated request helpers.
- `sign`: WBI, bili_ticket, buvid-related signing and token helpers.
- `response`: Bilibili envelope parsing, data extraction, and success/error conversion.
- `error`: one canonical crate error type with clear categories and source preservation.
- domain modules: endpoint parameters, response models, and domain client methods.
- `testing`: fixtures, mock transport helpers, and gated live smoke test utilities.

The root client should own an `Arc<ClientInner>`, where `ClientInner` contains the reqwest client, cookie jar/session store, config, and signing caches. `BpiClient` and domain clients should be cheap to clone.

## Client State

`BpiClient::new()` should no longer return `&'static Self`. It should construct an independent client:

```rust
let client = BpiClient::new()?;
let client = BpiClient::builder().timeout(Duration::from_secs(15)).build()?;
```

Account setup should be explicit. Debug and test builds must not auto-load `account.toml`. Local test credentials may still be supported through test utilities or examples, but not as a side effect of constructing the client.

Cookie clearing must be real. If reqwest's jar cannot clear individual cookies cleanly, the client/session design should allow replacing the session jar or rebuilding the inner client in a controlled way.

## Request Flow

Endpoint methods should build typed request parameters, then call shared transport helpers:

1. Domain method validates and normalizes parameters.
2. Auth/signing helpers add csrf, WBI, bili_ticket, or other required fields.
3. Transport applies default headers and sends the request.
4. Response layer parses either a Bilibili envelope or a raw body, depending on endpoint type.
5. API codes are converted into `BpiError::Api`; HTTP, network, timeout, and decode failures remain distinct.

Most endpoints should return `BpiResult<T>` where `T` is the actual data payload, not `BpiResponse<T>`. The envelope can remain available for advanced callers through explicit methods if needed, but ordinary calls should not force every user to unwrap `resp.data`.

## Error Model

Use a single public alias:

```rust
pub type BpiResult<T> = Result<T, BpiError>;
```

`BpiError` should preserve source errors where possible and distinguish:

- `HttpStatus`
- `Transport`
- `Decode`
- `Api`
- `Auth`
- `InvalidParameter`
- `MissingData`
- `UnsupportedResponse`

API errors should keep the numeric Bilibili code, message, and category. Helper methods such as `requires_login()`, `requires_vip()`, and `is_risk_control()` should remain available.

## Response Model

The Bilibili envelope should be modeled as an internal or semi-public type:

```rust
pub struct ApiEnvelope<T> {
    pub code: i32,
    pub message: String,
    pub ttl: Option<i32>,
    pub data: Option<T>,
}
```

`result` should continue to deserialize as an alias for `data`. The parser should handle endpoints that return `data`, `result`, no payload, or non-envelope raw formats such as XML/protobuf/bytes.

The design should avoid pretending every endpoint behaves the same. Endpoint methods should choose the correct response strategy explicitly.

## Authentication and Signing

Auth and signing are shared capabilities, not endpoint-local utilities.

- `Account` should represent known cookie/account fields and be constructible from typed values or a cookie string.
- `Session` should own mutable login state and cookie management.
- csrf lookup should be centralized and return a clear auth error when missing.
- WBI key fetching and caching should be centralized, refreshable, and testable.
- bili_ticket and buvid helpers should live under signing/session utilities and be reused by endpoints.

Login APIs should update the session only through explicit methods. Fetching a QR code or checking status should not unexpectedly mutate global state.

## Domain Migration Pattern

Each domain module should follow the same shape:

```text
src/video/
  mod.rs
  client.rs      domain client methods
  model.rs       response models
  params.rs      request parameter builders/types
```

Small modules may combine files at first, but the public pattern should remain consistent. Large files should be split when they mix endpoint methods, models, and helpers.

The first migrated domains are:

1. `misc/sign` and shared signing helpers, because WBI/bili_ticket affect many endpoints.
2. `login`, because it exercises session and account behavior.
3. `video`, because it covers common public endpoints, WBI, typed IDs, and stream URL flows.
4. `user`, because it exercises logged-in and public account data.
5. `dynamic`, because it has complex models and real-world optional-field pressure.

Remaining domains should migrate after these patterns are validated.

## Feature Flags

Keep domain feature flags, but make them easier to reason about:

- `default = ["full"]` can remain for compatibility during 0.2 unless compile time becomes a problem.
- Each domain feature should enable only the dependencies it needs.
- Shared capabilities such as `sign`, `session`, and `transport` should be always-on if they are required by the root client.
- Optional heavy formats such as XML/protobuf/multipart should stay behind domain features when practical.

Taskfile entries should be cleaned up to match actual features; stale entries such as modules that no longer exist should be removed.

## Testing Strategy

Tests should be split into three classes:

- Offline unit tests: parameter validation, URL/query construction, response parsing, error conversion, signing algorithms.
- Mocked transport tests: endpoint methods against fixture JSON/XML/protobuf without hitting Bilibili.
- Live smoke tests: opt-in tests gated by environment variables and real credentials.

No normal `cargo test` should require `account.toml` or network access. Live tests should be named and documented clearly, for example behind `BPI_LIVE_TEST=1` and `BPI_COOKIE`.

Baseline verification for each migration phase:

```text
cargo fmt --check
cargo check --all-features
cargo test --all-features --lib
```

Live smoke tests are additional evidence, not the default correctness gate.

## Documentation

README should be rewritten around the 0.2 API style:

- installation
- quick start
- account/cookie setup
- module client examples
- error handling
- live test setup
- migration notes from 0.1

docs.rs examples should avoid requiring real credentials unless explicitly marked.

## Migration Phases

### Phase 0 — Baseline

Record current compile status, feature list, public API shape, and test behavior. This phase should not change behavior except for adding planning artifacts.

### Phase 1 — Client Foundation

Introduce independent `BpiClient`, `BpiClientBuilder`, `ClientInner`, explicit config, and session ownership. Remove the global singleton behavior and automatic account loading.

### Phase 2 — Transport, Response, Error

Centralize request sending, headers, response decoding, API envelope handling, and error conversion. Add offline tests for parsing and error cases.

### Phase 3 — Auth and Signing

Centralize account/cookie parsing, csrf, WBI, buvid, and bili_ticket behavior. Add deterministic tests for signing and fixture-based tests for key fetching.

### Phase 4 — Core Domains

Migrate `misc/sign`, `login`, `video`, `user`, and `dynamic` to module clients. Keep compatibility shims only where they are cheap and clearly documented.

### Phase 5 — Remaining Domains

Migrate the rest of the API modules in batches. Each batch should follow the established domain pattern and include basic offline tests.

### Phase 6 — Release Cleanup

Remove obsolete APIs, stale task entries, dead helpers, and outdated docs. Update README, examples, changelog, and crate metadata for `0.2.0`.

## Non-Goals

- Do not verify or rewrite every Bilibili endpoint in the first pass.
- Do not preserve the old flat API as a hard requirement.
- Do not add code generation unless repeated endpoint boilerplate becomes a proven maintenance problem.
- Do not make Go portability a constraint for Rust design.

## Acceptance Criteria

The migration is successful when:

- `BpiClient` instances are independent and cloneable.
- Normal tests do not require network or local credentials.
- Core domains use the module client API.
- Shared auth/signing behavior is centralized.
- Public methods return `BpiResult<T>` for ordinary data access.
- README and examples match the new API.
- Public API design follows the Rust API Guidelines where applicable: meaningful newtypes, builders for complex construction, type-directed arguments instead of booleans, and documented public items.
- Library code uses `thiserror`-backed typed errors and does not use `anyhow` in public/library APIs.
- Async code does not hold locks across `.await`, does not spawn unbounded work, and uses structured tracing spans for important request/session operations.
- Default tests are offline and deterministic; live Bilibili tests are opt-in.
- `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo check --all-features`, and `cargo test --all-features --lib` pass before release.

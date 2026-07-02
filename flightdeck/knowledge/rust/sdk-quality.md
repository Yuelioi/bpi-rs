# Rust SDK quality checklist

SUMMARY: Always apply this Rust API, async, error, docs, and test quality bar before implementing or reviewing bpi-rs SDK migration work.
READ WHEN: before any Rust implementation, implementation planning, or code review for the bpi-rs 0.2 migration.
RECHECK WHEN: Rust edition, reqwest, Tokio, tracing, or crate public API policy changes.

---

Use this as the working quality bar for bpi-rs 0.2 implementation.

## API design

- Follow the Rust API Guidelines where they fit this crate: meaningful types, predictable names, builders for complex construction, and type-directed arguments instead of ambiguous booleans.
- Prefer module clients such as `client.video().view(...)` over a flat root method surface.
- Use newtypes for IDs and parameters when raw `u64`, `String`, or `bool` would make call sites ambiguous.
- Keep public APIs documented with rustdoc examples where examples can compile without live credentials.

## Ownership and state

- Prefer borrowed parameters such as `&str`, `&[T]`, and `&T` unless ownership transfer is required.
- Make cloning explicit and cheap. `BpiClient` and domain clients may clone an `Arc<ClientInner>`; avoid cloning large endpoint data or maps.
- Do not use hidden global client/session state. Client construction and account/session setup must be explicit.

## Errors

- Library APIs return typed `BpiError` through `BpiResult<T>`.
- Use `thiserror` for crate errors. Do not expose `anyhow` from library APIs.
- Preserve source errors where useful, and keep HTTP status, transport, decode, API, auth, invalid parameter, and missing data failures distinct.
- Avoid `unwrap()` and `expect()` outside tests and genuinely impossible internal invariants.

## Async and diagnostics

- Do not hold mutex/RwLock guards across `.await`.
- Do not spawn unbounded async work; use structured concurrency, `JoinSet`, buffering limits, or semaphores when parallel work is needed.
- Use `tracing` spans/events for request/session/signing operations. Avoid logging secrets such as cookies, csrf tokens, SESSDATA, or full signed URLs when they may expose credentials.

## Tests

- Normal `cargo test` must be offline and deterministic.
- Live Bilibili smoke tests must be opt-in through explicit environment variables.
- Prefer small unit tests for parsing, signing, parameter validation, response envelopes, and error conversion.
- Use fixture/mocked transport tests for endpoint methods instead of direct network calls.
- Test names should describe behavior, not just say `test_*`.

## Verification

Use these gates before claiming migration work is ready:

```text
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features --lib
```

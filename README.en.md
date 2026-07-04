# bpi-rs

Rust Bilibili API SDK built on `reqwest` and `tokio`.

`bpi-rs` 0.2 focuses on module-oriented APIs, explicit session management, typed parameters, payload-returning methods, and offline contract-backed verification. It is designed for Rust tools, automation, data workflows, and backend services that need broad Bilibili API coverage.

[中文 README](./README.md)

## Why bpi-rs

| Area | What it provides |
| --- | --- |
| Broad API coverage | Video, bangumi, user, search, live, dynamic, comment, favorite, audio, creative center, message, VIP, wallet, watch-later, and more than twenty domain modules. |
| Module clients | Use `client.video().view(...)`, `client.login().nav()`, `client.user().card(...)`, and similar domain clients instead of a large flat method surface. |
| Typed parameters | Common IDs and request inputs use dedicated types such as `Bvid`, `Aid`, `Mid`, `MediaId`, and `VideoViewParams`. |
| Payload-first returns | Migrated module methods return `BpiResult<T>` where `T` is the decoded payload. Use `ApiEnvelope<T>` only when you need the full response envelope. |
| Explicit credentials | Client construction does not read local credentials. Cookies and `Account` values are supplied explicitly. |
| Contract evidence | Migrated endpoints are backed by sanitized contracts and response fixtures under `tests/contracts/**`. |

## Installation

```toml
[dependencies]
bpi-rs = "0.2"
```

or:

```bash
cargo add bpi-rs
```

## Quick Start

```rust
use bpi_rs::ids::Bvid;
use bpi_rs::video::VideoViewParams;
use bpi_rs::{BpiClient, BpiResult};

#[tokio::main]
async fn main() -> BpiResult<()> {
    let client = BpiClient::builder().build()?;
    let bvid: Bvid = "BV1xx411c7mD".parse()?;

    let view = client.video().view(VideoViewParams::from_bvid(bvid)).await?;

    println!("{} by {}", view.title, view.owner.name);
    Ok(())
}
```

## Authenticated APIs

Seed a session explicitly when an endpoint requires login:

```rust
use bpi_rs::{BpiClient, BpiResult};

#[tokio::main]
async fn main() -> BpiResult<()> {
    let client = BpiClient::builder()
        .cookie("DedeUserID=123; SESSDATA=...; bili_jct=...; buvid3=...")
        .build()?;

    let nav = client.login().nav().await?;
    println!("logged in: {}", nav.is_login);
    Ok(())
}
```

## Module Clients

Available module clients include:

```text
activity, article, audio, bangumi, cheese, clientinfo, comment,
creativecenter, danmaku, dynamic, electric, fav, historytoview,
live, login, manga, message, misc, note, opus, search, user,
video, video_ranking, vip, wallet, web_widget
```

The compile-checked example lives in [`examples/module_clients.rs`](examples/module_clients.rs):

```bash
cargo check --all-features --examples
BPI_RUN_EXAMPLE=1 cargo run --example module_clients
```

Set `BPI_COOKIE` to include authenticated calls such as `client.login().nav()`.

## Return Values And Errors

Most migrated module-client methods return decoded payloads directly:

```rust
let view = client.video().view(params).await?;
```

The public result alias is:

```rust
pub type BpiResult<T> = Result<T, BpiError>;
```

`BpiError` keeps transport, HTTP status, decode, API, auth, invalid parameter, missing data, and unsupported-response failures distinct. Helpers include `requires_login()`, `requires_vip()`, `is_risk_control()`, and `semantic_error()`.

Use `ApiEnvelope<T>` when the full Bilibili response envelope is required. Remaining manga envelope aliases are compatibility names over `ApiEnvelope<T>`; paid manga reading is not promised as a working 0.2 capability.

## Custom Requests

When an endpoint is not wrapped yet, use the shared request helpers:

```rust
use bpi_rs::{BilibiliRequest, BpiClient, BpiResult};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Payload {
    value: serde_json::Value,
}

async fn custom_read(client: &BpiClient) -> BpiResult<Payload> {
    client
        .get("https://api.bilibili.com/x/example")
        .send_bpi_payload("custom.example")
        .await
}
```

Use `send_bpi_optional_payload` when success may return `data: null`.

## Development

Default checks are intended to stay offline:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features --lib
cargo test --doc
```

Do not commit `account.toml`, cookies, `SESSDATA`, `bili_jct`, `buvid`, raw Probe output, or account-specific response data.

See [docs/migration-0.2.md](docs/migration-0.2.md) for migration examples.

## License

[MIT](./LICENSE)

## References

- [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect)
- [reqwest](https://github.com/seanmonstar/reqwest)

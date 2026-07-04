# bpi-rs

Rust Bilibili API SDK built on `reqwest`.

This repository is in the 0.2 migration track. The current direction is a
module-oriented SDK: use `client.video().view(...)`, `client.login().nav()`,
`client.bangumi().info(...)`, and similar domain clients instead of relying on a
large flat `BpiClient` method surface.

## Current Highlights

| Area | Status |
| --- | --- |
| Client construction | `BpiClient::builder()` creates independent clients without reading local credential files or initializing global logging. |
| Domain API style | Read APIs are exposed through module clients such as `video()`, `login()`, `user()`, `live()`, `bangumi()`, and `search()`. |
| Return style | New module-client methods return `BpiResult<T>` where `T` is the decoded payload. Legacy/custom helpers may still return `BpiResponse<T>`. |
| Testing evidence | Promoted endpoint contracts live under `tests/contracts/**/contract.json` with sanitized response fixtures. |
| Credentials | Cookies can be supplied explicitly through the builder, `Account`, or `set_account_from_cookie_str`; local `account.toml` is for Probe/dev tooling only. |

## Installation

```toml
[dependencies]
bpi-rs = "*"
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

For authenticated APIs, seed a session explicitly:

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

## Common Module Clients

```rust
use bpi_rs::bangumi::BangumiInfoParams;
use bpi_rs::ids::{Bvid, MediaId};
use bpi_rs::video::VideoViewParams;
use bpi_rs::{BpiClient, BpiResult};

#[tokio::main]
async fn main() -> BpiResult<()> {
    let client = BpiClient::new()?;

    let video = client
        .video()
        .view(VideoViewParams::from_bvid("BV1xx411c7mD".parse::<Bvid>()?))
        .await?;
    println!("video: {}", video.title);

    let bangumi = client
        .bangumi()
        .info(BangumiInfoParams::new(MediaId::new(28_220_978)?))
        .await?;
    println!("bangumi: {}", bangumi.media.title);

    Ok(())
}
```

Available module clients include:

```text
activity, article, audio, bangumi, cheese, clientinfo, comment,
creativecenter, danmaku, dynamic, electric, fav, historytoview,
live, login, manga, message, misc, note, opus, search, user,
video, video_ranking, vip, wallet, web_widget
```

The compile-checked quickstart lives in
[`examples/module_clients.rs`](examples/module_clients.rs):

```bash
cargo check --all-features --examples
BPI_RUN_EXAMPLE=1 cargo run --example module_clients
```

Set `BPI_COOKIE` to include authenticated calls such as `client.login().nav()`.

## Login And Session Setup

### Cookie string

```rust
let client = BpiClient::builder()
    .cookie("DedeUserID=123; SESSDATA=...; bili_jct=...; buvid3=...")
    .build()?;
```

### Account struct

```rust
use bpi_rs::{Account, BpiClient};

let account = Account {
    dede_user_id: "123".into(),
    dede_user_id_ckmd5: "...".into(),
    sessdata: "...".into(),
    bili_jct: "...".into(),
    buvid3: "...".into(),
};

let client = BpiClient::builder().account(account).build()?;
```

### Update an existing client

```rust
client.set_account_from_cookie_str(
    "DedeUserID=123; SESSDATA=...; bili_jct=...; buvid3=...",
)?;
```

### QR login primitives

The crate provides QR login endpoint calls, but it does not render QR codes or
own the user interaction loop.

```rust
use bpi_rs::login::LoginQrPollParams;

let generated = client.login().qr_generate().await?;
println!("scan url: {}", generated.url);

let status = client
    .login()
    .qr_poll(LoginQrPollParams::new(generated.qrcode_key)?)
    .await?;

if status.code == 0 {
    println!("login cookies: {:?}", status.cookies);
}
```

## Return Values And Errors

Most migrated module-client methods return the decoded payload directly:

```rust
let view = client.video().view(params).await?;
```

The public result alias is:

```rust
pub type BpiResult<T> = Result<T, BpiError>;
```

`BpiError` keeps transport, HTTP status, decode, API, auth, invalid parameter,
missing data, and unsupported-response failures distinct. It also exposes helper
methods such as `requires_login()`, `requires_vip()`, `is_risk_control()`, and
`semantic_error()` for stable API-error handling.

Legacy APIs and custom low-level extensions may still use `BpiResponse<T>`:

```rust
pub struct BpiResponse<T> {
    pub code: i32,
    pub data: Option<T>,
    pub message: String,
    pub status: bool,
}
```

## Custom Requests

When a new endpoint is not yet wrapped, use the shared request helpers. Prefer
payload-returning helpers for ordinary envelope JSON endpoints.

```rust
use bpi_rs::{BilibiliRequest, BpiClient, BpiResult};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SectionPayload {
    main_section: Option<serde_json::Value>,
}

async fn season_sections(client: &BpiClient, season_id: u64) -> BpiResult<SectionPayload> {
    client
        .get("https://api.bilibili.com/pgc/web/season/section")
        .query(&[("season_id", season_id.to_string())])
        .send_bpi_payload("custom.bangumi.season_sections")
        .await
}
```

For authenticated POST-like requests, obtain CSRF explicitly:

```rust
let csrf = client.csrf()?;
```

## Development

Default checks should be offline and deterministic:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features --lib
```

Live Probe work is opt-in and keeps raw output under `target/`:

```text
target/bpi-contract-drafts/...
target/bpi-probe-runs/...
target/bpi-probe-notes/...
```

Local credentials may be stored in `account.toml` for Probe/dev workflows. Do
not commit `account.toml`, cookies, `SESSDATA`, `bili_jct`, `buvid`, raw Probe
output, or account-specific response data.

Example local profile shape:

```toml
[probe.normal]
bili_jct = "..."
dede_user_id = 123
dede_user_id_ckmd5 = "..."
sessdata = "..."
buvid3 = "..."

[probe.vip]
bili_jct = "..."
dede_user_id = 456
dede_user_id_ckmd5 = "..."
sessdata = "..."
buvid3 = "..."
```

## Migration Notes

- The old flat API is no longer the primary documentation path.
- New module clients return payloads directly where the endpoint contract
  supports that shape.
- Endpoint contracts and sanitized fixtures are the source of truth for migrated
  request/response behavior.
- Mutating endpoints and sensitive login/session flows remain gated and are not
  run by default.

See [docs/migration-0.2.md](docs/migration-0.2.md) for call-site migration
examples and local development rules.

## License

[MIT](./LICENSE)

## References

- [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect)
- [reqwest](https://github.com/seanmonstar/reqwest)

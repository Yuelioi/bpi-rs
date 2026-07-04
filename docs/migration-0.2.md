# Migrating to bpi-rs 0.2

This guide tracks the current 0.2 migration surface in this repository. The
crate is moving from a broad flat API wrapper toward an idiomatic Rust SDK with
independent clients, explicit sessions, module clients, typed parameters,
payload-returning methods, and offline contract-backed tests.

The migration is intentionally breaking. Prefer updating call sites to the new
module-client style instead of wrapping the old flat method names.

## Client Construction

### 0.1 style

Older examples treated `BpiClient::new()` as a side-effecting constructor and
often assumed local account state could be loaded implicitly.

### 0.2 style

`BpiClient::new()` and `BpiClient::builder().build()` return a `BpiResult`.
Construction is explicit and does not read `account.toml` or install a global
tracing subscriber.

```rust
use bpi_rs::{BpiClient, BpiResult};

fn anonymous_client() -> BpiResult<BpiClient> {
    BpiClient::builder().build()
}
```

Use the builder when you need HTTP/session customization:

```rust
use std::time::Duration;

use bpi_rs::{BpiClient, BpiResult};

fn configured_client() -> BpiResult<BpiClient> {
    BpiClient::builder()
        .timeout(Duration::from_secs(15))
        .connect_timeout(Duration::from_secs(10))
        .user_agent("my-app/0.1")
        .referer("https://www.bilibili.com/")
        .origin("https://www.bilibili.com")
        .build()
}
```

## Session And Credentials

Credentials are explicit. Do not rely on constructor side effects.

### Cookie string

```rust
use bpi_rs::{BpiClient, BpiResult};

fn logged_in_client(cookie: &str) -> BpiResult<BpiClient> {
    BpiClient::builder().cookie(cookie).build()
}
```

### Account struct

```rust
use bpi_rs::{Account, BpiClient, BpiResult};

fn client_from_account(account: Account) -> BpiResult<BpiClient> {
    BpiClient::builder().account(account).build()
}
```

### Updating an existing client

```rust
client.set_account_from_cookie_str(
    "DedeUserID=123; SESSDATA=...; bili_jct=...; buvid3=...",
)?;
```

Use `client.clear_account()` when a client should return to guest state.

## Flat Methods To Module Clients

The preferred 0.2 API groups endpoints by domain:

```rust
let view = client.video().view(params).await?;
let nav = client.login().nav().await?;
let info = client.bangumi().info(params).await?;
```

Examples:

```rust
use bpi_rs::ids::{Bvid, MediaId};
use bpi_rs::video::VideoViewParams;
use bpi_rs::bangumi::BangumiInfoParams;

let bvid: Bvid = "BV1xx411c7mD".parse()?;
let video = client.video().view(VideoViewParams::from_bvid(bvid)).await?;

let bangumi = client
    .bangumi()
    .info(BangumiInfoParams::new(MediaId::new(28_220_978)?))
    .await?;
```

Available module clients currently include:

```text
activity, article, audio, bangumi, cheese, clientinfo, comment,
creativecenter, danmaku, dynamic, electric, fav, historytoview,
live, login, manga, message, misc, note, opus, search, user,
video, video_ranking, vip, wallet, web_widget
```

Mutating and flow-sensitive operations are still gated or intentionally kept out
of default examples.

## Response Handling

Migrated module-client methods generally return the decoded payload:

```rust
let view = client.video().view(params).await?;
println!("{}", view.title);
```

The public result alias is:

```rust
pub type BpiResult<T> = Result<T, BpiError>;
```

When a full Bilibili response envelope is required, use `ApiEnvelope<T>`.
When you are migrating a call site, prefer a module-client method that returns
`BpiResult<T>` directly. Remaining manga envelope aliases are compatibility
names over `ApiEnvelope<T>`.

If an endpoint can legitimately return success with `data: null`, its module
client method may return `BpiResult<Option<T>>`.

## Error Handling

Handle `BpiError` by category or semantic helper instead of matching only on
raw messages.

```rust
match result {
    Ok(payload) => {
        println!("ok: {payload:?}");
    }
    Err(error) if error.requires_login() => {
        eprintln!("login required");
    }
    Err(error) if error.is_risk_control() => {
        eprintln!("request was blocked by risk control");
    }
    Err(error) => return Err(error),
}
```

Useful helpers include:

```text
requires_login()
requires_vip()
is_permission_error()
is_risk_control()
semantic_error()
```

## QR Login

The crate exposes QR login primitives. Applications own QR rendering, polling
policy, timeouts, and session persistence.

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

The promoted `login.qr.flow` contract is a Probe flow that composes
`qr_generate` and `qr_poll` with a runtime `qrcode_key`; it is not a separate
module-client method.

## Custom Requests

For endpoints not yet wrapped by a module client, use the shared request
helpers. Prefer payload-returning helpers for normal JSON envelope endpoints.

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

Use `send_bpi_optional_payload` when an observed success response may omit or
null out the payload.

## Tests, Probe, And Local Credentials

Default development checks are intended to stay offline:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features --lib
```

Live Probe work is separate. Raw Probe output and drafts stay local under
`target/`:

```text
target/bpi-contract-drafts/...
target/bpi-probe-runs/...
target/bpi-probe-notes/...
```

Committed endpoint evidence belongs under:

```text
tests/contracts/<domain>/<endpoint>/contract.json
tests/contracts/<domain>/<endpoint>/responses/<case>.json
```

Do not commit `account.toml`, cookies, `SESSDATA`, `bili_jct`, `buvid`, raw
Probe output, or account-specific response data.

## Migration Checklist

- Replace flat calls with `client.<domain>().<method>(...)`.
- Change `BpiClient::new()` call sites to handle `BpiResult`.
- Seed credentials explicitly through the builder, `Account`, or
  `set_account_from_cookie_str`.
- Prefer typed IDs such as `Aid`, `Bvid`, `Cid`, `Mid`, `MediaId`, `SeasonId`,
  and `EpisodeId`.
- Prefer payload-returning `BpiResult<T>` methods over envelope-returning legacy
  helpers.
- Keep live and mutating behavior behind explicit opt-in controls.
- Run the offline verification gates before publishing changes.

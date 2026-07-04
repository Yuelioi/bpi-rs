# bpi-rs

面向 Rust 的 Bilibili API SDK，基于 `reqwest` 和 `tokio`。

`bpi-rs` 0.2 主打模块化 API、显式登录态、类型化参数、直接返回业务 payload，以及可离线验证的接口合同。它适合需要在 Rust 项目里批量接入 B 站接口的工具、自动化程序、数据采集程序和服务端应用。

[English README](./README.en.md)

## 项目优势

| 特性 | 说明 |
| --- | --- |
| API 覆盖广 | 覆盖视频、番剧、用户、搜索、直播、动态、评论、收藏夹、音频、创作中心、消息、大会员、钱包、稍后再看等二十多个模块。 |
| 模块化调用 | 使用 `client.video().view(...)`、`client.login().nav()`、`client.user().card(...)` 这类领域客户端，不再依赖巨大扁平方法列表。 |
| 类型化参数 | 常见 ID 和请求参数有专门类型，例如 `Bvid`、`Aid`、`Mid`、`MediaId`、`VideoViewParams`，能提前挡住一部分无效调用。 |
| 直接返回业务数据 | 迁移后的模块 API 返回 `BpiResult<T>`，`T` 是解码后的业务 payload。需要完整响应外壳时再使用 `ApiEnvelope<T>`。 |
| 登录态显式可控 | 不会在构造客户端时偷偷读取本地账号。Cookie、`Account` 和测试账号配置都需要显式传入。 |
| 有合同和探针证据 | 已迁移接口配有 `tests/contracts/**` 下的合同和脱敏响应样例，便于回归测试和后续升级。 |
| 发布前验证完整 | 默认检查覆盖格式化、clippy、全 feature 编译、库测试、文档测试和 crates.io dry run。 |

## 安装

```toml
[dependencies]
bpi-rs = "0.2"
```

或：

```bash
cargo add bpi-rs
```

## 快速开始

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

## 登录态接口

需要登录的接口请显式传 Cookie：

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

也可以传入 `Account`：

```rust
use bpi_rs::{Account, BpiClient};

let account = Account {
    dede_user_id: "123".into(),
    sessdata: "...".into(),
    bili_jct: "...".into(),
    buvid3: "...".into(),
};

let client = BpiClient::builder().account(account).build()?;
```

## 常用模块

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

当前模块客户端包括：

```text
activity, article, audio, bangumi, cheese, clientinfo, comment,
creativecenter, danmaku, dynamic, electric, fav, historytoview,
live, login, manga, message, misc, note, opus, search, user,
video, video_ranking, vip, wallet, web_widget
```

可编译示例见 [`examples/module_clients.rs`](examples/module_clients.rs)：

```bash
cargo check --all-features --examples
BPI_RUN_EXAMPLE=1 cargo run --example module_clients
```

设置 `BPI_COOKIE` 后，示例可以额外调用 `client.login().nav()` 等登录态接口。

## 返回值和错误

迁移后的模块 API 默认直接返回业务 payload：

```rust
let view = client.video().view(params).await?;
```

公共结果类型是：

```rust
pub type BpiResult<T> = Result<T, BpiError>;
```

`BpiError` 会区分网络、HTTP 状态码、JSON 解码、API 业务错误、认证、参数错误、缺失数据和不支持的响应格式。它还提供：

```text
requires_login()
requires_vip()
is_permission_error()
is_risk_control()
semantic_error()
```

需要完整 B 站响应外壳时使用 `ApiEnvelope<T>`：

```rust
use bpi_rs::ApiEnvelope;
```

漫画模块里剩余的 envelope 返回别名是兼容名，本质上也是 `ApiEnvelope<T>`。付费漫画阅读这类接口目前不作为 0.2 可用能力承诺。

## 自定义请求

如果某个接口还没有封装，可以使用共享请求 helper。普通 JSON envelope 接口优先使用 payload 返回：

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

如果成功响应可能没有 `data`，使用 `send_bpi_optional_payload`。

需要 CSRF 的接口请显式获取：

```rust
let csrf = client.csrf()?;
```

## 二维码登录

SDK 提供二维码登录的基础接口，但不负责渲染二维码、轮询策略、超时控制或本地持久化：

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

## 开发和验证

默认检查应保持离线、稳定：

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features --lib
cargo test --doc
```

Live Probe 是显式 opt-in 工作流，原始输出保存在 `target/`：

```text
target/bpi-contract-drafts/...
target/bpi-probe-runs/...
target/bpi-probe-notes/...
```

本地凭据可放在 `account.toml` 供 Probe/开发使用。不要提交 `account.toml`、Cookie、`SESSDATA`、`bili_jct`、`buvid`、原始 Probe 输出或账号相关响应数据。

示例本地配置：

```toml
[probe.normal]
bili_jct = "..."
dede_user_id = 123
sessdata = "..."
buvid3 = "..."

[probe.vip]
bili_jct = "..."
dede_user_id = 456
sessdata = "..."
buvid3 = "..."
```

## 迁移说明

- 旧扁平 API 不再是主文档路径。
- 新模块客户端在合同支持时直接返回业务 payload。
- `tests/contracts/**` 下的接口合同和脱敏 fixtures 是迁移后的行为依据。
- 变更类接口和敏感登录/session 流程默认不运行 live 测试。

更多调用迁移示例见 [docs/migration-0.2.md](docs/migration-0.2.md)。

## License

[MIT](./LICENSE)

## References

- [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect)
- [reqwest](https://github.com/seanmonstar/reqwest)

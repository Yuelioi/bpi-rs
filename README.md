# bpi-rs

## 相关项目

- [bpi-py](https://github.com/Yuelioi/bpi-py)：Python 版本。
- [bpi-go](https://github.com/Yuelioi/bpi-go)：Go 版本。

一个面向 Rust 的非官方异步 Bilibili Web API SDK，基于 `reqwest` 和 `tokio`。

`bpi-rs` 提供模块化 API、显式登录态、类型化参数、直接返回业务 payload，以及可离线验证的接口契约，用来减少重复处理 Bilibili Web 接口细节的工作。

> [!IMPORTANT]
> `bpi-rs` 是非官方第三方项目，与哔哩哔哩（Bilibili）无隶属、合作、授权或官方支持关系。
>
> 本项目涉及的 Web 接口并非官方公开稳定 API，可能随时变更、失效或受到访问限制。本项目仅用于个人学习、研究与技术交流。使用者应自行遵守适用法律法规、平台服务条款与账号权限要求，并对自己的使用行为负责。
>
> 请勿将本项目用于绕过访问控制、未经授权获取数据、侵犯隐私或其他合法权益、大规模高频请求、干扰平台正常服务等用途。

## 为什么用 bpi-rs？

从使用者角度，主要是少写很多和 Bilibili Web 接口本身有关的重复代码。

| 特性 | 说明 |
| --- | --- |
| API 覆盖广 | 覆盖视频、番剧、用户、搜索、直播、动态、评论、收藏夹、音频、创作中心、消息、大会员、钱包、稍后再看等二十多个模块。 |
| 模块化调用 | 使用 `client.video().view(...)`、`client.login().nav()`、`client.user().card(...)` 这类领域客户端，不再依赖巨大扁平方法列表。 |
| 类型化参数 | 常见 ID 和请求参数有专门类型，例如 `Bvid`、`Aid`、`Mid`、`MediaId`、`VideoViewParams`，能提前挡住一部分无效调用。 |
| 直接返回业务数据 | 迁移后的模块 API 返回 `BpiResult<T>`，`T` 是解码后的业务 payload。需要完整响应外壳时再使用 `ApiEnvelope<T>`。 |
| 登录态显式可控 | 不会在构造客户端时偷偷读取本地账号。Cookie、`Account` 和测试账号配置都需要显式传入。 |
| 有契约和探针证据 | 已迁移接口配有 `tests/contracts/**` 下的契约和脱敏响应样例，便于回归测试和后续升级。 |
| 发布前验证完整 | 默认检查覆盖格式化、clippy、全 feature 编译、库测试、文档测试和 crates.io dry run。 |

## 安装

```toml
[dependencies]
bpi-rs = "0.3"
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

请只使用你有权使用的账号凭据，并妥善保管 Cookie、`SESSDATA`、CSRF token 等敏感信息，不要将它们提交到仓库、日志或公开环境。

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

漫画模块里剩余的 envelope 返回别名是兼容名，本质上也是 `ApiEnvelope<T>`。付费漫画阅读这类接口目前不作为 0.3 可用能力承诺。

## 响应字段变化导致解析失败

已封装的领域方法可能因为 Bilibili 调整响应字段类型而解码失败。例如，原本记录为 `u64` 的字段开始返回负数时，当前版本的内置模型就无法解析。

这类错误会返回 `BpiError::ResponseDecode`。可以通过 `response_body()` 取得同一次请求的原始响应，再使用临时模型恢复。领域方法已经完成的参数构造、WBI 签名、Cookie 和请求头都会保留，不需要重新发送请求：

```rust
use bpi_rs::{ApiEnvelope, BpiError, BpiResult};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TemporaryPayload {
    last_play_time: i64,
}

fn recover_temporary_payload(error: &BpiError) -> BpiResult<Option<TemporaryPayload>> {
    let Some(body) = error.response_body() else {
        return Ok(None);
    };

    let payload = ApiEnvelope::<TemporaryPayload>::from_slice(body)?.into_payload()?;
    Ok(Some(payload))
}
```

`response_body()` 只对 HTTP 响应模型解码失败返回 `Some`。原始响应不会进入错误的 `Display`、`Debug`、序列化结果或 tracing 日志。

临时模型只用于等待 bpi-rs 发布正式修复。发现新的响应字段变化后，请提交 issue 或 PR。

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

## 项目文档

| 文档 | 内容 |
| --- | --- |
| [API 索引](docs/api-index.md) | 自动生成的接口清单，包含函数说明、风险分类、profiles、URL、契约路径和 Rust 模型，可作为 AI 理解本仓库 API 的接口地图。 |
| [开发和验证](docs/development.md) | 本地检查、Taskfile 入口、API 索引生成、只读 Probe、账号配置和风险门控。 |
| [新 API 探针开发指南](docs/api-probe-development.md) | 新增或迁移接口时如何先跑 Probe、沉淀契约、脱敏响应并补测试。 |
| [API 风险分类](docs/api-risk-classification.md) | `public-read`、`authenticated-read`、`private-read`、`mutating`、`spending`、`login-session` 的定义和门控规则。 |
| [0.2 迁移指南](docs/migration-0.2.md) | 从旧扁平 API 迁移到模块客户端、显式登录态和 payload 返回风格。 |
| [发布检查清单](docs/release-checklist.md) | 发布前检查、打包和版本确认流程。 |
| [贡献指南](CONTRIBUTING.md) | 贡献流程和代码协作约定。 |
| [安全策略](SECURITY.md) | 安全问题报告方式。 |
| [变更日志](CHANGELOG.md) | 版本变更记录。 |

## 接口稳定性

Bilibili Web API 并不是官方稳定公开 API，上游接口、字段和错误码可能随时变化。

`bpi-rs` 不保证第三方接口的可用性、稳定性或持续兼容，也不代表使用这些接口已经获得平台授权。请根据自己的使用场景控制请求频率、数据范围和账号权限。

## License

[MIT](./LICENSE)

## References

- [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect)
- [reqwest](https://github.com/seanmonstar/reqwest)

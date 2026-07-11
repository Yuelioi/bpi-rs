# 迁移到 bpi-rs 0.2

这份文档记录当前仓库的 0.2 迁移面。`bpi-rs` 正在从宽泛的扁平 API 包装，迁移到更符合 Rust 习惯的 SDK：独立模块客户端、显式登录态、类型化参数、直接返回业务 payload，以及由离线契约支撑的测试。

0.2 是有意设计的破坏性迁移。请优先把调用方改到新的模块客户端风格，不要继续包一层旧扁平方法名。

## 客户端构造

### 0.1 风格

旧示例通常把 `BpiClient::new()` 当作带副作用的构造函数，并默认本地账号状态会被隐式加载。

### 0.2 风格

`BpiClient::new()` 和 `BpiClient::builder().build()` 返回 `BpiResult`。构造过程是显式的，不会读取 `account.toml`，也不会安装全局 tracing subscriber。

```rust
use bpi_rs::{BpiClient, BpiResult};

fn anonymous_client() -> BpiResult<BpiClient> {
    BpiClient::builder().build()
}
```

需要自定义 HTTP/session 行为时使用 builder：

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

## 登录态和凭据

凭据必须显式传入。不要依赖构造函数副作用。

### Cookie 字符串

```rust
use bpi_rs::{BpiClient, BpiResult};

fn logged_in_client(cookie: &str) -> BpiResult<BpiClient> {
    BpiClient::builder().cookie(cookie).build()
}
```

### Account 结构体

```rust
use bpi_rs::{Account, BpiClient, BpiResult};

fn client_from_account(account: Account) -> BpiResult<BpiClient> {
    BpiClient::builder().account(account).build()
}
```

### 更新已有客户端

```rust
client.set_account_from_cookie_str(
    "DedeUserID=123; SESSDATA=...; bili_jct=...; buvid3=...",
)?;
```

客户端需要回到游客状态时，调用 `client.clear_account()`。

## 从扁平方法迁移到模块客户端

0.2 推荐按领域分组调用接口：

```rust
let view = client.video().view(params).await?;
let nav = client.login().nav().await?;
let info = client.bangumi().info(params).await?;
```

示例：

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

当前模块客户端包括：

```text
activity, article, audio, bangumi, cheese, clientinfo, comment,
creativecenter, danmaku, dynamic, electric, fav, historytoview,
live, login, manga, message, misc, note, opus, search, user,
video, video_ranking, vip, wallet, web_widget
```

变更类和流程敏感接口仍然需要门控，或者有意不放进默认示例。

## 返回值处理

已迁移的模块客户端方法通常直接返回解码后的业务 payload：

```rust
let view = client.video().view(params).await?;
println!("{}", view.title);
```

公共结果别名是：

```rust
pub type BpiResult<T> = Result<T, BpiError>;
```

需要完整 B 站响应外壳时使用 `ApiEnvelope<T>`。迁移调用点时，优先选择直接返回 `BpiResult<T>` 的模块客户端方法。漫画模块里剩余的 envelope 别名只是 `ApiEnvelope<T>` 的兼容名称。

如果接口确实可能在成功时返回 `data: null`，对应模块客户端方法可以返回 `BpiResult<Option<T>>`。

## 错误处理

处理 `BpiError` 时，优先按类别或语义 helper 分支，不要只匹配原始消息。

```rust
match result {
    Ok(payload) => {
        println!("ok: {payload:?}");
    }
    Err(error) if error.requires_login() => {
        eprintln!("需要登录");
    }
    Err(error) if error.is_risk_control() => {
        eprintln!("请求被风控拦截");
    }
    Err(error) => return Err(error),
}
```

常用 helper：

```text
requires_login()
requires_vip()
is_permission_error()
is_risk_control()
semantic_error()
response_body()
```

## 二维码登录

SDK 暴露二维码登录基础接口。应用侧负责二维码渲染、轮询策略、超时控制和 session 持久化。

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

已提升的 `login.qr.flow` 契约是一个 Probe flow，会把 `qr_generate` 和带运行时 `qrcode_key` 的 `qr_poll` 组合起来；它不是单独的模块客户端方法。

## 自定义请求

尚未由模块客户端封装的接口，可以使用共享请求 helper。普通 JSON envelope 接口优先使用 payload 返回 helper。

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

观察到成功响应可能省略 payload 或返回 `null` 时，使用 `send_bpi_optional_payload`。

如果模块客户端已经封装了 endpoint，但内置响应模型暂时跟不上真实 schema，可以在 `BpiError::ResponseDecode` 上通过 `response_body()` 取得同一次请求的原始响应，再使用调用方临时定义的 `ApiEnvelope<T>` 解析。该恢复路径会保留模块客户端已经完成的签名、认证和参数构造，不需要重新发送请求。

原始响应是显式访问的敏感数据，不会出现在错误格式化、序列化或 tracing 日志中。临时模型不应成为长期分叉；确认响应变化后请向 bpi-rs 提交 issue 或 PR。

0.2 为此新增了 `BpiError::ResponseDecode`。下游匹配 `BpiError` 时应保留通配分支并优先使用语义 helper，避免依赖错误枚举永远不增加变体。

## 测试、探针和本地凭据

默认开发检查应保持离线：

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features --lib
```

Live Probe 是独立工作流。原始 Probe 输出和草稿只保存在本地 `target/`：

```text
target/bpi-contract-drafts/...
target/bpi-probe-runs/...
target/bpi-probe-notes/...
```

可提交的接口证据放在：

```text
tests/contracts/<domain>/<endpoint>/contract.json
tests/contracts/<domain>/<endpoint>/responses/<case>.json
```

不要提交 `account.toml`、Cookie、`SESSDATA`、`bili_jct`、`buvid3`、原始 Probe 输出或账号相关响应数据。

新增或迁移接口前，先按 [API 风险分类](api-risk-classification.md) 判断属于 `public-read`、`authenticated-read`、`private-read`、`mutating`、`spending` 还是 `login-session`。`mutating` 和 `spending` 测试必须使用 `#[ignore]` 和显式环境变量门控。

## 迁移检查清单

- 把扁平调用替换为 `client.<domain>().<method>(...)`。
- 修改 `BpiClient::new()` 调用点，处理 `BpiResult`。
- 通过 builder、`Account` 或 `set_account_from_cookie_str` 显式注入凭据。
- 优先使用类型化 ID，例如 `Aid`、`Bvid`、`Cid`、`Mid`、`MediaId`、`SeasonId` 和 `EpisodeId`。
- 优先使用直接返回 payload 的 `BpiResult<T>` 方法，不继续依赖旧 envelope helper。
- 只有文档或真实响应证明字段可能缺失/为 `null` 时才使用 `Option<T>`。
- live、变更类和消费资产行为必须放在显式 opt-in 门控后。
- 发布改动前运行离线验证门禁。

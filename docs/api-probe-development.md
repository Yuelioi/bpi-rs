# 新 API 探针开发指南

这份文档给想给 `bpi-rs` 添加新接口的人看。目标不是“先写模型再猜字段”，而是先用真实探针结果确认请求和响应，再把稳定证据落到代码、契约和测试里。

## 基本原则

- 真实 Probe 结果是接口行为的主要依据。
- 不要提交 `account.toml`、Cookie、`SESSDATA`、`bili_jct`、`buvid3`、原始 Probe 输出或账号相关响应数据。
- 原始 Probe 输出只放在 `target/bpi-probe-runs/...`。
- 可提交的只有审查过的接口契约和脱敏响应样例。
- 默认按模块或子模块成批开发，不要一个接口一个提交。
- 新接口必须先按 [API 风险分类](api-risk-classification.md) 标记风险。
- `mutating` 和 `spending` 接口必须显式说明风险，并用环境变量和 `#[ignore]` 双重门控。

## 账号配置

本地 `account.toml` 只支持结构化 profile：

```toml
[normal]
bili_jct = "..."
dede_user_id = "123"
sessdata = "..."
buvid3 = "..."

[vip]
bili_jct = "..."
dede_user_id = "456"
sessdata = "..."
buvid3 = "..."
```

旧格式 `bili_jct_vip`、`dede_user_id_vip`、`*_normal` 不再支持。`normal` 表示普通登录账号，`vip` 表示 VIP 登录账号，`anonymous` 表示不带账号。

## 开发流程

1. 选一个小批次
   - 优先选择同一个模块或子模块里的相关接口。
   - 标记每个接口的风险分类：`public-read`、`authenticated-read`、`private-read`、`mutating`、`spending` 或 `login-session`。
   - 同一个批次尽量只包含同一风险等级；`mutating`、`spending`、`login-session` 不要和普通只读接口混在一个批次里。

2. 写请求草稿
   - 草稿放到 `target/bpi-contract-drafts/<domain>/<batch>/`。
   - 写清 method、URL、query/body、auth profile、是否需要 cookie/csrf/wbi。
   - 写清风险分类、是否有副作用、是否消耗资产、是否涉及 session 或设备标识。
   - 草稿不是事实，只是用来跑探针。

3. 跑 Probe

```powershell
$env:BPI_PROBE = "1"
cargo run --quiet --bin bpi-probe -- `
  target\bpi-contract-drafts\<domain>\<batch>\<endpoint>.json `
  account.toml `
  target\bpi-probe-runs\<domain>\<batch>\<endpoint>\<profile>.response.json
```

常见 profile：

```text
anonymous
normal
vip
```

4. 对比结果
   - Rust 当前请求参数是否和 Probe 捕获请求一致。
   - 匿名、普通、VIP 的响应结构是否不同。
   - 成功响应字段是否可能缺失或为 `null`。
   - 错误码是否应该映射成稳定语义，比如 `requires_login`、`requires_vip`、`risk_control`。
   - 响应里是否含有 mid、昵称、私信正文、邮箱、手机号、IP、设备标识、Cookie 或 token。

5. 提交可审查证据
   - 契约放到 `tests/contracts/<domain>/<endpoint>/contract.json`。
   - 脱敏响应样例放到 `tests/contracts/<domain>/<endpoint>/responses/*.json`。
   - 不提交 `target/bpi-probe-runs` 里的原始输出。

6. 写 Rust 代码
   - 新接口挂到对应模块客户端，例如 `client.video()`、`client.login()`、`client.live()`。
   - 用类型化参数封装请求参数。
   - 成功 API 优先直接返回业务 payload。
   - 响应字段只有在文档或真实响应证明可能缺失/为 `null` 时才用 `Option<T>`。

7. 补测试
   - 参数序列化测试。
   - 契约请求匹配测试。
   - 响应 fixture 反序列化测试。
   - 本地 Probe 输出存在时的兼容解析测试。
   - `mutating` 和 `spending` live 测试必须同时使用 `#[ignore]` 和显式环境变量门控。

## 风险分类速查

| 分类 | 例子 | 测试要求 |
| --- | --- | --- |
| `public-read` | 公开视频、公开评论、公开用户信息 | 可以进入默认离线契约测试 |
| `authenticated-read` | 登录状态、当前账号可见的只读信息 | 不默认 live，fixture 必须脱敏 |
| `private-read` | 钱包、消息、创作中心、直播管理数据 | 不默认 live，fixture 必须最小化和脱敏 |
| `mutating` | 点赞、收藏、关注、评论、发弹幕、开播、关播、删除 | `#[ignore]` 加环境变量门控 |
| `spending` | 投币、支付、购买、兑换、充电 | 双重门控，并要求显式目标参数 |
| `login-session` | 二维码登录、Cookie 刷新、短信、风控相关接口 | 不提交 token、Cookie、设备标识或原始响应 |

## 完成前检查

至少运行：

```powershell
cargo fmt --check
cargo check --all-features
cargo test --all-features
```

如果改了 examples：

```powershell
cargo check --examples --all-features
```

如果改了 release 条件编译或日志：

```powershell
cargo check --release --all-features
```

提交前确认没有把本地探针产物带进去：

```powershell
git status --short
git status --short --ignored=matching target\bpi-contract-drafts
git status --short --ignored=matching target\bpi-probe-runs
git status --short --ignored=matching target\bpi-probe-notes
```

## 提交建议

推荐一个模块批次一个提交：

```text
feat(<domain>): 验证 <batch> 接口契约
fix(<domain>): 修正 <endpoint> 响应模型
refactor(<domain>): 适配 <batch> 到模块客户端风格
```

不要把无关文档、格式化大扫除、工具重构和接口批次混在一个提交里。

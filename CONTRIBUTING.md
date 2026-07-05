# 贡献指南

感谢贡献 `bpi-rs`。本项目涉及登录态、Cookie、风控和变更类接口，贡献时请优先保证账号安全和可复现验证。

## 基本规则

- 对话、代码注释、提交信息使用中文；外部 API 字段、协议关键字、命令、路径、crate 名称保持原文。
- 不要提交 `account.toml`、Cookie、`SESSDATA`、`bili_jct`、`buvid3`、原始 Probe 输出或账号相关响应数据。
- fixtures 必须脱敏，不能包含真实账号昵称、mid、手机号、邮箱、IP、Cookie、签名 URL 或私有消息内容。
- 变更类接口测试必须默认不运行，并同时使用 `#[ignore]` 和显式环境变量门控。

## API 风险分类

新增或迁移 API 时，请先分类：

- `public-read`：不需要登录、不会改变状态。
- `authenticated-read`：需要登录，但只读取当前账号状态。
- `private-read`：读取私有账号数据，需要更严格脱敏。
- `mutating`：会点赞、收藏、关注、评论、发布、删除、开播、关播或修改账号状态。
- `spending`：会投币、支付、购买、兑换或消耗资产。
- `login-session`：涉及登录、二维码、Cookie、刷新登录态或风控。

`mutating`、`spending`、`login-session` 不能作为默认测试路径。

## 本地验证

默认验证应保持离线、稳定：

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features
cargo test --doc
cargo check --examples --all-features
```

需要网络探针时，显式设置：

```powershell
$env:BPI_PROBE = "1"
```

需要运行变更类 live 测试时，必须同时设置：

```powershell
$env:BPI_LIVE_TEST = "1"
$env:BPI_MUTATING_TEST = "1"
```

## PR 流程

维护者处理外部 PR 时优先使用 GitHub/`gh` 的正常流程：

```powershell
gh pr checkout <number>
```

在 PR 分支上 review、改成当前代码风格、补测试，再通过 GitHub merge 保留贡献者上下文。不要在 `main` 上直接复制外部 PR 内容后丢失贡献来源。

## 提交信息

提交信息使用中文，推荐格式：

```text
feat(video): 增加视频投币状态接口
fix(login): 处理每日奖励风控响应
docs(probe): 补充探针脱敏要求
test(video): 增加投币参数校验
```

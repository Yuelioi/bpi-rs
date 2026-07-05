# API 风险分类

新增、迁移或暴露 API 前，先给接口归类。分类决定是否允许默认测试、是否需要账号、是否必须脱敏，以及示例能否直接运行。

## 分类

| 分类 | 含义 | 默认测试 | 账号数据要求 |
| --- | --- | --- | --- |
| `public-read` | 不需要登录，只读取公开信息 | 可以运行 | fixture 仍需最小化 |
| `authenticated-read` | 需要登录，但只读取当前账号状态 | 默认不依赖 live；可用契约测试 | 必须脱敏账号字段 |
| `private-read` | 读取私有账号、创作中心、钱包、消息或管理数据 | 不默认 live | 必须脱敏并最小化字段 |
| `mutating` | 点赞、收藏、关注、评论、发弹幕、发布、删除、开播、关播等 | 必须 `#[ignore]` 或环境变量门控 | 不提交真实响应 |
| `spending` | 投币、支付、购买、兑换、充电等会消耗资产的接口 | 必须双重门控 | 不提交真实响应 |
| `login-session` | 登录、二维码、短信、Cookie 刷新、风控相关接口 | 不默认 live | 不提交 token、Cookie 或设备标识 |

## 测试规则

- 默认 `cargo test --all-features` 必须稳定、离线、无账号副作用。
- `mutating` 和 `spending` 测试必须同时满足：
  - `#[ignore]`
  - 明确环境变量，例如 `BPI_MUTATING_TEST=1`
  - 明确目标参数，例如 `BPI_VIDEO_AID`、`BPI_VIDEO_BVID`、`BILI_ROOM_ID`
- live 只读诊断测试也应 `#[ignore]`，避免 CI 依赖网络和账号。
- 探针结果进入 `tests/contracts/**` 前必须脱敏。

## 示例规则

- 只读示例可以直接运行，但应允许用环境变量替换目标 ID。
- 真实网络请求如果可能增加风控压力，应加显式开关，例如 `BPI_RUN_EXAMPLE=1`。
- 变更类示例必须在执行动作前检查 `BPI_MUTATING_TEST=1`。
- 默认路径不能开播、关播、投币、点赞、发评论、发弹幕、购买或删除内容。

## PR 检查

PR 必须说明 API 分类，并列出验证命令。新增 fixtures 时确认：

- 没有 Cookie、`SESSDATA`、`bili_jct`、`buvid3`
- 没有真实账号昵称、mid、手机号、邮箱、IP、私信正文
- 没有原始 Probe 输出或完整私有响应

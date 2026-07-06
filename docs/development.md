# 开发和验证

这份文档记录本仓库的本地开发、验证、API 索引和 Probe 工作流。面向维护者和协作开发者；普通使用者优先看 `README.md` 和 `docs/api-index.md`。

## 默认检查

默认检查应保持离线、稳定，不依赖账号，也不能触发有副作用接口。

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features --lib
cargo test --doc
cargo check --examples --all-features
cargo check --no-default-features --lib
```

常用 Taskfile 入口：

```powershell
task fmt_check
task test_offline
task contract_test
task probe_contract_audit
task probe_sanitize_audit
task check_feature_matrix
```

## API 索引

API 索引用契约和源码自动生成：

```powershell
task api_doc
```

生成结果写入 `docs/api-index.md`，包含模块、API 名称、风险分类、profiles、方法、URL、Rust 模型、契约路径和匹配到的 `pub async fn` 源码路径。函数匹配按模块优先；匹配不到时显示 `-`，后续可以通过统一契约名和函数名继续补齐。

## Probe 工作流

Live Probe 是显式 opt-in 工作流。原始输出只放在 `target/`：

```text
target/bpi-contract-drafts/...
target/bpi-probe-runs/...
target/bpi-probe-notes/...
```

不要提交 `account.toml`、Cookie、`SESSDATA`、`bili_jct`、`buvid3`、原始 Probe 输出或账号相关响应数据。

批量运行已提交契约：

```powershell
$env:BPI_PROBE = "1"
cargo run --bin bpi-probe -- batch-run tests/contracts/video/info-read `
  --account account.toml `
  --profiles anonymous,normal,vip `
  --pages 10 `
  --output target/bpi-probe-runs
```

只跑全量只读契约：

```powershell
task probe_read_only
```

`probe_read_only` 会覆盖 `anonymous`、`normal`、`vip` 三个 profile，并跳过 `mutating`、`spending`、`login-session` 风险分类。运行前必须确认本地 `account.toml` 里有 `[normal]` 和 `[vip]`，否则登录态用例会失败。

`batch-run` 默认不会运行网络请求，必须设置 `BPI_PROBE=1`。`mutating`、`spending`、`login-session` 还需要额外环境变量门控，避免误触发有副作用接口。`--pages` 默认为 `10`，会展开本身带 `page`、`pn` 或 `pageNum` 的普通分页契约；`historytoview.history_list` 使用响应里的 `data.cursor.max/view_at/business` 继续翻页；非分页契约仍只运行一次。

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

## 风险门控

变更类接口包括点赞、投币、收藏、关注、评论、发弹幕、开播/关播、发布/删除内容、支付或兑换等。相关示例和 live 测试默认不运行；确需执行时必须确认账号和目标资源，并显式设置对应环境变量。

风险分类和门控规则见 `docs/api-risk-classification.md`。

## 新接口开发

新增或迁移接口前，先阅读：

- `docs/api-probe-development.md`
- `docs/api-risk-classification.md`
- `docs/api-index.md`

新接口应先用真实 Probe 结果确认请求和响应，再把稳定证据落到代码、契约和测试里。

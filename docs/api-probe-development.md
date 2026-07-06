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

批量运行已提交契约：

```powershell
$env:BPI_PROBE = "1"
cargo run --bin bpi-probe -- batch-run tests/contracts/video/info-read `
  --account account.toml `
  --profiles anonymous,normal,vip `
  --pages 10 `
  --output target/bpi-probe-runs
```

`batch-run` 默认不会运行网络请求，必须设置 `BPI_PROBE=1`。`mutating`、`spending`、`login-session` 还需要额外环境变量门控，避免误触发有副作用接口。`--pages` 默认为 `10`，会展开本身带 `page`、`pn` 或 `pageNum` 的普通分页契约；`historytoview.history_list` 使用响应里的 `data.cursor.max/view_at/business` 继续翻页；非分页契约仍只运行一次。

只跑全量只读契约：

```powershell
task probe_read_only
```

这个任务等价于带 `--read-only` 的 `batch-run tests/contracts`，会覆盖 `anonymous`、`normal`、`vip` 三个 profile，并跳过 `mutating`、`spending`、`login-session` 风险分类。运行前必须确认本地 `account.toml` 里有 `[normal]` 和 `[vip]`，否则登录态用例会失败。

常见 profile：

```text
anonymous
normal
vip
```

离线检查契约字段、脱敏残留和结构一致性：

```powershell
task probe_fields
task probe_sanitize_audit
task probe_contract_audit
task contract_test
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
   - 新增契约优先补齐 `schema_version`、`module`、`batch`、`endpoint`、`risk`、`status`、`profiles`、`sanitize` 和 `provenance` 字段。

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

## 契约字段

新增或迁移契约时，推荐使用 v2 元数据字段。旧契约可以暂时缺省这些字段；一旦声明 `schema_version: 2`，`module`、`batch`、`endpoint`、`risk`、`status` 和 `profiles` 都是必填字段。

```json
{
  "schema_version": 2,
  "name": "video.view",
  "module": "video",
  "batch": "info-read",
  "endpoint": "view",
  "risk": "public-read",
  "status": "promoted",
  "profiles": ["anonymous", "normal", "vip"],
  "request": {
    "method": "GET",
    "url": "https://api.bilibili.com/x/web-interface/view",
    "query": {
      "bvid": "BV1xx411c7mD"
    },
    "auth": {
      "requires": []
    }
  },
  "sanitize": {
    "preset": ["account"],
    "replace": {
      "$.data.owner.mid": 1000001,
      "$.data.owner.name": "sanitized-user"
    },
    "drop": [],
    "keep": ["$.data.title", "$.data.bvid"]
  },
  "provenance": {
    "source": "local_probe_output",
    "observed_at": "2026-07-06",
    "tool": "bpi-probe",
    "tool_version": "0.2.0",
    "privacy": "account fields sanitized before commit"
  },
  "cases": []
}
```

字段说明：

| 字段 | 含义 |
| --- | --- |
| `schema_version` | 契约格式版本。旧契约缺省按 v1 读取；新契约使用 `2`。 |
| `module` | SDK 模块名，例如 `video`、`login`、`live`。 |
| `batch` | 探针批次名，对应目录层级，例如 `info-read`。 |
| `endpoint` | 批次内 endpoint 名，例如 `view`。 |
| `risk` | API 风险分类，必须来自上面的六类。 |
| `status` | 契约状态：`draft`、`probed`、`promoted`、`blocked`、`deprecated`。 |
| `profiles` | 计划覆盖的 profile 列表，例如 `anonymous`、`normal`、`vip`。 |
| `sanitize` | 当前 endpoint 的脱敏覆盖规则。通用规则放在代码内置字段表里。 |
| `provenance` | 探针来源和工具信息，不能包含账号身份、本机路径、Cookie、IP 或设备标识。 |

`sanitize` 只写 endpoint 特有规则。Cookie、token、csrf、`buvid3`、手机号、邮箱、IP、设备标识等通用敏感字段由内置脱敏字段表统一处理。

## 自动审计

`bpi-probe` 提供三个不需要账号的离线审计命令：

| 命令 | 用途 |
| --- | --- |
| `fields` | 统计契约和响应样例里的 JSON 字段路径，用来维护脱敏字段表。 |
| `sanitize-audit` | 扫描 `responses/*.json` 中高置信度的 Cookie、csrf、`buvid3`、邮箱等敏感残留。 |
| `contract-audit` | 检查 `contract.json` 能否解析、fixture 是否存在、`fixture_kind` 是否已知、profile 是否冲突、v2 元数据是否完整。 |

`contract-audit` 要求已迁移契约补齐 v2 元数据；旧格式兼容只用于读取历史文件，不应再用于新增契约。

`contract_test` 是统一离线契约测试入口，会复用结构审计，并额外检查 fixture JSON、`api_code` 一致性、高置信敏感残留，以及已注册 Rust 模型的反序列化。给契约声明新的 `rust_model` 后，应同步把模型加入 `src/probe/model.rs` 的注册表，否则统一测试和实时批量探针只能校验响应 code，不能发现字段类型漂移。

`batch-run` 是真实网络探针入口，输出到 `target/bpi-probe-runs/<module>/<batch>/<endpoint>/<case>.response.json`。分页契约会输出 `target/bpi-probe-runs/<module>/<batch>/<endpoint>/<case>.pageN.response.json`，这些原始输出不能提交。批量探针会对已注册的 `rust_model` 实时反序列化；多页响应里的字段类型异常会在这里失败。

API 索引用契约和源码自动生成：

```powershell
task api_doc
```

生成结果写入 `docs/api-index.md`，包含模块、API 名称、风险分类、profiles、方法、URL、Rust 模型、契约路径和匹配到的 `pub async fn` 源码路径。函数匹配按模块优先；匹配不到时显示 `-`，后续可以通过统一契约名和函数名继续补齐。

探针结果确认可用后，用 `promote` 生成可提交 fixture：

```powershell
cargo run --bin bpi-probe -- promote `
  target/bpi-probe-runs/video/info-read/view/anonymous.response.json
```

`promote` 会按路径反推 `tests/contracts/<module>/<batch>/<endpoint>/contract.json`，写入脱敏后的 `responses/*.json`，更新对应 case 的 `http_status`、`api_code`、`fixture`、`fixture_kind` 和观察日期，并强制执行契约结构审计。

## 完成前检查

至少运行：

```powershell
cargo fmt --check
cargo check --all-features
cargo test --all-features
task contract_test
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

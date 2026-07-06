# 探针升级详细计划

> **执行要求：** 按任务分批提交。每个任务都必须保持默认测试离线、稳定、无账号副作用。涉及真实网络或账号的命令必须显式 opt-in。

**Goal:** 把 `bpi-rs` 的 Probe/contract 工作流升级为内置工具链：可抽字段、可审计脱敏、可统一测试、可批量运行、可自动提升 fixtures。

**Architecture:** 以 `tests/contracts/**` 为契约源，`src/probe` 负责契约解析、字段抽取、脱敏和 Probe 执行，`src/bin/bpi-probe.rs` 暴露命令入口，`Taskfile.yml` 提供常用任务。第一阶段优先离线审计，后续再接真实网络。

**Tech Stack:** Rust 2024、serde_json、现有 `BpiError` / `BpiResult`、Cargo tests、Taskfile、`tests/contracts/**`。

---

## 总体方案

### 1. 契约字段来源

现有 `tests/contracts/**/*.json` 已经包含两类 JSON：

- `contract.json`：接口请求、用例、fixture 路径、模型名、错误语义。
- `responses/*.json`：脱敏后的响应样例。

先从这两类 JSON 抽取字段路径，生成字段频次报告：

```text
608 $.cases[].name
205 $.request.url
22  $.data.list[].owner.mid
22  $.data.list[].owner.name
20  $.data.mid
...
```

字段路径报告用于维护内置脱敏字段表，不作为提交产物长期保存；需要时由命令重新生成。

### 2. 脱敏策略

脱敏采用三层规则，避免只靠字段名误伤公开数据：

1. 全局硬规则
   - Cookie、token、csrf、buvid3、SESSDATA、bili_jct、手机号、邮箱、IP、设备标识、签名 URL 永远脱敏。
2. 字段名规则
   - `mid`、`uid`、`uname`、`face`、`avatar`、`owner.name` 等按 API 风险等级处理。
3. JSON path 覆盖规则
   - 对已知路径单独指定动作，例如 `$.data.owner.mid`、`$.data.list[].owner.name`。

脱敏动作先支持：

```rust
enum SanitizeAction {
    ReplaceString(&'static str),
    ReplaceNumber(u64),
    Drop,
    Keep,
}
```

默认假值：

```text
账号 mid: 1000001
账号昵称: sanitized-user
头像 URL: https://example.invalid/avatar.png
token/key/cookie: <redacted>
私有文本: <redacted>
```

### 3. 风险分类联动

契约和脱敏审计使用现有六类风险：

- `public-read`
- `authenticated-read`
- `private-read`
- `mutating`
- `spending`
- `login-session`

第一阶段允许旧契约没有 `risk` 字段，但审计命令要报告缺失。后续迁移到 `schema_version = 2` 后，`risk` 变成必填。

### 4. CLI 入口

扩展 `bpi-probe` 为子命令式 CLI：

```powershell
cargo run --bin bpi-probe -- fields tests/contracts
cargo run --bin bpi-probe -- sanitize-audit tests/contracts
cargo run --bin bpi-probe -- contract-audit tests/contracts
cargo run --bin bpi-probe -- batch-run tests/contracts/video/info-read --account account.toml
cargo run --bin bpi-probe -- promote target/bpi-probe-runs/video/info-read/view/vip.response.json
```

第一阶段只实现前三个离线命令，同时先让 `contract.json` 支持新增元数据字段的兼容读取。字段先允许缺省，审计命令只给 warning；等现有契约批量补齐后再升级成 error。

### 5. Taskfile 入口

新增常用任务：

```yaml
probe_fields:
  cmds:
    - cargo run --bin bpi-probe -- fields tests/contracts

probe_sanitize_audit:
  cmds:
    - cargo run --bin bpi-probe -- sanitize-audit tests/contracts

probe_contract_audit:
  cmds:
    - cargo run --bin bpi-probe -- contract-audit tests/contracts

contract_test:
  cmds:
    - cargo test --all-features contract_runner
```

后续可把 `probe_sanitize_audit` 和 `probe_contract_audit` 加入 CI。

---

## 数据结构设计

### Task 1: 增加契约元数据字段

**Files:**
- Modify: `src/probe/endpoint_contract.rs`
- Modify: `docs/api-probe-development.md`
- Modify: `docs/api-risk-classification.md` if needed
- Modify: selected `tests/contracts/**/contract.json`

- [x] **Step 1: 在契约模型里兼容新增字段**

先在 `EndpointContract` 上增加可选字段，兼容旧契约：

```rust
pub struct EndpointContract {
    pub schema_version: Option<u32>,
    pub name: String,
    pub module: Option<String>,
    pub batch: Option<String>,
    pub endpoint: Option<String>,
    pub risk: Option<ApiRisk>,
    pub status: Option<ContractStatus>,
    pub profiles: Vec<String>,
    pub request: ContractRequest,
    pub sanitize: Option<SanitizeSpec>,
    pub provenance: Option<ContractProvenance>,
    pub cases: Vec<EndpointCase>,
}
```

- [x] **Step 2: 定义字段语义**

新增字段含义：

| 字段 | 是否必填 | 含义 |
| --- | --- | --- |
| `schema_version` | v1 可缺省，v2 必填 | 契约格式版本。旧契约缺省按 v1 读取。 |
| `module` | v2 必填 | SDK 模块名，例如 `video`、`login`、`live`。 |
| `batch` | v2 必填 | 探针批次名，对应目录层级，例如 `info-read`。 |
| `endpoint` | v2 必填 | 批次内 endpoint 名，例如 `view`。 |
| `risk` | v2 必填 | API 风险分类，使用六类风险之一。 |
| `status` | v2 必填 | 契约状态：`draft`、`probed`、`promoted`、`blocked`、`deprecated`。 |
| `profiles` | v2 必填 | 计划覆盖的 profile 列表，例如 `["anonymous", "normal", "vip"]`。 |
| `sanitize` | 可选 | 当前契约的脱敏覆盖规则。 |
| `provenance` | 可选 | 探针来源、观察时间、工具版本和脱敏说明。 |

- [x] **Step 3: 定义枚举**

```rust
pub enum ApiRisk {
    PublicRead,
    AuthenticatedRead,
    PrivateRead,
    Mutating,
    Spending,
    LoginSession,
}

pub enum ContractStatus {
    Draft,
    Probed,
    Promoted,
    Blocked,
    Deprecated,
}
```

- [x] **Step 4: 定义脱敏字段结构**

契约里的 `sanitize` 不直接替代内置字段表，只做 endpoint 覆盖：

```json
{
  "sanitize": {
    "preset": ["account", "private_profile", "signed_url"],
    "replace": {
      "$.data.owner.mid": 1000001,
      "$.data.owner.name": "sanitized-user"
    },
    "drop": [
      "$.data.email",
      "$.data.phone"
    ],
    "keep": [
      "$.data.title",
      "$.data.bvid"
    ]
  }
}
```

字段含义：

| 字段 | 含义 |
| --- | --- |
| `preset` | 引用内置脱敏规则组。 |
| `replace` | 指定 JSON path 替换为稳定假值。 |
| `drop` | 删除无测试价值的敏感字段。 |
| `keep` | 明确保留容易误判但有测试价值的公开字段。 |

- [x] **Step 5: 定义来源字段结构**

```json
{
  "provenance": {
    "source": "local_probe_output",
    "observed_at": "2026-07-06",
    "tool": "bpi-probe",
    "tool_version": "0.2.0",
    "privacy": "account fields sanitized before commit"
  }
}
```

`provenance` 不能记录账号昵称、真实 mid、Cookie、IP、设备标识或本机路径。

- [x] **Step 6: 更新探针开发指南**

在 `docs/api-probe-development.md` 增加“契约字段”章节，写清：

- 新 contract 推荐字段。
- `risk` 必须来自 `docs/api-risk-classification.md`。
- `sanitize` 只写 endpoint 覆盖，通用规则放代码内置表。
- `provenance` 只能记录非敏感来源信息。
- v1 旧契约暂时兼容，新增契约优先写 v2 字段。

- [x] **Step 7: 选 3 个代表契约试迁移**

先只迁移：

```text
tests/contracts/video/info-read/view/contract.json
tests/contracts/login/vip-info/contract.json
tests/contracts/live/account-private-read/my-medals/contract.json
```

迁移后验证 parser、审计命令和现有测试不受影响。

- [x] **Step 8: 验证**

```powershell
cargo test --all-features --lib probe::endpoint_contract
cargo test --all-features --lib probe
```

### Task 2: 增加字段路径抽取模型

**Files:**
- Create: `src/probe/field_audit.rs`
- Modify: `src/probe/mod.rs`
- Modify: `src/bin/bpi-probe.rs`

- [x] **Step 1: 定义字段路径格式**

统一使用近似 JSONPath 的稳定字符串：

```text
$.data.owner.mid
$.data.list[].owner.name
$.request.query.bvid
```

数组统一折叠成 `[]`，不记录下标。

- [x] **Step 2: 实现字段抽取**

输入一个 `serde_json::Value`，递归输出：

```rust
pub struct FieldStat {
    pub path: String,
    pub count: usize,
    pub value_kinds: BTreeSet<ValueKind>,
}
```

`ValueKind` 至少区分：

```text
object, array, string, number, bool, null
```

- [x] **Step 3: 实现目录扫描**

扫描 `tests/contracts/**/*.json`，输出按出现频次排序的字段表。

- [x] **Step 4: 添加 CLI**

```powershell
cargo run --bin bpi-probe -- fields tests/contracts
```

- [x] **Step 5: 验证**

```powershell
cargo test --all-features --lib probe::field_audit
cargo run --bin bpi-probe -- fields tests/contracts
```

### Task 3: 增加内置脱敏字段表

**Files:**
- Create: `src/probe/sanitize.rs`
- Modify: `src/probe/mod.rs`

- [x] **Step 1: 复用契约风险分类类型**

`ApiRisk` 已在 Task 1 接入契约模型；脱敏策略直接复用它，不再定义第二套风险枚举。

- [x] **Step 2: 定义脱敏动作**

```rust
pub enum SanitizeAction {
    ReplaceString(&'static str),
    ReplaceNumber(u64),
    Drop,
    Keep,
}
```

- [x] **Step 3: 建立字段名规则**

全局硬规则：

```text
sessdata, bili_jct, csrf, cookie, authorization, set-cookie,
buvid3, token, qrcode_key, phone, mobile, email, ip
```

账号字段规则：

```text
mid, uid, owner_mid, author_uid, member_id, uname, nick_name, face, avatar
```

- [x] **Step 4: 建立路径覆盖表**

先从现有高频路径覆盖：

```text
$.data.mid -> ReplaceNumber(1000001)
$.data.name -> ReplaceString("sanitized-user")
$.data.owner.mid -> ReplaceNumber(1000001)
$.data.owner.name -> ReplaceString("sanitized-user")
$.data.list[].owner.mid -> ReplaceNumber(1000001)
$.data.list[].owner.name -> ReplaceString("sanitized-user")
$.data.list[].owner.face -> ReplaceString("https://example.invalid/avatar.png")
$.data.cooperators[].mid -> ReplaceNumber(1000001)
$.data.cooperators[].nick_name -> ReplaceString("sanitized-user")
```

- [x] **Step 5: 实现脱敏函数**

```rust
pub fn sanitize_value(value: &mut serde_json::Value, policy: &SanitizePolicy);
pub fn audit_value(value: &serde_json::Value, policy: &SanitizePolicy) -> Vec<SanitizeFinding>;
```

- [x] **Step 6: 验证**

单元测试覆盖：

- Cookie/token 永远脱敏。
- `mid` 替换为 `1000001`。
- 昵称替换为 `sanitized-user`。
- 公开普通 URL 不被误删。
- 签名 URL 或带 token URL 被替换。

### Task 4: 增加脱敏审计命令

**Files:**
- Modify: `src/bin/bpi-probe.rs`
- Modify: `src/probe/sanitize.rs`
- Modify: `Taskfile.yml`

- [x] **Step 1: 实现 `sanitize-audit`**

命令：

```powershell
cargo run --bin bpi-probe -- sanitize-audit tests/contracts
```

功能：

- 扫描所有 JSON。
- 根据字段表和正则检查敏感残留。
- 输出文件、字段路径、命中原因。
- 命中高危字段时返回非 0。

- [x] **Step 2: 默认高危正则**

```text
SESSDATA=
bili_jct=
buvid3=
DedeUserID=
手机号
邮箱
IPv4
Cookie-like header
```

- [x] **Step 3: 加 Taskfile**

```yaml
probe_sanitize_audit:
  cmds:
    - cargo run --bin bpi-probe -- sanitize-audit tests/contracts
```

- [x] **Step 4: 验证**

```powershell
task probe_sanitize_audit
cargo test --all-features --lib probe::sanitize
```

### Task 5: 增加契约结构审计命令

**Files:**
- Create: `src/probe/audit.rs`
- Modify: `src/bin/bpi-probe.rs`
- Modify: `Taskfile.yml`

- [x] **Step 1: 实现 `contract-audit`**

命令：

```powershell
cargo run --bin bpi-probe -- contract-audit tests/contracts
```

检查项：

- 每个 `contract.json` 能被 `EndpointContract` 解析。
- `cases` 非空。
- `case.response.fixture` 指向的文件存在。
- `api_code` / `http_status` / `error` 语义一致。
- `fixture_kind` 在允许列表中。
- `profile` 和 `auth.profile` 不冲突。
- 新格式契约若有 `risk` 字段，必须是六类之一。
- 新格式契约若有 `status` 字段，必须是允许状态之一。
- 新增契约缺少 `module`、`batch`、`endpoint`、`risk`、`status`、`profiles` 时给 warning；迁移完成后改为 error。

- [x] **Step 2: 输出审计报告**

输出格式：

```text
ok tests/contracts/video/info-read/view/contract.json
error tests/contracts/login/nav/contract.json case vip fixture missing ...
warning ... risk missing
```

- [x] **Step 3: 加 Taskfile**

```yaml
probe_contract_audit:
  cmds:
    - cargo run --bin bpi-probe -- contract-audit tests/contracts
```

- [x] **Step 4: 验证**

```powershell
task probe_contract_audit
```

### Task 6: 增加统一契约测试 runner

**Files:**
- Create: `tests/contract_runner.rs`
- Modify: `src/probe/endpoint_contract.rs` if needed

- [x] **Step 1: 遍历契约目录**

测试扫描 `tests/contracts/**/contract.json`，逐个解析。

- [x] **Step 2: 校验 fixture 基本一致性**

对每个 case：

- fixture 文件存在。
- fixture 是合法 JSON。
- fixture 中的 `code` 与 `api_code` 一致。
- fixture 可以通过脱敏审计。

- [x] **Step 3: 建立模型解析注册表**

第一版手写匹配常用模型：

```rust
match rust_model {
    "VideoView" => parse_envelope::<VideoView>(bytes),
    "LoginVipInfo" => parse_envelope::<LoginVipInfo>(bytes),
    _ => Ok(Skip),
}
```

未注册模型第一阶段只 warning 或 skip，不阻塞全仓库。

- [ ] **Step 4: 后续减少模块重复测试**

统一 runner 稳定后，再逐步删除模块中重复的 fixture 解析测试。

- [x] **Step 5: 验证**

```powershell
cargo test --all-features contract_runner
```

---

## 网络 Probe 和自动提升

### Task 7: 批量运行 Probe

**Files:**
- Modify: `src/bin/bpi-probe.rs`
- Modify: `src/probe/run.rs`

- [x] **Step 1: 实现 `batch-run`**

命令：

```powershell
$env:BPI_PROBE = "1"
cargo run --bin bpi-probe -- batch-run tests/contracts/video/info-read --account account.toml --profiles anonymous,normal,vip
```

- [x] **Step 2: 风险门控**

规则：

- `public-read` 可运行 anonymous。
- `authenticated-read` / `private-read` 需要 `--account`。
- `mutating` 需要 `BPI_MUTATING_TEST=1`。
- `spending` 需要 `BPI_MUTATING_TEST=1` 且 `BPI_SPENDING_TEST=1`。
- `login-session` 需要单独命令或显式 `BPI_LOGIN_SESSION_TEST=1`。

- [x] **Step 3: 输出路径规范**

```text
target/bpi-probe-runs/<module>/<batch>/<endpoint>/<case>.response.json
```

### Task 8: 自动 promote

**Files:**
- Create: `src/probe/promote.rs`
- Modify: `src/bin/bpi-probe.rs`

- [x] **Step 1: 输入 raw probe 输出**

```powershell
cargo run --bin bpi-probe -- promote target/bpi-probe-runs/video/info-read/view/vip.response.json
```

- [x] **Step 2: 自动脱敏**

使用 `src/probe/sanitize.rs` 的字段表和规则处理：

- request headers/body
- response headers/body
- account identifiers
- token/key/url

- [x] **Step 3: 写入 fixture**

默认写入：

```text
tests/contracts/<module>/<batch>/<endpoint>/responses/<profile>.success.json
```

失败响应按错误语义命名：

```text
responses/<profile>.requires_login.json
responses/<profile>.risk_control.json
```

- [x] **Step 4: 更新 contract cases**

补齐或更新：

- `profile`
- `auth`
- `http_status`
- `api_code`
- `fixture`
- `fixture_kind`
- `error`
- `observed_at`

- [x] **Step 5: promote 后强制审计**

自动执行：

```powershell
cargo run --bin bpi-probe -- sanitize-audit tests/contracts/<module>/<batch>/<endpoint>
cargo run --bin bpi-probe -- contract-audit tests/contracts/<module>/<batch>/<endpoint>
```

---

## 契约格式迁移

### Task 9: 批量迁移到 `schema_version = 2`

**Files:**
- Modify: `src/probe/endpoint_contract.rs`
- Modify: `tests/contracts/**/contract.json` gradually

- [x] **Step 1: 兼容读取旧格式**

旧格式无 `schema_version` 时按 v1 解析。

- [x] **Step 2: v2 字段全部必填**

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
  "request": {},
  "sanitize": {},
  "provenance": {},
  "cases": []
}
```

- [x] **Step 3: 渐进迁移**

先迁移 3 个代表模块：

1. `video/info-read`
2. `login`
3. `live/account-private-read`

已完成全量迁移；当前所有 `tests/contracts/**/contract.json` 都声明 `schema_version = 2`。

---

## 推荐执行顺序

1. `contract metadata`：先让契约解析器兼容新增字段。
2. `field_audit`：能看到字段全集。
3. `sanitize`：内置脱敏字段表。
4. `sanitize-audit`：CI 可挡住敏感信息。
5. `contract-audit`：CI 可挡住坏契约。
6. `contract_runner`：统一测试 JSON 和 fixtures。
7. `batch-run`：批量真实 Probe。
8. `promote`：自动脱敏并提升 fixtures。
9. `schema_version = 2`：补齐风险和元数据。

## 验证基线

每个阶段至少运行：

```powershell
cargo fmt --check
cargo test --all-features --lib probe
cargo check --all-features
```

涉及 CLI 时运行：

```powershell
cargo check --all-features --bin bpi-probe
```

涉及统一测试时运行：

```powershell
cargo test --all-features contract_runner
```

最终收口运行：

```powershell
task ci
task probe_sanitize_audit
task probe_contract_audit
```

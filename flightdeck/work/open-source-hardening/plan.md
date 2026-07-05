# Open Source Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 建立 `bpi-rs` 的开源维护基线，让外部贡献、CI 验证、账号隐私和变更类 API 使用都有明确规则。

**Architecture:** 这次升级不改 SDK 核心请求模型，只补齐维护文档、CI 门禁、账号示例和脱敏规则。风险控制集中在文档、示例门禁、测试 fixtures 和发布前验证命令。

**Tech Stack:** Rust 2024、Cargo、GitHub Actions、Taskfile、Markdown、PowerShell/rg 辅助审计。

---

### Task 1: 账号示例与 MSRV

**Files:**
- Modify: `account.example.toml`
- Modify: `Cargo.toml`

- [x] **Step 1: 把账号示例改为当前唯一支持的结构化 profile**

```toml
[normal]
bili_jct = "normal_bili_jct"
dede_user_id = "normal_dede_user_id"
dede_user_id_ckmd5 = "normal_dede_user_id_ckmd5"
sessdata = "normal_sessdata"
buvid3 = "normal_buvid3"

[vip]
bili_jct = "vip_bili_jct"
dede_user_id = "vip_dede_user_id"
dede_user_id_ckmd5 = "vip_dede_user_id_ckmd5"
sessdata = "vip_sessdata"
buvid3 = "vip_buvid3"
```

- [x] **Step 2: 在 `Cargo.toml` 声明 Rust 2024 对应 MSRV**

```toml
rust-version = "1.85"
```

- [x] **Step 3: 验证**

Run: `cargo check --all-features`

### Task 2: CI 和协作模板

**Files:**
- Create: `.github/workflows/ci.yml`
- Create: `.github/pull_request_template.md`
- Create: `.github/ISSUE_TEMPLATE/bug_report.md`
- Create: `.github/ISSUE_TEMPLATE/api_request.md`

- [x] **Step 1: CI 覆盖格式、clippy、全 feature 编译、测试、示例和无默认 feature 编译**

Run in CI:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features
cargo test --doc
cargo check --examples --all-features
cargo check --no-default-features --lib
```

- [x] **Step 2: PR 模板要求标明 API 风险类型、测试命令和账号数据处理**

- [x] **Step 3: Issue 模板区分 bug 与新 API 请求**

### Task 3: 维护文档

**Files:**
- Create: `CONTRIBUTING.md`
- Create: `SECURITY.md`
- Create: `CHANGELOG.md`
- Modify: `README.md`

- [x] **Step 1: 贡献指南写清默认验证、live/mutating 门禁、PR 接收流程和提交信息规范**

- [x] **Step 2: 安全政策写清不要提交 Cookie、账号响应、原始 Probe 输出和风控绕过内容**

- [x] **Step 3: 变更日志记录 0.2 的破坏性迁移和安全门禁**

- [x] **Step 4: README 链接贡献、安全、探针指南和变更日志**

### Task 4: 账号可识别信息脱敏

**Files:**
- Modify: `src/**`
- Modify: `tests/contracts/**`
- Modify: `flightdeck/work/**`

- [x] **Step 1: 统一将公开测试里的个人 mid 替换为稳定假值 `1000001`**

Run:

```powershell
rg -l "旧真实 mid|旧下划线 mid" src tests flightdeck | % { (Get-Content -Raw $_).Replace("旧真实 mid", "1000001").Replace("旧下划线 mid", "1_000_001") | Set-Content $_ -Encoding utf8NoBOM }
```

- [x] **Step 2: 统一将个人昵称替换为 `sanitized-user`**

Run:

```powershell
rg -l "旧真实昵称" src tests flightdeck | % { (Get-Content -Raw $_).Replace("旧真实昵称", "sanitized-user") | Set-Content $_ -Encoding utf8NoBOM }
```

- [x] **Step 3: 验证无残留**

Run:

```powershell
rg -n "旧真实 mid|旧下划线 mid|旧真实昵称" src tests flightdeck --glob "!target/**"
```

### Task 5: 最终验证

**Files:**
- Verify only

- [x] **Step 1: 格式检查**

Run: `cargo fmt --check`

- [x] **Step 2: 全 feature 编译**

Run: `cargo check --all-features`

- [x] **Step 3: 全 feature 测试**

Run: `cargo test --all-features`

- [x] **Step 4: 示例编译**

Run: `cargo check --examples --all-features`

### Task 6: 流程收口

**Files:**
- Create: `docs/api-risk-classification.md`
- Create: `docs/release-checklist.md`
- Modify: `Taskfile.yml`
- Modify: `README.md`
- Modify: `src/request.rs`

- [x] **Step 1: 增加 API 风险分类文档**

记录 `public-read`、`authenticated-read`、`private-read`、`mutating`、`spending`、`login-session` 的测试和示例门禁要求。

- [x] **Step 2: 增加发布检查清单**

记录状态检查、默认验证、打包验证、隐私扫描、文档确认和发布命令。

- [x] **Step 3: 让 Taskfile 暴露 CI 同口径命令**

`task ci` 执行与 GitHub Actions 相同的默认验证；`task package_check` 执行打包验证。

- [x] **Step 4: 清理 no-default warning**

`send_bpi_envelope` 和 `decode_bpi_envelope_response` 只在 `manga` feature 下使用，因此加 `#[cfg(feature = "manga")]`。

- [x] **Step 5: 验证**

Run:

```powershell
task ci
task package_check
```

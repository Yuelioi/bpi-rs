# 开源规范化 Topic

## 目标

把 `bpi-rs` 从“能本地协作的 SDK”推进到“外部贡献者可理解、CI 可验证、发布前风险可控”的开源库状态。

## 当前风险

- `account.example.toml` 与当前 `[vip]` / `[normal]` 账号配置格式不一致，会误导新用户。
- 仓库缺少 CI、贡献指南、安全政策、PR/Issue 模板和变更日志。
- 变更类 API 已经存在，但公开文档对“只读 / 变更 / 消费资产 / 风控敏感”的分类还不够显式。
- 契约 fixtures 和测试常量里存在账号可识别信息，虽然不是密钥，但不适合长期保留在公开仓库。
- `Cargo.toml` 未声明 `rust-version`，edition 2024 用户在旧工具链上会得到较差错误体验。

## 规范方向

1. 默认命令必须离线、只读、稳定。
2. 任何会修改账号状态、消费资产、发消息、开播/关播、发布内容或触发风控的入口必须显式 opt-in。
3. 契约和 fixtures 只能保留验证所需字段，账号相关信息必须脱敏。
4. 外部贡献者应能从 README、CONTRIBUTING、SECURITY、探针指南理解开发流程。
5. PR 接收应保留贡献者上下文，并通过 CI 与 review 修正风格，而不是手动复制改动。

## 跟踪文件

- 升级方案：`flightdeck/work/open-source-hardening/plan.md`
- 探针开发规范：`docs/api-probe-development.md`
- API 风险分类：`docs/api-risk-classification.md`
- 发布检查清单：`docs/release-checklist.md`
- 项目入口说明：`README.md`

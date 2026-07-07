# Changelog

## 0.2.2

- 修复 `0.2.1` 发布流程中新版 clippy 报出的私有模块响应模型 `dead_code` 和 `ResponseDecoding` 手写 `Default` 问题。

## 0.2.1

- 修复历史记录响应中 `total: -1` 导致反序列化失败的问题。
- 升级契约探针工具链，新增契约审计、字段统计、脱敏审计、批量只读 Probe、fixture promote 和 API 索引生成。
- 将已提交接口契约迁移到 v2 元数据，补齐模块、批次、endpoint、风险分类、profiles、脱敏和来源信息。
- 新增 `docs/api-index.md`，自动汇总 API、函数说明、风险分类、profiles、URL、契约路径和 Rust 模型，方便维护者和 AI 使用。
- 新增 `docs/development.md`，整理本地验证、Probe、账号配置和风险门控流程。
- 修复 `probe::model` 在关闭默认 feature 或只启用部分 feature 时引用被裁剪模块导致的编译失败。

## 0.2.0

- 使用模块客户端作为主 API 形态，例如 `client.video().view(...)`、`client.login().nav()`。
- 迁移后的模块 API 默认返回业务 payload：`BpiResult<T>`。
- 引入契约 fixtures，使用 `tests/contracts/**` 验证 endpoint、请求参数和响应模型。
- 账号加载改为显式 `[vip]` / `[normal]` profile，不再兼容 `*_vip` / `*_normal` 旧格式。
- 账号必须保留 `buvid3`；缺少 `buvid3` 可能触发 Bilibili 风控。
- 变更类示例和 live 测试必须显式 opt-in，使用 `BPI_MUTATING_TEST=1` 等环境变量门控。
- 新增探针开发指南，要求 Probe 输出和账号相关响应先脱敏再进入契约。

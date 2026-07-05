# Changelog

## 0.2.0

- 使用模块客户端作为主 API 形态，例如 `client.video().view(...)`、`client.login().nav()`。
- 迁移后的模块 API 默认返回业务 payload：`BpiResult<T>`。
- 引入契约 fixtures，使用 `tests/contracts/**` 验证 endpoint、请求参数和响应模型。
- 账号加载改为显式 `[vip]` / `[normal]` profile，不再兼容 `*_vip` / `*_normal` 旧格式。
- 账号必须保留 `buvid3`；缺少 `buvid3` 可能触发 Bilibili 风控。
- 变更类示例和 live 测试必须显式 opt-in，使用 `BPI_MUTATING_TEST=1` 等环境变量门控。
- 新增探针开发指南，要求 Probe 输出和账号相关响应先脱敏再进入契约。

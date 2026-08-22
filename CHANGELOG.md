# Changelog

## 0.3.0

- **破坏性变更**：`DashFlac.audio` 改为 `Option<DashStream>`，与上游 FLAC 单对象/`null` 形态一致；`BangumiDetailResult.total` 与 `BangumiPendant.pid` 改为有符号整数，以保留负数哨兵。
- 修复 DASH/MP4 备用地址为 `null`、番剧 `durl`/`durls` 混淆，以及非零 API code 被错误 payload 解码遮蔽的问题。
- 兼容 2009～2026 动态中的数字/空动态 ID，以及跨年代番剧、电影、付费、未开播、国创等精简或可缺字段。
- 新增代表性 live 响应矩阵与离线回归：动态 49 条、番剧 37 个 season、33 个 episode；修复后相关矩阵 `ResponseDecode` 均归零。
- 修复只启用 `video` feature 时的测试导入、合法空合集断言和 Windows 下只读 Probe 任务的环境变量配置。

## 0.2.4

- 新增可恢复的响应模型解码错误：领域方法因上游 schema 漂移而失败时，可通过 `BpiError::response_body()` 使用临时模型解析同一次响应；默认错误输出、序列化和 tracing 保持脱敏。
- 修复视频播放信息中 `last_play_time` 和 `last_play_cid` 返回负数哨兵值时的反序列化失败，并补齐视频详情封面字段。
- 将响应字段变化导致的解析失败恢复单列为独立文档分类。

## 0.2.3

- 修复创作中心封面上传路径中新版 clippy 报出的 `unused_parens` 警告，确保 `-D warnings` 的 CI 检查通过。
- 新增 tag 驱动的 GitHub Actions 发布流程：推送 `v*` tag 后先跑完整 CI、打包和 dry-run，再自动发布到 crates.io。

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

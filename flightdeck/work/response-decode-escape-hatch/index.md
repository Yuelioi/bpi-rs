# Index — 响应解码逃生口

## State

正在设计一个面向下游开发者的临时兼容机制：当 Bilibili 响应字段类型漂移、bpi-rs 内置模型暂未修复时，调用方仍能复用库负责的 endpoint 参数、WBI 签名、cookie 和请求头，并用自定义结构解析同一次原始响应。

当前仓库已经支持 `BpiClient::get/post` 配合公开的 `BilibiliRequest::send_request` 或 `send_bpi_payload::<T>` 发起自定义请求；但调用方必须重新拼请求，且 `get_wbi_sign2` 是 crate 内部接口。领域方法发生 `BpiError::Decode` 时只保留 `serde_json::Error`，原始响应字节会丢失。

## Next

- 请用户确认 `design.md` 推荐的“可恢复响应解码错误”方向。
- 确认后先写错误恢复行为测试，再实现 raw response 访问器和文档示例。
- 评估新增错误变体对下游穷举匹配及 `Serialize` 行为的兼容性。

## Read now

- design.md
- flightdeck/knowledge/rust/sdk-quality.md

## Read if

- docs/migration-0.2.md — 更新迁移和自定义请求文档时。
- README.md — 更新公开用法示例时。

## Progress

Done:

- 检查提交 `a6dca49`，确认 `last_play_time` 和 `last_play_cid` 因 `-1000` 哨兵值从 `u64` 改为 `i64`。
- 确认请求执行、transport、envelope 解码和领域方法的当前边界。
- 确认普通 endpoint 已有手动自定义请求绕过方式，WBI endpoint 无法完整复用该方式。
- 形成可恢复响应解码错误的推荐方案。

Current:

- 等待用户确认公开 API 方向。

Verified:

- 只读检查 `src/request.rs`、`src/transport/response.rs`、`src/err/error.rs`、`src/video/client.rs` 和 `README.md`。

## Open questions

- 公开最小接口是只提供 `response_body(&self) -> Option<&[u8]>`，还是同时提供泛型便捷解析方法？
- 是否需要在错误中同时保留 HTTP/endpoint 元数据，便于 issue 报告但避免泄露敏感响应？

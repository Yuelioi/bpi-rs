# 可恢复响应解码检查清单
SUMMARY: Always preserve a safely redacted recovery path when a typed bpi-rs HTTP response can fail because the upstream schema drifted.
READ WHEN: before changing any transport response decoding, response-model error, or raw-response fallback behavior.
RECHECK WHEN: BpiError, TransportResponse, serde_json, bytes, or the response logging policy changes.

---

## Boundary

- 在 `TransportResponse::decode_api_envelope` 区分 HTTP 响应模型解码错误和普通 JSON 转换错误。
- HTTP 响应模型解码失败使用 `BpiError::ResponseDecode`；其他 `serde_json::Error` 继续使用 `BpiError::Decode`。
- 通过 `BpiError::response_body()` 显式借用同一次响应字节，让下游可用临时 `ApiEnvelope<T>` 恢复，不重新发送请求，也不复制 endpoint 的 WBI 签名、认证或参数构造。
- 使用 `bytes::Bytes` 保留响应，共享底层存储，避免复制完整 body。

## Redaction

- `Display`、`Debug`、`Serialize` 和 tracing 默认不得输出原始 body。
- `serde_json::Error` 文本可能包含触发类型错误的字段值，因此默认输出只报告 `classify()`、行、列和响应长度。
- 底层 `serde_json::Error` 通过标准 error source 链保留，供调用方显式诊断。

## Verification

- 类型漂移时保留完全相同的响应字节，并可用临时模型成功解析。
- 普通 JSON 错误的 `response_body()` 返回 `None`。
- 非零 API code 仍返回 `BpiError::Api`。
- 用真实会出现在 `serde_json::Error` 文本中的敏感标记验证所有默认输出均已脱敏。

# 设计 — 响应解码逃生口

## 问题

Bilibili 的响应 schema 会漂移。提交 `a6dca49` 展示了一个典型案例：`last_play_time` 和 `last_play_cid` 会返回 `-1000`，导致声明为 `u64` 的模型反序列化失败。维护者修复并发布新版本之前，下游开发者需要一种不绕开 bpi-rs 请求能力的临时恢复方式。

## 当前可用绕过方式

公开的 `BpiClient::get/post` 会复用客户端默认请求头和 cookie；公开的 `BilibiliRequest` 支持：

- `send_request`：取得原始响应字节；
- `send_bpi_payload::<T>`：用调用方自定义类型解析 payload；
- `send_bpi_optional_payload::<T>`：解析可空 payload。

这适合普通 endpoint，但调用方要复制 URL 和参数构造。WBI endpoint 还需要 crate 内部的 `get_wbi_sign2`，因此不能完整绕过。已封装的领域方法也不能选择自定义响应类型。

## 推荐方案

让 transport 响应解码失败成为可恢复错误：错误继续保留原始 `serde_json::Error`，同时持有本次 `TransportResponse` 或至少原始 body，并提供不会把 body 写入 `Display`、日志或序列化结果的访问器。

最小公开接口：

```rust
impl BpiError {
    pub fn response_body(&self) -> Option<&[u8]>;
}
```

调用方可以在领域方法失败后用临时结构恢复：

```rust
let result = client.video().play_url(params).await;

let data = match result {
    Ok(data) => data,
    Err(err) => {
        let body = err.response_body().ok_or(err)?;
        ApiEnvelope::<TemporaryPlayUrl>::from_slice(body)?.into_payload()?
    }
};
```

实际示例需要让两个 match 分支返回同一调用方类型，或在 fallback 分支立即消费临时类型；文档不应暗示内置模型和临时模型天然相同。

可以再提供一个便捷方法，但不应替代原始字节访问：

```rust
pub fn decode_response_as<T>(&self) -> Option<BpiResult<ApiEnvelope<T>>>
where
    T: DeserializeOwned;
```

## 为什么不首选每个 endpoint 的泛型变体

给所有领域方法增加 `view_as<T>`、`play_url_as<T>` 等变体会扩大数百个 endpoint 的公开 API，并重复请求构造代码。把恢复能力放在统一的 transport/decode 边界，只改变失败路径，正常路径仍保持强类型返回。

## 错误模型选择

不建议直接给现有 `BpiError::Decode { source }` 添加字段，因为下游使用字段模式匹配时会直接编译失败。新增专门的响应解码错误变体表达更准确，但仍会影响对 `BpiError` 做无通配分支的穷举匹配；实现和迁移文档需要明确这是 0.2 阶段的 API 变化。

新变体的 `Display`、`Debug` 和 `Serialize` 都不得暴露原始 body。若派生 `Debug` 无法满足这一点，应把 body 放入自定义包装类型并手写脱敏 `Debug`，或者只公开不可被默认格式化输出的恢复载体。

## 测试边界

- 内置 payload 类型不匹配时，错误保留完全相同的响应字节。
- 调用方可从错误中用自定义 payload 类型成功恢复。
- 非响应来源的 `serde_json::Error` 仍是普通 `Decode`，不伪造 response body。
- API code 非零时仍返回 `Api`，不错误归类为可恢复模型漂移。
- `Display`、`Debug` 和 `Serialize` 输出不包含响应 body 中的敏感标记。

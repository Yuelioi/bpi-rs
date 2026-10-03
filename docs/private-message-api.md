# 私信 API

`client.message()` 提供一页一请求的私信能力：

| 方法 | 参数 | 风险 | 契约 |
| --- | --- | --- | --- |
| `sessions` | `MessageSessionsParams` | `private-read` | `message.sessions` |
| `session_messages` | `MessageSessionMessagesParams` | `private-read` | `message.session_messages` |
| `send` | `MessageSendParams` | `mutating` | `message.private.send` |

会话读取默认 `session_type=4`、固定 `sort_rule=2`，包含未关注用户；`with_session_type` 支持 `UserAndSystem=1`、`Unfollowed=2`、`All=4`；消息记录固定用户私信 `session_type=1`。默认每页 100 条，支持 1–100 条。接口不会标记已读，不会自动翻页、排序或过滤系统消息。调用方需要按业务处理 `session_type`、`system_msg_type`、`msg_status` 和 `msg_source`，不要把所有消息当成可自动回复的文本。

会话 `begin_ts`、`end_ts`、`session_ts` 使用**微秒**；消息 `timestamp` 使用**秒**。获取更早会话使用上一页最小 `session_ts`，获取更早消息使用最小 `msg_seqno` 作为 `end_seqno`。增量读取分别使用 `begin_ts`、`begin_seqno`。依据 `has_more` 与游标是否前进决定继续或退出，并处理重复页。空会话或空消息页实测返回 `null`，对应 `Option<Vec<_>>`。

`content` 保留嵌套 JSON 字符串，`msg_type` 保留原始数值，允许未知类型。历史消息可能缺少 `new_face_version`；`at_uids` 可为 `null`。会话模型只声明稳定私信字段，特殊系统卡片的 `account_info` 和未观察到有效值的 `user_label` 未建模。

发送沿用既有文本/图片入口，增加 `MessageSendParams::text`。文本拒绝空白，嵌套 JSON UTF-8 编码不超过 2000 字节；正文中的换行和首尾空白保留。需要已配置账号、CSRF 与 WBI，表单中 `csrf`、`csrf_token` 相同，签名使用发送者、接收者和同一 UUID。SDK 不重试写操作。调用方应核对非零 `msg_key`；网络结果不确定时先回读消息，避免重复发送。

本批证据：匿名读取返回 `-101`；normal profile 凭据过期，也返回 `-101`；vip profile 成功，包括空页与游标分页。脱敏 fixture 不含真实 UID、正文、设备、时间或序列号。发送曾由消费项目 Go 适配层实测成功并回读，完整发送响应未留存；发送 fixture 明确标记 `synthetic_verified_shape`，只声明已观察到的非零 `msg_key`，不声称完整响应模型已 live 验证。

默认测试离线。单独的发送 live 测试同时要求 `--ignored`、`BPI_MUTATING_TEST=1`、`BPI_MESSAGE_RECEIVER_ID`、`BPI_MESSAGE_TEXT`；账号由 `BPI_ACCOUNT_FILE`（默认 `account.toml`）与 `BPI_ACCOUNT_PROFILE`（默认 `vip`）指定。不要对整个旧 live 测试集合运行 `--ignored`。消费项目本次实测不授权未来自动发送。

分页限制：本账号实测 `All=4` 忽略 `end_ts`，返回相同首条，即使减一微秒也不推进；不能承诺全部会话历史分页。`UserAndSystem=1` 与 `Unfollowed=2` 的两页探针均推进，可分别读取后由调用方合并、去重；默认增量查询 `begin_ts` 的未来空页已验证。

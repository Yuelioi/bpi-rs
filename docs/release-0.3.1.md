# 0.3.1 跨 SDK 对齐

此版本保留 Rust `v0.3.0` 提交 `94bcf43e46d6e11b55ec4d36d4848692cecd4213` 的原始来源基准，追加已审核的能力增量；各 SDK 不重新解释历史 inventory。

| 0.3.0 后变化 | Rust | Go | Python |
| --- | --- | --- | --- |
| 课程优惠券可空、无折扣描述 | `CourseInfo` / `CoursePayment` | `cheese.Course` | 生成课程模型 |
| 课程省略画质字段、DRM/HLS/试看元数据 | `CourseVideoStreamData` | `cheese.PlayURL` | `CourseVideoStreamData` |
| 公共播放流缺省画质字段 | `VideoStreamData` 默认值 | JSON 缺省零值 | 视频/番剧/课程模型默认值 |
| 会话列表、分类与时间游标 | `message().sessions` | `Message().Sessions` | `message.sessions` |
| 用户私信记录与序列号游标 | `message().session_messages` | `Message().SessionMessages` | `message.session_messages` |
| 文本/既有图片发送、UTF-8 JSON 长度校验、无重试 | `message().send` | `Message().Send` | `message.send` |
| 最近消息可空或缺失、空页、历史缺省字段 | 可选模型和回归 | 可选模型和回归 | 可选模型和回归 |
| 凭据及私信身份/设备/正文脱敏 | Debug 和请求日志 | Account 格式化和请求日志 | Account repr 和请求日志 |

课程回归样例：`no-coupon.sanitized.json`、`drm.anonymous.sanitized.json`、`preview.anonymous.sanitized.json`；三端来自同一脱敏响应。私信契约/样例从 Rust 增量同步，来源锁定 `24aa6b36712184422b852e50b2cd59c6e5df8de9`，其中发送响应是已标记的合成形态，不能据此声称完整发送响应已 live 验证。

分页保留已知限制：`All=4` 可能忽略 `end_ts`，历史分页应分别查询分类 1/2 并检查游标推进。SDK 不自动标记已读、不自动翻页，不重试发送。

Rust 特有的依赖升级和最低 Rust 1.88 不迁移为 Go/Python 依赖版本。语言特有工具、账号配置形式和公开方法命名沿用各 SDK 惯例。

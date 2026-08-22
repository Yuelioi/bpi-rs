# 全部非写网络函数接口执行基线报告

> 本报告只证明接口执行覆盖，不证明各模块的代表性数据形态覆盖。用户进一步澄清后，本 Work 已重新打开；最终结论以新的代表性样本矩阵报告为准。

测试日期：2026-08-22

## 结论

bpi-rs 对外公开、会向 Bilibili 发出网络请求、且不改变远端状态的函数共 204 个。本轮严格执行覆盖为 204/204。

- 199 个函数具有 promoted 只读契约。
- 其中 183 个完成完整契约身份矩阵、live 响应采集和模块强类型复验。
- 另外 16 个函数原先被陈旧 expectation 或非 JSON runner 提前拦截，本轮改为直接调用公开 client 方法补齐。
- 5 个无独立契约或便利入口函数全部直接调用：`activity.list_default`、`bangumi.detail`、`cheese.info`、`live_fetch_web_up_stream_addr`、`video.coin_status`。
- 21 个缺口函数共执行 69 次 exact-client 调用；Bangumi/Cheese 通用入口分别覆盖 season 与 episode 参数。

204 个函数中，202 个至少取得一个成功强类型响应。剩余两个函数已实际执行，但当前没有成功响应：

- `DynamicClient::pics`：契约样本及追加 3 个当前公开图文动态样本，匿名均返回 `-101`，登录档返回 `-403` 或 `-404`。
- `LoginClient::daily_reward`：匿名、普通、会员均被 HTTP 412 风控拦截。

因此“执行覆盖”为 204/204，“至少一个成功载荷覆盖”为 202/204。

## 安全边界

- 排除 mutating、spending、login-session 和所有点赞、投币、收藏、关注、评论、弹幕、账号、直播状态、历史进度等写操作。
- 位于 action 模块但只读取状态的 `video.coin_status` 已纳入。
- 匿名、普通登录非有效会员、有效大会员三档均使用；不适用身份记录稳定登录/权限错误。
- 未输出或提交 Cookie、MID、私有 payload、推流地址、播放 URL 或签名参数。

## 本轮新发现与修复

匿名 `UserClient::uploaded_videos` 实时返回 `code=-352`，同时携带不符合成功模型的不完整 `data`。旧 transport 会先按 `UserUploadedVideos` 解码 data，错误返回：

```text
ResponseDecode at data: missing field `list`
```

修复后 `TransportResponse::decode_api_envelope` 先用 `IgnoredAny` 读取 envelope 元信息：

- `code != 0`：跳过成功 payload 模型，返回正确 `BpiError::Api`。
- `code == 0`：继续严格按 `T` 解码；不兼容 payload 仍返回保留响应体的 `ResponseDecode`。

离线回归测试先红后绿；相同 69 次 exact-client live 矩阵复跑后，模型解码失败由 1 降为 0，匿名 uploaded videos 正确分类为 `Api(-352)`。

## 缺口函数 live 结果摘要

- 69 次调用：56 次成功、9 次稳定 API 错误、3 次 HTTP 412、1 次匿名权限错误、0 次模型解码失败。
- `article.cards`、`dynamic.forwards`、`user.uploaded_videos` 仅匿名触发 `-352`，登录档成功。
- `electric.recharge_list`、`login.notice`、`video.coin_status` 匿名要求登录，normal/vip 成功。
- `live_fetch_web_up_stream_addr` 匿名无权限，normal/vip 成功；响应中的推流凭据立即丢弃。
- 六个弹幕 bytes/protobuf 函数三档均成功返回。
- `live.danmu_info` 与 `video_ranking.popular_series_one` 三档均成功。

## 验证

- `task pre_commit`：通过。
  - library tests：1005 passed、0 failed、198 ignored。
  - contract runner：1 passed。
  - strict Clippy、all-features check、doc tests、examples、no-default build、Git whitespace：通过。
- `task check_feature_matrix`：全部通过。
- transport 回归模块：7 passed、0 failed。
- exact-client 补测修复后：69 calls、0 response decode failures。

## 残余风险

- `task probe_read_only` 仍会因旧契约 expectation 漂移或非 JSON 响应缺少 `code` 而提前停止；本轮通过逐契约续跑与 exact-client 调用完成函数覆盖，但 runner 本身还需要独立改造成“失败后继续”和 bytes-aware expectation。
- `dynamic.pics` 和 `login.daily_reward` 没有在当前账号/IP/样本下取得成功载荷，无法实时验证它们的成功 payload 模型；已有离线 fixture 模型测试通过。
- 原始 live 响应与临时 harness 仅保留在 Git 忽略的 `target/` 中供本机复核。

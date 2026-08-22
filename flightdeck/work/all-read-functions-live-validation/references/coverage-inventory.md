# 非写网络函数覆盖清单

## 分母

- 源码中共有 320 个 `pub async fn`。
- `src/probe/` 和 `src/transport/` 的 3 个基础设施函数不属于对外 Bilibili endpoint。
- 按契约风险、请求方法语义和源码实现排除 mutating、spending、login-session，以及所有会改变点赞、投币、收藏、关注、评论、弹幕、账号、直播或历史状态的函数。
- 199 个 promoted 只读契约各自对应一个公开 endpoint 函数。
- 另有 5 个允许测试、但没有独立 promoted 只读契约或属于便利入口的公开函数。

当前验收分母：204 个公开非写网络函数。

## 五个契约外函数

| 模块 | 函数 | 性质 | 需要的 live 证据 |
| --- | --- | --- | --- |
| activity | `ActivityClient::list_default` | `list` 的默认参数便利入口 | 实际调用成功并解析 `ActivityListData` |
| bangumi | `BangumiClient::detail` | 通用 season/episode 详情入口 | season 与 episode 参数至少各调用一次 |
| cheese | `CheeseClient::info` | 通用 season/episode 课程详情入口 | season 与 episode 参数至少各调用一次 |
| live | `LiveClient::live_fetch_web_up_stream_addr` | 私有只读推流地址 | normal/vip 调用，响应或稳定权限错误均脱敏记录 |
| video | `VideoClient::coin_status` | 位于 action 模块的只读状态接口 | normal/vip 调用并解析投币状态 |

`MiscClient::bili_ticket_string` 是 `login-session` 风险函数 `bili_ticket` 的便利入口，不在本 Work 的安全分母内。

## 已知契约映射盲区

API 索引当前有 19 个只读契约没有自动关联源码函数，其中 17 个可通过函数内的 operation label 直接映射；另外两个为原始 XML 调用：

- Bangumi：review user、season detail（season/episode）、section、playurl。
- Cheese：season detail（season/episode）、playurl。
- Creative center：season aid、season section。
- Danmaku：history XML、mobile/web protobuf segment、comment XML、list.so。
- Misc：b23 short link。
- Video：pagelist。

这些是 catalog 名称匹配不足，不代表函数不存在。

## 当前 live 状态

- 199 个只读契约中，183 个已完整执行通过。
- 原先提前停止的 16 个函数已绕过陈旧 expectation/非 JSON runner，改用公开 client 方法逐一调用三档身份。
- 5 个契约外函数均已使用公开 client 方法实际调用；Bangumi/Cheese 通用入口另分别覆盖 season 与 episode 两种参数。
- 缺口 harness 共执行 69 次调用：修复后 56 次成功、9 次稳定 API 错误、3 次 HTTP 412 风控、1 次匿名权限错误，0 次模型解码失败。

当前严格执行覆盖：204/204。

其中 202 个函数至少取得一个成功强类型响应。两个函数已执行但当前没有成功响应：

- `DynamicClient::pics`：追加 3 个当前公开图文动态样本，匿名均为 `-101`，登录档为 `-403` 或 `-404`。
- `LoginClient::daily_reward`：三档均被当前环境 HTTP 412 风控拦截。

在补测中发现并修复 transport 顺序错误：非零 API code 携带不兼容 `data` 时，旧实现会先解码 payload 并错误返回 `ResponseDecode`；现在先读取 envelope code，仅成功响应才严格解码 payload。匿名 `UserClient::uploaded_videos` 已从 `ResponseDecode(data missing field list)` 恢复为正确的 `BpiError::Api(-352)`。

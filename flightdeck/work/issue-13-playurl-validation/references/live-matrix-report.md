# Issue #13 三身份只读验证报告

测试日期：2026-08-22

## 安全与身份

- 全程只调用 public-read、authenticated-read、private-read 接口；未调用点赞、投币、收藏、关注、评论、发弹幕或任何其他远端写操作。
- 匿名档、普通登录非有效会员档（`vip_status=0`）和有效大会员档（`vip_type=2`、`vip_status=1`）均独立验证。
- 报告和源码不包含 Cookie、账号 ID、播放 URL 或请求签名；原始临时响应仅位于 Git 忽略的 `target/`。

## UGC 播放流矩阵

- 21 个不同视频，分为旧低质量、中期标准、新内容、高画质视频、高音质、Dolby、长多 P 共 7 组，每组 3 个样本。
- 长多 P 组选取首、中、尾 P，实际覆盖 8P、203P、264P 内容；总计 27 个页面。
- 每页执行匿名、普通、会员三档以及 MP4 低清、DASH 低清、基础 DASH、增强 DASH、最高质量 5 种请求，共 405 次。
- 内容覆盖 640×480 到 7680×4320，AVC、HEVC/HVC、AV1、Dolby Vision 视频 codec，以及 AAC 主音轨、FLAC、Dolby 音轨。

修复前首次矩阵为 402/405 成功，捕获 3 个真实模型解码失败：

- 2 次 `data.dash.video[0].backupUrl=null`，涉及两个不同视频。
- 1 次 `data.durl[0].backup_url=null`。

该形态具有低频非确定性：针对两个样本追加 5 轮、150 次复跑未再次出现。最小回归测试确认旧模型稳定报 `invalid type: null, expected a sequence`。将缺失或 `null` 的备用地址归一化为空 Vec 后，相同 405 次矩阵为 405/405 成功，无 API、风控或模型解码错误。

增强音轨观察：

- 36 个成功响应包含 FLAC 描述；12 个会员响应真实下发单对象 `flac.audio`，匿名和普通档对应响应为 `null`。
- 9 个会员响应真实下发单条 Dolby 音轨；匿名和普通档未下发。
- Issue #13 指定视频在匿名/普通档为 `flac.audio=null`，会员档 4 种 DASH 请求均为单对象音轨。

## 其他内容播放流

番剧、课程、音频 App 播放地址、音频 Web 播放地址各执行匿名/普通/会员三档，共 12 次 live probe；全部响应均通过对应 Rust 模型解析。

## 全库只读契约审计

- 206 个契约中筛出 199 个 promoted 只读契约，排除 mutating、spending、login-session。
- 按契约容错续跑后，183 个契约通过、16 个失败；成功契约写入 1693 份响应，失败契约另留下 4 份已完成 case，共 1697 份 live 响应。
- 将 live 响应镜像到模块测试约定路径后，全库强类型模型测试通过：1004 passed、0 failed、198 ignored；contract runner 另有 1 个通过。

16 个 probe gate 失败均不是 Rust 反序列化错误：

- 6 个弹幕 XML/protobuf/bytes 契约被通用 JSON expectation runner 误判为缺少 `code`。
- 7 个契约的实时 API code 与已提交预期漂移：`article.cards`、`dynamic.forwards`、`dynamic.pics`、`live.danmu_info`、`login.login_notice`、`user.bangumi_follow_list`、`video_ranking.popular_series_one`。
- 3 个契约当前返回无 `code` 的响应：`electric.recharge_list`、`login.daily_reward`、`user.uploaded_videos`，需要单独审计 HTTP/非 JSON 处理。

## 已实施改动

- `DashFlac.audio` 改为 `Option<DashStream>` 并建模可选 `display`。
- `DashStream.backup_url` 与 `DurlInfo.backup_url` 接受缺失/`null` 并归一化为空 Vec，保持公开字段类型不变。
- 新增 FLAC 与 nullable backup URL 回归测试。
- 修正 `video` 单 feature 单元测试所需的 `ApiEnvelope` 导入。
- live 合集模型测试不再假定线上列表永久非空，改为验证稳定分页结构。
- `task probe_read_only` 使用 Task 的 `env` 配置，不再依赖 PowerShell 专用赋值语法。

## 最终验证

- `task pre_commit`：通过。
- `task check_feature_matrix`：通过。
- `cargo test --no-default-features --features video video::videostream_url::tests`：6 passed、0 failed、2 ignored。
- 修复后 UGC live 矩阵：405/405 成功。

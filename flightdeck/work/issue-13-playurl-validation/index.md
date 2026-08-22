# Issue #13 播放流响应矩阵验证

Status: Finished

Goal: 用匿名、普通登录、会员三种身份，对有代表性的 Bilibili 视频播放流响应做只读矩阵测试，验证 `DashInfo` 及相关公开模型不会因 FLAC、Dolby 或主轨字段形态变化而整体解析失败，并将稳定、脱敏的代表性形态固化为回归测试。

Current: 三身份、多年代、多质量播放流矩阵及全库只读契约审计已完成。修复了 FLAC 和备用 URL 两类真实响应漂移，live 模型复验与全部本地门禁通过；16 个剩余 probe gate 失败均已分类为契约预期或非 JSON runner 问题。

Next: None

## Progress

- UGC 修复前矩阵 402/405，发现 3 个 nullable backup URL 模型失败；修复后同一矩阵 405/405。
- 共审计 199 个只读契约并采集 1697 份 live 响应；全库 live 模型复验 1004 passed、0 failed、198 ignored。
- 严格 pre-commit gate 和全部单 feature 构建通过；完整脱敏结论见 [live matrix 报告](references/live-matrix-report.md)。

## References

- [Issue #13](https://github.com/Yuelioi/bpi-rs/issues/13)
- [播放流模型](../../../src/video/videostream_url.rs)
- [三身份只读验证报告](references/live-matrix-report.md)

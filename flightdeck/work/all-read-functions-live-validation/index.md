# 全部非写 API 代表性数据形态验证

Status: Open

Goal: 对 bpi-rs 暴露的每一个非写网络 API 函数执行 live 验证，并针对会因内容年代、类型、状态或嵌套层级改变响应结构的函数建立代表性数据形态矩阵；需要身份差异的函数分别覆盖匿名、普通登录、会员三档，排除所有远端写操作。每个重要响应等价类必须有多个样本和可追溯的强类型解析结论。

Current: 代表性形态矩阵已实证接口基线不足。动态 49 条（2009～2026）435 calls，解码错误 18 → 0。番剧 37 season 的 info/detail/sections 333 calls，错误 90 → 69 → 30 → 24 → 9 → 0；从 12 个代表 season 抽 33 个首/中/尾/付费/预告 episode 做 198 次播放流对照，错误 58 → 0。课程和其他内容模块尚未展开。

Next: 当出现相关 issue 或用户明确恢复本 Work 时，先读 [研究结果](references/representative-shape-findings.md) 和 [代表性样本矩阵](references/representative-shape-matrix.md)，将 issue 映射到对应等价类并补充同类样本；若无特定 issue，则从课程免费/付费/试看矩阵继续。安全边界见 [上下文](context.md)。

## Progress

- 204/204 接口执行覆盖仍作为基线保留，但不再等同于代表性数据形态覆盖。
- `dynamic.pics` 与 `login.daily_reward` 已多样本/三身份执行，但当前上游仅返回权限、内容失效或风控错误。
- 匿名 `user.uploaded_videos` 暴露 transport envelope 顺序 bug；离线测试先红后绿，live 结果现为正确 `Api(-352)`。
- 动态代表性矩阵：49 条、435 calls，`ResponseDecode` 18 → 0；覆盖旧数字 ID 和删除原动态 null ID。
- 番剧核心矩阵：37 season、333 calls，`ResponseDecode` 90 → 69 → 30 → 24 → 9 → 0；覆盖老/新/付费/未开播/电影/国创结构。
- 番剧播放流矩阵：33 episode、198 calls，`ResponseDecode` 58 → 0；覆盖跨年代、会员、付费、电影、国创和预告形态。

## References

- [上一轮播放流验证](../issue-13-playurl-validation/index.md)
- [非写网络函数覆盖清单](references/coverage-inventory.md)
- [接口执行基线报告](references/final-report.md)
- [代表性样本矩阵](references/representative-shape-matrix.md)
- [代表性数据形态研究结果](references/representative-shape-findings.md)

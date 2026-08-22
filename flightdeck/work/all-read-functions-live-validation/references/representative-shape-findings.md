# 代表性数据形态研究结果

记录日期：2026-08-23

## 核心结论

单个契约样本乘三身份只能证明 endpoint 的常见路径可执行，不能证明 SDK 能解析跨年代、跨内容类型和边界状态的响应。有效方法是：先从大候选池按实际响应字段划分等价类，再为每类选择多个样本，对相关接口和身份矩阵执行强类型解析。

本轮只完成动态和番剧阶段；课程及其他模块保留为 Open Work。

## Issue #13 播放流

Issue #13 已解决并经过真实响应验证：

- `flac.audio=null`：匿名和普通账号真实复现。
- `flac.audio` 为单个 stream 对象：会员账号真实复现，id 为 `30251`。
- `DashFlac.audio` 已从错误的 `Vec<DashStream>` 改为 `Option<DashStream>`，并建模可选 `display`。
- 播放流矩阵额外发现 `DashStream.backupUrl=null` 和 `DurlInfo.backup_url=null`，现统一归一化为空 Vec。
- 21 个视频、7 组内容、27 个页面、3 身份、5 请求形态：修复前 402/405，修复后 405/405。

## 动态

候选发现：

- 5 个公开空间分页得到 1714 条候选，时间覆盖 2009～2026。
- 类型包含 AV 1000、转发 481、图文 186、纯文字 33，以及 ARTICLE、直播和其他业务卡片。
- 转发原内容覆盖 AV、图文、专栏、PGC、直播和 `DYNAMIC_TYPE_NONE`（删除/不可见）。

分层样本：49 条。对所有样本执行 `detail`、`reactions`；转发额外执行 `forwards`、`forward_item`；图文额外执行 `pics`，并覆盖三身份，共 435 calls。

结果：

- 修复前 `ResponseDecode`：18。
- 修复后同一矩阵：389 成功、46 个稳定登录/权限/风控错误、0 个 `ResponseDecode`。
- 真实差异 1：2009 年旧 AV 的 `item.id_str` 为数字。
- 真实差异 2：转发原动态已删除时，`orig` 占位对象存在但 `orig.id_str=null`。
- 修复：数字 ID 规范化为文本，null ID 规范化为空文本，同时保留 tombstone 的其他结构。

## 番剧

候选按番剧索引真实过滤值分层，最终选择 37 个 season，覆盖：

- 70 年代及更早、80s、90s、2000s、2010s、2016/2020、2024/2026。
- 免费/限免、大会员、显式付费。
- 完结/连载，TV/电影/OVA/其他。
- 日番、国创、美国/其他地区。

### Info / Detail / Sections

每个 season 执行 `info`、`detail_by_season_id`、`sections` × 三身份，共 333 calls。迭代错误曲线：

```text
90 → 69 → 30 → 24 → 9 → 0
```

发现并兼容的真实形态：

- section episode stat 使用 `coin/play`，并缺少顶层 stat 的其他展示字段。
- `freya.bubble_desc`、`payment.tip` 可缺。
- 未开播 media 没有 `rating`。
- 未开播 section 没有 `main_section`。
- 历史/精简 section episode 缺少大量展示字段。
- section report、up_info 为部分对象。
- `total=-1` 表示未知，必须使用有符号整数。
- `episode_ids` 冗余列表可缺。
- 国创 pendant pid 可能为负数哨兵 `-1014771399`。

### 播放流

从 12 个代表 season 选择 33 个首/中/尾/付费/预告 episode，执行基础/增强 DASH × 三身份，共 198 calls。

- 修复前：140 成功、58 个 `ResponseDecode`。
- 修复后：198/198 成功。
- 根因：`VideoStreamData.durl` 被错误重命名为 `durls`。上游 `durl` 是当前画质分段，`durls` 是按画质分组的容器；空数组 fixture 掩盖了这个错误。

## 额外系统性修复

匿名 `user.uploaded_videos` 返回非零 API code 时携带不完整 `data`。transport 旧实现会在检查 code 前先解码成功 payload，错误返回 `ResponseDecode`。现改为两阶段 envelope 解码：非零 code 直接返回 `BpiError::Api`，仅成功响应严格解析 payload。

## 验证状态

- `cargo check --all-features`：通过。
- `cargo test --all-features --lib`：1015 passed、0 failed、198 ignored。
- 动态与番剧所有新增回归均按先红后绿执行。
- 原始样本、候选池和矩阵输出保存在 Git 忽略的 `target/representative-shape-matrix/`。

## Open Work

后续遇到相关 issue 时从本 Work 恢复，优先顺序：

1. 课程：免费/付费/试看、老/新、单/多 section、播放流。
2. 文章/OPUS 与评论：旧专栏、新 OPUS、不同节点类型、楼中楼/删除/置顶等。
3. 用户空间、音频、漫画、直播、收藏、历史、笔记、搜索、创作中心等各模块的领域等价类。
4. 最终重跑严格 pre-commit、单 feature 矩阵并替换此前仅接口覆盖的基线报告。

# Index — 注释中文化

## State

本 topic 负责把仓库里的英文注释逐步翻译为中文。`flightdeck/briefing.md` 已记录新约定：对话、代码注释、提交信息一律使用中文；外部 API 字段名、协议关键字、命令、路径、错误码、crate 名称等不可翻译的技术标识保持原文。

当前只建立计划和扫描工具，不执行批量翻译。用户明确要求：本 topic 的计划和后续翻译先不要提交。

## Next

- 使用 `scripts/comment-language-audit.ps1` 获取英文注释候选。
- 按 `plan.md` 的顺序分批翻译。
- 每批只改注释，不改代码行为。
- 每批后运行格式、编译或相关测试检查。

## Read now

- plan.md
- scripts/comment-language-audit.ps1
- flightdeck/briefing.md

## Read if

- flightdeck/knowledge/rust/sdk-quality.md — 如果翻译 rustdoc 时影响公开 API 文档表达。

## Progress

Done:

- 新增中文协作约定。
- 新增英文注释候选扫描脚本。
- 建立本 topic 的翻译计划。

Current:

- 等待按批次开始翻译。

Verified:

- `powershell -ExecutionPolicy Bypass -File scripts\comment-language-audit.ps1`
- 当前默认扫描结果：948 个文件，1144 行英文注释候选，分布在 125 个文件。

## Open questions

- 无。

---
format: 4
project: bpi-rs
focus: all-read-functions-live-validation
---

# Flightdeck

## Conventions

- 对话、代码注释、提交信息一律使用中文；外部 API 字段名、协议关键字、命令、路径、错误码、crate 名称等不可翻译的技术标识保持原文。
- 使用 `flightdeck_inspect` 查看知识路由；本项目当前没有活动 topic。恢复活动工作时使用 `flightdeck_resume`，有意义的恢复点使用 `flightdeck_checkpoint`，完成主题使用 `flightdeck_finish`。
- 不要批量读取 `knowledge/`；按 frontmatter 的 `read_when` 只加载与当前任务匹配的知识。
- 提交前读取 `knowledge/git/pre-commit-checklist.md` 并运行仓库规定的 pre-commit gate。

## Open questions

## Open Work

- **Focus:** [全部非写 API 代表性数据形态验证](work/all-read-functions-live-validation/index.md)

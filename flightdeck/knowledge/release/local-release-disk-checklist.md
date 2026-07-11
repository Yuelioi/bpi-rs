# 本地发布磁盘空间检查清单
SUMMARY: Always verify sufficient free disk and control Rust incremental caches before running bpi-rs CI, package, and publish verification locally.
READ WHEN: before any local bpi-rs release validation or repeated full-feature package verification.
RECHECK WHEN: Cargo target layout, release tasks, build drive, or CI verification commands change.

---

## Before validation

- 检查构建盘剩余空间；本仓库一次冷的全 feature CI 加 package/dry-run 可能需要数十 GB。
- 检查 `target/debug` 和 `target/package` 的占用，避免重复验证在磁盘接近满载时启动。
- 本地发布复验可设置 `CARGO_INCREMENTAL=0`，减少一次性发布检查产生的增量缓存；这不改变检查内容。

## Safe cleanup

- 只清理可重新生成的 `target/debug`、`target/package`、`target/release` 和 flycheck 缓存。
- 不要一并删除 `target/bpi-probe-*`、本地研究资料或其他可能包含尚未提升证据的输出。
- 删除前解析并验证绝对路径位于当前仓库的 `target/` 下。

## Failure signature

`os error 112` / “磁盘空间不足” 写入 Rust incremental query cache 时，不代表代码或测试失败。释放构建缓存空间后，从完整发布检查重新开始；不要绕过 CI、package 或 publish dry-run。

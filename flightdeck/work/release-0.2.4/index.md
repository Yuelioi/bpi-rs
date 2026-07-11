# Index — 发布 0.2.4

## State

准备发布 bpi-rs 0.2.4。版本号、锁文件和 changelog 已提交；完整 CI、隐私扫描、正式 package 和 publish dry-run 已通过。0.2.3 已存在于 crates.io，远端 `v0.2.3` 指向 `713ea5d`。

仓库在推送 `v*` tag 后由 GitHub Actions 自动执行 `cargo publish --locked`。为避免本地 publish 与 CI 竞速，本次使用单一 tag workflow 发布路径。

## Next

- 刷新 origin，确认 main 无远端新提交后推送。
- 创建并推送 `v0.2.4`，监控 workflow 和 crates.io。

## Read now

- docs/release-checklist.md
- flightdeck/knowledge/git/pre-commit-checklist.md

## Read if

- .github/workflows/release.yml — workflow 行为或失败时。

## Progress

Done:

- 确认 crates.io 当前版本为 0.2.3。
- 确认本地和远端不存在 `v0.2.4`。
- 确认 origin/main 无远端独有提交，本地领先 4 个提交。
- 更新 Cargo.toml、Cargo.lock 和 CHANGELOG.md 到 0.2.4。
- 隐私扫描命中均为文档占位符、测试假值或字段名，没有真实凭据。
- `task ci` 全部通过。
- 提交发布准备：`98e7d2e chore(release): 准备 0.2.4`。
- 干净工作区 `cargo package --locked` 通过：982 个文件，4.2 MiB，压缩后 676.9 KiB。
- `cargo publish --locked --dry-run` 通过。

Current:

- 推送 main 前的远端复核。

Verified:

- `cargo search bpi-rs --limit 5`
- `git ls-remote --tags origin`
- `git fetch origin --prune`
- `task ci`：1002 passed，0 failed，198 ignored；clippy、doc、examples 和 no-default-features 全部通过。
- 发布隐私扫描：无真实凭据或已知真实账号标识。
- `cargo package --locked`
- `cargo publish --locked --dry-run`

## Open questions

- 无。

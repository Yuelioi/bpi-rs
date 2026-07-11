# Index — 发布 0.2.4

## State

准备发布 bpi-rs 0.2.4。版本号、锁文件和 changelog 已更新；完整 CI 与隐私扫描已通过。0.2.3 已存在于 crates.io，远端 `v0.2.3` 指向 `713ea5d`；本地 main 在 origin/main 之前 4 个提交，无分叉。

仓库在推送 `v*` tag 后由 GitHub Actions 自动执行 `cargo publish --locked`。为避免本地 publish 与 CI 竞速，本次使用单一 tag workflow 发布路径。

## Next

- 提交已经通过 CI 的版本准备。
- 在干净提交上运行 package 和 publish dry-run。
- 推送 main。
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

Current:

- 提交版本准备，以便在干净工作区运行正式 package。

Verified:

- `cargo search bpi-rs --limit 5`
- `git ls-remote --tags origin`
- `git fetch origin --prune`
- `task ci`：1002 passed，0 failed，198 ignored；clippy、doc、examples 和 no-default-features 全部通过。
- 发布隐私扫描：无真实凭据或已知真实账号标识。

## Open questions

- 无。

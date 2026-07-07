# 发布检查清单

发布前按顺序执行。不要在工作区有未知改动时发布。

## 1. 状态检查

```powershell
git status --short
git log --oneline -5
```

确认没有未审查的本地文件、账号配置、Probe 输出或临时产物。

## 2. 默认验证

发布前必须跑和 GitHub Actions 相同的检查，尤其是 `cargo clippy --all-targets --all-features --locked -- -D warnings`。这一步会把 clippy warning 当成错误处理。

推荐使用 Taskfile 入口：

```powershell
task ci
```

等价的 cargo 命令如下：

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features
cargo test --doc
cargo check --examples --all-features
cargo check --no-default-features --lib
```

## 3. 打包验证

```powershell
cargo package --allow-dirty
```

正式发布前应去掉 `--allow-dirty`，确保发布包来自干净工作区：

```powershell
cargo package
```

## 4. 隐私扫描

```powershell
rg -n "SESSDATA|bili_jct|buvid3|account.toml" . --glob "!target/**" --glob "!.git/**"
rg -n "已知真实 mid|已知真实昵称" . --glob "!target/**" --glob "!.git/**"
```

命中测试假值或文档占位符时人工确认；真实账号信息必须移除。

## 5. 文档确认

- `README.md` 已同步关键行为。
- `CHANGELOG.md` 已记录破坏性变更。
- `account.example.toml` 与当前账号加载逻辑一致。
- 变更类 API 已在文档中标明风险。

## 6. 发布

GitHub Actions 会在推送 `v*` tag 后自动发布到 crates.io。发布 job 会先重复默认验证、校验 tag 版本与 `Cargo.toml` 版本一致、运行 `cargo package --locked` 和 `cargo publish --locked --dry-run`，全部通过后才执行正式发布。

仓库需要配置 GitHub Actions secret：

```text
CARGO_REGISTRY_TOKEN
```

本地确认无误后创建并推送版本 tag：

```powershell
git tag v0.2.3
git push origin v0.2.3
```

如需手动发布，仍可使用：

```powershell
cargo publish
```

发布后创建 GitHub release，并粘贴 `CHANGELOG.md` 对应版本内容。

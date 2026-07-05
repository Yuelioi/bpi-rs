# 发布检查清单

发布前按顺序执行。不要在工作区有未知改动时发布。

## 1. 状态检查

```powershell
git status --short
git log --oneline -5
```

确认没有未审查的本地文件、账号配置、Probe 输出或临时产物。

## 2. 默认验证

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

```powershell
cargo publish
```

发布后创建 GitHub release，并粘贴 `CHANGELOG.md` 对应版本内容。

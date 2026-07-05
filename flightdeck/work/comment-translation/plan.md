# 注释中文化计划

## 目标

把代码里的英文注释逐步翻译为中文，降低维护时的语言切换成本。范围限定为源码、测试、示例和脚本中的注释；不翻译 Flightdeck 历史计划文档。只翻译说明性自然语言，不翻译代码、协议、字段、命令、路径、crate 名称、类型名、函数名、环境变量、错误码、URL、Bilibili 原始字段名。

## 工具

统计英文注释候选：

```powershell
powershell -ExecutionPolicy Bypass -File scripts\comment-language-audit.ps1
```

查看样例：

```powershell
powershell -ExecutionPolicy Bypass -File scripts\comment-language-audit.ps1 -ShowExamples -ExampleLimit 80
```

只看指定目录：

```powershell
powershell -ExecutionPolicy Bypass -File scripts\comment-language-audit.ps1 -Paths src\video,src\client
```

## 翻译规则

- 保持技术标识原文：`aid`、`bvid`、`SESSDATA`、`csrf`、`reqwest`、`BpiClient`、`BPI_MUTATING_TEST` 等。
- 保持 API 路径原文：`/x/web-interface/archive/coins`、`coin/add` 等。
- 保持代码示例和命令原文，必要时只翻译周围解释。
- 保持 rustdoc 的语气准确：`Gets ...` 可翻译为“获取...”，`Creates ...` 可翻译为“创建...”，`Sets ...` 可翻译为“设置...”。
- 不为了翻译改动公开 API、测试逻辑、序列化字段、错误分类、契约 fixture。
- 遇到语义不确定的注释，先保留原文并标记到本 topic，而不是猜译。

## 分批顺序

优先处理候选最多、风险较低的模型和参数文件：

1. `src/user/model.rs`
2. `src/user/params.rs`
3. `src/video/model.rs`
4. `src/login/model.rs`
5. `src/video/params.rs`
6. `src/client.rs`
7. 各 domain `client.rs`
8. action/transport/error 文件

每批建议控制在 5 到 15 个文件，避免大 diff 难审。

## 每批流程

1. 运行扫描脚本，选定本批文件。
2. 只翻译注释，不改代码。
3. 运行：

```powershell
cargo fmt --check
cargo check --all-features
```

1. 若改到 doctest 或 README 示例，再运行：

```powershell
cargo test --doc
```

1. 复跑扫描脚本确认候选数量下降。
2. 按用户要求：翻译阶段先不要提交。

## 风险点

- rustdoc 是公开文档，翻译不能改变 API 语义。
- 有些英文是 Bilibili 原始响应字段或固定术语，不能翻译。
- 扫描脚本是候选检测，不是精确分类；结果需要人工确认。
- `#[ignore = "..."]` 等属性不是注释，当前脚本默认不统计。

## 当前基线

默认扫描范围：`src`、`tests`、`examples`、`scripts`。

当前结果：

```text
FilesScanned        : 948
CandidateLines      : 1144
FilesWithCandidates : 125
```

候选最多：

```text
src/user/model.rs                  279
src/user/params.rs                  57
src/client.rs                       52
src/video/model.rs                  45
src/login/model.rs                  43
src/video/params.rs                 33
src/video/collection/params.rs      24
src/live/client.rs                  21
src/video/client.rs                 18
src/audio/client.rs                 17
src/user/client.rs                  17
```

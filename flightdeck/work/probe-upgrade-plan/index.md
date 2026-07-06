# 探针升级计划 Topic

## 目标

把当前“手写 JSON 契约 + 模块内零散测试 + 人工脱敏”的探针流程，升级成一套仓库内置、可审计、可批量运行、可自动脱敏、可统一测试的开发工具链。

## 当前判断

- 现有 `tests/contracts/**` 已经积累了足够多的契约和 fixtures，可以先从字段全集反推脱敏字段表。
- 第一阶段不需要设计复杂 DSL，也不需要马上做自动 promote；先把字段提取、脱敏审计和统一契约测试做起来。
- `bpi-probe` 当前只支持单个 contract/flow 的网络执行，还缺少离线审计、批量运行、脱敏提升和统一测试入口。
- 只保留 JSON 不够。贡献者需要明确命令来检查、运行、脱敏、提升和验证契约，否则每次新增 API 都会退回手写脚本。

## 设计方向

1. 以现有 JSON 为事实来源，先抽取字段路径和出现频次。
2. 建立内置脱敏字段表：字段名规则 + JSON path 覆盖规则 + 风险等级策略。
3. 新增离线命令，让 CI 能检查契约 schema、fixtures、脱敏残留和字段表覆盖。
4. 新增统一契约测试 runner，减少每个模块重复写 `include_bytes!` fixture 测试。
5. 后续再做批量 Probe 和自动 promote，把 raw probe 输出转换成可提交 fixtures。

## 跟踪文件

- 升级方案：`flightdeck/work/probe-upgrade-plan/plan.md`
- 现有探针代码：`src/probe/**`
- 探针 CLI：`src/bin/bpi-probe.rs`
- 契约样例：`tests/contracts/**`
- 探针开发指南：`docs/api-probe-development.md`
- API 风险分类：`docs/api-risk-classification.md`

## 非目标

- 第一阶段不直接跑真实网络 Probe。
- 第一阶段不重写所有现有契约格式。
- 第一阶段不删除模块内已有测试，只先补统一工具和审计入口。

## 当前进度

- 已完成第一批兼容性切片：`EndpointContract` 可读取 v2 元数据字段，并迁移了 `video.view`、`login.vip_info`、`live.my_medals` 三个代表契约。
- 已同步 `docs/api-probe-development.md`，新增契约字段说明。
- 已验证 `cargo fmt --check`、`cargo test --all-features --lib probe::endpoint_contract`、`cargo test --all-features --lib probe`。
- 已开始第二批离线工具切片：新增字段路径抽取模块和 `bpi-probe fields <path>` 命令，用于从现有契约 JSON 反推脱敏字段表。
- 已开始第三批脱敏能力切片：新增 `src/probe/sanitize.rs`，内置凭据字段、私有账号字段、明显凭据字符串的基础脱敏和审计函数。
- 已接入 `bpi-probe sanitize-audit <path>` 和 `task probe_sanitize_audit`，第一版只扫描 response fixtures 里的高置信敏感凭据残留。
- 已接入 `bpi-probe contract-audit <path>` 和 `task probe_contract_audit`，当前全量 206 个契约为 0 个结构错误、203 个旧格式迁移 warning。
- 已新增统一离线契约测试 `tests/contract_runner.rs` 和 `task contract_test`，当前覆盖全量契约解析、fixture JSON、`api_code` 一致性、高置信脱敏审计和少量注册模型解析。
- 已新增 `bpi-probe batch-run <path>` 网络批量探针入口，默认必须 `BPI_PROBE=1`，并对 `mutating`、`spending`、`login-session` 做额外环境变量门控；本轮只验证门控，不执行真实批量网络请求。
- `batch-run` 已支持 `--pages`，默认 10；普通分页展开已有 `page`、`pn` 或 `pageNum`，`historytoview.history_list` 通过响应 `data.cursor.max/view_at/business` 继续翻页，非分页契约仍只运行一次。
- `batch-run` 和 `contract_test` 已共用 `src/probe/model.rs` 模型注册表；已注册 `rust_model` 会在 fixture 测试和实时批量探针中反序列化，避免只校验 code 漏掉字段类型漂移。
- 已新增 `bpi-probe promote <probe-output.response.json>`，可从 batch-run 输出路径反推契约，脱敏 response body 后写入 fixture，并更新对应 case。
- 已在 `EndpointContract` 解析器层面强制 v2 必填元数据；旧契约继续按 v1 兼容读取。
- 已完成 Task 9 全量迁移：当前 `tests/contracts/**/contract.json` 共 206 个契约，均已迁到 v2；全量契约审计为 0 个结构错误、0 个 warning。
- 已补齐第一版内置路径覆盖表，覆盖 `$.data.owner.*`、`$.data.list[].owner.*`、`$.data.cooperators[]` 等高频账号字段路径。

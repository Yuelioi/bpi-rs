## 变更说明

-

## API 风险分类

- [ ] `public-read`：公开只读接口
- [ ] `authenticated-read`：登录态只读接口
- [ ] `private-read`：账号私有数据读取接口
- [ ] `mutating`：点赞、收藏、关注、评论、开播、删除等变更类接口
- [ ] `spending`：投币、支付、购买、兑换、充电等消费资产接口
- [ ] `login-session`：登录、二维码、短信、Cookie 刷新、风控敏感接口
- [ ] 仅文档、测试或内部重构

## 验证

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets --all-features --locked -- -D warnings`
- [ ] `cargo test --all-features`
- [ ] `cargo check --examples --all-features`

## 账号和隐私

- [ ] 没有提交 Cookie、`SESSDATA`、`bili_jct`、`buvid3`、`account.toml` 或原始 Probe 输出
- [ ] fixtures 已脱敏，未包含账号可识别信息
- [ ] 变更类测试已使用 `#[ignore]` 和显式环境变量门控

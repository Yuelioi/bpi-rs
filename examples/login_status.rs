//! 读取当前账号的登录状态、硬币余额和每日奖励。
//!
//! ```powershell
//! $env:BPI_ACCOUNT_TOML = "account.toml"
//! $env:BPI_ACCOUNT_PROFILE = "vip"
//! cargo run --example login_status --features login
//! ```

#[path = "common/account.rs"]
mod account;

use bpi_rs::BpiResult;

#[tokio::main]
async fn main() -> BpiResult<()> {
    let client = account::authenticated_client()?;
    let login = client.login();

    let nav = login.nav().await?;
    println!(
        "登录状态: is_login={} mid={:?} uname={}",
        nav.is_login,
        nav.mid,
        nav.uname.as_deref().unwrap_or("<unknown>")
    );

    let coin = login.coin().await?;
    println!("硬币余额: {:.2}", coin.money);

    match login.daily_reward().await {
        Ok(reward) => println!(
            "每日奖励: login={} watch={} coins={} share={}",
            reward.login, reward.watch, reward.coins, reward.share
        ),
        Err(err) if err.is_risk_control() => {
            println!("每日奖励: 当前请求被风控拦截，已跳过 ({err})");
        }
        Err(err) => return Err(err),
    }

    let vip = login.vip_info().await?;
    println!(
        "VIP: active={} type={} status={}",
        vip.is_active(),
        vip.vip_type,
        vip.vip_status
    );

    Ok(())
}

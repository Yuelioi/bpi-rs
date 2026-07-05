//! 验证网页 link 中心自动开播（FetchWebUpStreamAddr + WebLiveCenterStartLive）
//!
//! ```powershell
//! cd bpi-rs
//! $env:BPI_ACCOUNT_TOML = "account.toml"
//! $env:BILI_ROOM_ID = "4354019"
//! $env:BILI_AREA_ID = "309"
//! cargo run --example live_web_start --features live,misc
//! ```
//!
//! 若已在直播，加 `--stop-if-live` 会先关播再开播（会短暂断流）。
//! 只关播并验证状态，使用 `--stop-only`。

use bpi_rs::{Account, BpiClient, BpiError, BpiResult};
use config::{Config, ConfigError, File};
use std::path::PathBuf;

const DEFAULT_ROOM_ID: i64 = 4354019;
const DEFAULT_AREA_ID: u64 = 309;

fn mask(s: &str) -> String {
    if s.len() <= 8 {
        return "***".to_string();
    }
    format!("{}...{}", &s[..4], &s[s.len() - 4..])
}

fn account_path() -> PathBuf {
    std::env::var("BPI_ACCOUNT_TOML")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("account.toml"))
}

fn load_account() -> BpiResult<Account> {
    let path = account_path();
    if !path.exists() {
        return Err(BpiError::invalid_parameter(
            "account_path",
            "account config file does not exist",
        ));
    }

    let settings = Config::builder()
        .add_source(File::from(path))
        .build()
        .map_err(|err| BpiError::parse(format!("读取账号配置失败: {err}")))?;

    let account = match settings.get::<Account>("vip") {
        Ok(account) => account,
        Err(ConfigError::NotFound(_)) => match load_suffixed_account(&settings, "_vip")? {
            Some(account) => account,
            None => settings
                .try_deserialize::<Account>()
                .map_err(|err| BpiError::parse(format!("解析账号配置失败: {err}")))?,
        },
        Err(err) => {
            return Err(BpiError::parse(format!("解析 vip 账号配置失败: {err}")));
        }
    };

    account.validate_complete()?;
    Ok(account)
}

fn load_suffixed_account(settings: &Config, suffix: &str) -> BpiResult<Option<Account>> {
    let dede_user_id = config_string(settings, &format!("dede_user_id{suffix}"))?;
    let sessdata = config_string(settings, &format!("sessdata{suffix}"))?;
    let bili_jct = config_string(settings, &format!("bili_jct{suffix}"))?;
    let buvid3 = config_string(settings, &format!("buvid3{suffix}"))?;

    if dede_user_id.is_none() && sessdata.is_none() && bili_jct.is_none() && buvid3.is_none() {
        return Ok(None);
    }

    let account = Account::new(
        dede_user_id.ok_or_else(incomplete_account)?,
        sessdata.ok_or_else(incomplete_account)?,
        bili_jct.ok_or_else(incomplete_account)?,
        buvid3.ok_or_else(incomplete_account)?,
    );
    Ok(Some(account))
}

fn config_string(settings: &Config, key: &str) -> BpiResult<Option<String>> {
    match settings.get_string(key) {
        Ok(value) => Ok(Some(value)),
        Err(ConfigError::NotFound(_)) => Ok(None),
        Err(err) => Err(BpiError::parse(format!("解析账号配置项 {key} 失败: {err}"))),
    }
}

fn incomplete_account() -> BpiError {
    BpiError::invalid_parameter("account", "account profile is incomplete")
}

#[tokio::main]
async fn main() -> BpiResult<()> {
    let stop_if_live = std::env::args().any(|a| a == "--stop-if-live");
    let stop_only = std::env::args().any(|a| a == "--stop-only");
    let room_id: i64 = std::env::var("BILI_ROOM_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_ROOM_ID);
    let area_id: u64 = std::env::var("BILI_AREA_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_AREA_ID);

    let bpi = BpiClient::new()?;
    bpi.set_account(load_account()?)?;

    println!("=== Rust 网页自动开播验证 ===");
    println!("room_id={room_id} area_v2={area_id}");

    let live = bpi.live();
    let info = live.room_info(room_id).await?;
    println!(
        "当前状态: live_status={} title={} area={}·{}",
        info.live_status, info.title, info.parent_area_name, info.area_name
    );

    if stop_only {
        if info.live_status == 1 {
            println!("执行关播...");
            let stop = live.live_stop(room_id as u64, "pc").await?;
            println!("关播结果: status={}", stop.status);
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        } else {
            println!("当前未开播，跳过关播接口。");
        }

        let after = live.room_info(room_id).await?;
        println!(
            "验证: live_status={} ({})",
            after.live_status,
            if after.live_status == 0 {
                "关播成功"
            } else {
                "仍在直播中"
            }
        );
        return Ok(());
    }

    if info.live_status == 1 {
        if stop_if_live {
            println!("已在直播，--stop-if-live：先关播...");
            let stop = live.live_stop(room_id as u64, "pc").await?;
            println!("关播结果: status={}", stop.status);
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        } else {
            println!("已在直播中。若要完整验证开播链路，请加 --stop-if-live 或先手动关播。");
            println!("仍尝试 FetchWebUpStreamAddr（只读）...");
            let stream = live.live_fetch_web_up_stream_addr().await?;
            println!("推流地址: {}{}", stream.addr.addr, mask(&stream.addr.code));
            return Ok(());
        }
    }

    println!("\n[1/2] FetchWebUpStreamAddr...");
    let stream = live.live_fetch_web_up_stream_addr().await?;
    println!("  rtmp: {}{}", stream.addr.addr, mask(&stream.addr.code));

    println!("\n[2/2] WebLiveCenterStartLive (WBI)...");
    let start = live.live_web_center_start(room_id as u64, area_id).await?;
    println!("  status={} live_key={}", start.status, start.live_key);
    if let Some(rtmp) = &start.rtmp {
        println!("  rtmp(响应): {}{}", rtmp.addr, mask(&rtmp.code));
    } else {
        println!("  rtmp(响应): null（正常，推流码以 Fetch 为准）");
    }

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let after = live.room_info(room_id).await?;
    println!(
        "\n验证: live_status={} ({})",
        after.live_status,
        if after.live_status == 1 {
            "开播成功"
        } else {
            "未变为直播中（可能需 OBS 推流）"
        }
    );

    Ok(())
}

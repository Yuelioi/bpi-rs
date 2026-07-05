//! 验证网页 link 中心自动开播（FetchWebUpStreamAddr + WebLiveCenterStartLive）
//!
//! ```powershell
//! cd bpi-rs
//! $env:BPI_ACCOUNT_TOML = "account.toml"
//! $env:BILI_ROOM_ID = "4354019"
//! $env:BILI_AREA_ID = "309"
//! $env:BPI_MUTATING_TEST = "1"
//! cargo run --example live_web_start --features live,misc
//! ```
//!
//! 若已在直播，加 `--stop-if-live` 会先关播再开播（会短暂断流）。
//! 只关播并验证状态，使用 `--stop-only`。

#[path = "common/account.rs"]
mod account;

use bpi_rs::BpiResult;

const DEFAULT_ROOM_ID: i64 = 4354019;
const DEFAULT_AREA_ID: u64 = 309;
const MUTATING_ENV: &str = "BPI_MUTATING_TEST";

fn mask(s: &str) -> String {
    if s.len() <= 8 {
        return "***".to_string();
    }
    format!("{}...{}", &s[..4], &s[s.len() - 4..])
}

fn mutating_enabled() -> bool {
    std::env::var(MUTATING_ENV).ok().as_deref() == Some("1")
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

    let bpi = account::authenticated_client()?;

    println!("=== Rust 网页自动开播验证 ===");
    println!("room_id={room_id} area_v2={area_id}");

    let live = bpi.live();
    let info = live.room_info(room_id).await?;
    println!(
        "当前状态: live_status={} title={} area={}·{}",
        info.live_status, info.title, info.parent_area_name, info.area_name
    );

    if !mutating_enabled() {
        println!("{MUTATING_ENV}=1 未设置，只读取直播间状态，不执行开播、关播或获取推流码。");
        return Ok(());
    }

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

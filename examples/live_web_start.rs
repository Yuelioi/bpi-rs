//! 验证网页 link 中心自动开播（FetchWebUpStreamAddr + WebLiveCenterStartLive）
//!
//! ```powershell
//! cd bpi-rs
//! $env:SETTINGS_JSON = "D:\Projects\bilibili-bot\settings.json"
//! $env:BILI_ROOM_ID = "4354019"
//! $env:BILI_AREA_ID = "309"
//! cargo run --example live_web_start --features live,misc
//! ```
//!
//! 若已在直播，加 `--stop-if-live` 会先关播再开播（会短暂断流）。

use bpi_rs::{Account, BpiClient, BpiError};
use serde::Deserialize;
use std::path::PathBuf;

const DEFAULT_ROOM_ID: i64 = 4354019;
const DEFAULT_AREA_ID: u64 = 309;

#[derive(Deserialize)]
struct FileRoot {
    credential: serde_json::Value,
}

fn cred_string(v: &serde_json::Value, keys: &[&str]) -> String {
    for k in keys {
        if let Some(s) = v.get(*k).and_then(|x| x.as_str()) {
            if !s.is_empty() {
                return s.to_string();
            }
        }
    }
    String::new()
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut out = Vec::with_capacity(input.len());
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let h = (bytes[i + 1] as char).to_digit(16);
            let l = (bytes[i + 2] as char).to_digit(16);
            if let (Some(h), Some(l)) = (h, l) {
                out.push(((h << 4) | l) as u8);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn mask(s: &str) -> String {
    if s.len() <= 8 {
        return "***".to_string();
    }
    format!("{}...{}", &s[..4], &s[s.len() - 4..])
}

async fn load_account(bpi: &'static BpiClient) -> Account {
    let path = std::env::var("SETTINGS_JSON")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../settings.json")
        });
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("读取 {:?}: {}", path, e));
    let root: FileRoot = serde_json::from_str(&text).expect("解析 settings JSON");
    let cred = &root.credential;

    let sessdata = percent_decode(&cred_string(cred, &["sessdata", "SESSDATA"]));
    let dede_user_id = cred_string(cred, &["dede_user_id", "DedeUserID", "dedeuserid"]);
    let ckmd5 = {
        let s = cred_string(cred, &["dede_user_id__ckmd5"]);
        if s.is_empty() {
            std::env::var("BILI_DEDE_USER_ID_CKMD5").unwrap_or_default()
        } else {
            s
        }
    };
    let bili_jct = cred_string(cred, &["bili_jct"]);
    let buvid3_raw = cred_string(cred, &["buvid3"]);

    let buvid3 = if buvid3_raw.trim().is_empty() {
        bpi.misc_buvid3()
            .await
            .and_then(|r| r.into_data())
            .expect("获取 buvid3")
            .buvid
    } else {
        buvid3_raw
    };

    Account::new(dede_user_id, ckmd5, sessdata, bili_jct, buvid3)
}

#[tokio::main]
async fn main() -> Result<(), Box<BpiError>> {
    let stop_if_live = std::env::args().any(|a| a == "--stop-if-live");
    let room_id: i64 = std::env::var("BILI_ROOM_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_ROOM_ID);
    let area_id: u64 = std::env::var("BILI_AREA_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_AREA_ID);

    let bpi = BpiClient::new();
    bpi.set_account(load_account(bpi).await);

    println!("=== Rust 网页自动开播验证 ===");
    println!("room_id={room_id} area_v2={area_id}");

    let info = bpi.live_room_info(room_id).await?.into_data()?;
    println!(
        "当前状态: live_status={} title={} area={}·{}",
        info.live_status, info.title, info.parent_area_name, info.area_name
    );

    if info.live_status == 1 {
        if stop_if_live {
            println!("已在直播，--stop-if-live：先关播...");
            let stop = bpi.live_stop(room_id as u64, "pc").await?.into_data()?;
            println!("关播结果: status={}", stop.status);
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        } else {
            println!("已在直播中。若要完整验证开播链路，请加 --stop-if-live 或先手动关播。");
            println!("仍尝试 FetchWebUpStreamAddr（只读）...");
            let stream = bpi.live_fetch_web_up_stream_addr().await?.into_data()?;
            println!(
                "推流地址: {}{}",
                stream.addr.addr,
                mask(&stream.addr.code)
            );
            return Ok(());
        }
    }

    println!("\n[1/2] FetchWebUpStreamAddr...");
    let stream = bpi.live_fetch_web_up_stream_addr().await?.into_data()?;
    println!(
        "  rtmp: {}{}",
        stream.addr.addr,
        mask(&stream.addr.code)
    );

    println!("\n[2/2] WebLiveCenterStartLive (WBI)...");
    let start = bpi
        .live_web_center_start(room_id as u64, area_id)
        .await?
        .into_data()?;
    println!("  status={} live_key={}", start.status, start.live_key);
    if let Some(rtmp) = &start.rtmp {
        println!("  rtmp(响应): {}{}", rtmp.addr, mask(&rtmp.code));
    } else {
        println!("  rtmp(响应): null（正常，推流码以 Fetch 为准）");
    }

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let after = bpi.live_room_info(room_id).await?.into_data()?;
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

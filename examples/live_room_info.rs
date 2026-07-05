//! 读取直播间基础信息。
//!
//! ```powershell
//! $env:BILI_ROOM_ID = "3818081"
//! cargo run --example live_room_info --features live
//! ```

use bpi_rs::{BpiClient, BpiError, BpiResult};

const DEFAULT_ROOM_ID: i64 = 3818081;

fn room_id() -> BpiResult<i64> {
    match std::env::var("BILI_ROOM_ID") {
        Ok(value) => value.parse().map_err(|_| {
            BpiError::invalid_parameter("room_id", "room id must be a signed integer")
        }),
        Err(_) => Ok(DEFAULT_ROOM_ID),
    }
}

#[tokio::main]
async fn main() -> BpiResult<()> {
    let room_id = room_id()?;
    let client = BpiClient::new()?;
    let info = client.live().room_info(room_id).await?;

    println!("直播间: {} ({})", info.title, info.room_id);
    println!("主播 mid: {}", info.uid);
    println!(
        "状态: live_status={} 分区={}·{}",
        info.live_status, info.parent_area_name, info.area_name
    );
    println!("在线: {} 关注: {}", info.online, info.attention);

    Ok(())
}

//! 读取视频基础信息、分 P 列表和简介。
//!
//! ```powershell
//! $env:BPI_VIDEO_BVID = "BV1xx411c7mD"
//! cargo run --example video_info --features video
//! ```

use bpi_rs::ids::Bvid;
use bpi_rs::video::{VideoDescParams, VideoPageListParams, VideoViewParams};
use bpi_rs::{BpiClient, BpiResult};

const DEFAULT_BVID: &str = "BV1xx411c7mD";

fn video_bvid() -> BpiResult<Bvid> {
    std::env::var("BPI_VIDEO_BVID")
        .unwrap_or_else(|_| DEFAULT_BVID.to_string())
        .parse()
}

#[tokio::main]
async fn main() -> BpiResult<()> {
    let bvid = video_bvid()?;
    let client = BpiClient::new()?;
    let video = client.video();

    let view = video.view(VideoViewParams::from_bvid(bvid.clone())).await?;
    println!(
        "视频: {} ({}) UP={} 播放={} 点赞={}",
        view.title, view.bvid, view.owner.name, view.stat.view, view.stat.like
    );

    let pages = video
        .page_list(VideoPageListParams::from_bvid(bvid.clone()))
        .await?;
    println!("分 P 数: {}", pages.len());
    for page in pages.iter().take(5) {
        println!("  P{} cid={} {}", page.page, page.cid, page.part);
    }

    let desc = video.desc(VideoDescParams::from_bvid(bvid)).await?;
    let first_line = desc.lines().next().unwrap_or("");
    println!("简介首行: {first_line}");

    Ok(())
}

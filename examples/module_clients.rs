use std::env;

use bpi_rs::bangumi::BangumiInfoParams;
use bpi_rs::ids::{Bvid, MediaId};
use bpi_rs::video::VideoViewParams;
use bpi_rs::{BpiClient, BpiResult};

#[tokio::main]
async fn main() -> BpiResult<()> {
    let client = client_from_env()?;
    let video_params = VideoViewParams::from_bvid("BV1xx411c7mD".parse::<Bvid>()?);
    let bangumi_params = BangumiInfoParams::new(MediaId::new(28_220_978)?);

    if !run_live_example() {
        println!("module-client 快速示例已编译；设置 BPI_RUN_EXAMPLE=1 后会发起真实网络请求");
        return Ok(());
    }

    let video = client.video().view(video_params).await?;
    println!("video: {}", video.title);

    let bangumi = client.bangumi().info(bangumi_params).await?;
    println!("bangumi: {}", bangumi.media.title);

    if env::var_os("BPI_COOKIE").is_some() {
        let nav = client.login().nav().await?;
        println!("logged in: {}", nav.is_login);
    }

    Ok(())
}

fn client_from_env() -> BpiResult<BpiClient> {
    match env::var("BPI_COOKIE") {
        Ok(cookie) if !cookie.trim().is_empty() => BpiClient::builder().cookie(cookie).build(),
        _ => BpiClient::builder().build(),
    }
}

fn run_live_example() -> bool {
    env::var("BPI_RUN_EXAMPLE").is_ok_and(|value| value == "1")
}

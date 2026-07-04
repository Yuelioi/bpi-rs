use bpi_rs::ids::MediaId;
use bpi_rs::{BpiClient, BpiResult, bangumi::BangumiInfoParams};

#[tokio::main]
async fn main() -> BpiResult<()> {
    let client = BpiClient::new()?;
    let params = BangumiInfoParams::new(MediaId::new(28_220_978)?);

    if std::env::var("BPI_RUN_EXAMPLE").as_deref() != Ok("1") {
        println!("bpi-rs quickstart compiled; set BPI_RUN_EXAMPLE=1 to call live APIs");
        return Ok(());
    }

    let info = client.bangumi().info(params).await?;
    println!("bangumi: {}", info.media.title);
    Ok(())
}

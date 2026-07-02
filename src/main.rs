use bpi_rs::ids::MediaId;
use bpi_rs::{BpiClient, bangumi::BangumiInfoParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bpi = BpiClient::new()?;

    let result = bpi
        .bangumi_info(BangumiInfoParams::new(MediaId::new(28220978)?))
        .await;
    match result {
        Ok(result) => {
            tracing::info!("{:#?}", result.data);
        }
        Err(e) => {
            tracing::error!("{:#?}", e);
        }
    }

    Ok(())
}

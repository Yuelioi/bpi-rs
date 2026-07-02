use bpi_rs::BpiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bpi = BpiClient::new()?;

    let result = bpi.bangumi_info(28220978).await;
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

use bpi_rs::{ Account, BpiClient };

#[tokio::main]
async fn main() {
    let bpi = BpiClient::new();
    bpi.set_account(Account {
        dede_user_id: "".to_string(),
        dede_user_id_ckmd5: "".to_string(),
        sessdata: "".to_string(),
        bili_jct: "".to_string(),
        buvid3: "".to_string(),
    });

    // bpi.set_account_from_cookie_str("dede_user_id=123;bili_jct=456...");

    let result = bpi.bangumi_info(28220978).await;
    match result {
        Ok(result) => {
            tracing::info!("{:#?}", result.data);
        }
        Err(e) => {
            tracing::error!("{:#?}", e);
        }
    }
}

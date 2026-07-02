use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::sign::wbi::{WbiKeys, sign_params_at};
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

const WBI_NAV_ENDPOINT: &str = "https://api.bilibili.com/x/web-interface/nav";

#[derive(Deserialize, Serialize)]
struct WbiImgData {
    img_url: String,
    sub_url: String,
}

#[derive(Deserialize, Serialize)]
struct NavData {
    wbi_img: WbiImgData,
}

impl BpiClient {
    pub(crate) async fn sign_wbi_params<I, K, V>(
        &self,
        params: I,
    ) -> Result<Vec<(String, String)>, BpiError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: ToString,
        V: ToString,
    {
        let bucket = current_wbi_cache_bucket();
        let keys = if let Some(keys) = self.wbi_key_cache().get(&bucket)? {
            keys
        } else {
            let keys = self.fetch_wbi_keys().await?;
            self.wbi_key_cache().insert(bucket, keys.clone())?;
            keys
        };

        sign_params_at(params, &keys, current_unix_timestamp()?)
    }

    async fn fetch_wbi_keys(&self) -> Result<WbiKeys, BpiError> {
        let resp: BpiResponse<NavData> =
            self.get(WBI_NAV_ENDPOINT).send_bpi("获取 wbi 签名").await?;

        let data = resp
            .data
            .ok_or_else(|| BpiError::parse("获取 wbi 签名失败"))?;

        WbiKeys::from_nav_urls(&data.wbi_img.img_url, &data.wbi_img.sub_url)
    }
}

pub(crate) fn current_wbi_cache_bucket() -> String {
    Local::now().format("%Y-%m-%d %H").to_string()
}

fn current_unix_timestamp() -> Result<u64, BpiError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| BpiError::network(format!("获取时间戳失败: {error}")))
        .map(|duration| duration.as_secs())
}

#[cfg(test)]
mod tests {
    use super::current_wbi_cache_bucket;
    use crate::sign::wbi::WbiKeys;
    use crate::{BpiClient, BpiError};

    #[tokio::test]
    async fn sign_wbi_params_uses_cached_client_keys() -> Result<(), BpiError> {
        let client = BpiClient::new()?;
        client.wbi_key_cache().insert(
            current_wbi_cache_bucket(),
            WbiKeys::new(
                "abcdefghijklmnopqrstuvwxyz123456",
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ654321",
            )?,
        )?;

        let signed = client.sign_wbi_params([("mid", "1001")]).await?;

        assert!(signed.contains(&("mid".to_string(), "1001".to_string())));
        assert!(signed.iter().any(|(key, _)| key == "wts"));
        assert!(signed.iter().any(|(key, _)| key == "w_rid"));
        Ok(())
    }
}

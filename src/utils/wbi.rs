use crate::models::WbiData;
use crate::{BpiClient, BpiError};

impl BpiClient {
    pub async fn get_wbi_sign(&self) -> Result<WbiData, BpiError> {
        let params = self
            .sign_wbi_params(std::iter::empty::<(String, String)>())
            .await?;

        Ok(WbiData {
            wts: param_value(&params, "wts")?
                .parse::<u64>()
                .map_err(|_| BpiError::parse("wts 转换失败"))?,
            w_rid: param_value(&params, "w_rid")?.to_string(),
        })
    }

    pub async fn get_wbi_sign2<I, K, V>(&self, params: I) -> Result<Vec<(String, String)>, BpiError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: ToString,
        V: ToString,
    {
        self.sign_wbi_params(params).await
    }
}

fn param_value<'a>(params: &'a [(String, String)], key: &str) -> Result<&'a str, BpiError> {
    params
        .iter()
        .find_map(|(param_key, value)| (param_key == key).then_some(value.as_str()))
        .ok_or_else(|| BpiError::parse(format!("缺少 {key}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sign::wbi::WbiKeys;
    use crate::sign::wbi_client::current_wbi_cache_bucket;

    #[tokio::test]
    async fn get_wbi_sign_uses_cached_client_keys() -> Result<(), BpiError> {
        let client = BpiClient::new()?;
        client.wbi_key_cache().insert(
            current_wbi_cache_bucket(),
            WbiKeys::new(
                "abcdefghijklmnopqrstuvwxyz123456",
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ654321",
            )?,
        )?;

        let wbi = client.get_wbi_sign().await?;

        assert!(wbi.wts > 0);
        assert!(!wbi.w_rid.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_wts_and_rid2() -> Result<(), BpiError> {
        if std::env::var("BPI_LIVE_TEST").ok().as_deref() != Some("1") {
            return Ok(());
        }

        let bpi = BpiClient::new()?;

        let params = vec![
            ("bvid", "BV18x411c74j".to_string()),
            ("cid", "21448".to_string()),
            ("up_mid", "46473".to_string()),
            ("web_location", "0.0".to_string()),
        ];

        let wbi = bpi.get_wbi_sign2(params.clone()).await?;
        assert!(wbi.iter().any(|(key, _)| key == "w_rid"));

        let wbi = bpi.get_wbi_sign2(params).await?;
        assert!(wbi.iter().any(|(key, _)| key == "wts"));
        Ok(())
    }
}

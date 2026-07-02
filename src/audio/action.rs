//! 音频投币&收藏
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/audio/action.md)
//!

use crate::audio::params::AudioSongParams;
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptData {
    /// 是否为未关注用户收藏
    prompt: bool,
}

impl BpiClient {
    /// 收藏音频到收藏夹(同视频收藏夹)
    ///
    /// # 参数
    /// | 名称   | 类型 | 说明       |
    /// | ------ | ---- | ---------- |
    /// | `rid`  | u64  | 音频 auid  |
    /// | `add_media_ids` | `Vec<&str>`|添加的合集ids|
    /// | `del_media_ids` | `Vec<&str>`|从中删除的合集ids|
    ///
    /// 与视频收藏几乎一样
    pub async fn audio_collection_to_fav(
        &self,
        rid: u64,
        add_media_ids: Option<Vec<&str>>,
        del_media_ids: Option<Vec<&str>>,
    ) -> Result<BpiResponse<PromptData>, BpiError> {
        if add_media_ids.is_none() && del_media_ids.is_none() {
            return Err(BpiError::InvalidParameter {
                field: "media_ids",
                message: "请至少指定一个操作",
            });
        }
        let csrf = self.csrf()?;
        let mut params = HashMap::new();

        params.extend([
            ("rid", rid.to_string()),
            ("type", "12".to_string()),
            ("csrf", csrf),
        ]);

        if let Some(ids) = add_media_ids {
            let s = ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.insert("add_media_ids", s);
        }
        if let Some(ids) = del_media_ids {
            let s = ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.insert("del_media_ids", s);
        }
        let result = self
            .get("https://api.bilibili.com/medialist/gateway/coll/resource/deal")
            .form(&params)
            .send_bpi("收藏音频到收藏夹")
            .await?;
        Ok(result)
    }

    /// 查询音频收藏状态
    ///
    /// # 参数
    /// | 名称   | 类型 | 说明       |
    /// | ------ | ---- | ---------- |
    /// | `sid`  | u64  | 音频 auid  |
    /// | `cids` | u64  | 歌单 id    |
    ///
    /// # 返回
    /// | 值       | 说明     |
    /// | -------- | -------- |
    /// | `true`   | 操作成功?   |
    pub async fn audio_collection_to(
        &self,
        sid: u64,
        cids: u64,
    ) -> Result<BpiResponse<bool>, BpiError> {
        let csrf = self.csrf()?;

        let result = self
            .get("https://www.bilibili.com/audio/music-service-c/web/collections/songs-coll")
            .form(&[
                ("sid", sid.to_string()),
                ("cids", cids.to_string()),
                ("csrf", csrf),
            ])
            .send_bpi("收藏音频到歌单")
            .await?;
        Ok(result)
    }

    /// 查询音频收藏状态
    ///
    /// # 参数
    /// | 名称   | 类型 | 说明       |
    /// | ------ | ---- | ---------- |
    /// | `sid`  | u64  | 音频 auid  |
    ///
    /// # 返回
    /// | 值       | 说明     |
    /// | -------- | -------- |
    /// | `true`   | 已收藏   |
    /// | `false`  | 未收藏   |
    ///
    /// # 文档
    /// [查询音频收藏状态](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/audio/action.md#查询音频收藏状态)
    pub async fn audio_collection_status(
        &self,
        params: AudioSongParams,
    ) -> Result<BpiResponse<bool>, BpiError> {
        let result = self
            .get("https://www.bilibili.com/audio/music-service-c/web/collections/songs-coll")
            .query(&params.query_pairs())
            .send_bpi("查询音频收藏状态")
            .await?;
        Ok(result)
    }

    /// 查询音频投币数
    ///
    /// # 参数
    /// | 名称     | 类型              | 说明           |
    /// | -------- | ----------------- | -------------- |
    /// | `params` | `AudioSongParams` | 音频 auid 参数 |
    ///
    /// # 返回
    /// 投币数量，`0` 为未投币，上限为 `2`
    ///
    /// # 文档
    /// [查询音频投币数](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/audio/action.md#查询音频投币数)
    pub async fn audio_coin_count(
        &self,
        params: AudioSongParams,
    ) -> Result<BpiResponse<i32>, BpiError> {
        let result = self
            .get("https://www.bilibili.com/audio/music-service-c/web/coin/audio")
            .query(&params.query_pairs())
            .send_bpi("查询音频投币数")
            .await?;
        Ok(result)
    }

    /// 投币音频
    ///
    /// # 参数
    /// | 名称       | 类型 | 说明                  |
    /// | ---------- | ---- | --------------------- |
    /// | `sid`      | u64  | 音频 auid             |
    /// | `multiply` | i32  | 投币数量（最大为 `2`）|
    ///
    /// # 返回
    /// 当前投币数量
    ///
    /// # 文档
    /// [投币音频](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/audio/action.md#投币音频)
    pub async fn audio_coin(
        &self,
        sid: u64,
        multiply: u32,
    ) -> Result<BpiResponse<String>, BpiError> {
        let multiply = normalize_audio_coin_multiply(multiply);
        let csrf = self.csrf()?;
        self.post("https://www.bilibili.com/audio/music-service-c/web/coin/add")
            .form(&[
                ("sid", sid.to_string()),
                ("multiply", multiply.to_string()),
                ("csrf", csrf),
            ])
            .send_bpi("投币音频")
            .await
    }
}

fn normalize_audio_coin_multiply(multiply: u32) -> u32 {
    multiply.clamp(1, 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::params::AudioSongParams;
    use crate::ids::AudioId;

    // https://www.bilibili.com/audio/au13598

    const TEST_SID: u64 = 13603;

    #[test]
    fn normalize_audio_coin_multiply_clamps_to_supported_range() {
        assert_eq!(normalize_audio_coin_multiply(0), 1);
        assert_eq!(normalize_audio_coin_multiply(1), 1);
        assert_eq!(normalize_audio_coin_multiply(2), 2);
        assert_eq!(normalize_audio_coin_multiply(3), 2);
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_audio_collection_status() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new().expect("client should build");
        let result = bpi
            .audio_collection_status(AudioSongParams::new(AudioId::new(TEST_SID)?))
            .await?;

        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_audio_coin_count() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new().expect("client should build");
        let result = bpi
            .audio_coin_count(AudioSongParams::new(AudioId::new(TEST_SID)?))
            .await?;

        let data = result.data.unwrap();
        assert!((0..=2).contains(&data));

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_coin_audio() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new().expect("client should build");
        bpi.audio_coin(TEST_SID, 1).await.map(|_| ()).or_else(|e| {
            // 34005 代表投币已经投过2个了  API错误 [34005]: 未知错误
            if e.code() == Some(34005) {
                Ok(())
            } else {
                Err(Box::new(e))
            }
        })
    }
}

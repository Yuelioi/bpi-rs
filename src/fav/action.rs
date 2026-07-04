use super::info::FavFolderInfo;
use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResponse;
use crate::fav::FavClient;

impl<'a> FavClient<'a> {
    /// 新建收藏夹
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `title` | &str | 收藏夹标题 |
    /// | `intro` | `Option<&str>` | 介绍 |
    /// | `privacy` | `Option<u8>` | 0 公开，1 私密 |
    /// | `cover` | `Option<&str>` | 封面 URL |
    pub async fn fav_folder_add(
        &self,
        title: &str,
        intro: Option<&str>,
        privacy: Option<u8>,
        cover: Option<&str>,
    ) -> Result<BpiResponse<FavFolderInfo>, BpiError> {
        let csrf = self.client.csrf()?;

        let mut form = vec![("title", title.to_string()), ("csrf", csrf)];
        if let Some(intro) = intro {
            form.push(("intro", intro.to_string()));
        }
        if let Some(privacy) = privacy {
            form.push(("privacy", privacy.to_string()));
        }
        if let Some(cover) = cover {
            form.push(("cover", cover.to_string()));
        }

        self.client
            .post("https://api.bilibili.com/x/v3/fav/folder/add")
            .form(&form)
            .send_bpi("新建收藏夹")
            .await
    }

    /// 修改收藏夹
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `media_id` | u64 | 收藏夹 media_id |
    /// | `title` | &str | 标题 |
    /// | `intro` | `Option<&str>` | 介绍 |
    /// | `privacy` | `Option<u8>` | 0 公开，1 私密 |
    /// | `cover` | `Option<&str>` | 封面 URL |
    pub async fn fav_folder_edit(
        &self,
        media_id: u64,
        title: &str,
        intro: Option<&str>,
        privacy: Option<u8>,
        cover: Option<&str>,
    ) -> Result<BpiResponse<FavFolderInfo>, BpiError> {
        let csrf = self.client.csrf()?;

        let mut form = vec![
            ("media_id", media_id.to_string()),
            ("title", title.to_string()),
            ("csrf", csrf),
        ];
        if let Some(intro) = intro {
            form.push(("intro", intro.to_string()));
        }
        if let Some(privacy) = privacy {
            form.push(("privacy", privacy.to_string()));
        }
        if let Some(cover) = cover {
            form.push(("cover", cover.to_string()));
        }

        self.client
            .post("https://api.bilibili.com/x/v3/fav/folder/edit")
            .form(&form)
            .send_bpi("修改收藏夹")
            .await
    }

    /// 删除收藏夹
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `media_ids` | &`[u64]` | 多个收藏夹 media_id |
    pub async fn fav_folder_del(&self, media_ids: &[u64]) -> Result<BpiResponse<i32>, BpiError> {
        let csrf = self.client.csrf()?;
        let ids_str = media_ids
            .iter()
            .map(|&id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let form = [("media_ids", ids_str), ("csrf", csrf)];

        self.client
            .post("https://api.bilibili.com/x/v3/fav/folder/del")
            .form(&form)
            .send_bpi("删除收藏夹")
            .await
    }

    /// 批量复制内容
    /// `resources`: "{内容id}:{内容类型},..."
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `src_media_id` | u64 | 源收藏夹 media_id |
    /// | `tar_media_id` | u64 | 目标收藏夹 media_id |
    /// | `mid` | u64 | 用户 mid |
    /// | `resources` | &str | 形如 "{内容id}:{内容类型},..." |
    pub async fn fav_resource_copy(
        &self,
        src_media_id: u64,
        tar_media_id: u64,
        mid: u64,
        resources: &str,
    ) -> Result<BpiResponse<i32>, BpiError> {
        let csrf = self.client.csrf()?;

        let form = [
            ("src_media_id", src_media_id.to_string()),
            ("tar_media_id", tar_media_id.to_string()),
            ("mid", mid.to_string()),
            ("resources", resources.to_string()),
            ("platform", "web".to_string()),
            ("csrf", csrf),
        ];

        self.client
            .post("https://api.bilibili.com/x/v3/fav/resource/copy")
            .form(&form)
            .send_bpi("批量复制内容")
            .await
    }

    /// 批量移动内容
    /// `resources`: "{内容id}:{内容类型},..."
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `src_media_id` | u64 | 源收藏夹 media_id |
    /// | `tar_media_id` | u64 | 目标收藏夹 media_id |
    /// | `mid` | u64 | 用户 mid |
    /// | `resources` | &str | 形如 "{内容id}:{内容类型},..." |
    pub async fn fav_resource_move(
        &self,
        src_media_id: u64,
        tar_media_id: u64,
        mid: u64,
        resources: &str,
    ) -> Result<BpiResponse<i32>, BpiError> {
        let csrf = self.client.csrf()?;

        let form = [
            ("src_media_id", src_media_id.to_string()),
            ("tar_media_id", tar_media_id.to_string()),
            ("mid", mid.to_string()),
            ("resources", resources.to_string()),
            ("platform", "web".to_string()),
            ("csrf", csrf),
        ];

        self.client
            .post("https://api.bilibili.com/x/v3/fav/resource/move")
            .form(&form)
            .send_bpi("批量移动内容")
            .await
    }

    /// 批量删除内容
    /// `resources`: "{内容id}:{内容类型},..."
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `media_id` | u64 | 收藏夹 media_id |
    /// | `resources` | &str | 形如 "{内容id}:{内容类型},..." |
    pub async fn fav_resource_batch_del(
        &self,
        media_id: u64,
        resources: &str,
    ) -> Result<BpiResponse<i32>, BpiError> {
        let csrf = self.client.csrf()?;

        let form = [
            ("media_id", media_id.to_string()),
            ("resources", resources.to_string()),
            ("platform", "web".to_string()),
            ("csrf", csrf),
        ];

        self.client
            .post("https://api.bilibili.com/x/v3/fav/resource/batch-del")
            .form(&form)
            .send_bpi("批量删除内容")
            .await
    }

    /// 清空所有失效内容
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `media_id` | u64 | 收藏夹 media_id |
    pub async fn fav_resource_clean(&self, media_id: u64) -> Result<BpiResponse<i32>, BpiError> {
        let csrf = self.client.csrf()?;

        let form = [("media_id", media_id.to_string()), ("csrf", csrf)];

        self.client
            .post("https://api.bilibili.com/x/v3/fav/resource/clean")
            .form(&form)
            .send_bpi("清空所有失效内容")
            .await
    }
}

#[cfg(test)]
mod tests {}

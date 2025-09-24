use super::info::FavFolderInfo;
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

impl BpiClient {
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
        cover: Option<&str>
    ) -> Result<BpiResponse<FavFolderInfo>, BpiError> {
        let csrf = self.csrf()?;

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

        self
            .post("https://api.bilibili.com/x/v3/fav/folder/add")
            .form(&form)
            .send_bpi("新建收藏夹").await
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
        cover: Option<&str>
    ) -> Result<BpiResponse<FavFolderInfo>, BpiError> {
        let csrf = self.csrf()?;

        let mut form = vec![
            ("media_id", media_id.to_string()),
            ("title", title.to_string()),
            ("csrf", csrf)
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

        self
            .post("https://api.bilibili.com/x/v3/fav/folder/edit")
            .form(&form)
            .send_bpi("修改收藏夹").await
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
        let csrf = self.csrf()?;
        let ids_str = media_ids
            .iter()
            .map(|&id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let form = [
            ("media_ids", ids_str),
            ("csrf", csrf),
        ];

        self
            .post("https://api.bilibili.com/x/v3/fav/folder/del")
            .form(&form)
            .send_bpi("删除收藏夹").await
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
        resources: &str
    ) -> Result<BpiResponse<i32>, BpiError> {
        let csrf = self.csrf()?;

        let form = [
            ("src_media_id", src_media_id.to_string()),
            ("tar_media_id", tar_media_id.to_string()),
            ("mid", mid.to_string()),
            ("resources", resources.to_string()),
            ("platform", "web".to_string()),
            ("csrf", csrf),
        ];

        self
            .post("https://api.bilibili.com/x/v3/fav/resource/copy")
            .form(&form)
            .send_bpi("批量复制内容").await
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
        resources: &str
    ) -> Result<BpiResponse<i32>, BpiError> {
        let csrf = self.csrf()?;

        let form = [
            ("src_media_id", src_media_id.to_string()),
            ("tar_media_id", tar_media_id.to_string()),
            ("mid", mid.to_string()),
            ("resources", resources.to_string()),
            ("platform", "web".to_string()),
            ("csrf", csrf),
        ];

        self
            .post("https://api.bilibili.com/x/v3/fav/resource/move")
            .form(&form)
            .send_bpi("批量移动内容").await
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
        resources: &str
    ) -> Result<BpiResponse<i32>, BpiError> {
        let csrf = self.csrf()?;

        let form = [
            ("media_id", media_id.to_string()),
            ("resources", resources.to_string()),
            ("platform", "web".to_string()),
            ("csrf", csrf),
        ];

        self
            .post("https://api.bilibili.com/x/v3/fav/resource/batch-del")
            .form(&form)
            .send_bpi("批量删除内容").await
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
        let csrf = self.csrf()?;

        let form = [
            ("media_id", media_id.to_string()),
            ("csrf", csrf),
        ];

        self
            .post("https://api.bilibili.com/x/v3/fav/resource/clean")
            .form(&form)
            .send_bpi("清空所有失效内容").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_fav_folder_add_and_del() {
        let bpi = BpiClient::new();
        let title = "Test_Fav_Folder_Add2";
        let intro = "This is a test folder2.";

        // 1. 新建一个私密收藏夹
        let add_resp = bpi.fav_folder_add(title, Some(intro), Some(1), None).await;
        info!("Add folder result: {:?}", add_resp);
        assert!(add_resp.is_ok());

        let new_folder_id = add_resp.unwrap().data.unwrap().id;
        info!("New folder ID: {}", new_folder_id);

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        // 2. 删除新建的收藏夹
        let del_resp = bpi.fav_folder_del(&[new_folder_id]).await;
        info!("Delete folder result: {:?}", del_resp);
        assert!(del_resp.is_ok());
    }

    #[tokio::test]
    async fn test_fav_folder_edit() {
        let bpi = BpiClient::new();
        // 替换为你的测试收藏夹ID
        let media_id = 3717139570;
        let title = "Edited Title";
        let intro = "Edited Intro";

        let resp = bpi.fav_folder_edit(media_id, title, Some(intro), Some(0), None).await;
        info!("Edit folder result: {:?}", resp);
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn test_fav_resource_operations() {
        let bpi = BpiClient::new();
        // 替换为你的源/目标收藏夹ID和资源ID
        let src_media_id = 3717139570;
        let tar_media_id = 3641682570;
        let mid = 4279370;
        let resources_copy = "115087859779103:2";
        let resources_del = "442608504:2";
        let resources_move = "739661210:2";

        // 1. 批量复制
        let copy_resp = bpi.fav_resource_copy(
            src_media_id,
            tar_media_id,
            mid,
            resources_copy
        ).await;
        info!("Copy resources result: {:?}", copy_resp);
        assert!(copy_resp.is_ok());

        // 2. 批量移动
        let move_resp = bpi.fav_resource_move(
            src_media_id,
            tar_media_id,
            mid,
            resources_move
        ).await;
        info!("Move resources result: {:?}", move_resp);
        assert!(move_resp.is_ok());

        // 3. 批量删除
        let del_resp = bpi.fav_resource_batch_del(tar_media_id, resources_del).await;
        info!("Batch delete resources result: {:?}", del_resp);
        assert!(del_resp.is_ok());
    }

    #[tokio::test]
    async fn test_fav_resource_clean() {
        let bpi = BpiClient::new();
        // 替换为你的测试收藏夹ID
        let media_id = 3717139570;
        let resp = bpi.fav_resource_clean(media_id).await;
        info!("Clean invalid resources result: {:?}", resp);
        assert!(resp.is_ok());
    }
}

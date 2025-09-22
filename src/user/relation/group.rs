//! B站用户分组相关接口
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// --- 响应数据结构体 ---

/// 创建分组响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateTagResponseData {
    /// 创建的分组的 ID
    pub tagid: i64,
}

// --- API 实现 ---

impl BpiClient {
    /// 创建分组
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user/relation/group#创建分组
    ///
    /// # 参数
    /// | 名称      | 类型         | 说明           |
    /// | --------- | ------------| -------------- |
    /// | `group_name` | &str      | 分组名，最长16字|
    pub async fn user_group_create_tag(
        &self,
        group_name: &str
    ) -> Result<BpiResponse<CreateTagResponseData>, BpiError> {
        let csrf = self.csrf()?;
        let form = reqwest::multipart::Form
            ::new()
            .text("tag", group_name.to_string())
            .text("csrf", csrf.to_string());

        self
            .post("https://api.bilibili.com/x/relation/tag/create")
            .multipart(form)
            .send_bpi("创建分组").await
    }

    /// 重命名分组
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user/relation/group#重命名分组
    ///
    /// # 参数
    /// | 名称      | 类型         | 说明           |
    /// | --------- | ------------| -------------- |
    /// | `tag_id`  | i64         | 分组ID         |
    /// | `new_name`| &str        | 新名称，最长16字|
    pub async fn user_group_update_tag(
        &self,
        tag_id: i64,
        new_name: &str
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let form = reqwest::multipart::Form
            ::new()
            .text("tagid", tag_id.to_string())
            .text("name", new_name.to_string())
            .text("csrf", csrf.to_string());

        self
            .post("https://api.bilibili.com/x/relation/tag/update")
            .multipart(form)
            .send_bpi("重命名分组").await
    }

    /// 删除分组
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user/relation/group#删除分组
    ///
    /// # 参数
    /// | 名称      | 类型         | 说明           |
    /// | --------- | ------------| -------------- |
    /// | `tag_id`  | i64         | 分组ID         |
    pub async fn user_group_delete_tag(
        &self,
        tag_id: i64
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let form = reqwest::multipart::Form
            ::new()
            .text("tagid", tag_id.to_string())
            .text("csrf", csrf.to_string());

        self
            .post("https://api.bilibili.com/x/relation/tag/del")
            .multipart(form)
            .send_bpi("删除分组").await
    }

    /// 修改分组成员（添加）
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user/relation/group#修改分组成员
    ///
    /// # 参数
    /// | 名称      | 类型         | 说明           |
    /// | --------- | ------------| -------------- |
    /// | `fids`    | &[u64]      | 目标用户 mid 列表|
    /// | `tagids`  | &[i64]      | 分组ID列表      |
    pub async fn user_group_add_users_to_tags(
        &self,
        fids: &[u64],
        tagids: &[i64]
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let fids_str = fids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let tagids_str = tagids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let form = reqwest::multipart::Form
            ::new()
            .text("fids", fids_str)
            .text("tagids", tagids_str)
            .text("csrf", csrf.to_string());

        self
            .post("https://api.bilibili.com/x/relation/tags/addUsers")
            .multipart(form)
            .send_bpi("修改分组成员").await
    }

    // 修改分组成员（删除）
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user/relation/group#修改分组成员
    ///
    /// # 参数
    /// | 名称      | 类型         | 说明           |
    /// | --------- | ------------| -------------- |
    /// | `fids`    | &[u64]      | 目标用户 mid 列表|
    pub async fn user_group_remove_users_(
        &self,
        fids: &[u64]
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let fids_str = fids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let form = reqwest::multipart::Form
            ::new()
            .text("fids", fids_str)
            .text("tagids", "0".to_string())
            .text("csrf", csrf.to_string());

        self
            .post("https://api.bilibili.com/x/relation/tags/addUsers")
            .multipart(form)
            .send_bpi("修改分组成员").await
    }

    /// 复制关注到分组
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user/relation/group#复制关注到分组
    ///
    /// # 参数
    /// | 名称      | 类型         | 说明           |
    /// | --------- | ------------| -------------- |
    /// | `fids`    | &[u64]      | 用户 mid 列表   |
    /// | `tagids`  | &[i64]      | 目标分组ID列表  |
    pub async fn user_group_copy_users_to_tags(
        &self,
        fids: &[u64],
        tagids: &[i64]
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let fids_str = fids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let tagids_str = tagids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let form = reqwest::multipart::Form
            ::new()
            .text("fids", fids_str)
            .text("tagids", tagids_str)
            .text("csrf", csrf.to_string());

        self
            .post("https://api.bilibili.com/x/relation/tags/copyUsers")
            .multipart(form)
            .send_bpi("复制关注到分组").await
    }

    /// 移动关注到分组
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user/relation/group#移动关注到分组
    ///
    /// # 参数
    /// | 名称            | 类型         | 说明           |
    /// | --------------- | ------------| -------------- |
    /// | `fids`          | &[u64]      | 用户 mid 列表   |
    /// | `before_tag_ids`| &[i64]      | 原分组ID列表    |
    /// | `after_tag_ids` | &[i64]      | 新分组ID列表    |
    pub async fn user_group_move_users_to_tags(
        &self,
        fids: &[u64],
        before_tag_ids: &[i64],
        after_tag_ids: &[i64]
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let fids_str = fids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let before_tag_ids_str = before_tag_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let after_tag_ids_str = after_tag_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let form = reqwest::multipart::Form
            ::new()
            .text("fids", fids_str)
            .text("beforeTagids", before_tag_ids_str)
            .text("afterTagids", after_tag_ids_str)
            .text("csrf", csrf.to_string());

        self
            .post("https://api.bilibili.com/x/relation/tags/moveUsers")
            .multipart(form)
            .send_bpi("移动关注到分组").await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    const TEST_FID: u64 = 107997089;

    const DEFAULT_GROUP_FID: u64 = 3493257409464519;

    #[tokio::test]
    async fn test_tag_operations() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let test_tag_name = "测试分组1";
        let new_tag_name = "新测试分组2";

        // 1. 创建分组
        info!("正在创建分组...");
        let create_resp = bpi.user_group_create_tag(&test_tag_name).await?;
        let tag_id = create_resp.into_data()?.tagid;
        info!("创建分组成功，ID: {}", tag_id);

        // 2. 重命名分组
        info!("正在重命名分组...");
        let update_resp = bpi.user_group_update_tag(tag_id, &new_tag_name).await?;
        info!("重命名分组成功");
        assert_eq!(update_resp.code, 0);

        // 3. 修改分组成员
        info!("正在将用户添加到默认分组...");
        let add_resp = bpi.user_group_add_users_to_tags(&[TEST_FID], &[-10]).await?;
        info!("添加用户到默认分组成功");
        assert_eq!(add_resp.code, 0);

        // 5. 移动关注到分组
        // 假设存在一个默认分组（tagid=-10）
        info!("正在将用户从默认分组移动到新分组...");
        let move_resp = bpi.user_group_move_users_to_tags(
            &[DEFAULT_GROUP_FID],
            &[-10],
            &[tag_id]
        ).await?;
        info!("移动用户到分组成功");
        assert_eq!(move_resp.code, 0);

        // 4. 复制关注到分组
        info!("正在将用户复制到分组...");
        let copy_resp = bpi.user_group_copy_users_to_tags(&[TEST_FID], &[tag_id]).await?;
        info!("复制用户到分组成功");
        assert_eq!(copy_resp.code, 0);

        // 6. 删除分组
        info!("正在删除分组...");
        let delete_resp = bpi.user_group_delete_tag(tag_id).await?;
        info!("删除分组成功");
        assert_eq!(delete_resp.code, 0);

        Ok(())
    }
}

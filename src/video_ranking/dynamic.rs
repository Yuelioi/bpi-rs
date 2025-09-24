use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// --- 获取分区最新视频列表 ---

/// 分区最新视频的页面信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegionPage {
    /// 总计视频数
    pub count: u32,
    /// 当前页码
    pub num: u32,
    /// 每页项数
    pub size: u32,
}

/// 分区最新视频列表的数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegionArchivesData {
    /// 视频列表
    pub archives: Vec<serde_json::Value>, // archives内容复杂，这里用Value代替
    /// 页面信息
    pub page: RegionPage,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewListRankResult {
    /// 发布时间
    #[serde(rename = "pubdate")]
    pub pub_date: String,
    /// 封面图
    pub pic: String,
    /// 标签
    pub tag: String,
    /// 时长 (秒)
    pub duration: u32,
    /// avid
    pub id: u64,
    /// 排序分数
    pub rank_score: Option<u64>,
    /// 是否有角标
    pub badgepay: bool,
    /// 发送时间 (UNIX 秒级时间戳)
    pub senddate: Option<u64>,
    /// UP 主名
    pub author: String,
    /// 评论数
    pub review: u64,
    /// UP 主 mid
    pub mid: u64,
    /// 是否为联合投稿
    pub is_union_video: u8,
    /// 排序索引号
    pub rank_index: Option<u64>,
    /// 类型
    #[serde(rename = "type")]
    pub type_name: String,
    /// 播放数
    pub play: String,
    /// 弹幕数
    #[serde(rename = "video_review")]
    pub video_review: u64,
    /// 是否付费
    pub is_pay: u8,
    /// 收藏数
    pub favorites: u64,
    /// 视频播放页 URL
    pub arcurl: String,
    /// bvid
    pub bvid: String,
    /// 标题
    pub title: String,
    /// 简介
    pub description: String,
    // 忽略其他作用不明确的字段
}

/// 带排序的分区投稿列表数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewListRankData {
    /// 结果本体
    pub result: Option<Vec<NewListRankResult>>,
    /// 总计视频数
    #[serde(rename = "numResults")]
    pub num_results: u32,
    /// 页码
    pub page: u32,
    /// 视频数
    pub pagesize: u32,
    /// 结果信息
    pub msg: String,
}

impl BpiClient {
    /// 获取分区最新视频列表
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/video_ranking/dynamic.html#获取分区最新视频列表)
    ///
    /// # 参数
    /// | 名称   | 类型         | 说明                 |
    /// | ------ | ------------| -------------------- |
    /// | `rid`  | u32         | 分区ID               |
    /// | `pn`   | `Option<u32>` | 页码，可选           |
    /// | `ps`   | `Option<u32>` | 每页数量，可选       |
    pub async fn video_region_dynamic(
        &self,
        rid: u32,
        pn: Option<u32>,
        ps: Option<u32>
    ) -> Result<BpiResponse<RegionArchivesData>, BpiError> {
        let mut request = self
            .get("https://api.bilibili.com/x/web-interface/dynamic/region")
            .query(&[("rid", rid.to_string())]);

        if let Some(pn) = pn {
            request = request.query(&[("pn", pn.to_string())]);
        }
        if let Some(ps) = ps {
            request = request.query(&[("ps", ps.to_string())]);
        }

        request.send_bpi("获取分区最新视频列表").await
    }

    /// 获取分区标签近期互动列表
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/video_ranking/dynamic.html#获取分区标签近期互动列表)
    ///
    /// # 参数
    /// | 名称    | 类型         | 说明                 |
    /// | ------- | ------------| -------------------- |
    /// | `rid`   | u32         | 分区ID               |
    /// | `tag_id`| u64         | 标签ID               |
    /// | `pn`    | `Option<u32>` | 页码，可选           |
    /// | `ps`    | `Option<u32>` | 每页数量，可选       |
    pub async fn video_region_tag_dynamic(
        &self,
        rid: u32,
        tag_id: u64,
        pn: Option<u32>,
        ps: Option<u32>
    ) -> Result<BpiResponse<RegionArchivesData>, BpiError> {
        let mut request = self.get("https://api.bilibili.com/x/web-interface/dynamic/tag").query(
            &[
                ("rid", rid.to_string()),
                ("tag_id", tag_id.to_string()),
            ]
        );

        if let Some(pn) = pn {
            request = request.query(&[("pn", pn.to_string())]);
        }
        if let Some(ps) = ps {
            request = request.query(&[("ps", ps.to_string())]);
        }

        request.send_bpi("获取分区标签近期互动列表").await
    }

    /// 获取分区近期投稿列表
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/video_ranking/dynamic.html#获取分区近期投稿列表)
    ///
    /// # 参数
    /// | 名称   | 类型         | 说明                 |
    /// | ------ | ------------| -------------------- |
    /// | `rid`  | u32         | 分区ID               |
    /// | `pn`   | `Option<u32>` | 页码，可选           |
    /// | `ps`   | `Option<u32>` | 每页数量，可选       |
    /// | `typ`  | `Option<u32>` | 类型，可选           |
    pub async fn video_region_newlist(
        &self,
        rid: u32,
        pn: Option<u32>,
        ps: Option<u32>,
        typ: Option<u32>
    ) -> Result<BpiResponse<RegionArchivesData>, BpiError> {
        let mut request = self
            .get("https://api.bilibili.com/x/web-interface/newlist")
            .query(&[("rid", rid.to_string())]);

        if let Some(pn) = pn {
            request = request.query(&[("pn", pn.to_string())]);
        }
        if let Some(ps) = ps {
            request = request.query(&[("ps", ps.to_string())]);
        }
        if let Some(t) = typ {
            request = request.query(&[("type", t.to_string())]);
        }

        request.send_bpi("获取分区近期投稿列表").await
    }

    /// 获取分区近期投稿列表（带排序）
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/video_ranking/dynamic.html#获取分区近期投稿列表带排序)
    ///
    /// # 参数
    /// | 名称        | 类型           | 说明                 |
    /// | ----------- | --------------| -------------------- |
    /// | `cate_id`   | u32           | 分类ID               |
    /// | `order`     | `Option<&str>`  | 排序方式，可选       |
    /// | `page`      | `Option<u32>`   | 页码，可选           |
    /// | `pagesize`  | u32           | 每页数量             |
    /// | `time_from` | &str          | 起始日期(YYYYMMDD)   |
    /// | `time_to`   | &str          | 结束日期(YYYYMMDD)   |
    pub async fn video_region_newlist_rank(
        &self,
        cate_id: u32,
        order: Option<&str>,
        page: Option<u32>,
        pagesize: u32,
        time_from: &str,
        time_to: &str
    ) -> Result<BpiResponse<NewListRankData>, BpiError> {
        let cate_id = cate_id.to_string();
        let pagesize = pagesize.to_string();
        let mut request = self.get("https://api.bilibili.com/x/web-interface/newlist_rank").query(
            &[
                ("search_type", "video"),
                ("view_type", "hot_rank"),
                ("cate_id", cate_id.as_str()),
                ("pagesize", pagesize.as_str()),
                ("time_from", time_from),
                ("time_to", time_to),
            ]
        );

        if let Some(o) = order {
            request = request.query(&[("order", o)]);
        }
        if let Some(p) = page {
            request = request.query(&[("page", p.to_string())]);
        }

        request.send_bpi("获取分区近期投稿列表 (带排序)").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{ Duration, Local };
    use tracing::info;

    #[tokio::test]
    async fn test_video_region_dynamic() {
        let bpi = BpiClient::new();
        let rid = 21; // 日常分区
        let ps = Some(2);
        let pn = Some(1);
        let resp = bpi.video_region_dynamic(rid, pn, ps).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("total videos: {}", data.page.count);
            info!("first item: {:?}", data.archives.first());
        }
    }

    #[tokio::test]
    async fn test_video_region_tag_dynamic() {
        let bpi = BpiClient::new();
        let rid = 136; // 音游分区
        let tag_id = 10026108; // Phigros
        let ps = Some(2);
        let pn = Some(1);
        let resp = bpi.video_region_tag_dynamic(rid, tag_id, pn, ps).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("total videos: {}", data.page.count);
            info!("first item: {:?}", data.archives.first());
        }
    }

    #[tokio::test]
    async fn test_video_region_newlist_rank() {
        let bpi = BpiClient::new();
        let cate_id = 231; // 计算机技术
        let pagesize = 2;
        let today = Local::now().date_naive();
        let seven_days_ago = today - Duration::days(7);

        let time_from = seven_days_ago.format("%Y%m%d").to_string();
        let time_to = today.format("%Y%m%d").to_string();

        let resp = bpi.video_region_newlist_rank(
            cate_id,
            Some("click"),
            Some(1),
            pagesize,
            &time_from,
            &time_to
        ).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("total results: {}", data.num_results);
            info!("first result: {:?}", data.result.unwrap().first());
        }
    }
}

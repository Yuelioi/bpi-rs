use chrono::NaiveDate;
use serde::{ Deserialize, Serialize };

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

/// 发送充电留言的请求体
#[derive(Debug, Clone, Serialize)]
pub struct SendElecMessageBody<'a> {
    pub order_id: &'a str,
    pub message: &'a str,
    pub csrf: &'a str,
}

/// 充电留言列表分页信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ElecRemarkPager {
    /// 当前页数
    pub current: u64,
    /// 当前分页大小
    pub size: u64,
    /// 记录总数
    pub total: u64,
}

/// 充电留言列表中的单条留言
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ElecRemarkRecord {
    pub aid: u64,
    pub bvid: String,
    pub id: u64,
    pub mid: u64,
    pub reply_mid: u64,
    pub elec_num: u64,
    /// UP是否已经回复这条留言 0: 未回复 1: 已回复
    pub state: u8,
    /// 留言信息
    pub msg: String,
    pub aname: String,
    pub uname: String,
    pub avator: String,
    pub reply_name: String,
    pub reply_avator: String,
    pub reply_msg: String,
    /// 留言时间毫秒级时间戳
    pub ctime: u64,
    pub reply_time: u64,
}

/// 充电留言列表数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ElecRemarkList {
    pub list: Vec<ElecRemarkRecord>,
    pub pager: ElecRemarkPager,
}

/// 充电留言详情数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ElecRemarkDetail {
    pub aid: u64,
    pub bvid: String,
    pub id: u64,
    /// 留言者mid（充电用户）
    pub mid: u64,
    /// UP主mid
    pub reply_mid: u64,
    pub elec_num: u64,
    /// UP是否已经回复这条留言 0: 未回复 1: 已回复
    pub state: u8,
    /// 留言内容
    pub msg: String,
    pub aname: String,
    /// 留言者用户名
    pub uname: String,
    /// 留言者头像
    pub avator: String,
    /// UP主用户名
    pub reply_name: String,
    /// UP主头像
    pub reply_avator: String,
    /// 回复内容
    pub reply_msg: String,
    /// 留言时间毫秒级时间戳
    pub ctime: u64,
    /// 回复时间毫秒级时间戳
    pub reply_time: u64,
}

impl BpiClient {
    /// 发送充电留言
    ///
    /// 注意: 此接口需要登录态 (Cookie: SESSDATA)
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/electric)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `order_id` | &str | 留言 token |
    /// | `message` | &str | 留言内容 |
    pub async fn electric_message_send(
        &self,
        order_id: &str,
        message: &str
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let body = [
            ("order_id", order_id),
            ("message", message),
            ("csrf", &csrf),
        ];

        self
            .post("https://api.bilibili.com/x/ugcpay/trade/elec/message")
            .form(&body)
            .send_bpi("发送充电留言").await
    }

    /// 查询我收到的充电留言
    ///
    /// 注意: 此接口需要登录态 (Cookie: SESSDATA)
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/electric)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `pn` | `Option<u64>` | 页数，默认 1 |
    /// | `ps` | `Option<u64>` | 分页大小，默认 10，范围 `[1,12]` |
    /// | `begin` | `Option<NaiveDate>` | 开始日期 YYYY-MM-DD |
    /// | `end` | `Option<NaiveDate>` | 结束日期 YYYY-MM-DD |
    pub async fn electric_remark_list(
        &self,
        pn: Option<u64>,
        ps: Option<u64>,
        begin: Option<NaiveDate>,
        end: Option<NaiveDate>
    ) -> Result<BpiResponse<ElecRemarkList>, BpiError> {
        let mut req = self.get("https://member.bilibili.com/x/web/elec/remark/list");

        if let Some(page) = pn {
            req = req.query(&[("pn", page)]);
        }
        if let Some(size) = ps {
            req = req.query(&[("ps", size)]);
        }
        if let Some(begin_date) = begin {
            req = req.query(&[("begin", begin_date.format("%Y-%m-%d").to_string())]);
        }
        if let Some(end_date) = end {
            req = req.query(&[("end", end_date.format("%Y-%m-%d").to_string())]);
        }

        req.send_bpi("查询收到的充电留言").await
    }

    /// 查询充电留言详情
    ///
    /// 注意: 此接口需要登录态 (Cookie: SESSDATA)
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/electric)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `id` | u64 | 留言 id |
    pub async fn electric_remark_detail(
        &self,
        id: u64
    ) -> Result<BpiResponse<ElecRemarkDetail>, BpiError> {
        self
            .get("https://member.bilibili.com/x/web/elec/remark/detail")
            .query(&[("id", id)])
            .send_bpi("查询充电留言详情").await
    }

    /// 回复充电留言
    ///
    /// 注意: 此接口需要登录态 (Cookie: SESSDATA)
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/electric)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `id` | u64 | 留言 id |
    /// | `msg` | &str | 回复内容 |
    pub async fn electric_remark_reply(
        &self,
        id: u64,
        msg: &str
    ) -> Result<BpiResponse<u64>, BpiError> {
        let csrf = self.csrf()?;

        let body = [
            ("id", id.to_string()),
            ("msg", msg.to_string()),
            ("csrf", csrf.to_string()),
        ];

        self
            .post("https://member.bilibili.com/x/web/elec/remark/reply")
            .form(&body)
            .send_bpi("回复充电留言").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    /// 未测试
    async fn test_send_elec_message() {
        let bpi = BpiClient::new();
        // 替换为有效的 order_id 和留言
        let resp = bpi.electric_message_send("ORDER_ID_HERE", "测试留言").await;
        info!("响应: {:?}", resp);
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn test_get_elec_remark_list() {
        let bpi = BpiClient::new();
        let resp = bpi.electric_remark_list(Some(1), Some(10), None, None).await;
        info!("响应: {:?}", resp);
        assert!(resp.is_ok());

        if let Ok(response) = resp {
            let data = response.data.unwrap();
            info!("留言总记录数: {}", data.pager.total);
            info!("当前页留言记录数: {}", data.list.len());
        }
    }

    #[tokio::test]
    async fn test_get_elec_remark_detail() {
        let bpi = BpiClient::new();
        // 替换为有效的留言id
        let resp = bpi.electric_remark_detail(6507563).await;
        info!("响应: {:?}", resp);
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn test_reply_elec_remark() {
        let bpi = BpiClient::new();
        // 替换为有效的留言id和回复内容
        let resp = bpi.electric_remark_reply(6507563, "测试回复").await;
        info!("响应: {:?}", resp);
        assert!(resp.is_ok());
    }
}

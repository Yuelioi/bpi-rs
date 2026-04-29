//! Web / App 弹幕二进制接口（protobuf，见 bilibili-API-collect `danmaku_proto.md`、`danmaku_view_proto.md`）
//!
//! 响应体需使用官方 [`dm.proto`](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/grpc_api/bilibili/community/service/dm/v1)
//! 中的 `DmSegMobileReply`、`DmWebViewReply` 等自行反序列化。

use bytes::Bytes;

use crate::{ BilibiliRequest, BpiClient, BpiError };

fn append_seg_extra_query(
    q: &mut Vec<(String, String)>,
    pull_mode: Option<u32>,
    ps: Option<u32>,
    pe: Option<u32>
) {
    if let Some(v) = pull_mode {
        q.push(("pull_mode".to_string(), v.to_string()));
    }
    if let Some(v) = ps {
        q.push(("ps".to_string(), v.to_string()));
    }
    if let Some(v) = pe {
        q.push(("pe".to_string(), v.to_string()));
    }
}

impl BpiClient {
    /// 获取实时弹幕分包（Web，`DmSegMobileReply` protobuf）
    ///
    /// `GET https://api.bilibili.com/x/v2/dm/web/seg.so`
    ///
    /// # 参数
    /// - `typ`: 1 视频 / 2 漫画
    /// - `oid`: 视频 cid
    /// - `segment_index`: 6 分钟一包，从 1 起
    /// - `pid`: 稿件 avid（可选，建议填写）
    pub async fn danmaku_web_seg_proto(
        &self,
        typ: u8,
        oid: u64,
        segment_index: u32,
        pid: Option<u64>,
        pull_mode: Option<u32>,
        ps: Option<u32>,
        pe: Option<u32>
    ) -> Result<Bytes, BpiError> {
        let mut q = vec![
            ("type".to_string(), typ.to_string()),
            ("oid".to_string(), oid.to_string()),
            ("segment_index".to_string(), segment_index.to_string()),
        ];
        if let Some(p) = pid {
            q.push(("pid".to_string(), p.to_string()));
        }
        append_seg_extra_query(&mut q, pull_mode, ps, pe);

        self
            .get("https://api.bilibili.com/x/v2/dm/web/seg.so")
            .with_bilibili_headers()
            .query(&q)
            .send_request("弹幕 web 分段 seg.so")
            .await
    }

    /// 获取实时弹幕分包（Web + WBI，`DmSegMobileReply` protobuf）
    ///
    /// `GET https://api.bilibili.com/x/v2/dm/wbi/web/seg.so`
    pub async fn danmaku_web_seg_wbi_proto(
        &self,
        typ: u8,
        oid: u64,
        segment_index: u32,
        pid: Option<u64>,
        pull_mode: Option<u32>,
        ps: Option<u32>,
        pe: Option<u32>
    ) -> Result<Bytes, BpiError> {
        let mut params = vec![
            ("type".to_string(), typ.to_string()),
            ("oid".to_string(), oid.to_string()),
            ("segment_index".to_string(), segment_index.to_string()),
        ];
        if let Some(p) = pid {
            params.push(("pid".to_string(), p.to_string()));
        }
        append_seg_extra_query(&mut params, pull_mode, ps, pe);

        let signed = self.get_wbi_sign2(params).await?;

        self
            .get("https://api.bilibili.com/x/v2/dm/wbi/web/seg.so")
            .with_bilibili_headers()
            .query(&signed)
            .send_request("弹幕 WBI web 分段 seg.so")
            .await
    }

    /// 获取弹幕元数据（互动弹幕、BAS 专包 URL、个人弹幕配置等，`DmWebViewReply` protobuf）
    ///
    /// `GET https://api.bilibili.com/x/v2/dm/web/view`
    ///
    /// 文档注明需登录 Cookie（`SESSDATA`）方可拿到完整个人配置。
    pub async fn danmaku_web_view_proto(
        &self,
        typ: u8,
        oid: u64,
        pid: Option<u64>
    ) -> Result<Bytes, BpiError> {
        let mut q = vec![
            ("type".to_string(), typ.to_string()),
            ("oid".to_string(), oid.to_string()),
        ];
        if let Some(p) = pid {
            q.push(("pid".to_string(), p.to_string()));
        }

        self
            .get("https://api.bilibili.com/x/v2/dm/web/view")
            .with_bilibili_headers()
            .query(&q)
            .send_request("弹幕 web/view 元数据 protobuf")
            .await
    }

    /// 获取实时弹幕分包（移动客户端路径，`DmSegMobileReply` protobuf）
    ///
    /// `GET https://api.bilibili.com/x/v2/dm/list/seg.so`
    pub async fn danmaku_mobile_seg_proto(
        &self,
        typ: u8,
        oid: u64,
        segment_index: u32,
        pid: Option<u64>,
        pull_mode: Option<u32>,
        ps: Option<u32>,
        pe: Option<u32>
    ) -> Result<Bytes, BpiError> {
        let mut q = vec![
            ("type".to_string(), typ.to_string()),
            ("oid".to_string(), oid.to_string()),
            ("segment_index".to_string(), segment_index.to_string()),
        ];
        if let Some(p) = pid {
            q.push(("pid".to_string(), p.to_string()));
        }
        append_seg_extra_query(&mut q, pull_mode, ps, pe);

        self
            .get("https://api.bilibili.com/x/v2/dm/list/seg.so")
            .with_bilibili_headers()
            .query(&q)
            .send_request("弹幕 APP list/seg.so")
            .await
    }

    /// 获取指定日期的历史弹幕分包（protobuf）
    ///
    /// `GET https://api.bilibili.com/x/v2/dm/web/history/seg.so`
    ///
    /// 需登录（历史弹幕）。
    pub async fn danmaku_web_history_seg_proto(
        &self,
        typ: u8,
        oid: u64,
        date: &str
    ) -> Result<Bytes, BpiError> {
        let q = vec![
            ("type".to_string(), typ.to_string()),
            ("oid".to_string(), oid.to_string()),
            ("date".to_string(), date.to_string()),
        ];

        self
            .get("https://api.bilibili.com/x/v2/dm/web/history/seg.so")
            .with_bilibili_headers()
            .query(&q)
            .send_request("历史弹幕 web/history/seg.so")
            .await
    }

    /// 获取指定日期的历史弹幕（压缩 XML 正文，需自行 inflate）
    ///
    /// `GET https://api.bilibili.com/x/v2/dm/history`
    ///
    /// 需登录。响应一般为 deflate 压缩的 XML，与 `danmaku_xml` 模块解析格式一致。
    pub async fn danmaku_history_xml_bytes(
        &self,
        typ: u8,
        oid: u64,
        date: &str
    ) -> Result<Bytes, BpiError> {
        let q = vec![
            ("type".to_string(), typ.to_string()),
            ("oid".to_string(), oid.to_string()),
            ("date".to_string(), date.to_string()),
        ];

        self
            .get("https://api.bilibili.com/x/v2/dm/history")
            .with_bilibili_headers()
            .query(&q)
            .send_request("历史弹幕 XML /dm/history")
            .await
    }
}

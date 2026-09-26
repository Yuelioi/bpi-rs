# 课程 DRM 样本

`preview.anonymous.sanitized.json` 来自 2026-09-26 匿名读取 ss18619 的 ep644827。响应为 is_preview=1、MP4 durl、没有 DASH；保留模型字段并将全部地址替换为 example.invalid，覆盖部分试看结构。

`no-coupon.sanitized.json` 来自 2026-09-26 匿名读取公开课程 ss877726892 的目录响应，按已有模型裁剪、全部文本替换为占位符，保留 null、缺字段和分集 ID/数量。它覆盖无优惠券和无折扣描述的目录形态。

`drm.anonymous.sanitized.json` 来自 2026-09-26 匿名读取 ss710811134 的免费先导片 ep2129649，fnval=4048。实际响应包含 DRM/HLS 且省略三个 accept_* 字段。保留模型字段，播放地址全部替换为 example.invalid，移除 md5；不包含登录信息或已购账号的私有响应。

免费/付费属于访问权限，DRM 属于媒体加密，二者独立。SDK 保留 DRM 元数据，不负责解密。

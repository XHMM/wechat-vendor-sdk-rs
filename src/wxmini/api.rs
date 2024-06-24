use serde::{Deserialize, Serialize};

use crate::{common::AccessTokenData, wxmini_api_get, wxmini_api_post};

wxmini_api_get!(
    /// [获取 access token](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-access-token/getAccessToken.html)
    request_access_token,
    "https://api.weixin.qq.com/cgi-bin/token",
    (appid: &str, secret: &str, grant_type: &str),
    AccessTokenData
);

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchdownloadfileFileListRequestItem {
    pub fileid: String,
    pub max_age: u32,
}

#[derive(Debug, Serialize)]
pub struct BatchdownloadfileRequestBody {
    pub env: String,
    pub file_list: Vec<BatchdownloadfileFileListRequestItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BatchdownloadfileFileListResponseItem {
    pub fileid: String,
    pub download_url: String,
    pub status: u32,
    pub errmsg: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BatchdownloadfileResponseData {
    pub file_list: Vec<BatchdownloadfileFileListResponseItem>,
}

wxmini_api_post!(
    /// [获取文件下载链接](https://developers.weixin.qq.com/miniprogram/dev/wxcloudrun/src/development/storage/service/download.html)
    request_batch_download_file,
    "https://api.weixin.qq.com/tcb/batchdownloadfile",
    (access_token: &str),
    &BatchdownloadfileRequestBody,
    BatchdownloadfileResponseData
);

#[derive(Debug, Serialize)]
pub struct MediaSecCheckParams {
    pub media_url: String,
    pub media_type: u8,
    /// 1 资料；2 评论；3 论坛；4 社交日志
    pub scene: u8,
    pub openid: String,
}

#[derive(Debug, Serialize)]
pub struct MediaSecCheckBody {
    #[serde(flatten)]
    pub params: MediaSecCheckParams,
    pub version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaSecCheckResponseData {
    pub trace_id: String,
}

wxmini_api_post!(
    /// [音视频内容安全识别](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/sec-center/sec-check/mediaCheckAsync.html)
    request_media_sec_check,
    "http://api.weixin.qq.com/wxa/media_check_async",
    (access_token: &str),
    &MediaSecCheckBody,
    MediaSecCheckResponseData
);

#[derive(Debug, Serialize)]
pub struct MsgSecCheckParams {
    pub content: String,
    /// 1 资料；2 评论；3 论坛；4 社交日志
    pub scene: u8,
    pub openid: String,
}

#[derive(Debug, Serialize)]
pub struct MsgSecCheckBody {
    #[serde(flatten)]
    pub params: MsgSecCheckParams,
    pub version: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MsgSecCheckDetailItem {
    pub strategy: String,
    pub errcode: i32,
    pub suggest: Option<String>,
    pub label: Option<u32>,
    pub keyword: Option<String>,
    pub prob: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MsgSecCheckResult {
    pub suggest: String,
    pub label: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MsgSecCheckResponseData {
    pub detail: Vec<MsgSecCheckDetailItem>,
    pub trace_id: String,
    pub result: MsgSecCheckResult,
}

wxmini_api_post!(
    /// [文本内容安全识别](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/sec-center/sec-check/msgSecCheck.html)
    request_msg_sec_check,
    "http://api.weixin.qq.com/wxa/msg_sec_check",
    (access_token: &str),
    &MsgSecCheckBody,
    MsgSecCheckResponseData
);

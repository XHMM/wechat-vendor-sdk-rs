use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{common::AccessTokenData, wxmini_api_get, wxmini_api_post};

wxmini_api_get!(
    /// [获取 access token](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-access-token/getAccessToken.html)
    request_access_token,
    "api.weixin.qq.com/cgi-bin/token",
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
    "api.weixin.qq.com/tcb/batchdownloadfile",
    (access_token: Option<&str>),
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
    "api.weixin.qq.com/wxa/media_check_async",
    (access_token: Option<&str>),
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
    "api.weixin.qq.com/wxa/msg_sec_check",
    (access_token: Option<&str>),
    &MsgSecCheckBody,
    MsgSecCheckResponseData
);

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeMessageRequestBody {
    pub template_id: String,
    pub page: Option<String>,
    pub touser: String,
    pub data: Value,
    pub miniprogram_state: Option<String>,
    pub lang: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeMessageResponseData {}

wxmini_api_post!(
  /// [发送订阅消息](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/subscribe-message/sendMessage.html)
  request_subscribe_message,
  "api.weixin.qq.com/cgi-bin/message/subscribe/send",
  (access_token: Option<&str>),
  &SubscribeMessageRequestBody,
  SubscribeMessageResponseData
);

#[derive(Serialize, Debug)]
pub struct UploadfileRequestBody {
    pub env: String,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct UploadfileResponseData {
    pub url: String,
    pub token: String,
    pub authorization: String,
    pub file_id: String,
    pub cos_file_id: String,
}

wxmini_api_post!(
  /// [获取文件上传链接](https://developers.weixin.qq.com/miniprogram/dev/wxcloudrun/src/development/storage/service/upload.html)
  request_uploadfile,
  "api.weixin.qq.com/tcb/uploadfile",
  (access_token: Option<&str>),
  &UploadfileRequestBody,
  UploadfileResponseData
);

#[derive(Serialize, Debug)]
pub struct BatchdeletefileRequestBody {
    pub env: String,
    pub fileid_list: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct BatchdeletefileResponseData {}

wxmini_api_post!(
  /// [删除文件](https://developers.weixin.qq.com/miniprogram/dev/wxcloudrun/src/development/storage/service/delete.html)
  request_batchdeletefile,
  "api.weixin.qq.com/tcb/batchdeletefile",
  (access_token: Option<&str>),
  &BatchdeletefileRequestBody,
  BatchdeletefileResponseData
);

// #[derive(Debug, Serialize)]
// pub struct GetUnlimitedQrcodeRequestBody {
//     pub env: String,
//     pub file_list: Vec<BatchdownloadfileFileListRequestItem>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct GetUnlimitedQrcodeResponseData {
//     pub file_list: Vec<BatchdownloadfileFileListResponseItem>,
// }

// wxmini_api_post!(
//     /// [获取不限制的小程序码](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/qrcode-link/qr-code/getUnlimitedQRCode.html)
//     request_get_unlimited_qrcode,
//     "api.weixin.qq.com/wxa/getwxacodeunlimit",
//     (access_token: Option<&str>),
//     &GetUnlimitedQrcodeRequestBody,
//     GetUnlimitedQrcodeResponseData
// );

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ActionScene {
    Str { scene_str: String },
    Id { scene_id: u32 },
}
#[derive(Debug, Serialize)]
pub struct ActionInfo {
    pub scene: ActionScene,
}
#[derive(Debug, Serialize)]
pub struct QrcodeCreateRequestBody {
    pub expire_seconds: Option<i32>,
    pub action_name: String,
    pub action_info: ActionInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QrcodeCreateResponseData {
    pub ticket: String,
    pub expire_seconds: i32,
    pub url: String,
}

wxmini_api_post!(
    /// [生成带参数的二维码](https://developers.weixin.qq.com/doc/offiaccount/Account_Management/Generating_a_Parametric_QR_Code.html)
    request_qrcode_create,
    "api.weixin.qq.com/cgi-bin/qrcode/create",
    (access_token: Option<&str>),
    &QrcodeCreateRequestBody,
    QrcodeCreateResponseData
);

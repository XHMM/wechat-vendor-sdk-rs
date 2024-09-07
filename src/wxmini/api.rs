use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{common::AccessTokenData, wxmini_api_get, wxmini_api_post};

wxmini_api_get!(
    /// [获取 access token](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-access-token/getAccessToken.html)
    request_access_token,
    "api.weixin.qq.com/cgi-bin/token",
    (appid: Option<&str>, secret: Option<&str>, grant_type: Option<&str>),
    AccessTokenData
);

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchdownloadfileFileListRequestItem {
    pub fileid: String,
    pub max_age: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchdownloadfileRequestBody {
    pub env: String,
    pub file_list: Vec<BatchdownloadfileFileListRequestItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchdownloadfileFileListResponseItem {
    pub fileid: String,
    pub download_url: String,
    pub status: u32,
    pub errmsg: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaSecCheckParams {
    pub media_url: String,
    pub media_type: u8,
    /// 1 资料；2 评论；3 论坛；4 社交日志
    pub scene: u8,
    pub openid: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgSecCheckParams {
    pub content: String,
    /// 1 资料；2 评论；3 论坛；4 社交日志
    pub scene: u8,
    pub openid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgSecCheckBody {
    #[serde(flatten)]
    pub params: MsgSecCheckParams,
    pub version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgSecCheckDetailItem {
    pub strategy: String,
    pub errcode: i32,
    pub suggest: Option<String>,
    pub label: Option<u32>,
    pub keyword: Option<String>,
    pub prob: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgSecCheckResult {
    pub suggest: String,
    pub label: u32,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeMessageRequestBody {
    pub template_id: String,
    pub touser: String,
    pub data: Value,
    pub page: Option<String>,
    pub miniprogram_state: Option<String>,
    pub lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeMessageResponseData {}

wxmini_api_post!(
  /// [发送订阅消息](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-message-management/subscribe-message/sendMessage.html)
  request_subscribe_message,
  "api.weixin.qq.com/cgi-bin/message/subscribe/send",
  (access_token: Option<&str>),
  &SubscribeMessageRequestBody,
  SubscribeMessageResponseData
);

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadfileRequestBody {
    pub env: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchdeletefileRequestBody {
    pub env: String,
    pub fileid_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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

// #[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionScene {
    Str { scene_str: String },
    Id { scene_id: u32 },
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ActionInfo {
    pub scene: ActionScene,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QrcodeCreateRequestBody {
    pub expire_seconds: Option<i32>,
    pub action_name: String,
    pub action_info: ActionInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QrcodeCreateResponseData {
    pub ticket: String,
    pub expire_seconds: Option<i32>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuCreateRequestBody {
    pub button: Vec<Value>,
}
wxmini_api_post!(
    /// [自定义菜单-创建接口](https://developers.weixin.qq.com/doc/offiaccount/Custom_Menus/Creating_Custom-Defined_Menu.html)
    request_menu_create,
    "api.weixin.qq.com/cgi-bin/menu/create",
    (access_token: Option<&str>),
    &MenuCreateRequestBody,
    Value
);

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsCreateRequestItem {
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TagsCreateRequestBody {
    pub tag: TagsCreateRequestItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsCreateResponseItem {
    pub id: i32,
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TagsCreateResponseData {
    pub tag: TagsCreateResponseItem,
}
wxmini_api_post!(
    /// [用户标签管理-创建标签](https://developers.weixin.qq.com/doc/offiaccount/User_Management/User_Tag_Management.html)
    request_tags_create,
    "api.weixin.qq.com/cgi-bin/tags/create",
    (access_token: Option<&str>),
    &TagsCreateRequestBody,
    TagsCreateResponseData
);

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsGetResponseItem {
    pub id: i32,
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TagsGetResponseData {
    pub tags: Vec<TagsCreateResponseItem>,
}
wxmini_api_get!(
    /// [用户标签管理-获取公众号已创建的标签](https://developers.weixin.qq.com/doc/offiaccount/User_Management/User_Tag_Management.html)
    request_tags_get,
    "api.weixin.qq.com/cgi-bin/tags/get",
    (access_token: Option<&str>),
    TagsGetResponseData
);

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsMembersBatchtaggingRequestBody {
    pub openid_list: Vec<String>,
    pub tagid: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsMembersBatchtaggingResponseData {}
wxmini_api_post!(
    /// [用户标签管理-批量为用户打标签](https://developers.weixin.qq.com/doc/offiaccount/User_Management/User_Tag_Management.html)
    request_tags_members_batchtagging,
    "api.weixin.qq.com/cgi-bin/tags/members/batchtagging",
    (access_token: Option<&str>),
    &TagsMembersBatchtaggingRequestBody,
    Value
);

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGetResponseItemData {
    pub openid: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserGetResponseData {
    pub total: i32,
    pub count: i32,
    pub data: UserGetResponseItemData,
    pub next_openid: String,
}
wxmini_api_get!(
    /// [获取用户列表](https://developers.weixin.qq.com/doc/offiaccount/User_Management/Getting_a_User_List.html)
    request_user_get,
    "api.weixin.qq.com/cgi-bin/user/get",
    (access_token: Option<&str>, next_openid: Option<&str>),
    UserGetResponseData
);

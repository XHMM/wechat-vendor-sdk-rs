//!
//! 企业微信相关接口
//!

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{common::AccessTokenData, wxcorp_api_get, wxcorp_api_post};

wxcorp_api_get!(
    /// [获取 access token](https://developer.work.weixin.qq.com/document/path/91039)
    request_access_token,
    "https://qyapi.weixin.qq.com/cgi-bin/gettoken",
    (corpid: Option<&str>, corpsecret: Option<&str>),
    AccessTokenData
);

/// 接口响应的数据信息。（企业微信开发文档里写的是 snake_case，但实际开发返回的却是 PascalCase，汗颜）
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all_fields(deserialize = "PascalCase"))]
pub enum UserIdInfoByAuthCode {
    Inner {
        user_id: String,
    },
    External {
        open_id: String,
        external_user_id: Option<String>,
    },
}
wxcorp_api_get!(
    /// [根据 code 获取用户 id 信息](https://developer.work.weixin.qq.com/document/path/98176)
    request_user_id_by_auth_code,
    "https://qyapi.weixin.qq.com/cgi-bin/user/getuserinfo",
    (access_token: Option<&str>, code: Option<&str>),
    UserIdInfoByAuthCode
);

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub userid: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
    /// 需要设置应用的可见范围
    pub department: Vec<i32>,
    pub order: Vec<i32>,
}
wxcorp_api_get!(
    /// [根据 userid 获取用户详情](https://developer.work.weixin.qq.com/document/path/90196)
    request_user_info_by_user_id,
    "https://qyapi.weixin.qq.com/cgi-bin/user/get",
    (access_token: Option<&str>, userid: Option<&str>),
    UserInfo
);

wxcorp_api_post!(
    request_send,
    "https://qyapi.weixin.qq.com/cgi-bin/message/send",
    (access_token: Option<&str>),
    &Value,
    Value
);

wxcorp_api_post!(
    request_webhook_send,
    "https://qyapi.weixin.qq.com/cgi-bin/webhook/send",
    (key: Option<&str>),
    &Value,
    Value
);

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalContactListResponseData {
    pub external_userid: Vec<String>,
}
wxcorp_api_get!(
    /// [获取客户列表](https://developer.work.weixin.qq.com/document/path/92113)
    request_external_contact_list,
    "https://qyapi.weixin.qq.com/cgi-bin/externalcontact/list",
    (access_token: Option<&str>,  userid: Option<&str>),
    ExternalContactListResponseData
);

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalContactGetResponseData {
    pub external_contact: Value,
    pub follow_user: Vec<Value>,
    pub next_cursor: Option<String>,
}
wxcorp_api_get!(
    /// [获取客户详情](https://developer.work.weixin.qq.com/document/path/92114)
    request_external_contact_get,
    "https://qyapi.weixin.qq.com/cgi-bin/externalcontact/get",
    (access_token: Option<&str>,  external_userid: Option<&str>, cursor: Option<&str>),
    ExternalContactGetResponseData
);

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalContactBatchGetByUserRequestBody {
    pub userid_list: Vec<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalContactBatchGetByUserResponseData {
    pub external_contact_list: Vec<Value>,
    pub next_cursor: Option<String>,
    pub fail_info: Option<Value>,
}
wxcorp_api_post!(
    /// [批量获取客户详情](https://developer.work.weixin.qq.com/document/path/92994)
    request_external_contact_batch_get_by_user,
    "https://qyapi.weixin.qq.com/cgi-bin/externalcontact/batch/get_by_user",
    (access_token: Option<&str>),
    &ExternalContactBatchGetByUserRequestBody,
    ExternalContactBatchGetByUserResponseData
);

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalContactGroupChatListRequestBody {
    pub status_filter: Option<i64>,
    pub owner_filter: Option<Value>,
    pub cursor: Option<String>,
    pub limit: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalContactGroupChatListItem {
    pub chat_id: String,
    pub status: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalContactGroupChatListResponseData {
    pub group_chat_list: Vec<ExternalContactGroupChatListItem>,
    pub next_cursor: Option<String>,
}
wxcorp_api_post!(
    request_external_contact_group_chat_list,
    "https://qyapi.weixin.qq.com/cgi-bin/externalcontact/groupchat/list",
    (access_token: Option<&str>),
    &ExternalContactGroupChatListRequestBody,
    ExternalContactGroupChatListResponseData
);

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalContactGroupChatGetRequestBody {
    pub chat_id: String,
    pub need_name: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalContactGroupChatGetResponseData {
    pub group_chat: Value,
}
wxcorp_api_post!(
    request_external_contact_group_chat_get,
    "https://qyapi.weixin.qq.com/cgi-bin/externalcontact/groupchat/get",
    (access_token: Option<&str>),
    &ExternalContactGroupChatGetRequestBody,
    ExternalContactGroupChatGetResponseData
);

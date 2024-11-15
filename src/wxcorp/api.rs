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

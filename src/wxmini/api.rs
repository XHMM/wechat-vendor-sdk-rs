use serde::{Deserialize, Serialize};

use crate::{common::AccessTokenData, wxmini_api_get, wxmini_api_post};

wxmini_api_get!(
    /// [获取 access token](https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-access-token/getAccessToken.html)
    request_access_token,
    "https://api.weixin.qq.com/cgi-bin/token",
    (appid: &str, secret: &str, grant_type: &str),
    AccessTokenData
);

#[derive(Serialize)]
pub struct BatchdownloadfileFileListRequestItem {
    pub fileid: String,
    pub max_age: u32,
}

#[derive(Serialize)]
pub struct BatchdownloadfileRequestBody {
    pub env: String,
    pub file_list: Vec<BatchdownloadfileFileListRequestItem>,
}

#[derive(Deserialize)]
pub struct BatchdownloadfileFileListResponseItem {
    pub fileid: String,
    pub download_url: String,
    pub status: u32,
    pub errmsg: String,
}

#[derive(Deserialize)]
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

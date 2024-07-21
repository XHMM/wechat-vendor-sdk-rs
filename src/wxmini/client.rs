use serde::Serialize;
use serde_json::Value;
use tracing::{error, trace};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WxminiApiError {
    /// 微信 api 返回的错误码不为成功
    #[error("bad errorcode: {0}")]
    ApiCodeNotOk(Value),
    /// 微信 api 的响应内容解析失败，一般是响应内容的 struct 定义和响应内容不一致所致
    #[error("deserialize response error: {0}")]
    /// 请求微信 api 网络出错
    Deserialize(#[from] serde_json::Error),
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),
}

pub struct WxminiClient {
    is_http: bool,
}

impl WxminiClient {
    pub fn new() -> Self {
        Self { is_http: false }
    }
    pub fn without_https() -> Self {
        Self { is_http: false }
    }
}

impl WxminiClient {
    pub(crate) async fn call_get<D, F>(
        &self,
        endpoint_without_protocol: &str,
        query: &[(&str, &str)],
        map: F,
    ) -> Result<D, WxminiApiError>
    where
        F: FnOnce(Value) -> Result<D, serde_json::Error>,
    {
        let client = reqwest::Client::new();
        let response = client
            .get(&format!(
                "{}://{}",
                if self.is_http { "http" } else { "https" },
                endpoint_without_protocol
            ))
            .query(query)
            .send()
            .await?;
        let data: Value = response.json().await?;
        trace!("wxmini api respone: {:?}", data);

        // 小程序的 api 响应字段并不标准..
        // 比如碰到过  {"request_id": String("xx"), "error_type": String("SafeLinkError"), "error_code": String("85107"), "error_message": String("URL不在白名单内，请前往「微信云托管控制台-服务管理-云调用-微信令牌」配置")}
        if data["error_code"] != 0 || data["errcode"] != 0 {
            return Err(WxminiApiError::ApiCodeNotOk(data));
        }

        match map(data) {
            Ok(data) => Ok(data),
            Err(err) => Err(WxminiApiError::Deserialize(err)),
        }
    }

    pub(crate) async fn call_post<D, B, F>(
        &self,
        endpoint_without_protocol: &str,
        query: &[(&str, Option<&str>)],
        body: &B,
        map: F,
    ) -> Result<D, WxminiApiError>
    where
        B: Serialize,
        F: FnOnce(Value) -> Result<D, serde_json::Error>,
    {
        let client = reqwest::Client::new();
        let response = client
            .post(&format!(
                "{}://{}",
                if self.is_http { "http" } else { "https" },
                endpoint_without_protocol
            ))
            .query(query)
            .json(body)
            .send()
            .await?;
        let text = response.text().await?;
        let data: Value = serde_json::from_str(&text).map_err(|err| {
            trace!("decode json error: {}, raw text: {}", err, text);
            err
        })?;

        trace!("wxmini api post response: {:?}", data);

        if data["errcode"] == 0 || data["errcode"].is_null() {
            match map(data) {
                Ok(data) => Ok(data),
                Err(err) => Err(WxminiApiError::Deserialize(err)),
            }
        } else {
            Err(WxminiApiError::ApiCodeNotOk(data))
        }
    }
}

#[macro_export]
macro_rules! wxmini_api_get {
    ($(#[$attr:meta])* $name: ident, $endpoint_without_protocol: tt, ($($v:ident: $t:ty),*), $ret_type: ty) => {
        impl $crate::wxmini::WxminiClient {
            $(#[$attr])*
            pub async fn $name(&self, $($v: $t),*) -> Result<$ret_type, $crate::wxmini::WxminiApiError> {
                self.call_get(
                    &format!(
                        $endpoint_without_protocol
                    ),
                    // stringify! 将 ident 转为字符串形式
                    &[$((stringify!($v), $v)),*],
                    |data| serde_json::from_value::<$ret_type>(data),
                )
                .await
            }
        }
    };
}

#[macro_export]
macro_rules! wxmini_api_post {
    ($(#[$attr:meta])* $name: ident, $endpoint_without_protocol: tt, ($($v:ident: $t:ty),*), $req_body:ty, $ret_type:ty) => {
        impl $crate::wxmini::WxminiClient {
            $(#[$attr])*
            pub async fn $name(&self, body: $req_body, $($v: $t),*) -> Result<$ret_type, $crate::wxmini::WxminiApiError> {
                self.call_post(
                    &format!(
                        $endpoint_without_protocol
                    ),
                    // stringify! 将 ident 转为字符串形式
                    &[$((stringify!($v), $v)),*],
                    &body,
                    |data| serde_json::from_value::<$ret_type>(data),
                )
                .await
            }
        }
    };
}

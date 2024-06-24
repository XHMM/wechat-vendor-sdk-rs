use serde::Serialize;
use serde_json::Value;
use tracing::{error, trace};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WxminiApiError {
    #[error("bad errorcode: {0}")]
    ApiCodeNotOk(Value),
    #[error("deserialize response error: {0}")]
    Deserialize(#[from] serde_json::Error),
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),
}

pub(crate) async fn get<D, F>(
    url: &str,
    query: &[(&str, &str)],
    map: F,
) -> Result<D, WxminiApiError>
where
    F: FnOnce(Value) -> Result<D, serde_json::Error>,
{
    let client = reqwest::Client::new();
    let response = client.get(url).query(query).send().await?;
    let data: Value = response.json().await?;
    trace!("wxmini api respone: {:?}", data);

    // 小程序的 api 响应字段并不标准，errcode 字段可能不存在
    if data["errcode"] == 0 || data["errcode"].is_null() {
        match map(data) {
            Ok(data) => Ok(data),
            Err(err) => Err(WxminiApiError::Deserialize(err)),
        }
    } else {
        Err(WxminiApiError::ApiCodeNotOk(data))
    }
}

#[macro_export]
macro_rules! wxmini_api_get {
    ($(#[$attr:meta])* $name: ident, $url: tt, ($($v:ident: $t:ty),*), $ret_type: ty) => {
        $(#[$attr])*
        pub async fn $name($($v: $t),*) -> Result<$ret_type, crate::wxmini::base::WxminiApiError> {
            crate::wxmini::base::get(
                &format!(
                    $url
                 ),
                // stringify! 将 ident 转为字符串形式
                &[$((stringify!($v), $v)),*],
                |data| serde_json::from_value::<$ret_type>(data),
            )
            .await
        }
    };
}

pub(crate) async fn post<D, B, F>(
    url: &str,
    query: &[(&str, &str)],
    body: &B,
    map: F,
) -> Result<D, WxminiApiError>
where
    B: Serialize,
    F: FnOnce(Value) -> Result<D, serde_json::Error>,
{
    let client = reqwest::Client::new();
    let response = client.post(url).query(query).json(body).send().await?;
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

#[macro_export]
macro_rules! wxmini_api_post {
    ($(#[$attr:meta])* $name: ident, $url: tt, ($($v:ident: $t:ty),*), $req_body:ty, $ret_type:ty) => {
        $(#[$attr])*
        pub async fn $name(body: $req_body, $($v: $t),*) -> Result<$ret_type, crate::wxmini::base::WxminiApiError> {
            crate::wxmini::base::post(
                &format!(
                    $url
                 ),
                // stringify! 将 ident 转为字符串形式
                &[$((stringify!($v), $v)),*],
                &body,
                |data| serde_json::from_value::<$ret_type>(data),
            )
            .await
        }
    };
}

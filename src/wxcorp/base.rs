use serde_json::Value;
use tracing::{error, trace};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WxcorpApiError {
    #[error("bad errorcode: {0}")]
    ApiCodeNotOk(Value),
    #[error("deserialize response error: {0}")]
    Deserialize(#[from] serde_json::Error),
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),
}

pub async fn __private_get<D, F>(
    url: &str,
    query: &[(&str, &str)],
    map: F,
) -> Result<D, WxcorpApiError>
where
    F: FnOnce(Value) -> Result<D, serde_json::Error>,
{
    let client = reqwest::Client::new();
    let response = client.get(url).query(query).send().await?;
    let data: Value = response.json().await?;
    trace!("wxcorp api respone: {:?}", data);

    if data["errcode"] == 0 {
        match map(data) {
            Ok(data) => Ok(data),
            Err(err) => Err(WxcorpApiError::Deserialize(err)),
        }
    } else {
        Err(WxcorpApiError::ApiCodeNotOk(data))
    }
}

#[macro_export]
macro_rules! wxcorp_api_get {
    ($(#[$attr:meta])* $name: ident, $url: tt, ($($v:ident: $t:ty),*), $ret_type: ty) => {
        $(#[$attr])*
        pub async fn $name($($v: $t),*) -> Result<$ret_type, $crate::wxcorp::base::WxcorpApiError> {
            $crate::wxcorp::base::__private_get(
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

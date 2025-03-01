use std::fmt::Debug;

use serde::Serialize;
use serde_json::Value;
use tracing::error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WxcorpApiError {
    #[error("bad errorcode: {0}")]
    ApiCodeNotOk(Value),
    #[error("deserialize response error: {0}")]
    WxcorpResDeserializeErr(#[from] serde_json::Error),
    #[error("request error: {0}")]
    RequestErr(#[from] reqwest::Error),
}

pub struct WxcorpClient {}

impl WxcorpClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl WxcorpClient {
    pub(crate) async fn call_get<D, F>(
        &self,
        url: &str,
        // TODO: 待改造为 bon
        query: &[(&str, Option<&str>)],
        map: F,
    ) -> Result<D, WxcorpApiError>
    where
        F: FnOnce(Value) -> Result<D, serde_json::Error>,
    {
        let client = reqwest::Client::new();
        let response = client.get(url).query(query).send().await?;
        let data: Value = response.json().await?;
        // trace!("wxcorp api response: {:?}", data);

        if data["errcode"] == 0 {
            match map(data) {
                Ok(data) => Ok(data),
                Err(err) => Err(WxcorpApiError::WxcorpResDeserializeErr(err)),
            }
        } else {
            Err(WxcorpApiError::ApiCodeNotOk(data))
        }
    }

    pub(crate) async fn call_post<D, B, F>(
        &self,
        url: &str,
        query: &[(&str, Option<&str>)],
        body: &B,
        map: F,
    ) -> Result<D, WxcorpApiError>
    where
        B: Serialize + Debug,
        F: FnOnce(Value) -> Result<D, serde_json::Error>,
    {
        let client = reqwest::Client::new();
        let response = client.post(url).query(query).json(body).send().await?;

        let data: Value = response.json().await?;
        // trace!("wxcorp api post response: {:?}", data);

        if data["errcode"] == 0 {
            match map(data) {
                Ok(data) => Ok(data),
                Err(err) => Err(WxcorpApiError::WxcorpResDeserializeErr(err)),
            }
        } else {
            Err(WxcorpApiError::ApiCodeNotOk(data))
        }
    }
}

#[macro_export]
macro_rules! wxcorp_api_get {
    ($(#[$attr:meta])* $name: ident, $url: tt, ($($v:ident: $t:ty),*), $ret_type: ty) => {
        impl $crate::wxcorp::WxcorpClient {
            $(#[$attr])*
            pub async fn $name(&self, $($v: $t),*) -> Result<$ret_type, $crate::wxcorp::WxcorpApiError> {
                self.call_get(
                    &format!(
                        $url
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
macro_rules! wxcorp_api_post {
    ($(#[$attr:meta])* $name: ident, $url: tt, ($($v:ident: $t:ty),*), $req_body:ty, $ret_type:ty) => {
        impl $crate::wxcorp::WxcorpClient {
            $(#[$attr])*
            pub async fn $name(&self, body: $req_body, $($v: $t),*) -> Result<$ret_type, $crate::wxcorp::WxcorpApiError> {
                self.call_post(
                    $url,
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

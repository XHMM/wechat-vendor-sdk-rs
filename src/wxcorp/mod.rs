//! 企业微信相关 api 封装
//!
//! ```
//! let client = WxcorpClient::new();
//! let res_data = client.request_user_id_by_auth_code(access_token, code)
//! ```
//!
//! 对于该 crate 未封装的 api，你可以使用 [`wxcorp_api_get!`](crate::wxcorp_api_get) 自行封装
//!

mod api;

mod client;
pub use client::WxcorpApiError;
pub use client::WxcorpClient;

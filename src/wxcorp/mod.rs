//! 企业微信相关 api 封装
//!
//! ```
//! let client = WxcorpClient::new();
//! let res_data = client.request_user_id_by_auth_code(access_token, code)
//! ```
//!

mod api;
pub use api::*;

mod client;
pub use client::WxcorpApiError;
pub use client::WxcorpClient;

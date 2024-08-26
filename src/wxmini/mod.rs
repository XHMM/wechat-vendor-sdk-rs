//! 微信小程序/微信公众号相关 api 的封装
//!
//! ## 使用场景
//! ### 在微信云托管环境
//! 此时无需提供 access token，且 api 需要使用 http 协议进行调用：
//! ```
//! let client = WxminiClient::without_https();
//! let res_data = client.request_msg_sec_check(&body, None).await;
//! ```
//!
//!
//! ### 在非云托管环境
//! 比如在自己的服务器内请求微信 api，这种场景需要提供 access token，且需要使用 https 协议：
//! ```
//! let client = WxminiClient::new();
//! let res_data = client.request_msg_sec_check(&body, Some(access_token)).await;
//! ```
//!

mod api;
pub use api::*;

mod client;
pub use client::WxminiApiError;
pub use client::WxminiClient;

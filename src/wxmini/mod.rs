//! 微信小程序相关 Api 封装。
//!
//! 关于 access token：在云托管环境里无需传递 access token，对应传入 `None` 即可，其他场景则需要传入 `Some("token value")`。
//!
//! 对于该 crate 未封装的 Api，你可以使用 [`wxmini_api_post!`](crate::wxmini_api_post) 和 [`wxmini_api_get!`](crate::wxmini_api_get) 自行封装。

pub mod api;
mod base;
pub use base::WxminiApiError;

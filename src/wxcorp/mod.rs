//! 企业微信相关 Api 封装。
//!
//! 对于该 crate 未封装的 Api，你可以使用 [`wxcorp_api_get!`](crate::wxcorp_api_get) 自行封装。
//!

pub mod api;
mod base;
pub use base::WxcorpApiError;

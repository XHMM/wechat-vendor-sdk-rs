//! You can use `RUST_LOG=wechat_vendor_sdk=trace cargo run` to get response tracing log when getting such as `WxcorpResDeserializeErr` error
//!
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod common;

#[cfg(feature = "wxcorp")]
#[cfg_attr(docsrs, doc(cfg(feature = "wxcorp")))]
pub mod wxcorp;
#[cfg(feature = "wxmini")]
#[cfg_attr(docsrs, doc(cfg(feature = "wxmini")))]
pub mod wxmini;

#[cfg(feature = "wxpay")]
#[cfg_attr(docsrs, doc(cfg(feature = "wxpay")))]
pub mod wxpay;

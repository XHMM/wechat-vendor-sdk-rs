#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod common;

#[cfg(feature = "wxcorp")]
#[cfg_attr(docsrs, doc(cfg(feature = "wxcorp")))]
pub mod wxcorp;
#[cfg(feature = "wxmini")]
#[cfg_attr(docsrs, doc(cfg(feature = "wxmini")))]
pub mod wxmini;

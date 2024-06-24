//! 存放公用内容
//!
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccessTokenData {
    pub access_token: String,
    pub expires_in: u64,
}

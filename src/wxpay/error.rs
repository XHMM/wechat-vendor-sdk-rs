use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WxPayFailedResponse {
    pub code: String,
    pub message: String,
    pub detail: Option<serde_json::Value>,
}

#[derive(Error, Debug)]
pub enum WxpayApiError {
    #[error("timestamp is invalid")]
    InvalidTimestamp,

    #[error("invalid ciphertext: {0}")]
    InvalidCiphertext(#[from] aes_gcm::aes::cipher::InvalidLength),

    #[error("invalid public key")]
    InvalidPublicKey,

    #[error("invalid private key")]
    InvalidPrivateKey,

    #[error("decrypt failed")]
    DecryptFailed,

    #[error("request error: {0}")]
    RequestErr(#[from] reqwest::Error),

    #[error("base64 decode error: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("wxpay error: {}", .0.message)]
    WxpayError(WxPayFailedResponse),
}

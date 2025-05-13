use chrono::TimeZone;
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Sign};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{api::OutTradeNoResponseData, error::WxpayApiError};

/// 用于验证微信支付的回调请求签名
///
/// * `public_key` - 微信支付公钥内容。可以使用以下命令从微信提供的证书中提取： `openssl x509 -in apiclient_cert.pem -pubkey -noout > apiclient_pub.pem`
pub fn verify_wxpay_callback_signature(
    // 微信平台证书公钥
    wx_public_key: &str,
    // 从响应头获取的签名
    signature: &str,
    // 从响应头获取的时间戳
    timestamp: &str,
    // 从响应头获取的随机字符串
    nonce: &str,
    // 响应体
    body: &str,
    // 是否跳过时间戳检查
    skip_timestamp_check: Option<bool>,
) -> Result<bool, WxpayApiError> {
    use base64::{engine::general_purpose, Engine as _};
    use rsa::sha2::{Digest, Sha256};
    use rsa::RsaPublicKey;

    if skip_timestamp_check.unwrap_or(false) == false {
        // 验证时间戳
        use chrono::{Duration, Utc};
        let current_time = Utc::now();
        let timestamp_secs = timestamp
            .parse::<i64>()
            .map_err(|_| WxpayApiError::InvalidTimestamp)?;
        let timestamp_datetime = Utc
            .timestamp_opt(timestamp_secs, 0)
            .single()
            .ok_or(WxpayApiError::InvalidTimestamp)?;

        let time_diff = current_time - timestamp_datetime;
        if time_diff > Duration::seconds(300) {
            tracing::error!("timestamp is expired");
            return Ok(false);
        }
    }

    let signature_bytes = general_purpose::STANDARD.decode(signature)?;

    let message = format!("{}\n{}\n{}\n", timestamp, nonce, body);
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hash = hasher.finalize();

    let public_key = RsaPublicKey::from_public_key_pem(wx_public_key)
        .map_err(|e| WxpayApiError::InvalidPublicKey)?;
    let scheme = Pkcs1v15Sign::new::<Sha256>();
    let res = public_key.verify(scheme, &hash, signature_bytes.as_slice());
    match res {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
    // 下面的写法是错误的
    // let verifying_key = VerifyingKey::<Sha256>::new(public_key);
    // match verifying_key.verify(&hash, &signature_bytes.as_slice().try_into()?) {
    //     Ok(_) => Ok(true),
    //     Err(_) => Ok(false),
    // }
}

pub fn decrypt_wxpay_callback_resource(
    apiv3_key: &str,
    ciphertext: &str,
    nonce: &str,
    associated_data: &str,
) -> Result<Value, WxpayApiError> {
    use aes_gcm::aead::{Aead, Payload};
    use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
    use base64::{engine::general_purpose, Engine as _};

    let ciphertext = general_purpose::STANDARD.decode(ciphertext)?;

    let cipher = Aes256Gcm::new_from_slice(apiv3_key.as_bytes())?;
    let payload = Payload {
        msg: &ciphertext.as_slice(),
        aad: &associated_data.as_bytes(),
    };
    let nonce = Nonce::from_slice(nonce.as_bytes());

    let plaintext = cipher
        .decrypt(nonce, payload)
        .map_err(|_| WxpayApiError::DecryptFailed)?;
    let val = serde_json::from_slice(&plaintext);
    match val {
        Ok(val) => Ok(val),
        Err(e) => {
            tracing::error!("decrypt wxpay callback resource to json failed: {}", e);
            Err(WxpayApiError::DecryptFailed)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]

pub struct WxpayBatchTransferCallbackResourceDataClosed {
    pub mchid: String,
    pub out_batch_no: String,
    pub batch_id: String,
    pub batch_status: String,
    pub total_num: i32,
    pub total_amount: i32,
    pub close_reason: Option<String>,
    pub update_time: String,
}

#[derive(Debug, Deserialize, Serialize)]

pub struct WxpayBatchTransferCallbackResourceDataFinished {
    pub out_batch_no: String,
    pub batch_id: String,
    pub batch_status: String,
    pub total_amount: i32,
    pub total_num: i32,
    pub success_amount: i32,
    pub success_num: i32,
    pub fail_amount: i32,
    pub fail_num: i32,
    pub update_time: String,
}

/// [商家转账批次回调通知](https://pay.weixin.qq.com/doc/v3/merchant/4012269028)解密后的数据
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WxpayBatchTransferCallbackResourceData {
    Closed(WxpayBatchTransferCallbackResourceDataClosed),
    Finished(WxpayBatchTransferCallbackResourceDataFinished),
}

/// [支付成功回调通知](https://pay.weixin.qq.com/doc/v3/merchant/4012791902)解密后的数据（和查询订单详情返回格式一致）
pub type WxpayPayCallbackResourceData = OutTradeNoResponseData;

#[test]
fn test_verify_wxpay_callback_signature() {
    let wx_public_key = "-----BEGIN PUBLIC KEY-----
...
-----END PUBLIC KEY-----";

    let result = verify_wxpay_callback_signature(
        wx_public_key,
        "signature",
        "1727611989",
        "nonce",
        r#"payload"#,
        None,
    );
    println!("result: {:?}", result);
}

#[test]
fn test_timestamp_diff() {
    use chrono::{Duration, Utc};
    let current_time = Utc::now();
    let timestamp_secs = ("1717233600")
        .parse::<i64>()
        .map_err(|e| format!("无效的时间戳: {}", e))
        .unwrap();
    let timestamp_datetime = Utc
        .timestamp_opt(timestamp_secs, 0)
        .single()
        .ok_or("无效的时间戳")
        .unwrap();

    let time_diff = current_time - timestamp_datetime;
    if time_diff > Duration::seconds(300) {
        println!("duration11: {:?}", time_diff);
    } else {
        println!("duration22: {:?}", time_diff);
    }
}

#[test]
fn test_decrypt_wxpay_callback_resource() {
    let apiv3_key = "xxx";
    let nonce = "yyy";
    let ciphertext = "22qIR8j4SVcexi0PTqgsPPxXICxk+zz==";

    let result = decrypt_wxpay_callback_resource(apiv3_key, ciphertext, nonce, "mch_payment");
    println!("result: {:?}", result);
}

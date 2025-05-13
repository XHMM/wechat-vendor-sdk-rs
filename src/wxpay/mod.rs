use chrono::TimeZone;
use error::{WxPayFailedResponse, WxpayApiError};
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Sign};
use serde_json::Value;
use utils::generate_wxpay_request_signature;

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub mod error;
pub mod utils;

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

#[derive(Debug, Deserialize, Serialize)]

pub struct WxpayCallbackResourceDataClosed {
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

pub struct WxpayCallbackResourceDataFinished {
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WxpayCallbackResourceData {
    Closed(WxpayCallbackResourceDataClosed),
    Finished(WxpayCallbackResourceDataFinished),
}
pub fn decrypt_wxpay_callback_resource(
    apiv3_key: &str,
    ciphertext: &str,
    nonce: &str,
    associated_data: &str,
) -> Result<WxpayCallbackResourceData, WxpayApiError> {
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
pub struct BatchTransferRequestBody {
    pub appid: String,
    /// 只能是数字和字母的组合
    pub out_batch_no: String,
    pub batch_name: String,
    pub batch_remark: String,
    pub total_amount: u64,
    pub total_num: u64,
    pub transfer_detail_list: Vec<TransferDetail>,
    pub transfer_scene_id: Option<String>,
    pub notify_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransferDetail {
    pub out_detail_no: String,
    pub transfer_amount: u64,
    pub transfer_remark: String,
    pub openid: String,
    pub user_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BatchTransferResponse {
    pub out_detail_no: String,
    pub batch_id: String,
    pub create_time: String,
    pub out_batch_no: String,
}

/// [发起批量转账接口](https://pay.weixin.qq.com/doc/v3/partner/4012465738)
#[bon::builder]
pub async fn request_batch_transfer<'a>(
    body: BatchTransferRequestBody,
    // 商户号
    mchid: &'a str,
    // 商户私钥
    mch_private_key: &'a str,
    // 商户证书序列号
    mch_serial_no: &'a str,
    // 微信支付平台证书序列号
    wxpay_serial_no: &'a str,
) -> Result<serde_json::Value, WxpayApiError> {
    let url_base = "https://api.mch.weixin.qq.com";
    let endpoint = "/v3/transfer/batches";
    let url = format!("{}{}", url_base, endpoint);
    let method = "POST";

    let body = serde_json::to_string(&body).expect("failed to serialize body");

    let (signature, timestamp, nonce_str) =
        generate_wxpay_request_signature(method, endpoint, mch_private_key, Some(&body))?;

    let client = Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("User-Agent", "wechat-vendor-sdk/0.0.0")
        .header("Wechatpay-Serial", wxpay_serial_no)
        .header("Authorization", format!("WECHATPAY2-SHA256-RSA2048 mchid=\"{}\",nonce_str=\"{}\",signature=\"{}\",timestamp=\"{}\",serial_no=\"{}\"", 
            mchid, nonce_str, signature, timestamp, mch_serial_no))
        .body(body)
        .send()
        .await?;
    let status = response.status();
    if status == 200 {
        let result: serde_json::Value = response.json().await?;
        Ok(result)
    } else {
        let result: WxPayFailedResponse = response.json().await?;
        Err(WxpayApiError::WxpayError(result))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsapiOrderRequestBody<'a> {
    // 小程序appid
    pub appid: &'a str,
    // 商户号
    pub mchid: &'a str,
    pub description: &'a str,
    pub out_trade_no: &'a str,
    // 文档里有些字段写的是选填，但是请求时字段不存在还是会报错，所以这里未定义为 Option
    pub time_expire: &'a str,
    pub attach: &'a str,
    pub notify_url: &'a str,
    pub goods_tag: &'a str,
    pub support_fapiao: bool,
    pub amount: JsapiOrderAmount<'a>,
    pub payer: JsapiOrderPayer<'a>,
    pub detail: serde_json::Value,
    pub scene_info: serde_json::Value,
    pub settle_info: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsapiOrderAmount<'a> {
    pub total: i32,
    pub currency: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsapiOrderPayer<'a> {
    pub openid: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsapiOrderResponseData {
    pub prepay_id: String,
}

/// [JSAPI/小程序下单](https://pay.weixin.qq.com/doc/v3/merchant/4012791897)
#[bon::builder]
pub async fn request_jsapi_order<'a>(
    body: JsapiOrderRequestBody<'a>,
    mchid: &'a str,
    mch_private_key: &'a str,
    mch_serial_no: &'a str,
) -> Result<JsapiOrderResponseData, WxpayApiError> {
    let url_base = "https://api.mch.weixin.qq.com";
    let endpoint = "/v3/pay/transactions/jsapi";
    let url = format!("{}{}", url_base, endpoint);
    let method = "POST";

    let body = serde_json::to_string(&body).expect("failed to serialize body");

    let (signature, timestamp, nonce_str) =
        generate_wxpay_request_signature(method, endpoint, mch_private_key, Some(&body))?;

    let client = Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("User-Agent", "wechat-vendor-sdk/0.0.0")
        .header("Authorization", format!("WECHATPAY2-SHA256-RSA2048 mchid=\"{}\",nonce_str=\"{}\",signature=\"{}\",timestamp=\"{}\",serial_no=\"{}\"", 
            mchid, nonce_str, signature, timestamp, mch_serial_no))
        .body(body)
        .send()
        .await?;

    let status = response.status();
    if status == 200 {
        let result: JsapiOrderResponseData = response.json().await?;
        Ok(result)
    } else {
        let result: WxPayFailedResponse = response.json().await?;
        Err(WxpayApiError::WxpayError(result))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutTradeNoResponseData {
    pub appid: String,
    pub mchid: String,
    pub out_trade_no: String,
    pub transaction_id: Option<String>,
    pub trade_type: Option<String>,
    pub trade_state: String,
    pub trade_state_desc: String,
    pub bank_type: Option<String>,
    pub attach: Option<String>,
    pub success_time: Option<String>,
    pub payer: Option<Payer>,
    pub amount: Option<Amount>,
    pub scene_info: Option<SceneInfo>,
    pub promotion_detail: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Payer {
    pub openid: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Amount {
    pub total: Option<i32>,
    pub payer_total: Option<i32>,
    pub currency: Option<String>,
    pub payer_currency: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SceneInfo {
    pub device_id: Option<String>,
}

/// [商户订单号查询订单](https://pay.weixin.qq.com/doc/v3/merchant/4012791900)
#[bon::builder]
pub async fn request_order_detail<'a>(
    out_trade_no: &'a str,
    mchid: &'a str,
    mch_private_key: &'a str,
    mch_serial_no: &'a str,
) -> Result<OutTradeNoResponseData, WxpayApiError> {
    let url_base = "https://api.mch.weixin.qq.com";
    let endpoint = format!(
        "/v3/pay/transactions/out-trade-no/{}?mchid={}",
        out_trade_no, mchid
    );
    let url = format!("{}{}", url_base, endpoint);
    let method = "GET";

    let (signature, timestamp, nonce_str) =
        generate_wxpay_request_signature(method, &endpoint, mch_private_key, None)?;

    let client = Client::new();
    let response = client.get(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("User-Agent", "wechat-vendor-sdk/0.0.0")
        .header("Authorization", format!("WECHATPAY2-SHA256-RSA2048 mchid=\"{}\",nonce_str=\"{}\",signature=\"{}\",timestamp=\"{}\",serial_no=\"{}\"", 
            mchid, nonce_str, signature, timestamp, mch_serial_no))
        .send()
        .await?;

    let status = response.status();
    if status == 200 {
        let result: OutTradeNoResponseData = response.json().await?;
        Ok(result)
    } else {
        let result: WxPayFailedResponse = response.json().await?;
        Err(WxpayApiError::WxpayError(result))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CloseOrderRequestBody<'a> {
    pub mchid: &'a str,
}

/// [关闭订单](https://pay.weixin.qq.com/doc/v3/merchant/4012791901)
#[bon::builder]
pub async fn request_close_order<'a>(
    body: CloseOrderRequestBody<'a>,
    out_trade_no: &'a str,
    mchid: &'a str,
    mch_private_key: &'a str,
    mch_serial_no: &'a str,
) -> Result<(), WxpayApiError> {
    let url_base = "https://api.mch.weixin.qq.com";
    let endpoint = format!("/v3/pay/transactions/out-trade-no/{}/close", out_trade_no);
    let url = format!("{}{}", url_base, endpoint);
    let method = "POST";

    let body = serde_json::to_string(&body).expect("failed to serialize body");

    let (signature, timestamp, nonce_str) =
        generate_wxpay_request_signature(method, &endpoint, mch_private_key, Some(&body))?;

    let client = Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("User-Agent", "wechat-vendor-sdk/0.0.0")
        .header("Authorization", format!("WECHATPAY2-SHA256-RSA2048 mchid=\"{}\",nonce_str=\"{}\",signature=\"{}\",timestamp=\"{}\",serial_no=\"{}\"", 
            mchid, nonce_str, signature, timestamp, mch_serial_no))
        .body(body)
        .send()
        .await?;
    let status = response.status();
    if status == 204 {
        Ok(())
    } else {
        let result: WxPayFailedResponse = response.json().await?;
        Err(WxpayApiError::WxpayError(result))
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged, rename_all = "PascalCase")]
pub enum OrderId {
    OutTradeNo(String),
    TransactionId(String),
}

#[derive(Debug, Serialize)]
pub struct RefundAmount<'a> {
    pub refund: i32,
    pub from: Option<Value>,
    pub total: i32,
    pub currency: &'a str,
}
#[derive(Debug, Serialize)]
pub struct RefundOrderRequestBody<'a> {
    #[serde(flatten)]
    pub order_id: OrderId,

    pub out_refund_no: &'a str,
    pub reason: Option<&'a str>,
    pub notify_url: Option<&'a str>,
    pub funds_account: Option<&'a str>,
    pub amount: RefundAmount<'a>,
    pub goods_detail: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefundOrderResponseData {
    pub refund_id: String,
    pub out_refund_no: String,
    pub transaction_id: String,
    pub out_trade_no: String,
    pub channel: String,
    pub user_received_account: String,
    pub success_time: Option<String>,
    pub create_time: String,
    pub status: String,
    pub funds_account: String,
    pub amount: Value,
    pub promotion_detail: Option<Vec<Value>>,
}

/// [退款申请](https://pay.weixin.qq.com/doc/v3/merchant/4012791903)
#[bon::builder]
pub async fn request_refund_order<'a>(
    body: RefundOrderRequestBody<'a>,
    mchid: &'a str,
    mch_private_key: &'a str,
    mch_serial_no: &'a str,
) -> Result<RefundOrderResponseData, WxpayApiError> {
    let url_base = "https://api.mch.weixin.qq.com";
    let endpoint = "/v3/refund/domestic/refunds";
    let url = format!("{}{}", url_base, endpoint);
    let method = "POST";

    let body = serde_json::to_string(&body).expect("failed to serialize body");

    let (signature, timestamp, nonce_str) =
        generate_wxpay_request_signature(method, endpoint, mch_private_key, Some(&body))?;

    let client = Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("User-Agent", "wechat-vendor-sdk/0.0.0")
        .header("Authorization", format!("WECHATPAY2-SHA256-RSA2048 mchid=\"{}\",nonce_str=\"{}\",signature=\"{}\",timestamp=\"{}\",serial_no=\"{}\"", 
            mchid, nonce_str, signature, timestamp, mch_serial_no))
        .body(body)
        .send()
        .await?;

    let status = response.status();
    if status == 200 {
        let result: RefundOrderResponseData = response.json().await?;
        Ok(result)
    } else {
        let result: WxPayFailedResponse = response.json().await?;
        Err(WxpayApiError::WxpayError(result))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefundDetailResponseData {
    pub refund_id: String,
    pub out_refund_no: String,
    pub transaction_id: String,
    pub out_trade_no: String,
    pub channel: String,
    pub user_received_account: String,
    pub success_time: Option<String>,
    pub create_time: String,
    pub status: String,
    pub funds_account: String,
    pub amount: Value,
    pub promotion_detail: Option<Vec<Value>>,
}
/// [查询单笔退款（通过商户退款单号）](https://pay.weixin.qq.com/doc/v3/merchant/4012791904)
#[bon::builder]
pub async fn request_refund_detail<'a>(
    out_refund_no: &'a str,
    mchid: &'a str,
    mch_private_key: &'a str,
    mch_serial_no: &'a str,
) -> Result<RefundDetailResponseData, WxpayApiError> {
    let url_base = "https://api.mch.weixin.qq.com";
    let endpoint = format!("/v3/refund/domestic/refunds/{}", out_refund_no);
    let url = format!("{}{}", url_base, endpoint);
    let method = "GET";

    let (signature, timestamp, nonce_str) =
        generate_wxpay_request_signature(method, &endpoint, mch_private_key, None)?;

    let client = Client::new();
    let response = client.get(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("User-Agent", "wechat-vendor-sdk/0.0.0")
        .header("Authorization", format!("WECHATPAY2-SHA256-RSA2048 mchid=\"{}\",nonce_str=\"{}\",signature=\"{}\",timestamp=\"{}\",serial_no=\"{}\"", 
            mchid, nonce_str, signature, timestamp, mch_serial_no))
        .send()
        .await?;

    let status = response.status();
    if status == 200 {
        let result: RefundDetailResponseData = response.json().await?;
        Ok(result)
    } else {
        let result: WxPayFailedResponse = response.json().await?;
        Err(WxpayApiError::WxpayError(result))
    }
}

#[test]
fn test_generate_wxpay_signature() {
    let private_key = "-----BEGIN PRIVATE KEY-----
...
-----END PRIVATE KEY-----
";
    let method = "POST";
    let url_path = "/v3/transfer/batches";

    match generate_wxpay_request_signature(method, url_path, private_key, Some(r#"body"#)) {
        Ok((signature, timestamp, nonce_str)) => {
            println!("签名: {}", signature);
            println!("时间戳: {}", timestamp);
            println!("随机字符串: {}", nonce_str);
        }
        Err(e) => println!("生成签名失败: {}", e),
    }
}

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

use base64::{engine::general_purpose, Engine};
use chrono::TimeZone;
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    Pkcs1v15Sign,
};
use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BatchTransferRequest {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct WxPayFailedResponse {
    pub code: String,
    pub message: String,
    pub detail: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WxpayResponse<D> {
    Failed(WxPayFailedResponse),
    Success(D),
}

pub struct RequestBatchTransfer<'a> {
    // 商户号
    pub mchid: &'a str,
    // 商户私钥
    pub mch_private_key: &'a str,
    // 商户证书序列号
    pub mch_serial_no: &'a str,
    // 微信支付平台证书序列号
    pub wxpay_serial_no: &'a str,
    // 请求体
    pub request: BatchTransferRequest,
}

/// 发起批量转账接口
pub async fn request_batch_transfer<'a>(
    RequestBatchTransfer {
        mchid,
        mch_private_key,
        mch_serial_no,
        wxpay_serial_no,
        request,
    }: RequestBatchTransfer<'a>,
) -> Result<WxpayResponse<serde_json::Value>, Box<dyn std::error::Error>> {
    let url = "https://api.mch.weixin.qq.com/v3/transfer/batches";
    let method = "POST";

    let body = serde_json::to_string(&request)?;

    let (signature, timestamp, nonce_str) =
        generate_wxpay_signature(method, "/v3/transfer/batches", mch_private_key, Some(&body))?;

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

    let result: WxpayResponse<serde_json::Value> = response.json().await?;
    Ok(result)
}

/// 生成微信支付API请求签名
pub fn generate_wxpay_signature(
    method: &str,
    url_path: &str,
    private_key: &str,
    body: Option<&str>,
) -> Result<(String, String, String), Box<dyn std::error::Error>> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    let nonce_str = generate_noncestr(32);

    let mut content_to_sign = format!("{}\n{}\n{}\n{}\n", method, url_path, timestamp, nonce_str);

    if let Some(body_content) = body {
        content_to_sign.push_str(body_content);
        content_to_sign.push('\n');
    }
    println!("{content_to_sign}");

    let signature = sha256_ras_and_base64(private_key, &content_to_sign);

    Ok((signature, timestamp, nonce_str))
}

/// 随机字符串生成
pub fn generate_noncestr(length: usize) -> String {
    use rand::{distributions::Alphanumeric, Rng};

    let noncestr: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    noncestr
}

pub fn sha256_ras_and_base64(private_key: &str, content_to_be_signed: &str) -> String {
    use rsa::sha2::{Digest, Sha256};
    use rsa::RsaPrivateKey;

    let mut hasher = Sha256::new();
    hasher.update(content_to_be_signed);
    let hash = hasher.finalize();

    // 这里使用 from_pkcs8_pem 还是 from_pkcs1_pem 可以看你密钥文件的文件开头格式
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key).expect("failed to parse ");
    // Pkcs1v15Sign 是使用 PKCS#1 v1.5 规范进行签名，还有个 SigningKey 是用于生产签名所用的秘钥的，并不是用来签名的，所以这里不能用错..
    let padding = Pkcs1v15Sign::new::<rsa::sha2::Sha256>();
    let signature = private_key.sign(padding, &hash).expect("failed to sign");

    let sig = general_purpose::STANDARD.encode(&signature);
    sig
}

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
) -> Result<bool, Box<dyn std::error::Error>> {
    use base64::{engine::general_purpose, Engine as _};
    use rsa::sha2::{Digest, Sha256};
    use rsa::RsaPublicKey;

    if skip_timestamp_check.unwrap_or(false) == false {
        // 验证时间戳
        use chrono::{Duration, Utc};
        let current_time = Utc::now();
        let timestamp_secs = timestamp
            .parse::<i64>()
            .map_err(|e| format!("invalid timestamp: {}", e))?;
        let timestamp_datetime = Utc
            .timestamp_opt(timestamp_secs, 0)
            .single()
            .ok_or("timestamp is invalid")?;

        let time_diff = current_time - timestamp_datetime;
        if time_diff > Duration::seconds(300) {
            tracing::error!("timestamp is expired");
            println!("timestamp is expired");
            return Ok(false);
        }
    }

    let signature_bytes = general_purpose::STANDARD.decode(signature)?;

    let message = format!("{}\n{}\n{}\n", timestamp, nonce, body);
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hash = hasher.finalize();

    let public_key = RsaPublicKey::from_public_key_pem(wx_public_key)?;
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

#[test]
fn test_generate_wxpay_signature() {
    let private_key = "-----BEGIN PRIVATE KEY-----
...
-----END PRIVATE KEY-----
";
    let method = "POST";
    let url_path = "/v3/transfer/batches";

    match generate_wxpay_signature(method, url_path, private_key, Some(r#"body"#)) {
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

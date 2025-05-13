use base64::{engine::general_purpose, Engine};
use rsa::{pkcs8::DecodePrivateKey, Pkcs1v15Sign};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use super::error::WxpayApiError;

/// 生成微信支付API请求签名
pub fn generate_wxpay_request_signature(
    method: &str,
    url_path: &str,
    private_key: &str,
    body: Option<&str>,
) -> Result<(String, String, String), WxpayApiError> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    let nonce_str = generate_noncestr(32);

    let mut content_to_sign = format!("{}\n{}\n{}\n{}\n", method, url_path, timestamp, nonce_str);

    if let Some(body_content) = body {
        content_to_sign.push_str(body_content);
    }
    content_to_sign.push('\n');

    let signature = sha256_ras_and_base64(private_key, &content_to_sign)?;

    Ok((signature, timestamp, nonce_str))
}

/// 生成微信支付调起支付签名
pub fn generate_wxpay_pay_signature(
    app_id: &str,
    prepay_id: &str,
    private_key: &str,
) -> Result<(String, String, String), WxpayApiError> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    let nonce_str = generate_noncestr(32);

    let content_to_sign = format!(
        "{}\n{}\n{}\nprepay_id={}\n",
        app_id, timestamp, nonce_str, prepay_id
    );

    let signature = sha256_ras_and_base64(private_key, &content_to_sign)?;

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

pub fn sha256_ras_and_base64(
    private_key: &str,
    content_to_be_signed: &str,
) -> Result<String, WxpayApiError> {
    use rsa::sha2::{Digest, Sha256};
    use rsa::RsaPrivateKey;

    let mut hasher = Sha256::new();
    hasher.update(content_to_be_signed);
    let hash = hasher.finalize();

    // 这里使用 from_pkcs8_pem 还是 from_pkcs1_pem 可以看你密钥文件的文件开头格式
    let private_key =
        RsaPrivateKey::from_pkcs8_pem(private_key).map_err(|e| WxpayApiError::InvalidPrivateKey)?;
    // Pkcs1v15Sign 是使用 PKCS#1 v1.5 规范进行签名，还有个 SigningKey 是用于生产签名所用的秘钥的，并不是用来签名的，所以这里不能用错..
    let padding = Pkcs1v15Sign::new::<rsa::sha2::Sha256>();
    let signature = private_key.sign(padding, &hash).expect("failed to sign");

    let sig = general_purpose::STANDARD.encode(&signature);
    Ok(sig)
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

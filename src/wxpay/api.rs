use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::error::{WxPayFailedResponse, WxpayApiError};
use super::utils::generate_wxpay_request_signature;

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

/// [发起商家转账](https://pay.weixin.qq.com/doc/v3/merchant/4012458841)
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

#[derive(Debug, Serialize)]
pub struct JsapiOrderRequestBody<'a> {
    // 小程序appid
    pub appid: &'a str,
    // 商户号
    pub mchid: &'a str,
    pub description: &'a str,
    pub out_trade_no: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<&'a str>,
    pub notify_url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_fapiao: Option<bool>,
    pub amount: JsapiOrderAmount<'a>,
    pub payer: JsapiOrderPayer<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct JsapiOrderAmount<'a> {
    pub total: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<&'a str>,
}

#[derive(Debug, Serialize)]
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
pub async fn request_order_detail_by_out_trade_no<'a>(
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
#[serde(rename_all = "snake_case")]
pub enum OrderId {
    OutTradeNo(String),
    TransactionId(String),
}

#[derive(Debug, Serialize)]
pub struct RefundAmount<'a> {
    pub refund: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<Value>>,
    pub total: i32,
    pub currency: &'a str,
}
#[derive(Debug, Serialize)]
pub struct RefundOrderRequestBody<'a> {
    #[serde(flatten)]
    pub order_id: OrderId,

    pub out_refund_no: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funds_account: Option<&'a str>,
    pub amount: RefundAmount<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

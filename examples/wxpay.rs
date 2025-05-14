use serde_json::json;
use wechat_vendor_sdk::wxpay::api::{
    request_batch_transfer, request_close_order, request_jsapi_order,
    request_order_detail_by_out_trade_no, request_refund_detail, request_refund_order,
    BatchTransferRequestBody, CloseOrderRequestBody, JsapiOrderAmount, JsapiOrderPayer,
    JsapiOrderRequestBody, OrderId, OutTradeNoResponseData, RefundAmount, RefundDetailResponseData,
    RefundOrderRequestBody, RefundOrderResponseData, TransferDetail,
};

#[tokio::main]
async fn main() {
    // call test below
}

#[macro_export]
macro_rules! read_test_file {
    ($filename:expr) => {
        std::fs::read_to_string(format!(
            "{}/wxpay_local_test_data/{}",
            env!("CARGO_MANIFEST_DIR"),
            $filename
        ))
        .unwrap()
        .as_str()
    };
}

#[tokio::test]
async fn test_batch_transfer() {
    let res = request_batch_transfer()
        .body(BatchTransferRequestBody {
            appid: "xxx".to_string(),
            out_batch_no: "testbatch1".to_string(),
            batch_name: "test batch name".to_string(),
            batch_remark: "test batch remark".to_string(),
            total_amount: 10,
            total_num: 1,
            transfer_detail_list: vec![TransferDetail {
                openid: "xxx".into(),
                out_detail_no: "testbatch1detail1".to_string(),
                transfer_amount: 10,
                transfer_remark: "transfer item remark ".to_string(),
                user_name: None,
            }],
            transfer_scene_id: Some("1000".into()),
            notify_url: Some("https://xxx".into()),
        })
        .mch_private_key(read_test_file!("mch_private_key"))
        .mch_serial_no(read_test_file!("mch_serial_no"))
        .mchid(read_test_file!("mchid"))
        .wxpay_serial_no(read_test_file!("wxpay_serial_no"))
        .call()
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn test_jsapi_order() {
    let res = request_jsapi_order()
        .body(JsapiOrderRequestBody {
            appid: "xxx",
            notify_url: "https://xxx",
            mchid: "xxx",
            description: "test description",
            out_trade_no: "testouttrade2",
            time_expire: Some("2025-05-20T13:29:35+08:00"),
            attach: None,
            goods_tag: None,
            support_fapiao: None,
            amount: JsapiOrderAmount {
                total: 1,
                currency: None,
            },
            payer: JsapiOrderPayer { openid: "xxx" },
            detail: None,
            scene_info: None,
            settle_info: None,
        })
        .mch_private_key(read_test_file!("mch_private_key"))
        .mch_serial_no(read_test_file!("mch_serial_no"))
        .mchid(read_test_file!("mchid"))
        .call()
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn test_order_detail() {
    let res = request_order_detail_by_out_trade_no()
        .out_trade_no("testouttrade2")
        .mch_private_key(read_test_file!("mch_private_key"))
        .mch_serial_no(read_test_file!("mch_serial_no"))
        .mchid(read_test_file!("mchid"))
        .call()
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn test_close_order() {
    let res = request_close_order()
        .body(CloseOrderRequestBody {
            mchid: read_test_file!("mchid"),
        })
        .out_trade_no("testouttrade2")
        .mch_private_key(read_test_file!("mch_private_key"))
        .mch_serial_no(read_test_file!("mch_serial_no"))
        .mchid(read_test_file!("mchid"))
        .call()
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn test_refund_order() {
    let res = request_refund_order()
        .body(RefundOrderRequestBody {
            order_id: OrderId::OutTradeNo("order id".to_string()),
            out_refund_no: "testrefund1",
            reason: Some("test reason"),
            funds_account: None,
            goods_detail: None,
            amount: RefundAmount {
                refund: 1,
                from: None,
                total: 1,
                currency: "CNY",
            },
            notify_url: Some("xxx"),
        })
        .mch_private_key(read_test_file!("mch_private_key"))
        .mch_serial_no(read_test_file!("mch_serial_no"))
        .mchid(read_test_file!("mchid"))
        .call()
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn test_refund_detail() {
    let res = request_refund_detail()
        .out_refund_no("testrefund1")
        .mch_private_key(read_test_file!("mch_private_key"))
        .mch_serial_no(read_test_file!("mch_serial_no"))
        .mchid(read_test_file!("mchid"))
        .call()
        .await;
    println!("res: {:?}", res);
}

use serde_json::json;
use wechat_vendor_sdk::wxpay::{
    request_batch_transfer, request_close_order, request_jsapi_order, request_order_detail,
    request_refund_detail, request_refund_order, BatchTransferRequestBody, CloseOrderRequestBody,
    JsapiOrderAmount, JsapiOrderPayer, JsapiOrderRequestBody, OutTradeNoResponseData,
    RefundDetailResponseData, TransferDetail,
};

#[tokio::main]
async fn main() {
    // call test below
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
        .mch_private_key(
            std::fs::read_to_string("../wxpay_local_test_data/mch_private_key")
                .unwrap()
                .as_str(),
        )
        .mch_serial_no(
            std::fs::read_to_string("../wxpay_local_test_data/mch_serial_no")
                .unwrap()
                .as_str(),
        )
        .mchid(
            std::fs::read_to_string("../wxpay_local_test_data/mchid")
                .unwrap()
                .as_str(),
        )
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
            time_expire: "2025-05-20T13:29:35+08:00",
            attach: "",
            goods_tag: "",
            support_fapiao: false,
            amount: JsapiOrderAmount {
                total: 1,
                currency: "CNY",
            },
            payer: JsapiOrderPayer { openid: "xxx" },
            detail: json!({
                "goods_detail": json!([
                    {
                        "merchant_goods_id": "1234567890",
                        "quantity": 1,
                        "unit_price": 2,
                    }
                ])
            }),
            scene_info: json!({
                "payer_client_ip": "127.0.0.1",
            }),
            settle_info: json!({}),
        })
        .mch_private_key(
            std::fs::read_to_string("../wxpay_local_test_data/mch_private_key")
                .unwrap()
                .as_str(),
        )
        .mch_serial_no(
            std::fs::read_to_string("../wxpay_local_test_data/mch_serial_no")
                .unwrap()
                .as_str(),
        )
        .mchid(
            std::fs::read_to_string("../wxpay_local_test_data/mchid")
                .unwrap()
                .as_str(),
        )
        .call()
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn test_order_detail() {
    let res = request_order_detail()
        .out_trade_no("testouttrade2")
        .mch_private_key(
            std::fs::read_to_string("../wxpay_local_test_data/mch_private_key")
                .unwrap()
                .as_str(),
        )
        .mch_serial_no(
            std::fs::read_to_string("../wxpay_local_test_data/mch_serial_no")
                .unwrap()
                .as_str(),
        )
        .mchid(
            std::fs::read_to_string("../wxpay_local_test_data/mchid")
                .unwrap()
                .as_str(),
        )
        .call()
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn test_close_order() {
    let res = request_close_order()
        .body(CloseOrderRequestBody {
            mchid: std::fs::read_to_string("../wxpay_local_test_data/mchid")
                .unwrap()
                .as_str(),
        })
        .out_trade_no("testouttrade2")
        .mch_private_key(
            std::fs::read_to_string("../wxpay_local_test_data/mch_private_key")
                .unwrap()
                .as_str(),
        )
        .mch_serial_no(
            std::fs::read_to_string("../wxpay_local_test_data/mch_serial_no")
                .unwrap()
                .as_str(),
        )
        .mchid(
            std::fs::read_to_string("../wxpay_local_test_data/mchid")
                .unwrap()
                .as_str(),
        )
        .call()
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn test_refund_detail() {
    let res = request_refund_detail()
        .out_refund_no("testrefund2")
        .mch_private_key(
            std::fs::read_to_string("../wxpay_local_test_data/mch_private_key")
                .unwrap()
                .as_str(),
        )
        .mch_serial_no(
            std::fs::read_to_string("../wxpay_local_test_data/mch_serial_no")
                .unwrap()
                .as_str(),
        )
        .mchid(
            std::fs::read_to_string("../wxpay_local_test_data/mchid")
                .unwrap()
                .as_str(),
        )
        .call()
        .await;
    println!("res: {:?}", res);
}

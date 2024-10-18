use serde_json::json;
use wechat_vendor_sdk::wxpay::{
    request_batch_transfer, BatchTransferRequest, RequestBatchTransfer, TransferDetail,
};

#[tokio::main]
async fn main() {
    // call test below
}

#[tokio::test]
async fn wxpay() {
    let res = request_batch_transfer(RequestBatchTransfer {
        mchid: "xxx",
        mch_private_key: "-----BEGIN PRIVATE KEY-----
xxx
-----END PRIVATE KEY-----
",
        request: BatchTransferRequest {
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
        },
        mch_serial_no: "xxx",
        wxpay_serial_no: "yyy",
    })
    .await;
    println!("res: {:?}", res);
}

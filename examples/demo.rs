use serde_json::json;
use wechat_vendor_sdk::{
    wxcorp::WxcorpClient,
    wxmini::{
        ActionInfo, ActionScene, MenuCreateRequestBody, MessageTemplateSendMiniprogramData,
        MessageTemplateSendRequestBody, QrcodeCreateRequestBody, StableAccessTokenRequestBody,
        TagsCreateRequestBody, TagsCreateRequestItem, TagsMembersBatchtaggingRequestBody,
        UserInfoBatchgetItem, WxminiClient,
    },
    wxpay::{request_batch_transfer, BatchTransferRequest, RequestBatchTransfer, TransferDetail},
};

#[tokio::main]
async fn main() {
    // call test below
}

#[tokio::test]
async fn without_https() {
    let wxmini_client_http = WxminiClient::without_https();
}

#[tokio::test]
async fn wxcorp() {
    let wxcorp_client = WxcorpClient::new();
    let res = wxcorp_client
        .request_user_id_by_auth_code(Some("token"), Some("code"))
        .await;
}

#[tokio::test]
async fn at() {
    let wxmini_client = WxminiClient::new();
    let at = wxmini_client
        .request_access_token(Some("xx"), Some("xx"), Some("client_credential"))
        .await
        .unwrap();
    println!("at: {:?}", at);
}

#[tokio::test]
async fn stable_at() {
    let wxmini_client = WxminiClient::new();
    let at = wxmini_client
        .request_stable_access_token(&StableAccessTokenRequestBody {
            grant_type: "client_credential".into(),
            appid: "xx".into(),
            secret: "yy".into(),
            force_refresh: Some(false),
        })
        .await
        .unwrap();
    println!("stable at: {:?}", at);
}

const WXMINI_ACCESS_TOKEN: &str = "TOKEN_FOR_TEST";

#[tokio::test]
async fn qcode() {
    let wxmini_client = WxminiClient::new();
    let res = wxmini_client
        .request_qrcode_create(
            &QrcodeCreateRequestBody {
                expire_seconds: Some(10),
                action_name: "QR_SCENE".to_string(),
                action_info: ActionInfo {
                    scene: ActionScene::Str {
                        scene_str: "u1".to_string(),
                    },
                },
            },
            None,
        )
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn menu() {
    let wxmini_client = WxminiClient::new();

    let res = wxmini_client
        .request_menu_create(
            &MenuCreateRequestBody {
                button: vec![json!({
                    "type":"miniprogram",
                    "name":"小程序",
                    "appid":"wxxxcxxxxx",
                    "pagepath":"pages/index/index"
                })],
            },
            Some("token"),
        )
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn tags_get() {
    let wxmini_client = WxminiClient::new();

    let res = wxmini_client
        .request_tags_get(Some(WXMINI_ACCESS_TOKEN))
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn tags_create() {
    let wxmini_client = WxminiClient::new();

    let res = wxmini_client
        .request_tags_create(
            &TagsCreateRequestBody {
                tag: TagsCreateRequestItem {
                    name: "tag_test".into(),
                },
            },
            Some("token"),
        )
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn tags_batch_tagging() {
    let wxmini_client = WxminiClient::new();

    let res = wxmini_client
        .request_tags_members_batchtagging(
            &TagsMembersBatchtaggingRequestBody {
                openid_list: vec!["oEY-A6FOgegq_hZwTTOHpJHcfcok".into()],
                tagid: 101,
            },
            Some(WXMINI_ACCESS_TOKEN),
        )
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn user_list() {
    let wxmini_client = WxminiClient::new();

    let res = wxmini_client
        .request_user_get(
            Some(WXMINI_ACCESS_TOKEN),
            Some("oEY-A6JiOWCW0KecJy-ZAUOOrxbs"),
        )
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn message_template_send() {
    let wxmini_client = WxminiClient::new();

    let res = wxmini_client
        .request_message_template_send(
            &MessageTemplateSendRequestBody {
                touser: "oEY-A6JvR_H2rfKe-rIXsMvbI-j0".into(),
                template_id: "XL6xXLRsgirejTfPX1h73IJ3XVnO9ySmFcZ7xEgXpes".into(),
                url: Some("https://site.com".into()),
                miniprogram: Some(MessageTemplateSendMiniprogramData {
                    appid: "wxxx".into(),
                    pagepath: Some("pages/goods/index".into()),
                }),
                client_msg_id: None,
                data: json!({
                    "thing9":{
                       "value":"用户昵称"
                    },
                    "time6":{
                       "value":"15:01"
                    },
                    "thing17":{
                       "value":"这是内容"
                    },
                }),
            },
            Some(WXMINI_ACCESS_TOKEN),
        )
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn user_info() {
    let wxmini_client = WxminiClient::new();

    let res = wxmini_client
        .request_user_info_batchget(
            &wechat_vendor_sdk::wxmini::UserInfoBatchgetRequestBody {
                user_list: vec![UserInfoBatchgetItem {
                    openid: "oEY-A6JvR_H2rfKe-rIXsMvbI-j0".into(),
                    lang: None,
                }],
            },
            Some(WXMINI_ACCESS_TOKEN),
        )
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn jsticket() {
    let wxmini_client = WxminiClient::new();

    let res = wxmini_client
        .request_getticket(Some(WXMINI_ACCESS_TOKEN))
        .await;
    println!("res: {:?}", res);
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

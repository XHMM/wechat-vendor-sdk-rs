use serde_json::json;
use wechat_vendor_sdk::{
    wxcorp::WxcorpClient,
    wxmini::{
        ActionInfo, ActionScene, MenuCreateRequestBody, QrcodeCreateRequestBody,
        TagsCreateRequestBody, TagsCreateRequestItem, TagsMembersBatchtaggingRequestBody,
        WxminiClient,
    },
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
        .request_user_get(Some(WXMINI_ACCESS_TOKEN), None)
        .await;
    println!("res: {:?}", res);
}

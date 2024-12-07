use serde_json::json;
use wechat_vendor_sdk::wxmini::{
    ActionInfo, ActionScene, CreatewxaqrcodeRequestBody, GenerateSchemeJumpWxa,
    GenerateSchemeRequestBody, GenerateShortLinkRequestBody, GetwxacodeUnlimitRequestBody,
    MenuCreateRequestBody, MessageTemplateSendMiniprogramData, MessageTemplateSendRequestBody,
    QrcodeCreateRequestBody, ScanQrcodeRequestBody, StableAccessTokenRequestBody,
    TagsCreateRequestBody, TagsCreateRequestItem, TagsMembersBatchtaggingRequestBody,
    UserInfoBatchgetItem, WxminiClient,
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
        .request_getticket(Some(WXMINI_ACCESS_TOKEN), None)
        .await;
    println!("res: {:?}", res);
}

#[tokio::test]
async fn request_getwxacodeunlimit() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let wxmini_client = WxminiClient::new();
    let res = wxmini_client
        .request_getwxacodeunlimit(
            &GetwxacodeUnlimitRequestBody {
                scene: "_s=is&st=g&sid=1".into(),
                page: Some("pages/post/index".into()),
                check_path: Some(false),
                env_version: None,
                width: None,
                auto_color: None,
                line_color: None,
                is_hyaline: None,
            },
            Some("token"),
        )
        .await;

    println!("res: {:?}", res);
}

#[tokio::test]
async fn request_generatescheme() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let wxmini_client = WxminiClient::new();
    let res = wxmini_client
        .request_generatescheme(
            &GenerateSchemeRequestBody {
                jump_wxa: Some(GenerateSchemeJumpWxa {
                    path: Some("/pages/moment/index".into()),
                    query: None,
                    env_version: None,
                }),
                expire_time: None,
                expire_type: None,
                expire_interval: None,
            },
            Some("token"),
        )
        .await;

    println!("res: {:?}", res);
}

#[tokio::test]
async fn request_genwxashortlink() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let wxmini_client = WxminiClient::new();
    let res = wxmini_client
        .request_genwxashortlink(
            &GenerateShortLinkRequestBody {
                page_url: "pages/moment/index".into(),
                page_title: None,
                is_permanent: Some(false),
            },
            Some("token"),
        )
        .await;

    println!("res: {:?}", res);
}

#[tokio::test]
async fn request_scan_qrcode() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let wxmini_client = WxminiClient::new();
    let res = wxmini_client
        .request_scan_qrcode(
            &ScanQrcodeRequestBody {
                img_url: "https://xxx.jpg".into(),
            },
            Some("token"),
        )
        .await;

    println!("res: {:?}", res);
}

#[tokio::test]
async fn request_createwxaqrcode() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let wxmini_client = WxminiClient::new();
    let res = wxmini_client
        .request_createwxaqrcode(
            &CreatewxaqrcodeRequestBody {
                path: "pages/moment/index".into(),
                width: None,
            },
            Some("token"),
        )
        .await;

    println!("res: {:?}", res);
}

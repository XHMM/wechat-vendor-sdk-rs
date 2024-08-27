use wechat_vendor_sdk::{
    wxcorp::WxcorpClient,
    wxmini::{
        ActionInfo, ActionScene, MenuCreateRequestBody, QrcodeCreateRequestBody, WxminiClient,
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
async fn at() {
    let wxmini_client = WxminiClient::new();
    let at = wxmini_client
        .request_access_token("xx", "xx", "client_credential")
        .await
        .unwrap();
    println!("at: {:?}", at);
}

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
async fn wxcorp() {
    let wxcorp_client = WxcorpClient::new();
    let res = wxcorp_client
        .request_user_id_by_auth_code("token", "code")
        .await;
}

use wechat_vendor_sdk::{
    wxcorp::WxcorpClient,
    wxmini::{ActionInfo, ActionScene, QrcodeCreateRequestBody, WxminiClient},
};

#[tokio::main]
async fn main() {
    {
        let wxmini_client = WxminiClient::new();

        let at = wxmini_client
            .request_access_token("xx", "xx", "client_credential")
            .await
            .unwrap();
        println!("at: {:?}", at);

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

    {
        let wxmini_client_http = WxminiClient::without_https();
    }

    {
        let wxcorp_client = WxcorpClient::new();
        // let res = wxcorp_client
        //     .request_user_id_by_auth_code("token", "code")
        //     .await;
    }
}

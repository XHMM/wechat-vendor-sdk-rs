use wechat_vendor_sdk::{
    wxcorp::WxcorpClient,
    wxmini::{
        ActionInfo, ActionScene, QrcodeCreateRequestBody, UploadfileRequestBody, WxminiClient,
    },
};

#[tokio::main]
async fn main() {
    {
        let wxmini_client = WxminiClient::new();
        let res = wxmini_client
            .request_uploadfile(
                &UploadfileRequestBody {
                    env: "xx".into(),
                    path: "xx".into(),
                },
                Some("xx"),
            )
            .await;

        let res = wxmini_client
            .request_qrcode_create(
                &QrcodeCreateRequestBody {
                    expire_seconds: None,
                    action_name: "QR_LIMIT_SCENE".to_string(),
                    action_info: ActionInfo {
                        scene: ActionScene::Id { scene_id: 1 },
                    },
                },
                None,
            )
            .await;
    }

    {
        let wxmini_client_http = WxminiClient::without_https();
    }

    {
        let wxcorp_client = WxcorpClient::new();
        let res = wxcorp_client
            .request_user_id_by_auth_code("token", "code")
            .await;
    }
}

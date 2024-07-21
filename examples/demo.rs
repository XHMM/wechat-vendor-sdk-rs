use wechat_vendor_sdk::{
    wxcorp::WxcorpClient,
    wxmini::{UploadfileRequestBody, WxminiClient},
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

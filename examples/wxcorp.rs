use serde_json::json;
use wechat_vendor_sdk::wxcorp::WxcorpClient;

#[tokio::main]
async fn main() {
    // call test below
}

#[tokio::test]
async fn get_id_by_auth_code() {
    let wxcorp_client = WxcorpClient::new();
    let res = wxcorp_client
        .request_user_id_by_auth_code(Some("token"), Some("code"))
        .await;
}

#[tokio::test]
async fn get_user_info() {
    let wxcorp_client = WxcorpClient::new();
    let res = wxcorp_client
        .request_user_info_by_user_id(Some("token "), Some("nickname"))
        .await;

    println!("res: {:?}", res);
}

#[tokio::test]
async fn send() {
    let wxcorp_client = WxcorpClient::new();
    let res = wxcorp_client
        .request_send(
            &json!({
                "touser": "xx",
                "msgtype" : "text",
                "agentid": "1011112",
                "text": {
                    "content": "test"
                }
            }),
            Some("token"),
        )
        .await;

    println!("res: {:?}", res);
}

use serde_json::json;
use wechat_vendor_sdk::wxcorp::{
    ExternalContactGroupChatGetRequestBody, ExternalContactGroupChatListRequestBody, WxcorpClient,
};

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

#[tokio::test]
async fn webhook_send() {
    let wxcorp_client = WxcorpClient::new();
    let res = wxcorp_client
        .request_webhook_send(
            &json!({
                "msgtype": "text",
                "text": {
                    "content": "hello world"
                }
            }),
            Some("bot key"),
        )
        .await;

    println!("res: {:?}", res);
}

#[tokio::test]
async fn request_external_contact_list() {
    let wxcorp_client = WxcorpClient::new();
    let res = wxcorp_client
        .request_external_contact_list(Some("token"), Some("xx"))
        .await;

    println!("res: {:?}", res);
}

#[tokio::test]
async fn request_external_contact_get() {
    let wxcorp_client = WxcorpClient::new();
    let res = wxcorp_client
        .request_external_contact_get(Some("token"), Some("xxx"), None)
        .await;
    println!("res: {:?}", res);
}

// #[tokio::test]
// async fn request_external_contact_batch_get_by_user() {
//     let wxcorp_client = WxcorpClient::new();
//     let res = wxcorp_client
//         .request_external_contact_batch_get_by_user(Some("token "), Some("nickname"))
//         .await;
// }

#[tokio::test]
async fn request_external_contact_group_chat_list() {
    let wxcorp_client = WxcorpClient::new();
    let res = wxcorp_client
        .request_external_contact_group_chat_list(
            &ExternalContactGroupChatListRequestBody {
                status_filter: None,
                owner_filter: None,
                cursor: None,
                limit: 10,
            },
            Some("token"),
        )
        .await;

    println!("res: {:?}", res);
}

#[tokio::test]
async fn request_external_contact_group_chat_get() {
    let wxcorp_client = WxcorpClient::new();
    let res = wxcorp_client
        .request_external_contact_group_chat_get(
            &ExternalContactGroupChatGetRequestBody {
                chat_id: "xxxx".to_string(),
                need_name: Some(1),
            },
            Some("token"),
        )
        .await;

    println!("res: {:?}", res);
}

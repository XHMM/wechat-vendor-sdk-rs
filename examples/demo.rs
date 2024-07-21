use wechat_vendor_sdk::{wxcorp::WxcorpClient, wxmini::WxminiClient};

fn main() {
    let wxmini_client = WxminiClient::without_https();
    let wxmini_client2 = WxminiClient::new();

    let wxcorpclient = WxcorpClient::new();
}

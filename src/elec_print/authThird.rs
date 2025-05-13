use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{config, utils};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthThirdParam {
    #[serde(rename = "net", default, skip_serializing_if = "String::is_empty")]
    pub net: String,

    #[serde(rename = "callBackUrl", default, skip_serializing_if = "String::is_empty")]
    pub call_back_url: String,

    #[serde(rename = "partnerId", default, skip_serializing_if = "String::is_empty")]
    pub partner_id: String,

    #[serde(rename = "view", default, skip_serializing_if = "String::is_empty")]
    pub view: String,
}
///第三方平台账号授权
pub fn auth_third() -> Result<(), Box<dyn std::error::Error>> {
    let param = AuthThirdParam {
        net: "123".to_string(),
        call_back_url: "www.baidu.com".to_string(),
        partner_id: "123".to_string(),
        view: "web".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param).map_err(|e| e.to_string())?;

    // 生成时间戳
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    // 发送请求
    utils::http_util::do_request(&t, &param_json, config::url::AUTH_URL).map_err(|e| e.to_string())?;

    Ok(())
}
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{config, utils};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShopAuthorizeParam {
    #[serde(rename = "shopType", default, skip_serializing_if = "String::is_empty")]
    pub shop_type: String,

    #[serde(rename = "callbackUrl", default, skip_serializing_if = "String::is_empty")]
    pub callback_url: String,

    #[serde(rename = "salt", default, skip_serializing_if = "String::is_empty")]
    pub salt: String,
}
///获取店铺授权超链接接口
pub fn shop_authorize() -> Result<(), Box<dyn std::error::Error>> {
    let param = ShopAuthorizeParam {
        shop_type: "TAOBAO".to_string(),
        callback_url: "www.baidu.com".to_string(),
        salt: "taobao".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param).map_err(|e| e.to_string())?;

    // 生成时间戳（毫秒）
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis()
        .to_string();

    // 发送请求
    utils::http_util::do_request(
        &t,
        &param_json,
        config::url::SHOP_AUTHORIZE_URL
    ).map_err(|e| e.to_string())?;

    Ok(())
}
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{config, utils};

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryBalanceParam {
    #[serde(rename = "partnerId", default, skip_serializing_if = "String::is_empty")]
    pub partner_id: String,

    #[serde(rename = "partnerKey", default, skip_serializing_if = "String::is_empty")]
    pub partner_key: String,

    #[serde(rename = "net", default, skip_serializing_if = "String::is_empty")]
    pub net: String,

    #[serde(rename = "com", default, skip_serializing_if = "String::is_empty")]
    pub com: String,
}

///第三方平台网点&面单余额接口
pub fn get_third_info() -> Result<(), Box<dyn std::error::Error>> {
    let method = "getThirdInfo";
    
    let param = QueryBalanceParam {
        partner_id: "123".to_string(),
        partner_key: "123".to_string(),
        net: "taobao".to_string(),
        com: "shentong".to_string(),
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
    utils::http_util::do_method_request(
        method,
        &t,
        &param_json,
        config::url::THIRD_INFO_URL
    ).map_err(|e| e.to_string())?;

    Ok(())
}
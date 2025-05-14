use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{config, utils};

#[derive(Serialize, Deserialize, Debug)]
pub struct PollMapParameters {
    #[serde(rename = "callbackurl")]
    pub callback_url: String,

    #[serde(rename = "salt")]
    pub salt: String,

    #[serde(rename = "phone")]
    pub phone: String,

    #[serde(rename = "resultv2")]
    pub result_v2: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PollMapParam {
    #[serde(rename = "company")]
    pub company: String,

    #[serde(rename = "number")]
    pub number: String,

    #[serde(rename = "from")]
    pub from: String,

    #[serde(rename = "to")]
    pub to: String,

    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "parameters")]
    pub parameters: PollMapParameters,
}
/*
 * 轨迹推送
 */
pub fn poll_map() -> Result<(), Box<dyn std::error::Error>> {
    let parameters = PollMapParameters {
        callback_url: "http://www.您的域名.com/kuaidi?callbackid=...".to_string(),
        salt: "*".to_string(),
        phone: "".to_string(),
        result_v2: "5".to_string(),
    };

    let param = PollMapParam {
        company: "ems".to_string(),
        number: "1136281381675".to_string(),
        from: "广东省深圳市南山区".to_string(),
        to: "北京市朝阳区".to_string(),
        key: config::account::KEY.to_string(),
        parameters: parameters,
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param).map_err(|e| e.to_string())?;
    
    let mut m:HashMap<&str, &str> = std::collections::HashMap::new();
    m.insert("schema", "json");
    m.insert("param", &param_json);

    // 发送请求
    utils::http_util::do_map_request(m, config::url::POLL_MAP_URL).map_err(|e| e.to_string())?;

    Ok(())
}
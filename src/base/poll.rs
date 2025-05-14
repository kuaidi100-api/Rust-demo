use std::collections::HashMap;
use crate::{config, utils};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PollParameters {
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
pub struct PollParam {
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
    pub parameters: PollParameters,
}
/*
快递订阅接口
 */
pub fn poll() -> Result<(), Box<dyn std::error::Error>> {
    let parameters = PollParameters {
        callback_url: "http://www.您的域名.com/kuaidi?callbackid=...".to_string(),
        salt: "XXXXXXXXXX".to_string(),
        phone: "13868688888".to_string(),
        result_v2: "4".to_string(),
    };

    // 创建 PollParam 实例
    let param = PollParam {
        company: "yuantong".to_string(),
        number: "YT6186594166532".to_string(),
        from: "".to_string(),
        to: "".to_string(),
        key: config::account::KEY.to_string(),
        parameters: parameters,
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param).map_err(|e| e.to_string())?;

    let mut m:HashMap<&str, &str> = std::collections::HashMap::new();
    m.insert("schema", "json");
    m.insert("param", &param_json);

    // 发送请求
    utils::http_util::do_map_request(m, config::url::POLL_URL)
        .map_err(|e| e.to_string())?;

    Ok(())
}

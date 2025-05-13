use serde::{Deserialize, Serialize};
use crate::config;
use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParam {
    #[serde(rename = "com")]
    pub com: String,
    
    #[serde(rename = "num")]
    pub num: String,
    
    #[serde(rename = "phone")]
    pub phone: String,
    
    #[serde(rename = "from")]
    pub from: String,
    
    #[serde(rename = "to")]
    pub to: String,
    
    #[serde(rename = "resultv2")]
    pub resultv2: String,
    
    #[serde(rename = "show")]
    pub show: String,
    
    #[serde(rename = "order")]
    pub order: String,
    
    #[serde(rename = "lang")]
    pub lang: String,
}

/// 实时快递查询接口
pub fn query() -> Result<(), Box<dyn std::error::Error>> {
    // 创建查询参数
    let param = QueryParam {
        com: "yuantong".to_string(),
        num: "em263999513jp".to_string(),
        phone: "13868688888".to_string(),
        from: "广东省深圳市南山区".to_string(),
        to: "北京市朝阳区".to_string(),
        resultv2: "4".to_string(),
        show: "0".to_string(),
        order: "desc".to_string(),
        lang: "zh".to_string(),
    };
    
    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param).map_err(|e| e.to_string())?;

    // 发送请求
    utils::http_util::customer_request(&param_json, config::url::QUERY_URL).map_err(|e| e.to_string())?;

    Ok(())
}
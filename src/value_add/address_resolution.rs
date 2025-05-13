use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressResolutionParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,

    #[serde(rename = "pdfUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_url: Option<String>,

    #[serde(rename = "htmlUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}

///智能地址解析接口
pub fn address_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let param = AddressResolutionParam {
        content: Some("张三广东省深圳市南山区粤海街道科技南十二路金蝶软件园13088888888".to_string()),
        image: Some("qweasftefds".to_string()),
        image_url: Some("www.imageurl.png".to_string()),
        pdf_url: Some("www.pdfurl.pdf".to_string()),
        html_url: Some("www.htmlurl.html".to_string()),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 发送请求
    utils::http_util::do_request(&timestamp, &param_json, config::url::ADDRESS_RESOLUTION_URL)?;

    Ok(())
}
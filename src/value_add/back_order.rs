use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct BackOrderParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuaidicom: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuaidinum: Option<String>,

    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,

    #[serde(rename = "partnerKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(rename = "imgType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_type: Option<i32>,
}

///运单附件查询接口
pub fn back_order() -> Result<(), Box<dyn std::error::Error>> {
    let param = BackOrderParam {
        kuaidicom: Some("shunfeng".to_string()),
        kuaidinum: Some("SF1234567".to_string()),
        partner_id: Some("1234567".to_string()),
        partner_key: Some("".to_string()),
        phone: Some("13888888888".to_string()),
        img_type: Some(1),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "backOrder";

    // 发送请求
    utils::http_util::do_method_request(method, &timestamp, &param_json, config::url::LABEL_ORDER_URL)?;

    Ok(())
}
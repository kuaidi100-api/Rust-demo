use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::elec_print::elec_common::do_label_order_request;

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelParam {
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,

    #[serde(rename = "partnerKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_key: Option<String>,

    #[serde(rename = "partnerSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_secret: Option<String>,

    #[serde(rename = "partnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuaidicom: Option<String>,

    #[serde(rename = "kuaidinum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuaidinum: Option<String>,

    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_man: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

///电子面单取消接口
pub fn cancel() -> Result<(), Box<dyn std::error::Error>> {
    // 构建请求参数
    let param = CancelParam {
        partner_id: Some("123".to_string()),
        partner_key: Some("123".to_string()),
        partner_secret: Some("123".to_string()),
        partner_name: Some("123".to_string()),
        net: Some("123".to_string()),
        code: Some("123".to_string()),
        kuaidicom: Some("******".to_string()),
        kuaidinum: Some("******".to_string()),
        order_id: Some("123".to_string()),
        check_man: Some("123".to_string()),
        reason: Some("123".to_string()),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "cancel";

    // 发送请求
    let _ = do_label_order_request(method, &timestamp, &param_json)?;

    Ok(())
}
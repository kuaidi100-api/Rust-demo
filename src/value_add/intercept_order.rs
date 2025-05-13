use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecManInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,

    #[serde(rename = "printAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_addr: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InterceptOrderParam {
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    #[serde(rename = "kuaidicom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuaidicom: Option<String>,

    #[serde(rename = "kuaidinum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuaidinum: Option<String>,

    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,

    #[serde(rename = "partnerKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_key: Option<String>,

    #[serde(rename = "partnerSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_secret: Option<String>,

    #[serde(rename = "interceptType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intercept_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    #[serde(rename = "recManInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_info: Option<RecManInfo>,
}

///拦截改址接口
pub fn intercept_order() -> Result<(), Box<dyn std::error::Error>> {
    let rec_man_info = RecManInfo {
        name: Some("测试".to_string()),
        mobile: Some("138888888888".to_string()),
        print_addr: Some("广东深圳市南山区金蝶软件园".to_string()),
    };

    let param = InterceptOrderParam {
        order_id: None,
        kuaidicom: Some("debangkuaidi".to_string()),
        kuaidinum: Some("2222".to_string()),
        partner_id: Some("22222".to_string()),
        partner_key: None,
        partner_secret: None,
        intercept_type: Some("MODIFY_ADDR".to_string()),
        code: None,
        net: None,
        reason: None,
        rec_man_info: Some(rec_man_info),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "interceptOrder";

    // 发送请求
    utils::http_util::do_method_request(method, &timestamp, &param_json, config::url::LABEL_ORDER_URL)?;

    Ok(())
}
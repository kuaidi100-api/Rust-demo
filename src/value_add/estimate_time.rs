use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimateTimeParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuaidicom: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    #[serde(rename = "orderTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_time: Option<String>,

    #[serde(rename = "expType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_type: Option<String>,
}

///快递预估时效查询接口
pub fn estimate_time() -> Result<(), Box<dyn std::error::Error>> {
    let param = EstimateTimeParam {
        kuaidicom: Some("shunfeng".to_string()),
        from: Some("广东深圳南山区".to_string()),
        to: Some("北京海淀区".to_string()),
        order_time: Some("2023-08-08 08:08:08".to_string()),
        exp_type: Some("标准快递".to_string()),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "time";

    // 发送请求
    utils::http_util::do_method_request(method, &timestamp, &param_json, config::url::LABEL_ORDER_URL)?;

    Ok(())
}
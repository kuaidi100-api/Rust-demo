use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{config, utils};

#[derive(Serialize, Deserialize, Debug)]
pub struct RefundOrderTaskParam {
    #[serde(rename = "shopType", default, skip_serializing_if = "String::is_empty")]
    pub shop_type: String,

    #[serde(rename = "shopId", default, skip_serializing_if = "String::is_empty")]
    pub shop_id: String,

    #[serde(rename = "orderStatus", default, skip_serializing_if = "String::is_empty")]
    pub order_status: String,

    #[serde(rename = "updateAtMin", default, skip_serializing_if = "String::is_empty")]
    pub update_at_min: String,

    #[serde(rename = "updateAtMax", default, skip_serializing_if = "String::is_empty")]
    pub update_at_max: String,

    #[serde(rename = "callbackUrl", default, skip_serializing_if = "String::is_empty")]
    pub callback_url: String,

    #[serde(rename = "salt", default, skip_serializing_if = "String::is_empty")]
    pub salt: String,
}

///提交售后（退货）订单获取任务接口
pub fn refund_order_task() -> Result<(), Box<dyn std::error::Error>> {
    let param = RefundOrderTaskParam {
        shop_type: "TAOBAO".to_string(),
        shop_id: "123".to_string(),
        order_status: "UNPAY".to_string(),
        update_at_min: "2025-05-06 11:11:11".to_string(),
        update_at_max: "2025-06-06 11:11:11".to_string(),
        callback_url: "www.baidu.com".to_string(),
        salt: "abc".to_string(),
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
        config::url::REFUND_ORDER_TASK_URL
    ).map_err(|e| e.to_string())?;

    Ok(())
}
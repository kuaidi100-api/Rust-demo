use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct ManInfo {
    pub name: String,
    pub mobile: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendOutParam {
    pub sys_num: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plt_num: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_fee: Option<String>,

    pub pay_time: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_type: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro_delivery_time: Option<String>,

    pub source_platform: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_reseller: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub express_num: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub express_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_area_flag: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<ManInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<ManInfo>,
}

///物流全链路监控 订单发货接口
pub fn send_out() -> Result<(), Box<dyn std::error::Error>> {
    let param = SendOutParam {
        sys_num: "11111".to_string(),
        plt_num: None,
        order_fee: None,
        pay_time: "".to_string(), // 这里需要根据实际需求设置默认值或者使用 Option<String>
        delivery_time: Some("2023-09-04 14:08:00".to_string()),
        delivery_type: None,
        pro_delivery_time: None,
        source_platform: "".to_string(), // 这里需要根据实际需求设置默认值
        source_reseller: None,
        store_name: None,
        warehouse_name: None,
        express_num: Some("1111".to_string()),
        express_code: Some("shunfeng".to_string()),
        remote_area_flag: None,
        receiver: None,
        sender: None,
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "sendOut";

    // 发送请求
    utils::http_util::do_method_request(method, &timestamp, &param_json, config::url::MONITOR_ORDER_URL)?;

    Ok(())
}
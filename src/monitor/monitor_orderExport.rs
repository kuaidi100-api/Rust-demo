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
pub struct OrderExportParam {
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

///物流全链路监控 订单导入接口
pub fn order_export() -> Result<(), Box<dyn std::error::Error>> {
    let param = OrderExportParam {
        sys_num: "SYS123456".to_string(),
        plt_num: Some("PLT789012".to_string()),
        order_fee: Some("199.00".to_string()),
        pay_time: "2024-07-20 15:30:00".to_string(),
        delivery_time: Some("2024-07-21 09:00:00".to_string()),
        delivery_type: Some(0),
        pro_delivery_time: Some("2024-07-22 18:00:00".to_string()),
        source_platform: "JD".to_string(),
        source_reseller: Some("自营".to_string()),
        store_name: Some("官方旗舰店".to_string()),
        warehouse_name: Some("华北仓库".to_string()),
        express_num: Some("JD1234567890".to_string()),
        express_code: Some("jd".to_string()),
        remote_area_flag: Some("0".to_string()),
        receiver: Some(ManInfo {
            name: "张三".to_string(),
            mobile: "13800138000".to_string(),
            address: "北京市朝阳区某某街道123号".to_string(),
        }),
        sender: Some(ManInfo {
            name: "李四".to_string(),
            mobile: "13900139000".to_string(),
            address: "上海市浦东新区某某路456号".to_string(),
        }),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "orderExport";

    // 发送请求
    utils::http_util::do_method_request(method, &timestamp, &param_json, config::url::MONITOR_ORDER_URL)?;

    Ok(())
}
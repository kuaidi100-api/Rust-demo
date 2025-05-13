use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkOrderQueryParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consult_id: Option<String>,
}

///查询工单
pub fn work_order_query() -> Result<(), Box<dyn std::error::Error>> {
    let param = WorkOrderQueryParam {
        consult_id: Some("1".to_string()),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 发送请求
    utils::http_util::do_request(&timestamp, &param_json, config::url::WORK_ORDER_QUERY_URL)?;

    Ok(())
}
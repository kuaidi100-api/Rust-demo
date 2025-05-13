use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::border::border_common::do_border_request;

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailParam {
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// 商家寄件-订单详情
pub fn detail() -> Result<(), Box<dyn std::error::Error>> {
    // 构建请求参数
    let param = DetailParam {
        task_id: Some("9FC293CA417E431F33046E64F4C4EC20".to_string()),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "detail";

    // 发送请求
    let _ = do_border_request(method, &timestamp, &param_json)?;

    Ok(())
}
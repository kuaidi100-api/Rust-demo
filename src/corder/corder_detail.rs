use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::corder::corder_common::do_corder_request;

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailParam {
    #[serde(rename = "taskId")]
    pub task_id: String,
}

/// C端寄件-查询详情
pub fn detail() -> Result<(), Box<dyn std::error::Error>> {
    // 构建请求参数
    let param = DetailParam {
        task_id: "9FC293CA417E431F33046E64F4C4EC20".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "detail";

    // 发送请求
    let _ = do_corder_request(method, &timestamp, &param_json)?;

    Ok(())
}
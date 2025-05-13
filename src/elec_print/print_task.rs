use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{config, utils};

#[derive(Serialize, Deserialize, Debug)]
pub struct PrintTaskParam {
    #[serde(rename = "siid", default, skip_serializing_if = "String::is_empty")]
    pub siid: String,
}

///硬件状态查询接口
pub fn print_task() -> Result<(), Box<dyn std::error::Error>> {
    let method = "devstatus";

    let param = PrintTaskParam {
        siid: "12345678".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param).map_err(|e| e.to_string())?;

    // 生成时间戳（毫秒）
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis()
        .to_string();

    // 发送请求（更新后的调用路径）
    utils::http_util::do_method_request(
        method,
        &t,
        &param_json,
        config::url::PRINT_TASK_URL
    ).map_err(|e| e.to_string())?;

    Ok(())
}
use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::elec_print;
use crate::elec_print::elec_common::do_label_order_request;

#[derive(Serialize, Deserialize, Debug)]
pub struct Param {
    pub siid: String,

    #[serde(rename = "taskId")]
    pub task_id: String,
}

///自定义模板打印复打接口
pub fn print_old() -> Result<(), Box<dyn std::error::Error>> {
    // 构建请求参数
    let param = Param {
        siid: "KX100*******".to_string(),
        task_id: "1234".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "printOld";

    // 发送请求
    let _ = do_label_order_request(method, &timestamp, &param_json)?;

    Ok(())
}
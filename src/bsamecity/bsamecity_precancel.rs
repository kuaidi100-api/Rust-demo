use chrono::Utc;
use crate::bsamecity::bsamecity_common::{do_bsamecity_request, CancelParam};

/// 同城寄件-预取消
pub fn pre_cancel() -> Result<(), Box<dyn std::error::Error>> {
    // 构建请求参数
    let param = CancelParam {
        order_id: Some("100241".to_string()),
        cancel_msg_type: Some(1),
        cancel_msg: Some("测试寄件".to_string()),
        task_id: Some("DE49A5C45B0845328CE0AE8EF9951EC5".to_string()),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "precancel";

    // 发送请求
    let _ = do_bsamecity_request(method, &timestamp, &param_json)?;

    Ok(())
}
use chrono::Utc;
use crate::bsamecity::bsamecity_common::{do_bsamecity_request, AddFeeParam};

/// 同城寄件-添加小费
pub fn add_fee() -> Result<(), Box<dyn std::error::Error>> {
    // 构建请求参数
    let param = AddFeeParam {
        order_id: Some("100213".to_string()),
        remark: Some("".to_string()),
        task_id: Some("0E1536182BAE416080AC3C5DE6896F03".to_string()),
        tips: Some("10".to_string()),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "addfee";

    // 发送请求
    let _ = do_bsamecity_request(method, &timestamp, &param_json)?;

    Ok(())
}
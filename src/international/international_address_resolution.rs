use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct InternationalAddressResolutionParam {
    pub code: Option<String>,
    pub address: Option<String>,
    pub language: Option<String>,
}
///国际地址解析接口
pub fn international_address_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let param = InternationalAddressResolutionParam {
        code: Some("US".to_string()),
        address: Some("84 AlfordRd,GreatBarrington,MA01230,USA".to_string()),
        language: Some("en".to_string()),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 发送请求
    utils::http_util::do_request(&timestamp, &param_json, config::url::INTERNATIONAL_ADDRESS_RESOLUTION_URL)?;

    Ok(())
}
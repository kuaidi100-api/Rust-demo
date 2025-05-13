use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimatePriceParam {
    #[serde(rename = "sendAddr")]
    pub send_addr: String,

    #[serde(rename = "recAddr")]
    pub rec_addr: String,

    #[serde(rename = "kuaidicom")]
    pub kuaidicom: String,

    pub weight: String,
}

///快递预估价格查询接口
pub fn estimate_price() -> Result<(), Box<dyn std::error::Error>> {
    let param = EstimatePriceParam {
        send_addr: "深圳南山区".to_string(),
        rec_addr: "北京海淀区".to_string(),
        kuaidicom: "jd".to_string(),
        weight: "12".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "price";

    // 发送请求
    utils::http_util::do_method_request(method, &timestamp, &param_json, config::url::LABEL_ORDER_URL)?;

    Ok(())
}
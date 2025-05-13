use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::border::border_common::do_border_request;

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuaidicom: Option<String>,

    #[serde(rename = "recManPrintAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_print_addr: Option<String>,

    #[serde(rename = "sendManPrintAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man_print_addr: Option<String>,

    #[serde(rename = "serviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
}

///商家寄件-价格查询
pub fn price() -> Result<(), Box<dyn std::error::Error>> {
    // 构建请求参数
    let param = PriceParam {
        kuaidicom: Some("yuantong".to_string()),
        rec_man_print_addr: Some("广东深圳市深圳市南山区科技南十二路2号金蝶软件园".to_string()),
        send_man_print_addr: Some("广东省广州市华景软件园".to_string()),
        service_type: Some("标准快递".to_string()),
        weight: Some("1".to_string()),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "price";

    // 发送请求
    let _ = do_border_request(method, &timestamp, &param_json)?;

    Ok(())
}
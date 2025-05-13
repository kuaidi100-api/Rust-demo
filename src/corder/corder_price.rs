use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::corder::corder_common::do_corder_request;

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceParam {
    #[serde(rename = "kuaidicom")]
    pub kuaidicom: String,

    #[serde(rename = "sendManPrintAddr")]
    pub send_man_print_addr: String,

    #[serde(rename = "recManPrintAddr")]
    pub rec_man_print_addr: String,
}

///  C端寄件-查询价格
pub fn price() -> Result<(), Box<dyn std::error::Error>> {
    // 构建请求参数
    let param = PriceParam {
        kuaidicom: "shunfeng".to_string(),
        send_man_print_addr: "北京市海淀区颐和园路5号".to_string(),
        rec_man_print_addr: "北京市海淀区双清路30号".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "price";

    // 发送请求
    let _ = do_corder_request(method, &timestamp, &param_json)?;

    Ok(())
}
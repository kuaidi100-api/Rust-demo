use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReachableParam {
    #[serde(rename = "recManPrintAddr")]
    pub rec_man_print_addr: String,

    #[serde(rename = "sendManPrintAddr")]
    pub send_man_print_addr: String,

    #[serde(rename = "recManMobile")]
    pub rec_man_mobile: String,

    #[serde(rename = "sendManName")]
    pub send_man_name: String,

    #[serde(rename = "recManName")]
    pub rec_man_name: String,

    #[serde(rename = "kuaidicom")]
    pub kuaidicom: String,

    #[serde(rename = "sendManMobile")]
    pub send_man_mobile: String,
}

///快递可用性接口
pub fn reachable() -> Result<(), Box<dyn std::error::Error>> {
    let param = ReachableParam {
        rec_man_print_addr: "浙江省湖州市吴兴区织****".to_string(),
        send_man_print_addr: "福建省宁德市霞***".to_string(),
        rec_man_mobile: "****".to_string(),
        send_man_name: "****".to_string(),
        rec_man_name: "***".to_string(),
        kuaidicom: "yuantong".to_string(),
        send_man_mobile: "*****".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "reachable";

    // 发送请求
    utils::http_util::do_method_request(method, &timestamp, &param_json, config::url::REACHABLE_URL)?;

    Ok(())
}
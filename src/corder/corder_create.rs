use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::corder::corder_common::{do_corder_request};


#[derive(Serialize, Deserialize, Debug)]
pub struct CreateParam {
    pub kuaidicom: String,

    #[serde(rename = "recManName")]
    pub rec_man_name: String,

    #[serde(rename = "recManMobile")]
    pub rec_man_mobile: String,

    #[serde(rename = "recManPrintAddr")]
    pub rec_man_print_addr: String,

    #[serde(rename = "sendManName")]
    pub send_man_name: String,

    #[serde(rename = "sendManMobile")]
    pub send_man_mobile: String,

    #[serde(rename = "sendManPrintAddr")]
    pub send_man_print_addr: String,

    pub cargo: String,

    #[serde(rename = "callBackUrl")]
    pub call_back_url: String,

    pub payment: String,

    pub weight: String,

    pub remark: String,

    pub salt: String,

    #[serde(rename = "dayType")]
    pub day_type: String,

    #[serde(rename = "pickupStartTime")]
    pub pickup_start_time: String,

    #[serde(rename = "pickupEndTime")]
    pub pickup_end_time: String,

    pub op: String,

    #[serde(rename = "pollCallBackUrl")]
    pub poll_call_back_url: String,

    #[serde(rename = "resultv2")]
    pub resultv2: String,
}

/// C端寄件-下单
pub fn create() -> Result<(), Box<dyn std::error::Error>> {
    // 构建请求参数
    let param = CreateParam {
        kuaidicom: "yuantong".to_string(),
        rec_man_name: "王超".to_string(),
        rec_man_mobile: "12345678910".to_string(),
        rec_man_print_addr: "西藏日喀则市定日县珠穆朗玛峰".to_string(),
        send_man_name: "王大".to_string(),
        send_man_mobile: "12345678910".to_string(),
        send_man_print_addr: "西藏日喀则市定日县珠穆朗玛峰".to_string(),
        cargo: "文件".to_string(),
        call_back_url: "http://www.baidu.com".to_string(),
        payment: "SHIPPER".to_string(),
        weight: "1".to_string(),
        remark: "".to_string(),
        salt: "".to_string(),
        day_type: "".to_string(),
        pickup_start_time: "".to_string(),
        pickup_end_time: "".to_string(),
        op: "0".to_string(),
        poll_call_back_url: "".to_string(),
        resultv2: "0".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "cOrder";

    // 发送请求
    let _ = do_corder_request(method, &timestamp, &param_json)?;

    Ok(())
}
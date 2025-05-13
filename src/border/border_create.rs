use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::border::border_common::{do_border_request};


#[derive(Serialize, Deserialize, Debug)]
pub struct CreateParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuaidicom: Option<String>,

    #[serde(rename = "recManName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_name: Option<String>,

    #[serde(rename = "recManMobile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_mobile: Option<String>,

    #[serde(rename = "recManPrintAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_print_addr: Option<String>,

    #[serde(rename = "sendManName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man_name: Option<String>,

    #[serde(rename = "sendManMobile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man_mobile: Option<String>,

    #[serde(rename = "sendManPrintAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man_print_addr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo: Option<String>,

    #[serde(rename = "callBackUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_back_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<String>,

    #[serde(rename = "serviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,

    #[serde(rename = "dayType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_type: Option<String>,

    #[serde(rename = "pickupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_start_time: Option<String>,

    #[serde(rename = "pickupEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_end_time: Option<String>,

    #[serde(rename = "passwordSigning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_signing: Option<String>,

    #[serde(rename = "valinsPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valins_pay: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,

    #[serde(rename = "pollCallBackUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_call_back_url: Option<String>,

    #[serde(rename = "resultv2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resultv2: Option<String>,
}

///商家寄件-下单
pub fn create() -> Result<(), Box<dyn std::error::Error>> {
    // 构建请求参数
    let param = CreateParam {
        kuaidicom: Some("yuantong".to_string()),
        rec_man_name: Some("王超".to_string()),
        rec_man_mobile: Some("13842569988".to_string()),
        rec_man_print_addr: Some("广东深圳市深圳市南山区科技南十二路2号金蝶软件园".to_string()),
        send_man_name: Some("王大".to_string()),
        send_man_mobile: Some("13842569988".to_string()),
        send_man_print_addr: Some("广东省广州市华景软件园".to_string()),
        cargo: Some("文件".to_string()),
        call_back_url: Some("http://www.baidu.com".to_string()),
        payment: Some("SHIPPER".to_string()),
        service_type: Some("快递标准".to_string()),
        weight: Some("1".to_string()),
        remark: Some("".to_string()),
        salt: Some("".to_string()),
        day_type: Some("".to_string()),
        pickup_start_time: Some("".to_string()),
        pickup_end_time: Some("".to_string()),
        password_signing: Some("Y".to_string()),
        valins_pay: Some("".to_string()),
        op: Some("0".to_string()),
        poll_call_back_url: Some("".to_string()),
        resultv2: Some("0".to_string()),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "create";

    // 发送请求
    let _ = do_border_request(method, &timestamp, &param_json)?;

    Ok(())
}
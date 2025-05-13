use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelPickUpParam {
    #[serde(rename = "pickUpDate")]
    pub pick_up_date: String,

    pub location: String,

    #[serde(rename = "partnerId")]
    pub partner_id: String,

    #[serde(rename = "partnerName")]
    pub partner_name: String,

    #[serde(rename = "partnerSecret")]
    pub partner_secret: String,

    #[serde(rename = "partnerKey")]
    pub partner_key: String,

    pub code: String,

    #[serde(rename = "kuaidicom")]
    pub kuaidicom: String,

    #[serde(rename = "pickupConfirmationNumber")]
    pub pickup_confirmation_number: String,
}

///国际电子面单取消预约API
pub fn cancel_pick_up() -> Result<(), Box<dyn std::error::Error>> {
    let param = CancelPickUpParam {
        pick_up_date: "2022-05-31".to_string(),
        location: "CN".to_string(),
        partner_id: "xxx".to_string(),
        partner_name: "xxx".to_string(),
        partner_secret: "xxx".to_string(),
        partner_key: "xxx".to_string(),
        code: "xxx".to_string(),
        kuaidicom: "dhl".to_string(),
        pickup_confirmation_number: "CBJ220608002901".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 发送请求
    utils::http_util::do_request(&timestamp, &param_json, config::url::CANCEL_PICK_UP_URL)?;

    Ok(())
}
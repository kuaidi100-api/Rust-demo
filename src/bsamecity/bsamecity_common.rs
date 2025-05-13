use serde::{Deserialize, Serialize};
use crate::config;
use crate::utils;


#[derive(Serialize, Deserialize, Debug)]
pub struct Goods {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OrderParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuaidicom: Option<String>,

    #[serde(rename = "lbsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lbs_type: Option<i32>,

    #[serde(rename = "recManName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_name: Option<String>,

    #[serde(rename = "recManMobile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_mobile: Option<String>,

    #[serde(rename = "recManProvince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_province: Option<String>,

    #[serde(rename = "recManCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_city: Option<String>,

    #[serde(rename = "recManDistrict")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_district: Option<String>,

    #[serde(rename = "recManAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_addr: Option<String>,

    #[serde(rename = "recManLat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_lat: Option<String>,

    #[serde(rename = "recManLng")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man_lng: Option<String>,

    #[serde(rename = "sendManName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man_name: Option<String>,

    #[serde(rename = "sendManMobile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man_mobile: Option<String>,

    #[serde(rename = "sendManProvince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man_province: Option<String>,

    #[serde(rename = "sendManCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man_city: Option<String>,

    #[serde(rename = "sendManDistrict")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man_district: Option<String>,

    #[serde(rename = "sendManAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man_addr: Option<String>,

    #[serde(rename = "sendManLat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man_lat: Option<String>,

    #[serde(rename = "sendManLng")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man_lng: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,

    #[serde(rename = "callbackUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,

    #[serde(rename = "orderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<i32>,

    #[serde(rename = "expectPickupTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_pickup_time: Option<String>,

    #[serde(rename = "expectTimeFinish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_time_finish: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods: Option<Vec<Goods>>,

}


#[derive(Serialize, Deserialize, Debug)]
pub struct CancelParam {
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    #[serde(rename = "cancelMsgType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_msg_type: Option<i32>,

    #[serde(rename = "cancelMsg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_msg: Option<String>,

    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AddFeeParam {
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tips: Option<String>,
}

pub fn do_bsamecity_request(method: &str, t: &str, param: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 发送请求
    let result = utils::http_util::do_method_request(method, t, param, config::url::BSAMECITY_URL)?;

    Ok(result)
}
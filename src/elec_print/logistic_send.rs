use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{config, utils};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogisticSendParam {
    #[serde(rename = "shopType", default, skip_serializing_if = "String::is_empty")]
    pub shop_type: String,

    #[serde(rename = "shopId", default, skip_serializing_if = "String::is_empty")]
    pub shop_id: String,

    #[serde(rename = "orderNum", default, skip_serializing_if = "String::is_empty")]
    pub order_num: String,

    #[serde(rename = "KuaidiCom", default, skip_serializing_if = "String::is_empty")]
    pub kuaidi_com: String,

    #[serde(rename = "KuaidiNum", default, skip_serializing_if = "String::is_empty")]
    pub kuaidi_num: String,

    #[serde(rename = "SubOrderNums", default, skip_serializing_if = "String::is_empty")]
    pub sub_order_nums: String,
}
///提交售后（退货）订单获取任务接口
pub fn logistic_send() -> Result<(), Box<dyn std::error::Error>> {
    let param = LogisticSendParam {
        shop_type: "TAOBAO".to_string(),
        shop_id: "123".to_string(),
        order_num: "12345".to_string(),
        kuaidi_com: "shunfeng".to_string(),
        kuaidi_num: "SF54321".to_string(),
        sub_order_nums: "".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param).map_err(|e| e.to_string())?;

    // 生成时间戳（毫秒）
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis()
        .to_string();

    // 发送请求
    utils::http_util::do_request(&t, &param_json, config::url::LOGISTIC_SEND_URL).map_err(|e| e.to_string())?;

    Ok(())
}
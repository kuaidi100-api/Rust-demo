use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct Attach {
    pub r#type: i32, // 使用 r#type 来避免与 Rust 关键字冲突
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Desc {
    pub attach: Vec<Attach>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkOrderCreateParam {
    pub kuaidinum: String,

    #[serde(rename = "telWeight")]
    pub tel_weight: String,

    #[serde(rename = "callBackUrl")]
    pub call_back_url: String,

    #[serde(rename = "secondType")]
    pub second_type: i32,

    pub desc: Desc,

    pub content: String,
}

///创建工单
pub fn work_order_create() -> Result<(), Box<dyn std::error::Error>> {
    let attach = Attach {
        r#type: 0,
        uri: "http://xxxxxxxxxxxxxxxxxxxxx".to_string(),
    };

    let desc = Desc {
        attach: vec![attach],
    };

    let param = WorkOrderCreateParam {
        kuaidinum: "asdsd123123123".to_string(),
        tel_weight: "1".to_string(),
        call_back_url: "http://127.0.0.1:9100/apitest/apiOrder/callback".to_string(),
        second_type: 4,
        desc: desc,
        content: "重量异常".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 发送请求
    utils::http_util::do_request(&timestamp, &param_json, config::url::WORK_ORDER_CREATE_URL)?;

    Ok(())
}
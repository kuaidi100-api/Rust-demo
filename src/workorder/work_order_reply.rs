use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct Attach {
    pub uri: String,
    #[serde(rename = "type")]
    pub type_: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkOrderReplyParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consult_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    pub attach: Vec<Attach>,
}

///查询留言
pub fn work_order_reply() -> Result<(), Box<dyn std::error::Error>> {
    let param = WorkOrderReplyParam {
        consult_id: Some("1056".to_string()),
        content: None,
        attach: vec![],
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "queryReply";

    // 发送请求
    utils::http_util::do_method_request(method, &timestamp, &param_json, config::url::WORK_ORDER_REPLY_URL)?;

    Ok(())
}

///新增留言
pub fn work_order_add_reply() -> Result<(), Box<dyn std::error::Error>> {
    let attach = Attach {
        uri: "xxxxx".to_string(),
        type_: 0,
    };

    let param = WorkOrderReplyParam {
        consult_id: Some("1023".to_string()),
        content: Some("testApi".to_string()),
        attach: vec![attach],
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "addReply";

    // 发送请求
    utils::http_util::do_method_request(method, &timestamp, &param_json, config::url::WORK_ORDER_REPLY_URL)?;

    Ok(())
}
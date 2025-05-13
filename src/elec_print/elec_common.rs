use serde::{Serialize, Deserialize};
use crate::{config, utils};

// 定义联系人结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,

    #[serde(rename = "printAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_addr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
}


pub fn do_label_order_request(method: &str, t: &str, param: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 发送请求
    let result = utils::http_util::do_method_request(method, t, param, config::url::LABEL_ORDER_URL)?;

    Ok(result)
}
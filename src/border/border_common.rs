use crate::config;
use crate::utils;


pub fn do_border_request(method: &str, t: &str, param: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 发送请求
    let result = utils::http_util::do_method_request(method, t, param, config::url::B_ORDER_URL)?;

    Ok(result)
}
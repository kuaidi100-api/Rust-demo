use std::collections::HashMap;
use crate::config;
use crate::utils;

///快递智能识别单号
pub fn auto_number() -> Result<(), Box<dyn std::error::Error>> {
    let number = "906919164534";
    let key = config::account::KEY;
    let post_url = format!("{}?num={}&key={}", config::url::AUTO_NUMBER_URL, number, key);
    
    let m:  HashMap<&str, &str> = HashMap::new();

    // 发送请求
    utils::http_util::do_map_request(m, &post_url)?;

    Ok(())
}
use md5;
use hex;
use std::collections::HashMap;
use crate::config;
use crate::utils;
use crate::utils::http_util::calculate_sign;

///快递100短信发送接口
pub fn sms_send() -> Result<(), Box<dyn std::error::Error>> {
    // 创建签名字符串
    let sign_str = format!("{}{}", config::account::KEY, config::account::USER_ID);
    // 计算签名
    let sign = calculate_sign(&sign_str);

    // 构造请求参数
    let mut m:  HashMap<&str, &str> = HashMap::new();
    m.insert("sign", &sign);
    m.insert("userid", "9974ef2c377a4dbt9");
    m.insert("seller", "快递100");
    m.insert("phone", "13568688888");
    m.insert("tid", "11");
    m.insert("content", "{\"接收人姓名\":\"王帅\", \"公司名\":\"快递100\", \"快递单号\":\"154893238584\", \"url\": \"https://api.kuaidi100.com/home\"}");
    m.insert("outorder", "143255893");
    m.insert("callback", "http://xxx/callback");

    // 发送请求
    utils::http_util::do_map_request(m, config::url::SMS_SEND_URL)?;

    Ok(())
}
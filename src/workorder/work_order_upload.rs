use chrono::Utc;
use md5;
use hex;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use crate::config;
use crate::utils;
use crate::utils::http_util::calculate_sign;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkOrderUploadParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

///上传附件
pub fn work_order_upload(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let file_name = path.file_name().unwrap().to_string_lossy().to_string();
    let mut file = File::open(path)?;
    let file_ref: &mut File = &mut file;


    let param = WorkOrderUploadParam {
        file_name: Some(file_name.to_string()),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 计算签名
    let sign_str = format!("{}{}{}{}", param_json, timestamp, config::account::KEY, config::account::SECRET);
    let sign = calculate_sign(&sign_str);

    // 构造请求参数
    let mut m = HashMap::new();
    m.insert("key", config::account::KEY);
    m.insert("sign", &sign);
    m.insert("t", &timestamp);
    m.insert("param", &param_json);

    // 发送请求
    utils::http_util::do_file_request(m, &file_name, file_ref, config::url::WORK_ORDER_UPLOAD_URL)?;

    Ok(())
}
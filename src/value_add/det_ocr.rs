use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct DetOCRParam {
    #[serde(rename = "enableTilt")]
    pub enable_tilt: bool,

    pub include: Vec<String>,

    pub image: Option<String>,

    #[serde(rename = "imageUrl")]
    pub image_url: String,

    #[serde(rename = "pdfUrl")]
    pub pdf_url: Option<String>,

    #[serde(rename = "htmlUrl")]
    pub html_url: Option<String>,
}

///快递面单OCR识别接口
pub fn det_ocr() -> Result<(), Box<dyn std::error::Error>> {
    let param = DetOCRParam {
        enable_tilt: false,
        include: vec![
            "barcode".to_string(),
            "receiver".to_string(),
            "sender".to_string(),
        ],
        image: None,
        image_url: "https://cdn.kuaidi100.com/images/openapi/document/ocr_tem.png".to_string(),
        pdf_url: None,
        html_url: None,
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 构造请求参数
    let mut m:HashMap<&str, &str> = std::collections::HashMap::new();
    m.insert("key", config::account::KEY);
    m.insert("param", &param_json);

    // 发送请求
    utils::http_util::do_map_request(m, config::url::DET_OCR_URL)?;

    Ok(())
}
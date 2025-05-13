use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::elec_print::elec_common::do_label_order_request;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomSubParam {
    pub name: String,
    pub flag: String,
    pub count: i32,
    pub unit: String,
    pub total: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "customerId")]
    pub customer_id: String,
    pub cargo1: String,
    pub cargo2: String,
    pub num: String,
    pub label: String,
    #[serde(rename = "qrCode")]
    pub qr_code: String,
    pub org: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomParam {
    #[serde(rename = "tempId")]
    pub temp_id: String,

    #[serde(rename = "printType")]
    pub print_type: String,

    pub siid: String,

    #[serde(rename = "customParam")]
    pub custom_param: CustomSubParam,
}

/// 自定义模板打印接口
pub fn custom() -> Result<(), Box<dyn std::error::Error>> {
    // 构建自定义参数
    let custom_param = CustomSubParam {
        name: "12213".to_string(),
        flag: "出库".to_string(),
        count: 111,
        unit: "货柜".to_string(),
        total: "2000".to_string(),
        order_id: "8888888888888".to_string(),
        customer_id: "66666666".to_string(),
        cargo1: "苹果".to_string(),
        cargo2: "玉米".to_string(),
        num: "SF1536245218562".to_string(),
        label: "拼多多".to_string(),
        qr_code: "6666666666666".to_string(),
        org: "快递100".to_string(),
    };

    // 构建请求参数
    let param = CustomParam {
        temp_id: "476f6f769e57447fb84398eefae2ae28".to_string(),
        print_type: "CLOUD".to_string(),
        siid: "KX100siid".to_string(),
        custom_param: custom_param,
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "custom";

    // 发送请求
    let _ = do_label_order_request(method, &timestamp, &param_json)?;

    Ok(())
}
use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::elec_print::elec_common::{do_label_order_request, Contact};

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderParam {
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,

    #[serde(rename = "partnerKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuaidicom: Option<String>,

    #[serde(rename = "recMan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_man: Option<Contact>,

    #[serde(rename = "sendMan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_man: Option<Contact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo: Option<String>,

    #[serde(rename = "tempId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_id: Option<String>,

    #[serde(rename = "childTempId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_temp_id: Option<String>,

    #[serde(rename = "backTempId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_temp_id: Option<String>,

    #[serde(rename = "payType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_type: Option<String>,

    #[serde(rename = "expType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,

    #[serde(rename = "needChild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_child: Option<String>,

    #[serde(rename = "needBack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_back: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    #[serde(rename = "printType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub siid: Option<String>,

    #[serde(rename = "needDesensitization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_desensitization: Option<bool>,

    #[serde(rename = "needLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_logo: Option<bool>,

    #[serde(rename = "needOcr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_ocr: Option<bool>,
}


///电子面单下单接口
pub fn order() -> Result<(), Box<dyn std::error::Error>> {
    // 构建收件人和寄件人信息
    let rec_man = Contact {
        name: Some("张三".to_string()),
        mobile: Some("13888888888".to_string()),
        print_addr: Some("广东深圳市南山区金蝶软件园".to_string()),
        company: Some("".to_string()),
    };

    let send_man = Contact {
        name: Some("李四".to_string()),
        mobile: Some("13888888888".to_string()),
        print_addr: Some("广东深圳市南山区金蝶软件园".to_string()),
        company: Some("".to_string()),
    };

    // 构建请求参数
    let param = OrderParam {
        partner_id: Some("123456".to_string()),
        partner_key: Some("".to_string()),
        code: Some("".to_string()),
        kuaidicom: Some("zhaijisong".to_string()),
        rec_man: Some(rec_man),
        send_man: Some(send_man),
        cargo: Some("test".to_string()),
        temp_id: Some("60f6c17c7c223700131d8bc3".to_string()),
        child_temp_id: Some("61bff9efc66fb00013a1b168".to_string()),
        back_temp_id: Some("61bffa26c66fb00013a1b16c".to_string()),
        pay_type: Some("SHIPPER".to_string()),
        exp_type: Some("标准快递".to_string()),
        remark: Some("测试下单,请勿发货".to_string()),
        collection: Some("0".to_string()),
        need_child: Some("0".to_string()),
        need_back: Some("0".to_string()),
        count: Some(1),
        print_type: Some("CLOUD".to_string()),
        siid: Some("KX100*******".to_string()),
        need_desensitization: Some(true),
        need_logo: Some(true),
        need_ocr: Some(false),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "order";

    // 发送请求
    let _ = do_label_order_request(method, &timestamp, &param_json)?;

    Ok(())
}
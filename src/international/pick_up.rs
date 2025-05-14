use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMan {
    pub name: String,
    pub mobile: String,
    pub addr: String,
    pub country_code: String,
    pub city: String,
    pub zipcode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecMan {
    pub name: String,
    pub mobile: String,
    pub addr: String,
    pub country_code: String,
    pub city: String,
    pub zipcode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageInfo {
    pub height: String,
    pub width: String,
    pub length: String,
    pub weight: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PickUpParam {
    #[serde(rename = "pickupTimestamp")]
    pub pickup_timestamp: String,

    #[serde(rename = "pickupLocationCloseTime")]
    pub pickup_location_close_time: String,

    pub instruction: String,

    #[serde(rename = "partnerId")]
    pub partner_id: String,

    #[serde(rename = "partnerName")]
    pub partner_name: String,

    #[serde(rename = "partnerSecret")]
    pub partner_secret: String,

    pub kuaidicom: String,

    #[serde(rename = "sendMan")]
    pub send_man: SendMan,

    #[serde(rename = "recMan")]
    pub rec_man: RecMan,

    #[serde(rename = "packageInfos")]
    pub package_infos: Vec<PackageInfo>,
}
///预约取件API
pub fn pick_up() -> Result<(), Box<dyn std::error::Error>> {
    let send_man = SendMan {
        name: "Kaka".to_string(),
        mobile: "13500000000".to_string(),
        addr: "Kingdee Software Park".to_string(),
        country_code: "CN".to_string(),
        city: "SHEN ZHEN".to_string(),
        zipcode: "518057".to_string(),
    };

    let rec_man = RecMan {
        name: "Cindy Martinez / Ana Luz Medina".to_string(),
        mobile: "(86)13510002000".to_string(),
        addr: "Apoquindo 4001, of. 501. Las Condes".to_string(),
        country_code: "CL".to_string(),
        city: "Santiago".to_string(),
        zipcode: "7550000".to_string(),
    };

    let package_infos = vec![
        PackageInfo {
            height: "10".to_string(),
            width: "20".to_string(),
            length: "11".to_string(),
            weight: "0.1".to_string(),
        },
        PackageInfo {
            height: "10".to_string(),
            width: "20".to_string(),
            length: "11".to_string(),
            weight: "0.1".to_string(),
        },
        PackageInfo {
            height: "10".to_string(),
            width: "20".to_string(),
            length: "11".to_string(),
            weight: "0.1".to_string(),
        },
    ];

    let param = PickUpParam {
        pickup_timestamp: "2022-05-16 08:41:35".to_string(),
        pickup_location_close_time: "17:00".to_string(),
        instruction: "Collect at reception".to_string(),
        partner_id: "******".to_string(),
        partner_name: "******".to_string(),
        partner_secret: "******".to_string(),
        kuaidicom: "dhl".to_string(),
        send_man: send_man,
        rec_man: rec_man,
        package_infos: package_infos,
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 发送请求
    utils::http_util::do_request(&timestamp, &param_json, config::url::PICK_UP_URL)?;

    Ok(())
}
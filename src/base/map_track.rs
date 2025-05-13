use serde::{Serialize, Deserialize};
use crate::{config, utils};

#[derive(Serialize, Deserialize, Debug)]
pub struct MapTrackParam {
    #[serde(rename = "com")]
    pub com: String,

    #[serde(rename = "num")]
    pub num: String,

    #[serde(rename = "phone")]
    pub phone: String,

    #[serde(rename = "from")]
    pub from: String,

    #[serde(rename = "to")]
    pub to: String,

    #[serde(rename = "resultv2")]
    pub result_v2: String,

    #[serde(rename = "show")]
    pub show: String,

    #[serde(rename = "order")]
    pub order: String,
}

/*
*地图轨迹查询
 */
pub fn map_track() -> Result<(), Box<dyn std::error::Error>> {
    let param = MapTrackParam {
        com: "ems".to_string(),
        num: "em263999513jp".to_string(),
        phone: "13868688888".to_string(),
        from: "广东省深圳市南山区金蝶软件园".to_string(),
        to: "北京朝阳区国际金融大厦".to_string(),
        result_v2: "5".to_string(),
        show: "0".to_string(),
        order: "desc".to_string(),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param).map_err(|e| e.to_string())?;

    // 发送请求
    utils::http_util::customer_request(&param_json, config::url::MAP_TRACK_URL).map_err(|e| e.to_string())?;

    Ok(())
}
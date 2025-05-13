use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMan {
    pub name: String,
    pub mobile: String,
    pub addr: String,
    pub district: String,
    pub province: String,
    pub company: String,
    pub country_code: String,
    pub city: String,
    pub zipcode: String,
    pub tel: String,
    pub email: String,
    pub vat_num: String,
    pub eori_num: String,
    pub ioss_num: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecMan {
    pub name: String,
    pub mobile: String,
    pub addr: String,
    pub district: String,
    pub province: String,
    pub company: String,
    pub country_code: String,
    pub city: String,
    pub zipcode: String,
    pub tel: String,
    pub email: String,
    pub vat_num: String,
    pub eori_num: String,
    pub ioss_num: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageInfo {
    pub height: String,
    pub width: String,
    pub length: String,
    pub weight: String,
    pub package_reference: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportInfo {
    pub zh_name: String,
    pub en_name: String,
    pub net_weight: f64,
    pub gross_weight: f64,
    pub manufacturing_country_code: String,
    pub unit_price: String,
    pub quantity: f64,
    pub quantity_unit_of_measurement: String,
    pub desc: String,
    pub import_commodity_code: String,
    pub export_commodity_code: String,
    pub number_of_pieces: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceInfo {
    pub date: String,
    pub number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub title: String,
    pub signature: String,
    pub plt_enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DutiesPaymentType {
    pub payment_type: String,
    pub account: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreightPaymentType {
    pub payment_type: String,
    pub account: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomsClearance {
    pub purpose: String,
    pub document: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiCallParam {
    pub partner_id: String,
    pub partner_name: String,
    pub partner_secret: String,
    pub kuaidicom: String,
    pub send_man: SendMan,
    pub rec_man: RecMan,
    pub cargo: String,
    pub count: String,
    pub weight: String,
    pub exp_type: String,
    pub remark: String,
    pub package_infos: Vec<PackageInfo>,
    pub export_infos: Vec<ExportInfo>,
    pub customs_value: String,
    pub trade_term: String,
    pub need_invoice: bool,
    pub invoice_info: InvoiceInfo,
    pub duties_payment_type: DutiesPaymentType,
    pub freight_payment_type: FreightPaymentType,
    pub customs_clearance: CustomsClearance,
}

/*
*国际电子面单下单API
 */
pub fn api_call() -> Result<(), Box<dyn std::error::Error>> {
    let send_man = SendMan {
        name: "Kaka".to_string(),
        mobile: "13500000000".to_string(),
        addr: "Kingdee Software Park".to_string(),
        district: "Hi-tech Park,Nanshang District".to_string(),
        province: "".to_string(),
        company: "QIAN HAI BAI DI".to_string(),
        country_code: "CN".to_string(),
        city: "SHEN ZHEN".to_string(),
        zipcode: "518057".to_string(),
        tel: "0755-5890123".to_string(),
        email: "12344655@qq.com".to_string(),
        vat_num: "IOSS23249923".to_string(),
        eori_num: "IOSS23249923".to_string(),
        ioss_num: "IOSS23249923".to_string(),
    };

    let rec_man = RecMan {
        name: "Cindy Martinez / Ana Luz Medina".to_string(),
        mobile: "(86)13560312000".to_string(),
        addr: "Apoquindo 4001, of. 501. Las Condes".to_string(),
        district: "Santiago, Chile".to_string(),
        province: "".to_string(),
        company: "Lamaignere Chile S.A.".to_string(),
        country_code: "CL".to_string(),
        city: "Santiago".to_string(),
        zipcode: "7550000".to_string(),
        tel: " +56 (9) 1242355".to_string(),
        email: "12344699@qq.com".to_string(),
        vat_num: "IOSS23249923".to_string(),
        eori_num: "IOSS23249923".to_string(),
        ioss_num: "IOSS23249923".to_string(),
    };

    let package_info = PackageInfo {
        height: "10".to_string(),
        width: "20".to_string(),
        length: "11".to_string(),
        weight: "0.1".to_string(),
        package_reference: "just a user remark".to_string(),
    };

    let export_info = ExportInfo {
        zh_name: "".to_string(),
        en_name: "".to_string(),
        net_weight: 0.1,
        gross_weight: 0.1,
        manufacturing_country_code: "CN".to_string(),
        unit_price: "10.0".to_string(),
        quantity: 1.0,
        quantity_unit_of_measurement: "PCS".to_string(),
        desc: "test".to_string(),
        import_commodity_code: "6109100021".to_string(),
        export_commodity_code: "6109100021".to_string(),
        number_of_pieces: 1,
    };

    let duties_payment_type = DutiesPaymentType {
        payment_type: "DDU".to_string(),
        account: "60*****43".to_string(),
    };

    let freight_payment_type = FreightPaymentType {
        payment_type: "SHIPPER".to_string(),
        account: "60*****43".to_string(),
    };

    let customs_clearance = CustomsClearance {
        purpose: "".to_string(),
        document: true,
    };

    let invoice_info = InvoiceInfo {
        date: "2021-08-12".to_string(),
        number: "15462412".to_string(),
        type_: None,
        title: "test".to_string(),
        signature: "base64Str".to_string(),
        plt_enable: true,
    };

    let param = ApiCallParam {
        partner_id: "12344".to_string(),
        partner_name: "dsfgsgs".to_string(),
        partner_secret: "dfsfsfs".to_string(),
        kuaidicom: "dhl".to_string(),
        send_man: send_man,
        rec_man: rec_man,
        cargo: "test don't ship".to_string(),
        count: "1".to_string(),
        weight: "0.1".to_string(),
        exp_type: "parcel-normal".to_string(),
        remark: "just a test demo".to_string(),
        package_infos: vec![package_info],
        export_infos: vec![export_info],
        customs_value: "10.0".to_string(),
        trade_term: "DAP".to_string(),
        need_invoice: true,
        invoice_info: invoice_info,
        duties_payment_type: duties_payment_type,
        freight_payment_type: freight_payment_type,
        customs_clearance: customs_clearance,
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 发送请求
    utils::http_util::do_request(&timestamp, &param_json, config::url::API_CALL_URL)?;

    Ok(())
}
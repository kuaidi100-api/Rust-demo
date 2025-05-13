use chrono::Utc;
use crate::bsamecity::bsamecity_common::{do_bsamecity_request, Goods, OrderParam};


///同城寄件-询价
pub fn price() -> Result<(), Box<dyn std::error::Error>> {
    // 构建商品结构体
    let goods = vec![
        Goods {
            name: Some("外卖".to_string()),
            type_: Some("食品".to_string()),
            count: Some(0),
        },
    ];

    // 构建请求参数
    let param = OrderParam {
        kuaidicom: Some("shunfengtongcheng".to_string()),
        lbs_type: Some(2),
        rec_man_name: Some("张三".to_string()),
        rec_man_mobile: Some("13587654321".to_string()),
        rec_man_province: Some("北京市".to_string()),
        rec_man_city: Some("北京市".to_string()),
        rec_man_district: Some("海淀区".to_string()),
        rec_man_addr: Some("学清嘉创大厦A座15层".to_string()),
        rec_man_lat: Some("40.014838".to_string()),
        rec_man_lng: Some("116.352569".to_string()),
        send_man_name: Some("李四".to_string()),
        send_man_mobile: Some("13512345678".to_string()),
        send_man_province: Some("北京市".to_string()),
        send_man_city: Some("北京市".to_string()),
        send_man_district: Some("海淀区".to_string()),
        send_man_addr: Some("清华大学".to_string()),
        send_man_lat: Some("40.002436".to_string()),
        send_man_lng: Some("116.326582".to_string()),
        weight: Some("1".to_string()),
        remark: Some("测试下单".to_string()),
        volume: Some("".to_string()),
        order_type: Some(0),
        expect_pickup_time: Some("".to_string()),
        expect_time_finish: Some("".to_string()),
        insurance: Some("".to_string()),
        price: Some("0".to_string()),
        goods: Some(goods),
        callback_url: Some("http://www.baidu.com".to_string()),
        salt: Some("".to_string()),
    };

    // 将参数转换为JSON字符串
    let param_json = serde_json::to_string(&param)?;

    // 生成时间戳（毫秒）
    let timestamp = Utc::now().timestamp_millis().to_string();

    // 定义方法名
    let method = "price";

    // 发送请求
    let _ = do_bsamecity_request(method, &timestamp, &param_json)?;

    Ok(())
}
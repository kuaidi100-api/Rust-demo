pub const QUERY_URL: &str = "https://poll.kuaidi100.com/poll/query.do"; //实时快递查询接口

pub const POLL_URL: &str = "https://poll.kuaidi100.com/poll"; //快递信息推送服务  订阅接口
 
pub const MAP_TRACK_URL: &str = "https://poll.kuaidi100.com/poll/maptrack.do"; //快递查询地图轨迹

pub const POLL_MAP_URL: &str = "http://poll.kuaidi100.com/pollmap"; //地图轨迹推送

pub const B_ORDER_URL: &str = "https://poll.kuaidi100.com/order/borderapi.do"; //商家寄件

pub const BSAMECITY_URL: &str = "https://api.kuaidi100.com/bsamecity/order"; //同城寄件

pub const C_ORDER_URL: &str = "https://order.kuaidi100.com/order/corderapi.do"; // C端寄件

pub const AUTH_URL: &str = "https://poll.kuaidi100.com/printapi/authThird.do"; // 第三方平台账号授权

pub const LABEL_ORDER_URL: &str = "https://api.kuaidi100.com/label/order"; // 自定义模板打印、自定义模板打印复打、电子面单下单/复打/取消请求地址、快递预估时效查询接口、拦截改址接口、运单附件查询接口

pub const THIRD_INFO_URL: &str = "http://poll.kuaidi100.com/eorderapi.do"; // 第三方平台网点&面单余额接口

pub const SHOP_AUTHORIZE_URL: &str = "https://api.kuaidi100.com/ent/shop/authorize"; // 获取店铺授权超链接接口

pub const ORDER_TASK_URL: &str = "https://api.kuaidi100.com/ent/order/task"; // 提交销售订单获取任务接口

pub const REFUND_ORDER_TASK_URL: &str = "https://api.kuaidi100.com/ent/refundOrder/task"; // 提交售后（退货）订单获取任务接口

pub const LOGISTIC_SEND_URL: &str = "https://api.kuaidi100.com/ent/logistics/send"; //快递单号回传及订单发货接口

pub const PRINT_TASK_URL: &str = "https://poll.kuaidi100.com/printapi/printtask.do"; //硬件状态查询接口

pub const ADDRESS_RESOLUTION_URL: &str = "https://api.kuaidi100.com/address/resolution";

pub const SMS_SEND_URL: &str = "https://apisms.kuaidi100.com/sms/send.do"; //快递100短信发送接口

pub const AUTO_NUMBER_URL: &str = "http://www.kuaidi100.com/autonumber/auto"; //快递智能识别单号

pub const REACHABLE_URL: &str = "http://api.kuaidi100.com/reachable.do"; //快递可用性接口

pub const DET_OCR_URL: &str = "http://api.kuaidi100.com/elec/detocr"; //快递面单OCR识别接口

pub const API_CALL_URL: &str = "http://api.kuaidi100.com/sendAssistant/order/apiCall"; //国际电子面单下单API

pub const PICK_UP_URL: &str = "http://api.kuaidi100.com/sendAssistant/order/pickUp"; //国际电子面单预约取件API

pub const CANCEL_PICK_UP_URL: &str = "http://api.kuaidi100.com/sendAssistant/order/cancelPickUp"; //国际电子面单取消预约取件API

pub const INTERNATIONAL_ADDRESS_RESOLUTION_URL: &str = "https://api.kuaidi100.com/internationalAddress/resolution"; //国际地址解析接口

pub const WORK_ORDER_CREATE_URL: &str = "https://api.kuaidi100.com/workorder/api/create"; //创建工单

pub const WORK_ORDER_QUERY_URL: &str = "https://api.kuaidi100.com/workorder/api/status"; //查询工单详情

pub const WORK_ORDER_REPLY_URL: &str = "https://api.kuaidi100.com/workorder/api/reply"; //工单留言

pub const WORK_ORDER_UPLOAD_URL: &str = "https://api.kuaidi100.com/workorder/api/upload"; //上传附件

pub const MONITOR_ORDER_URL: &str = "http://api.kuaidi100.com/logistics/monitor/api/order"; //物流全链路监控 订单导入、发货接口

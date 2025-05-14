<h1 align="center"><a href="https://github.com/kuaidi100-api/kuadi100-api/" target="_blank">Rust-demo Project</a></h1>

## Introduce

Rust-demo 是由[快递100](https://api.kuaidi100.com/home)官方提供的Rust sdk，方便测试使用。

Rust-demo 集成了快递查询（base）、电子面单与云打印（elec_print）、物流服务（border、work_order、corder、bsamecity、monitor）、增值服务（value_add）、跨境服务（international）等接口

## Features

- 提供测试类调试。

## Getting started

Rust-demo使用和测试可参考`/test`目录下的`*_test.rs`文件。

```
# git clone https://github.com/kuaidi100-api/Rust-demo.git
```

## Add Config

使用前先配置[account.rs](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/config/account.rs)，账号信息可以登录快递100获取https://poll.kuaidi100.com/manager/page/myinfo/enterprise （注意不要泄露快递100的账号密码以及授权key等敏感信息，以防被他人盗用！！！）


## Use Junit Test

本仓库包含快递100 API 的 Rust 实现示例。以下是可用的 API 方法列表：

### 快递查询(base)接口
- [实时快递查询接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/base/query.rs)
- [快递信息推送服务-订阅接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/base/poll.rs)
- [快递查询地图轨迹](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/base/map_track.rs)
- [地图轨迹推送接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/base/poll_map.rs)

### 电子面单与云打印(elec_print)接口
- [电子面单下单接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/elec_print/elec_order.rs)
- [第三方平台账号授权](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/elec_print/authThird.rs)
- [第三方平台网点&面单余额接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/elec_print/third_info.rs)
- [电子面单取消接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/elec_print/elec_cancel.rs)

- [获取店铺授权超链接接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/elec_print/shop_authorize.rs)
- [提交销售订单获取任务接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/elec_print/order_task.rs)
- [提交售后（退货）订单获取任务接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/elec_print/refundOrder_task.rs)
- [物流信息发送接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/elec_print/logistic_send.rs)

- [自定义模板打印接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/elec_print/elec_custom.rs)
- [自定义模板打印复打接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/elec_print/elec_printOld.rs)
- [硬件状态查询接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/elec_print/print_task.rs)

### 商家寄件(border)接口
- [寄件下单接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/border/border_create.rs)
- [运费预估接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/border/border_price.rs)
- [订单详情查询接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/border/border_detail.rs)
- [取消寄件接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/border/border_cancel.rs)

### 寄件工单服务(work_order)接口
- [创建工单接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/workorder/work_order_create.rs)
- [查询工单接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/workorder/work_order_query.rs)
- [工单新增/查询留言接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/workorder/work_order_reply.rs)
- [工单附件上传接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/workorder/work_order_upload.rs)

### C端寄件(corder)接口
- [C端寄件 下单接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/corder/corder_create.rs)
- [C端寄件 价格预估接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/corder/corder_price.rs)
- [C端寄件 订单详情查询接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/corder/corder_detail.rs)
- [C端寄件 订单取消接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/corder/corder_cancel.rs)

### 同城配送(bsamecity)接口
- [同城配送 下单接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/bsamecity/bsamecity_order.rs)
- [同城配送 价格预估接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/bsamecity/bsamecity_price.rs)
- [同城配送 订单取消接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/bsamecity/bsamecity_cancel.rs)
- [同城配送 订单预取消接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/bsamecity/bsamecity_precancel.rs)
- [同城配送 订单加小费接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/bsamecity/bsamecity_addfee.rs)

### 物流全链路监控服务(monitor)接口
- [订单导出接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/monitor/monitor_orderExport.rs)
- [订单发货接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/monitor/monitor_sendOut.rs)

### 增值服务(value_add)接口
- [智能地址解析接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/value_add/address_resoluton.rs)
- [快递预估时效查询接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/value_add/estimate_time.rs)
- [拦截改址接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/value_add/intercept_order.rs)
- [运单附件查询接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/value_add/back_order.rs)
- [快递100短信发送接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/value_add/sms_send.rs)
- [快递智能识别单号](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/value_add/auto_number.rs)
- [快递可用性接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/value_add/reachable.rs)
- [快递面单OCR识别接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/value_add/det_ocr.rs)
- [快递预估价格查询接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/value_add/estimate_price.rs)

### 跨境服务(international)接口
- [国际电子面单下单API](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/international/api_call.rs)
- [预约取件API](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/international/pick_up.rs)
- [取消预约API](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/international/cancel_pick_up.rs)
- [国际地址解析接口](https://github.com/kuaidi100-api/Rust-demo/blob/master/src/international/international_address_resolution.rs)

## Tips
如需获取账号信息（如 key、customer、secret），或免费试用100单，请访问[API开放平台](https://api.kuaidi100.com/register/diff/)进行注册

pub mod plugin_market_handler;
pub mod router;

pub use plugin_market_handler::*;

use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};

pub fn plugin_category() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取插件分类列表"), get(plugin_market_handler::category_list))
        .route("/tree", WebPathType::Get, Some("获取插件分类树"), get(plugin_market_handler::category_tree))
        .route("/:id", WebPathType::Get, Some("获取插件分类详情"), get(plugin_market_handler::category_detail))
        .route("/add", WebPathType::Post, Some("创建插件分类"), post(plugin_market_handler::category_create))
        .route("/edit", WebPathType::Put, Some("编辑插件分类"), put(plugin_market_handler::category_update))
        .route("/delete", WebPathType::Delete, Some("删除插件分类"), delete(plugin_market_handler::category_delete))
}

pub fn plugin_market() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取插件市场列表"), get(plugin_market_handler::market_list))
        .route("/detail/:id", WebPathType::Get, Some("获取插件市场详情"), get(plugin_market_handler::market_detail))
        .route("/search", WebPathType::Get, Some("搜索插件"), get(plugin_market_handler::search))
        .route("/recommend", WebPathType::Get, Some("推荐插件"), get(plugin_market_handler::recommend))
        .route("/hot", WebPathType::Get, Some("热门插件"), get(plugin_market_handler::hot))
        .route("/categories", WebPathType::Get, Some("获取插件分类"), get(plugin_market_handler::categories))
}

pub fn plugin_developer() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("开发者插件列表"), get(plugin_market_handler::developer_list))
        .route("/add", WebPathType::Post, Some("提交插件"), post(plugin_market_handler::add))
        .route("/edit", WebPathType::Put, Some("编辑插件"), put(plugin_market_handler::edit))
        .route("/delete", WebPathType::Delete, Some("删除插件"), delete(plugin_market_handler::delete_plugin))
        .route("/audit", WebPathType::Put, Some("审核插件"), put(plugin_market_handler::audit))
        .route("/version/add", WebPathType::Post, Some("发布版本"), post(plugin_market_handler::publish_version))
        .route("/stats", WebPathType::Get, Some("销售统计"), get(plugin_market_handler::developer_stats))
}

pub fn plugin_version() -> WebPath {
    WebPath::new()
        .route("/list/:plugin_id", WebPathType::Get, Some("版本列表"), get(plugin_market_handler::version_list))
        .route("/detail/:id", WebPathType::Get, Some("版本详情"), get(plugin_market_handler::version_detail))
        .route("/latest/:plugin_id", WebPathType::Get, Some("最新版本"), get(plugin_market_handler::latest_version))
}

pub fn plan() -> WebPath {
    WebPath::new()
        .route("/list/:plugin_id", WebPathType::Get, Some("获取套餐列表"), get(plugin_market_handler::plan_list))
        .route("/detail/:id", WebPathType::Get, Some("获取套餐详情"), get(plugin_market_handler::plan_detail))
        .route("/add", WebPathType::Post, Some("创建套餐"), post(plugin_market_handler::plan_create))
        .route("/edit", WebPathType::Put, Some("编辑套餐"), put(plugin_market_handler::plan_update))
        .route("/delete", WebPathType::Delete, Some("删除套餐"), delete(plugin_market_handler::plan_delete))
}

pub fn cart() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("购物车列表"), get(plugin_market_handler::cart_list))
        .route("/add", WebPathType::Post, Some("添加购物车"), post(plugin_market_handler::cart_add))
        .route("/remove", WebPathType::Delete, Some("移除购物车"), delete(plugin_market_handler::cart_remove))
        .route("/clear", WebPathType::Delete, Some("清空购物车"), delete(plugin_market_handler::cart_clear))
}

pub fn order() -> WebPath {
    WebPath::new()
        .route("/create", WebPathType::Post, Some("创建订单"), post(plugin_market_handler::create_order))
        .route("/list", WebPathType::Get, Some("订单列表"), get(plugin_market_handler::order_list))
        .route("/detail/:id", WebPathType::Get, Some("订单详情"), get(plugin_market_handler::order_detail))
        .route("/cancel/:id", WebPathType::Post, Some("取消订单"), post(plugin_market_handler::cancel_order))
        .route("/pay/:id", WebPathType::Post, Some("发起支付"), post(plugin_market_handler::pay))
        .route("/payCallback", WebPathType::Post, Some("支付回调"), post(plugin_market_handler::pay_callback))
}

pub fn license() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("License列表"), get(plugin_market_handler::license_list))
        .route("/detail/:id", WebPathType::Get, Some("License详情"), get(plugin_market_handler::license_detail))
        .route("/bind", WebPathType::Post, Some("绑定设备"), post(plugin_market_handler::bind_device))
        .route("/unbind", WebPathType::Post, Some("解绑设备"), post(plugin_market_handler::unbind_device))
        .route("/renew", WebPathType::Post, Some("续费License"), post(plugin_market_handler::renew_license))
        .route("/revoke", WebPathType::Post, Some("吊销License"), post(plugin_market_handler::revoke_license))
        .route("/verify", WebPathType::Post, Some("验证License"), post(plugin_market_handler::verify_license))
        .route("/heartbeat", WebPathType::Post, Some("心跳检测"), post(plugin_market_handler::heartbeat))
}

pub fn verify() -> WebPath {
    WebPath::new()
        .route("/code/send", WebPathType::Post, Some("发送验证码"), post(plugin_market_handler::send_code))
        .route("/code/check", WebPathType::Post, Some("校验验证码"), post(plugin_market_handler::check_code))
        .route("/device/register", WebPathType::Post, Some("注册设备"), post(plugin_market_handler::register_device))
        .route("/obfuscation/config", WebPathType::Get, Some("混淆配置"), get(plugin_market_handler::obfuscation_config))
}

pub fn card() -> WebPath {
    WebPath::new()
        .route("/generate", WebPathType::Post, Some("生成卡密"), post(plugin_market_handler::generate_cards))
        .route("/batch/list", WebPathType::Get, Some("批次列表"), get(plugin_market_handler::card_batch_list))
        .route("/export/:batch_id", WebPathType::Get, Some("导出卡密"), get(plugin_market_handler::export_cards))
        .route("/redeem", WebPathType::Post, Some("兑换卡密"), post(plugin_market_handler::redeem_card))
        .route("/freeze", WebPathType::Post, Some("冻结卡密"), post(plugin_market_handler::freeze_card))
        .route("/unfreeze", WebPathType::Post, Some("解冻卡密"), post(plugin_market_handler::unfreeze_card))
}

pub fn review() -> WebPath {
    WebPath::new()
        .route("/list/:plugin_id", WebPathType::Get, Some("评论列表"), get(plugin_market_handler::review_list))
        .route("/create", WebPathType::Post, Some("发表评论"), post(plugin_market_handler::create_review))
        .route("/reply", WebPathType::Post, Some("回复评论"), post(plugin_market_handler::reply_review))
        .route("/stats/:plugin_id", WebPathType::Get, Some("评论统计"), get(plugin_market_handler::review_stats))
}

pub fn developer() -> WebPath {
    WebPath::new()
        .route("/register", WebPathType::Post, Some("注册开发者"), post(plugin_market_handler::register_developer))
        .route("/profile", WebPathType::Get, Some("开发者资料"), get(plugin_market_handler::developer_profile))
        .route("/update", WebPathType::Put, Some("更新开发者资料"), put(plugin_market_handler::update_developer))
}

pub fn subscription() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("订阅列表"), get(plugin_market_handler::subscription_list))
        .route("/renew", WebPathType::Post, Some("续费订阅"), post(plugin_market_handler::renew_subscription))
        .route("/cancel", WebPathType::Post, Some("取消订阅"), post(plugin_market_handler::cancel_subscription))
}
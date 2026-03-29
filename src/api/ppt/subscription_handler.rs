use crate::api::web_path::{WebPath, WebPathType};
use crate::application::ppt::subscription_service;
use crate::application::ppt::payment_service;
use crate::application::ppt::credit_service;
use crate::application::ppt::api_key_service;
use crate::service::prelude::*;
use crate::model::prelude::PageParams;
use crate::domain::args::a_ppt_subscription::{SubscriptionPlanListArgs, CreditRecordListArgs};
use axum::extract::Path;
use axum::routing::{delete, get, post};
use serde::{Deserialize, Serialize};

pub fn subscription_routes() -> WebPath {
    WebPath::new()
        .route("/plans", WebPathType::Get, Some("获取订阅套餐列表"), get(list_plans_handler))
        .route("/plan/:id", WebPathType::Get, Some("获取套餐详情"), get(get_plan_detail_handler))
        .route("/subscribe", WebPathType::Post, Some("订阅套餐"), post(subscribe_handler))
        .route("/my", WebPathType::Get, Some("获取我的订阅"), get(my_subscription_handler))
        .route("/cancel", WebPathType::Post, Some("取消订阅"), post(cancel_subscription_handler))
        .route("/history", WebPathType::Get, Some("订阅历史"), get(subscription_history_handler))
        .route("/check-limit", WebPathType::Get, Some("检查限制"), get(check_limit_handler))
}

pub fn payment_routes() -> WebPath {
    WebPath::new()
        .route("/create-order", WebPathType::Post, Some("创建订单"), post(create_order_handler))
        .route("/notify/:channel", WebPathType::Post, Some("支付回调"), post(payment_notify_handler))
        .route("/status/:order_no", WebPathType::Get, Some("查询订单状态"), get(order_status_handler))
        .route("/cancel/:order_no", WebPathType::Post, Some("取消订单"), post(cancel_order_handler))
}

pub fn credit_routes() -> WebPath {
    WebPath::new()
        .route("/balance", WebPathType::Get, Some("获取余额"), get(balance_handler))
        .route("/recharge", WebPathType::Post, Some("充值"), post(recharge_handler))
        .route("/consume", WebPathType::Post, Some("消费"), post(consume_handler))
        .route("/records", WebPathType::Get, Some("消费记录"), get(records_handler))
}

pub fn api_key_routes() -> WebPath {
    WebPath::new()
        .route("/create", WebPathType::Post, Some("创建API密钥"), post(create_key_handler))
        .route("/list", WebPathType::Get, Some("获取API密钥列表"), get(list_keys_handler))
        .route("/revoke/:id", WebPathType::Delete, Some("撤销API密钥"), delete(revoke_key_handler))
        .route("/stats/:id", WebPathType::Get, Some("获取使用统计"), get(stats_handler))
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct SubscribeRequest {
    pub plan_id: i64,
    pub auto_renew: Option<bool>,
}

pub async fn list_plans_handler() -> axum::response::Response {
    let page = PageParams { page_num: Some(1), page_size: Some(20) };
    let search = SubscriptionPlanListArgs { page_num: None, page_size: None, code: None, is_active: None, name: None };
    match subscription_service::list_plans(page, search).await {
        Ok(list) => crate::common::result::ApiResponse::success(list).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn get_plan_detail_handler(Path(id): Path<i64>) -> axum::response::Response {
    match subscription_service::get_plan_detail(id).await {
        Ok(detail) => crate::common::result::ApiResponse::success(detail).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn subscribe_handler(VJson(req): VJson<SubscribeRequest>) -> axum::response::Response {
    let args = crate::domain::args::a_ppt_subscription::SubscribeArgs {
        user_id: 0,
        plan_id: req.plan_id,
        payment_method: "default".to_string(),
        coupon_code: None,
    };
    match subscription_service::create_subscription(args).await {
        Ok(sub) => crate::common::result::ApiResponse::success(sub).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn my_subscription_handler() -> impl axum::response::IntoResponse {
    subscription_service::get_user_subscription_api().await
}

pub async fn cancel_subscription_handler() -> impl axum::response::IntoResponse {
    subscription_service::cancel_subscription_api().await
}

pub async fn subscription_history_handler() -> impl axum::response::IntoResponse {
    subscription_service::get_subscription_history_api().await
}

pub async fn check_limit_handler() -> impl axum::response::IntoResponse {
    subscription_service::check_limit_api().await
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct CreateOrderRequest {
    pub plan_id: i64,
    pub payment_method: String,
    pub coupon_code: Option<String>,
}

pub async fn create_order_handler(VJson(req): VJson<CreateOrderRequest>) -> axum::response::Response {
    match payment_service::create_order(0, req.plan_id, req.payment_method, req.coupon_code).await {
        Ok(order) => crate::common::result::ApiResponse::success(order).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn payment_notify_handler(Path(channel): Path<String>) -> axum::response::Response {
    match payment_service::handle_payment_callback(&channel, "").await {
        Ok(_) => crate::common::result::ApiResponse::success(()).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn order_status_handler(Path(order_no): Path<String>) -> axum::response::Response {
    match payment_service::query_order_by_no(&order_no).await {
        Ok(order) => crate::common::result::ApiResponse::success(order).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn cancel_order_handler(Path(order_no): Path<String>) -> axum::response::Response {
    // 需要先查询订单 ID
    match payment_service::query_order_by_no(&order_no).await {
        Ok(order) => {
            match payment_service::cancel_order(order.id).await {
                Ok(_) => crate::common::result::ApiResponse::success(()).into_response(),
                Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
            }
        }
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn balance_handler() -> axum::response::Response {
    match credit_service::get_balance(0).await {
        Ok(balance) => crate::common::result::ApiResponse::success(balance).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct RechargeRequest {
    pub amount: i64,
    pub payment_method: String,
}

pub async fn recharge_handler(VJson(req): VJson<RechargeRequest>) -> axum::response::Response {
    match credit_service::recharge(0, req.amount as i32, &req.payment_method, None, None).await {
        Ok(_) => crate::common::result::ApiResponse::success(()).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct ConsumeRequest {
    pub task_type: String,
    pub amount: i32,
    pub project_id: Option<i64>,
    pub description: Option<String>,
}

pub async fn consume_handler(VJson(req): VJson<ConsumeRequest>) -> axum::response::Response {
    match credit_service::consume(0, &req.task_type, req.amount, req.project_id, req.description).await {
        Ok(_) => crate::common::result::ApiResponse::success(()).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn records_handler() -> axum::response::Response {
    let page = PageParams { page_num: Some(1), page_size: Some(20) };
    let args = CreditRecordListArgs { 
        user_id: 0, 
        task_type: None, 
        source: None,
        page_num: None,
        page_size: None,
        project_id: None,
    };
    match credit_service::get_usage_history(page, args).await {
        Ok(records) => crate::common::result::ApiResponse::success(records).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct CreateKeyRequest {
    pub name: String,
    pub rate_limit: Option<i32>,
    pub daily_limit: Option<i32>,
}

pub async fn create_key_handler(VJson(req): VJson<CreateKeyRequest>) -> axum::response::Response {
    match api_key_service::generate_key(0, req.name, None, req.rate_limit, req.daily_limit, None).await {
        Ok(key) => crate::common::result::ApiResponse::success(key).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn list_keys_handler() -> axum::response::Response {
    let page = PageParams { page_num: Some(1), page_size: Some(20) };
    let args = crate::domain::args::a_ppt_subscription::APIKeyListArgs { page_num: None, page_size: None, user_id: None, is_active: None };
    match api_key_service::list_keys(page, args).await {
        Ok(list) => crate::common::result::ApiResponse::success(list).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn revoke_key_handler(Path(id): Path<i64>) -> axum::response::Response {
    match api_key_service::revoke_key(id).await {
        Ok(_) => crate::common::result::ApiResponse::success(()).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn stats_handler(Path(id): Path<i64>) -> axum::response::Response {
    match api_key_service::get_usage_stats(id, None, None).await {
        Ok(stats) => crate::common::result::ApiResponse::success(stats).into_response(),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

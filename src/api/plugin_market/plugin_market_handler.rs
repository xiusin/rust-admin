use axum::{extract::{Path, Query}, response::IntoResponse, Json};
use serde::Deserialize;
use crate::application::plugin_market::*;
use crate::domain::model::m_plugin::*;
use crate::domain::args::a_order::*;
use crate::model::prelude::*;

pub async fn category_list() -> impl IntoResponse {
    let result = plugin_category_service::list().await;
    ApiResponse::from_result(result)
}

pub async fn category_tree() -> impl IntoResponse {
    let result = plugin_category_service::tree().await;
    ApiResponse::from_result(result)
}

pub async fn category_detail(Path(id): Path<i64>) -> impl IntoResponse {
    let result = plugin_category_service::detail(id).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct CategoryCreateParams {
    pub name: String,
    pub icon: Option<String>,
    pub parent_id: i64,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn category_create(Json(params): Json<CategoryCreateParams>) -> impl IntoResponse {
    let result = plugin_category_service::create(params.into()).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct CategoryUpdateParams {
    pub id: i64,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn category_update(Json(params): Json<CategoryUpdateParams>) -> impl IntoResponse {
    let id = params.id;
    let result = plugin_category_service::update(id, params.into()).await;
    ApiResponse::from_result(result)
}

pub async fn category_delete(Path(id): Path<i64>) -> impl IntoResponse {
    let result = plugin_category_service::delete(id).await;
    ApiResponse::from_result(result)
}

pub async fn market_list(Query(params): Query<PluginSearchParams>) -> impl IntoResponse {
    let result = plugin_service::market_list(params).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total,
        page_num: 1,
        page_size: 10,
    }))
}

pub async fn market_detail(Path(id): Path<i64>) -> impl IntoResponse {
    let result = plugin_service::market_detail(id).await;
    ApiResponse::from_result(result)
}

pub async fn search(Query(params): Query<PluginSearchParams>) -> impl IntoResponse {
    let result = plugin_service::market_list(params).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total,
        page_num: 1,
        page_size: 10,
    }))
}

pub async fn recommend(Query(limit): Query<i32>) -> impl IntoResponse {
    let result = plugin_service::recommend(limit.unwrap_or(10)).await;
    ApiResponse::from_result(result)
}

pub async fn hot(Query(limit): Query<i32>) -> impl IntoResponse {
    let result = plugin_service::hot(limit.unwrap_or(10)).await;
    ApiResponse::from_result(result)
}

pub async fn categories() -> impl IntoResponse {
    let result = plugin_category_service::tree().await;
    ApiResponse::from_result(result)
}

pub async fn developer_list(Query(params): Query<PluginSearchParams>) -> impl IntoResponse {
    let result = plugin_service::developer_list(0, params.page_num.unwrap_or(1), params.page_size.unwrap_or(10)).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total,
        page_num: 1,
        page_size: 10,
    }))
}

pub async fn add(Json(params): Json<plugin_service::CreatePluginParams>) -> impl IntoResponse {
    let result = plugin_service::create(0, params).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(params): Json<plugin_service::UpdatePluginParams>) -> impl IntoResponse {
    let id = params.id;
    let result = plugin_service::update(id, params).await;
    ApiResponse::from_result(result)
}

pub async fn delete_plugin(Path(id): Path<i64>) -> impl IntoResponse {
    let result = plugin_service::delete(id).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct AuditParams {
    pub id: i64,
    pub status: i32,
}

pub async fn audit(Json(params): Json<AuditParams>) -> impl IntoResponse {
    let result = plugin_service::audit(params.id, params.status).await;
    ApiResponse::from_result(result)
}

pub async fn publish_version(Json(params): Json<plugin_version_service::PublishVersionParams>) -> impl IntoResponse {
    let result = plugin_version_service::publish(0, params).await;
    ApiResponse::from_result(result)
}

pub async fn developer_stats() -> impl IntoResponse {
    let result = developer_service::get_stats(0).await;
    ApiResponse::from_result(result)
}

pub async fn version_list(Path(plugin_id): Path<i64>) -> impl IntoResponse {
    let result = plugin_version_service::list(plugin_id).await;
    ApiResponse::from_result(result)
}

pub async fn version_detail(Path(id): Path<i64>) -> impl IntoResponse {
    let result = plugin_version_service::detail(id).await;
    ApiResponse::from_result(result)
}

pub async fn latest_version(Path(plugin_id): Path<i64>) -> impl IntoResponse {
    let result = plugin_version_service::latest(plugin_id).await;
    ApiResponse::from_result(result)
}

pub async fn plan_list(Path(plugin_id): Path<i64>) -> impl IntoResponse {
    let result = plan_service::list(plugin_id).await;
    ApiResponse::from_result(result)
}

pub async fn plan_detail(Path(id): Path<i64>) -> impl IntoResponse {
    let result = plan_service::detail(id).await;
    ApiResponse::from_result(result)
}

pub async fn plan_create(Json(params): Json<plan_service::CreatePlanParams>) -> impl IntoResponse {
    let result = plan_service::create(0, params).await;
    ApiResponse::from_result(result)
}

pub async fn plan_update(Json(params): Json<plan_service::UpdatePlanParams>) -> impl IntoResponse {
    let id = params.id;
    let result = plan_service::update(id, params).await;
    ApiResponse::from_result(result)
}

pub async fn plan_delete(Path(id): Path<i64>) -> impl IntoResponse {
    let result = plan_service::delete(id).await;
    ApiResponse::from_result(result)
}

pub async fn cart_list(Query(user_id): Query<i64>) -> impl IntoResponse {
    let result = cart_service::list(user_id.unwrap_or(0)).await;
    ApiResponse::from_result(result)
}

pub async fn cart_add(Json(params): Json<cart_service::AddCartParams>) -> impl IntoResponse {
    let result = cart_service::add(0, params).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct RemoveCartParams {
    pub ids: Vec<i64>,
}

pub async fn cart_remove(Json(params): Json<RemoveCartParams>) -> impl IntoResponse {
    let result = cart_service::remove(0, params.ids).await;
    ApiResponse::from_result(result)
}

pub async fn cart_clear(Query(user_id): Query<i64>) -> impl IntoResponse {
    let result = cart_service::clear(user_id.unwrap_or(0)).await;
    ApiResponse::from_result(result)
}

pub async fn create_order(Json(params): Json<order_service::CreateOrderParams>) -> impl IntoResponse {
    let result = order_service::create(0, params).await;
    ApiResponse::from_result(result)
}

pub async fn order_list(Query(params): Query<order_service::OrderSearchParams>) -> impl IntoResponse {
    let result = order_service::list(0, params).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total,
        page_num: 1,
        page_size: 10,
    }))
}

pub async fn order_detail(Path(id): Path<i64>) -> impl IntoResponse {
    let result = order_service::detail(id).await;
    ApiResponse::from_result(result)
}

pub async fn cancel_order(Path(id): Path<i64>) -> impl IntoResponse {
    let result = order_service::cancel(id, 0).await;
    ApiResponse::from_result(result)
}

pub async fn pay(Path(id): Path<i64>) -> impl IntoResponse {
    ApiResponse::success(order_service::PayResult {
        order_no: format!("PLM{}", id),
        amount: 0.0,
        qr_code: Some("weixin://qr".to_string()),
        pay_url: None,
    })
}

#[derive(Debug, Deserialize)]
pub struct PayCallbackParams {
    pub order_id: i64,
    pub payment_method: i32,
}

pub async fn pay_callback(Json(params): Json<PayCallbackParams>) -> impl IntoResponse {
    let result = order_service::pay_callback(params.order_id, params.payment_method).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct LicenseListParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

pub async fn license_list(Query(params): Query<LicenseListParams>) -> impl IntoResponse {
    let result = license_service::list(0, params.page_num.unwrap_or(1), params.page_size.unwrap_or(10)).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total,
        page_num: 1,
        page_size: 10,
    }))
}

pub async fn license_detail(Path(id): Path<i64>) -> impl IntoResponse {
    let result = license_service::detail(id).await;
    ApiResponse::from_result(result)
}

pub async fn bind_device(Json(params): Json<license_service::BindDeviceParams>) -> impl IntoResponse {
    let result = license_service::bind_device(params.license_id, params).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct UnbindDeviceParams {
    pub license_id: i64,
    pub device_id: String,
}

pub async fn unbind_device(Json(params): Json<UnbindDeviceParams>) -> impl IntoResponse {
    let result = license_service::unbind_device(params.license_id, params.device_id).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct RenewLicenseParams {
    pub license_id: i64,
    pub extend_days: i32,
}

pub async fn renew_license(Json(params): Json<RenewLicenseParams>) -> impl IntoResponse {
    let result = license_service::renew(params.license_id, params.extend_days).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct RevokeLicenseParams {
    pub license_id: i64,
}

pub async fn revoke_license(Json(params): Json<RevokeLicenseParams>) -> impl IntoResponse {
    let result = license_service::revoke(params.license_id).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct VerifyLicenseParams {
    pub license_key: String,
    pub device_id: String,
    pub device_info: DeviceInfo,
    pub timestamp: i64,
    pub sign: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
    pub device_name: String,
    pub device_type: String,
    pub os_version: String,
    pub app_version: String,
    pub mac_address: Option<String>,
}

pub async fn verify_license(Json(params): Json<VerifyLicenseParams>) -> impl IntoResponse {
    let result = verification_service::verify_license(
        params.license_key,
        params.device_id,
        params.device_info,
        params.timestamp,
        params.sign,
    ).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct HeartbeatParams {
    pub license_key: String,
    pub device_id: String,
    pub timestamp: i64,
    pub sign: String,
}

pub async fn heartbeat(Json(params): Json<HeartbeatParams>) -> impl IntoResponse {
    let result = verification_service::heartbeat(
        params.license_key,
        params.device_id,
        params.timestamp,
        params.sign,
    ).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct SendCodeParams {
    pub license_id: i64,
    pub plugin_id: i64,
    pub purpose: String,
    pub device_hash: Option<String>,
}

pub async fn send_code(Json(params): Json<SendCodeParams>) -> impl IntoResponse {
    let result = verify_code_service::send(params.license_id, 0, params.plugin_id, params.purpose, params.device_hash).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct CheckCodeParams {
    pub license_id: i64,
    pub code: String,
    pub device_hash: Option<String>,
}

pub async fn check_code(Json(params): Json<CheckCodeParams>) -> impl IntoResponse {
    let result = verify_code_service::check(params.license_id, params.code, params.device_hash).await;
    ApiResponse::from_result(result)
}

pub async fn register_device(Json(params): Json<device_service::RegisterDeviceParams>) -> impl IntoResponse {
    let result = device_service::register(params.license_id, params).await;
    ApiResponse::from_result(result)
}

pub async fn obfuscation_config(Query(plugin_id): Query<i64>) -> impl IntoResponse {
    let result = verification_service::get_obfuscation_config(plugin_id).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct GenerateCardsParams {
    pub plugin_id: i64,
    pub plan_id: i64,
    pub count: i32,
    pub price: f64,
    pub expire_days: i32,
}

pub async fn generate_cards(Json(params): Json<GenerateCardsParams>) -> impl IntoResponse {
    let result = card_service::generate_batch(params.plugin_id, params.plan_id, params.count, params.price, params.expire_days, 0).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct BatchListParams {
    pub plugin_id: Option<i64>,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

pub async fn card_batch_list(Query(params): Query<BatchListParams>) -> impl IntoResponse {
    let result = card_service::list_batches(params.plugin_id.unwrap_or(0), params.page_num.unwrap_or(1), params.page_size.unwrap_or(10)).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total,
        page_num: 1,
        page_size: 10,
    }))
}

pub async fn export_cards(Path(batch_id): Path<i64>) -> impl IntoResponse {
    let result = card_service::export_batch(batch_id).await;
    ApiResponse::from_result(result)
}

pub async fn redeem_card(Json(params): Json<card_service::RedeemCardParams>) -> impl IntoResponse {
    let result = card_service::redeem(0, params.card_no, params.card_pwd).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct FreezeCardParams {
    pub card_id: i64,
}

pub async fn freeze_card(Json(params): Json<FreezeCardParams>) -> impl IntoResponse {
    let result = card_service::freeze(params.card_id).await;
    ApiResponse::from_result(result)
}

pub async fn unfreeze_card(Json(params): Json<FreezeCardParams>) -> impl IntoResponse {
    let result = card_service::unfreeze(params.card_id).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct ReviewListParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

pub async fn review_list(Path(plugin_id): Path<i64>, Query(params): Query<ReviewListParams>) -> impl IntoResponse {
    let result = review_service::list(plugin_id, params.page_num.unwrap_or(1), params.page_size.unwrap_or(10)).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total,
        page_num: 1,
        page_size: 10,
    }))
}

pub async fn create_review(Json(params): Json<review_service::CreateReviewParams>) -> impl IntoResponse {
    let result = review_service::create(0, params).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct ReplyReviewParams {
    pub review_id: i64,
    pub reply_content: String,
}

pub async fn reply_review(Json(params): Json<ReplyReviewParams>) -> impl IntoResponse {
    let result = review_service::reply(params.review_id, params.reply_content).await;
    ApiResponse::from_result(result)
}

pub async fn review_stats(Path(plugin_id): Path<i64>) -> impl IntoResponse {
    let result = review_service::get_statistics(plugin_id).await;
    ApiResponse::from_result(result)
}

pub async fn register_developer(Json(params): Json<developer_service::RegisterDeveloperParams>) -> impl IntoResponse {
    let result = developer_service::register(0, params).await;
    ApiResponse::from_result(result)
}

pub async fn developer_profile() -> impl IntoResponse {
    let result = developer_service::get_by_user_id(0).await;
    ApiResponse::from_result(result)
}

pub async fn update_developer(Json(params): Json<developer_service::UpdateDeveloperParams>) -> impl IntoResponse {
    let result = developer_service::update(0, params).await;
    ApiResponse::from_result(result)
}

pub async fn subscription_list(Query(params): Query<LicenseListParams>) -> impl IntoResponse {
    let result = subscription_service::list(0, params.page_num.unwrap_or(1), params.page_size.unwrap_or(10)).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total,
        page_num: 1,
        page_size: 10,
    }))
}

#[derive(Debug, Deserialize)]
pub struct RenewSubscriptionParams {
    pub subscription_id: i64,
    pub extend_days: i32,
}

pub async fn renew_subscription(Json(params): Json<RenewSubscriptionParams>) -> impl IntoResponse {
    let result = subscription_service::renew(params.subscription_id, params.extend_days).await;
    ApiResponse::from_result(result)
}

pub async fn cancel_subscription(Path(id): Path<i64>) -> impl IntoResponse {
    let result = subscription_service::cancel(id).await;
    ApiResponse::from_result(result)
}

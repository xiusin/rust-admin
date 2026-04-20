use axum::{extract::{Path, Query}, response::IntoResponse, Json};
use serde::Deserialize;
use crate::application::plugin_market::*;
use crate::domain::model::m_plugin::*;
use crate::domain::args::a_order::OrderSearchParams;
use crate::domain::args::a_card::RedeemCardParams;
use crate::model::prelude::*;

// ==================== Category ====================

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
    let svc_params = plugin_category_service::CreateCategoryParams {
        name: params.name,
        icon: params.icon,
        parent_id: params.parent_id,
        sort: params.sort,
        status: params.status,
    };
    let result = plugin_category_service::create(svc_params).await;
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
    let svc_params = plugin_category_service::UpdateCategoryParams {
        id,
        name: params.name,
        icon: params.icon,
        sort: params.sort,
        status: params.status,
    };
    let result = plugin_category_service::update(id, svc_params).await;
    ApiResponse::from_result(result)
}

pub async fn category_delete(Path(id): Path<i64>) -> impl IntoResponse {
    let result = plugin_category_service::delete(id).await;
    ApiResponse::from_result(result)
}

// ==================== Plugin ====================

pub async fn market_list(Query(params): Query<PluginSearchParams>) -> impl IntoResponse {
    let result = plugin_service::market_list(params).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total: total as u64,
        total_pages: ((total as f64) / 10.0).ceil() as u64,
        page_num: 1,
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
        total: total as u64,
        total_pages: ((total as f64) / 10.0).ceil() as u64,
        page_num: 1,
    }))
}

pub async fn recommend(Query(limit): Query<i32>) -> impl IntoResponse {
    let result = plugin_service::recommend(limit).await;
    ApiResponse::from_result(result)
}

pub async fn hot(Query(limit): Query<i32>) -> impl IntoResponse {
    let result = plugin_service::hot(limit).await;
    ApiResponse::from_result(result)
}

pub async fn categories() -> impl IntoResponse {
    let result = plugin_category_service::tree().await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct DeveloperListParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

pub async fn developer_list(Query(params): Query<DeveloperListParams>) -> impl IntoResponse {
    let page_num = params.page_num.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let result = plugin_service::developer_list(0, page_num, page_size).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total: total as u64,
        total_pages: ((total as f64) / page_size as f64).ceil() as u64,
        page_num: page_num as u64,
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreatePluginParams {
    pub name: String,
    pub code: String,
    pub category_id: i64,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub version: String,
    pub price_type: i32,
    pub verify_level: i32,
}

pub async fn add(Json(params): Json<CreatePluginParams>) -> impl IntoResponse {
    let svc_params = plugin_service::CreatePluginParams {
        name: params.name,
        code: params.code,
        category_id: params.category_id,
        summary: params.summary,
        description: params.description,
        cover_image: None,
        screenshots: None,
        version: params.version,
        price_type: params.price_type,
        verify_level: params.verify_level,
        tags: None,
    };
    let result = plugin_service::create(0, svc_params).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct EditPluginParams {
    pub id: i64,
    pub name: Option<String>,
    pub category_id: Option<i64>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub screenshots: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn edit(Json(params): Json<EditPluginParams>) -> impl IntoResponse {
    let id = params.id;
    let svc_params = plugin_service::UpdatePluginParams {
        name: params.name,
        category_id: params.category_id,
        summary: params.summary,
        description: params.description,
        cover_image: params.cover_image,
        screenshots: params.screenshots,
        tags: params.tags,
        sort: params.sort,
        status: params.status,
    };
    let result = plugin_service::update(id, svc_params).await;
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

// ==================== Version ====================

#[derive(Debug, Deserialize)]
pub struct PublishVersionParams {
    pub plugin_id: i64,
    pub version: String,
    pub changelog: Option<String>,
    pub download_url: String,
    pub file_hash: String,
    pub file_size: Option<i64>,
    pub min_app_version: Option<String>,
}

pub async fn publish_version(Json(params): Json<PublishVersionParams>) -> impl IntoResponse {
    let plugin_id = params.plugin_id;
    let svc_params = plugin_version_service::PublishVersionParams {
        version: params.version,
        changelog: params.changelog,
        download_url: params.download_url,
        file_hash: params.file_hash,
        file_size: params.file_size.unwrap_or(0),
        min_app_version: params.min_app_version,
    };
    let result = plugin_version_service::publish(plugin_id, svc_params).await;
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

// ==================== Plan ====================

pub async fn plan_list(Path(plugin_id): Path<i64>) -> impl IntoResponse {
    let result = plan_service::list(plugin_id).await;
    ApiResponse::from_result(result)
}

pub async fn plan_detail(Path(id): Path<i64>) -> impl IntoResponse {
    let result = plan_service::detail(id).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct PlanCreateParams {
    pub plugin_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub period_type: i32,
    pub period_days: i32,
    pub price: f64,
    pub original_price: Option<f64>,
    pub features: Option<Vec<FeatureItem>>,
    pub max_devices: Option<i32>,
    pub max_users: Option<i32>,
    pub storage_limit: Option<i64>,
    pub api_calls_limit: Option<i64>,
    pub support_level: Option<i32>,
    pub sort: Option<i32>,
}

pub async fn plan_create(Json(params): Json<PlanCreateParams>) -> impl IntoResponse {
    let plugin_id = params.plugin_id;
    let svc_params = plan_service::CreatePlanParams {
        name: params.name,
        description: params.description,
        period_type: params.period_type,
        period_days: params.period_days,
        price: params.price,
        original_price: params.original_price.unwrap_or(params.price),
        features: params.features,
        max_devices: params.max_devices.unwrap_or(1),
        max_users: params.max_users.unwrap_or(1),
        storage_limit: params.storage_limit.unwrap_or(0),
        api_calls_limit: params.api_calls_limit.unwrap_or(0),
        support_level: params.support_level.unwrap_or(0),
        sort: params.sort,
    };
    let result = plan_service::create(plugin_id, svc_params).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct PlanUpdateParams {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub period_type: Option<i32>,
    pub period_days: Option<i32>,
    pub price: Option<f64>,
    pub features: Option<Vec<FeatureItem>>,
    pub max_devices: Option<i32>,
    pub max_users: Option<i32>,
    pub storage_limit: Option<i64>,
    pub api_calls_limit: Option<i64>,
    pub support_level: Option<i32>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn plan_update(Json(params): Json<PlanUpdateParams>) -> impl IntoResponse {
    let id = params.id;
    let svc_params = plan_service::UpdatePlanParams {
        name: params.name,
        description: params.description,
        period_type: params.period_type,
        period_days: params.period_days,
        price: params.price,
        features: params.features,
        max_devices: params.max_devices,
        max_users: params.max_users,
        storage_limit: params.storage_limit,
        api_calls_limit: params.api_calls_limit,
        support_level: params.support_level,
        sort: params.sort,
        status: params.status,
    };
    let result = plan_service::update(id, svc_params).await;
    ApiResponse::from_result(result)
}

pub async fn plan_delete(Path(id): Path<i64>) -> impl IntoResponse {
    let result = plan_service::delete(id).await;
    ApiResponse::from_result(result)
}

// ==================== Cart ====================

#[derive(Debug, Deserialize)]
pub struct CartListParams {
    pub user_id: i64,
}

pub async fn cart_list(Query(params): Query<CartListParams>) -> impl IntoResponse {
    let result = cart_service::list(params.user_id).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct AddCartParams {
    pub user_id: i64,
    pub plugin_id: i64,
    pub plan_id: i64,
}

pub async fn cart_add(Json(params): Json<AddCartParams>) -> impl IntoResponse {
    let user_id = params.user_id;
    let svc_params = crate::domain::model::m_cart::AddCartParams {
        plugin_id: params.plugin_id,
        plan_id: params.plan_id,
    };
    let result = cart_service::add(user_id, svc_params).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct RemoveCartParams {
    pub user_id: i64,
    pub ids: Vec<i64>,
}

pub async fn cart_remove(Json(params): Json<RemoveCartParams>) -> impl IntoResponse {
    let result = cart_service::remove(params.user_id, params.ids).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct ClearCartParams {
    pub user_id: i64,
}

pub async fn cart_clear(Query(params): Query<ClearCartParams>) -> impl IntoResponse {
    let result = cart_service::clear(params.user_id).await;
    ApiResponse::from_result(result)
}

// ==================== Order ====================

#[derive(Debug, Deserialize)]
pub struct CreateOrderParams {
    pub user_id: i64,
    pub plugin_id: i64,
    pub plan_id: i64,
    pub payment_method: Option<i32>,
    pub coupon_id: Option<i64>,
}

pub async fn create_order(Json(params): Json<CreateOrderParams>) -> impl IntoResponse {
    let user_id = params.user_id;
    let svc_params = order_service::CreateOrderParams {
        plugin_id: params.plugin_id,
        plan_id: params.plan_id,
        coupon_id: params.coupon_id,
        payment_method: params.payment_method.unwrap_or(0),
    };
    let result = order_service::create(user_id, svc_params).await;
    ApiResponse::from_result(result)
}

pub async fn order_list(Query(params): Query<OrderSearchParams>) -> impl IntoResponse {
    let result = order_service::list(0, params).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total: total as u64,
        total_pages: ((total as f64) / 10.0).ceil() as u64,
        page_num: 1,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PayResult {
    pub order_no: String,
    pub amount: f64,
    pub qr_code: Option<String>,
    pub pay_url: Option<String>,
}

pub async fn pay(Path(id): Path<i64>) -> impl IntoResponse {
    ApiResponse::success(PayResult {
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

// ==================== License ====================

#[derive(Debug, Deserialize)]
pub struct LicenseListParams {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

pub async fn license_list(Query(params): Query<LicenseListParams>) -> impl IntoResponse {
    let page_num = params.page_num.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let result = license_service::list(0, page_num, page_size).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total: total as u64,
        total_pages: ((total as f64) / page_size as f64).ceil() as u64,
        page_num: page_num as u64,
    }))
}

pub async fn license_detail(Path(id): Path<i64>) -> impl IntoResponse {
    let result = license_service::detail(id).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct BindDeviceParams {
    pub license_id: i64,
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
}

pub async fn bind_device(Json(params): Json<BindDeviceParams>) -> impl IntoResponse {
    let license_id = params.license_id;
    let svc_params = license_service::BindDeviceParams {
        device_id: params.device_id,
        device_name: params.device_name,
        device_type: params.device_type,
        os_version: params.os_version,
        mac_address: params.mac_address,
        ip_address: params.ip_address,
    };
    let result = license_service::bind_device(license_id, svc_params).await;
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

// ==================== Verification ====================

#[derive(Debug, Deserialize)]
pub struct VerifyLicenseParams {
    pub license_key: String,
    pub device_id: String,
    pub device_info: VerifyDeviceInfo,
    pub timestamp: i64,
    pub sign: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyDeviceInfo {
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
        params.device_info.device_name,
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

// ==================== Device ====================

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceParams {
    pub license_id: i64,
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
}

pub async fn register_device(Json(params): Json<RegisterDeviceParams>) -> impl IntoResponse {
    let license_id = params.license_id;
    let svc_params = device_service::RegisterDeviceParams {
        license_id,
        device_id: params.device_id,
        device_info: device_service::DeviceInfo {
            device_name: params.device_name,
            device_type: params.device_type,
            os_version: params.os_version,
            mac_address: params.mac_address,
        },
        ip_address: params.ip_address,
    };
    let result = device_service::register(license_id, svc_params).await;
    ApiResponse::from_result(result)
}

pub async fn obfuscation_config(Query(plugin_id): Query<i64>) -> impl IntoResponse {
    let result = verification_service::get_obfuscation_config(plugin_id).await;
    ApiResponse::from_result(result)
}

// ==================== Card ====================

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
    let page_num = params.page_num.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let result = card_service::list_batches(params.plugin_id, page_num, page_size).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total: total as u64,
        total_pages: ((total as f64) / page_size as f64).ceil() as u64,
        page_num: page_num as u64,
    }))
}

pub async fn export_cards(Path(batch_id): Path<i64>) -> impl IntoResponse {
    let result = card_service::export_batch(batch_id).await;
    ApiResponse::from_result(result)
}

pub async fn redeem_card(Json(params): Json<RedeemCardParams>) -> impl IntoResponse {
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

// ==================== Review ====================

pub async fn review_list(Path(plugin_id): Path<i64>, Query(params): Query<LicenseListParams>) -> impl IntoResponse {
    let page_num = params.page_num.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let result = review_service::list(plugin_id, page_num, page_size).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total: total as u64,
        total_pages: ((total as f64) / page_size as f64).ceil() as u64,
        page_num: page_num as u64,
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewParams {
    pub plugin_id: i64,
    pub rating: i32,
    pub content: String,
}

pub async fn create_review(Json(params): Json<CreateReviewParams>) -> impl IntoResponse {
    let svc_params = review_service::CreateReviewParams {
        plugin_id: params.plugin_id,
        rating: params.rating,
        content: params.content,
    };
    let result = review_service::create(0, svc_params).await;
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

// ==================== Developer ====================

#[derive(Debug, Deserialize)]
pub struct RegisterDeveloperParams {
    pub name: String,
    pub contact: Option<String>,
    pub description: Option<String>,
}

pub async fn register_developer(Json(params): Json<RegisterDeveloperParams>) -> impl IntoResponse {
    let svc_params = developer_service::RegisterDeveloperParams {
        name: params.name,
        description: params.description,
        contact: params.contact,
    };
    let result = developer_service::register(0, svc_params).await;
    ApiResponse::from_result(result)
}

pub async fn developer_profile() -> impl IntoResponse {
    let result = developer_service::get_by_user_id(0).await;
    ApiResponse::from_result(result)
}

#[derive(Debug, Deserialize)]
pub struct UpdateDeveloperParams {
    pub name: Option<String>,
    pub logo: Option<String>,
    pub contact: Option<String>,
    pub description: Option<String>,
}

pub async fn update_developer(Json(params): Json<UpdateDeveloperParams>) -> impl IntoResponse {
    let svc_params = developer_service::UpdateDeveloperParams {
        name: params.name,
        logo: params.logo,
        description: params.description,
        contact: params.contact,
    };
    let result = developer_service::update(0, svc_params).await;
    ApiResponse::from_result(result)
}

// ==================== Subscription ====================

pub async fn subscription_list(Query(params): Query<LicenseListParams>) -> impl IntoResponse {
    let page_num = params.page_num.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let result = subscription_service::list(0, page_num, page_size).await;
    ApiResponse::from_result(result.map(|(list, total)| ListData {
        list,
        total: total as u64,
        total_pages: ((total as f64) / page_size as f64).ceil() as u64,
        page_num: page_num as u64,
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

#[derive(serde::Deserialize)]
pub struct ChartParams {
    pub r#type: String,
    pub plugin_id: Option<i64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

pub async fn developer_chart(Query(_params): Query<ChartParams>) -> impl IntoResponse {
    let result = developer_service::get_chart(0).await;
    ApiResponse::from_result(result)
}

pub async fn developer_ranking(Query(_params): Query<ChartParams>) -> impl IntoResponse {
    let result = developer_service::get_ranking(0).await;
    ApiResponse::from_result(result)
}

#[derive(serde::Deserialize)]
pub struct PreviewCardParams {
    pub card_no: String,
    pub card_pwd: String,
}

pub async fn preview_card(Json(params): Json<PreviewCardParams>) -> impl IntoResponse {
    let result = card_service::preview_card(&params.card_no, &params.card_pwd).await;
    ApiResponse::from_result(result)
}

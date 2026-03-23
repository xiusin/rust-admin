use crate::common::error::Error;
use crate::common::result::ApiResponse;
use crate::application::consumer::after_sale_service;
use crate::application::consumer::refund_service::{self, RefundListParams};
use crate::application::consumer::after_sale_logistics_service;
use crate::application::consumer::timeout_service;
use crate::domain::model::m_after_sale::*;
use crate::api::web_path::{WebPath, WebPathType};
use axum::routing::{get, post};
use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;
use rust_decimal::Decimal;

pub async fn apply(
    Json(args): Json<ApplyAfterSaleArgs>,
) -> Result<Json<ApiResponse<AfterSaleModel>>, Error> {
    args.validate()?;
    let params = ApplyAfterSaleParams {
        order_id: args.order_id,
        order_no: args.order_no,
        consumer_id: args.consumer_id,
        r#type: args.r#type,
        reason: args.reason,
        description: args.description,
        evidence_urls: args.evidence_urls,
        items: args.items.into_iter().map(|i| AfterSaleItemParams {
            order_item_id: i.order_item_id,
            product_id: i.product_id,
            product_name: i.product_name,
            sku_id: i.sku_id,
            sku_name: i.sku_name,
            quantity: i.quantity,
            unit_price: i.unit_price,
            refund_amount: i.refund_amount,
        }).collect(),
    };
    let result = after_sale_service::apply(params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn audit(
    Json(args): Json<AuditAfterSaleArgs>,
) -> Result<Json<ApiResponse<AfterSaleModel>>, Error> {
    args.validate()?;
    let params = AuditAfterSaleParams {
        after_sale_id: args.after_sale_id,
        agree: args.agree,
        reject_reason: args.reject_reason,
        processor_id: args.processor_id,
        processor_name: args.processor_name,
    };
    let result = after_sale_service::audit(params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_detail(
    Query(args): Query<AfterSaleIdArgs>,
) -> Result<Json<ApiResponse<AfterSaleDetailModel>>, Error> {
    let result = after_sale_service::get_detail(args.after_sale_id).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn list(
    Query(args): Query<AfterSaleListArgs>,
) -> Result<Json<ApiResponse<AfterSaleListResp>>, Error> {
    let params = AfterSaleListParams {
        page_num: args.page_num,
        page_size: args.page_size,
        consumer_id: args.consumer_id,
        order_id: args.order_id,
        status: args.status,
        r#type: args.r#type,
        start_time: args.start_time,
        end_time: args.end_time,
    };
    let (list, total) = after_sale_service::list(params).await?;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);
    let total_pages = (total + page_size as u64 - 1) / page_size as u64;
    
    Ok(Json(ApiResponse::success(AfterSaleListResp {
        list,
        total,
        page_num,
        total_pages,
    })))
}

pub async fn close(
    Json(args): Json<CloseAfterSaleArgs>,
) -> Result<Json<ApiResponse<AfterSaleModel>>, Error> {
    let result = after_sale_service::close(
        args.after_sale_id,
        &args.operator_type,
        args.operator_id,
        args.reason,
    ).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_refund(
    Json(args): Json<CreateRefundArgs>,
) -> Result<Json<ApiResponse<AfterSaleRefundModel>>, Error> {
    let params = CreateRefundParams {
        after_sale_id: args.after_sale_id,
        refund_channel: args.refund_channel,
        transaction_id: args.transaction_id,
        refund_amount: args.refund_amount,
    };
    let result = refund_service::create_refund(params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn refund_callback(
    Json(args): Json<RefundCallbackArgs>,
) -> Result<Json<ApiResponse<AfterSaleRefundModel>>, Error> {
    let params = RefundCallbackParams {
        refund_no: args.refund_no,
        status: args.status,
        transaction_id: args.transaction_id,
        callback_data: args.callback_data,
        fail_reason: args.fail_reason,
    };
    let result = refund_service::handle_callback(params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn submit_logistics(
    Json(args): Json<SubmitLogisticsArgs>,
) -> Result<Json<ApiResponse<AfterSaleLogisticsModel>>, Error> {
    let params = SubmitLogisticsParams {
        after_sale_id: args.after_sale_id,
        logistics_company: args.logistics_company,
        tracking_no: args.tracking_no,
        sender_name: args.sender_name,
        sender_phone: args.sender_phone,
        sender_address: args.sender_address,
    };
    let result = after_sale_logistics_service::submit_logistics(params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn confirm_receive(
    Json(args): Json<ConfirmReceiveArgs>,
) -> Result<Json<ApiResponse<AfterSaleLogisticsModel>>, Error> {
    let params = ConfirmReceiveParams {
        after_sale_id: args.after_sale_id,
        operator_id: args.operator_id,
        operator_name: args.operator_name,
    };
    let result = after_sale_logistics_service::confirm_receive(params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_logistics(
    Query(args): Query<AfterSaleIdArgs>,
) -> Result<Json<ApiResponse<Option<AfterSaleLogisticsModel>>>, Error> {
    let result = after_sale_logistics_service::get_logistics(args.after_sale_id).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_statistics() -> Result<Json<ApiResponse<AfterSaleStatistics>>, Error> {
    let result = timeout_service::get_after_sale_statistics().await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_timeout_configs() -> Result<Json<ApiResponse<Vec<TimeoutConfigModel>>>, Error> {
    let result = timeout_service::get_timeout_configs().await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn refund_list(
    Query(args): Query<RefundListArgs>,
) -> Result<Json<ApiResponse<RefundListResp>>, Error> {
    let (list, total) = refund_service::list_refunds(RefundListParams {
        page_num: args.page_num,
        page_size: args.page_size,
        status: args.status.clone(),
        refund_no: args.refund_no.clone(),
    }).await?;
    let page_num = args.page_num.unwrap_or(1);
    let page_size = args.page_size.unwrap_or(10);
    let total_pages = (total + page_size as u64 - 1) / page_size as u64;
    
    Ok(Json(ApiResponse::success(RefundListResp {
        list,
        total,
        page_num,
        total_pages,
    })))
}

pub fn after_sale_api() -> WebPath {
    WebPath::new()
        .route("/apply", WebPathType::Post, Some("申请售后"), post(apply))
        .route("/audit", WebPathType::Post, Some("审核售后"), post(audit))
        .route("/detail", WebPathType::Get, Some("售后详情"), get(get_detail))
        .route("/list", WebPathType::Get, Some("售后列表"), get(list))
        .route("/close", WebPathType::Post, Some("关闭售后"), post(close))
        .route("/refund/create", WebPathType::Post, Some("创建退款"), post(create_refund))
        .route("/refund/callback", WebPathType::Post, Some("退款回调"), post(refund_callback))
        .route("/refund/list", WebPathType::Get, Some("退款列表"), get(refund_list))
        .route("/logistics/submit", WebPathType::Post, Some("提交物流"), post(submit_logistics))
        .route("/logistics/confirm", WebPathType::Post, Some("确认收货"), post(confirm_receive))
        .route("/logistics/get", WebPathType::Get, Some("获取物流"), get(get_logistics))
        .route("/statistics", WebPathType::Get, Some("售后统计"), get(get_statistics))
        .route("/timeout-configs", WebPathType::Get, Some("超时配置"), get(get_timeout_configs))
}

#[derive(Debug, Deserialize, Validate)]
pub struct ApplyAfterSaleArgs {
    pub order_id: i64,
    pub order_no: String,
    pub consumer_id: i64,
    pub r#type: AfterSaleType,
    #[validate(length(min = 1, max = 500, message = "原因长度1-500"))]
    pub reason: String,
    pub description: Option<String>,
    pub evidence_urls: Option<Vec<String>>,
    #[validate(length(min = 1, message = "至少选择一个商品"))]
    pub items: Vec<AfterSaleItemArgs>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AfterSaleItemArgs {
    pub order_item_id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub sku_id: Option<i64>,
    pub sku_name: Option<String>,
    #[validate(range(min = 1, message = "数量至少为1"))]
    pub quantity: i32,
    pub unit_price: Decimal,
    pub refund_amount: Decimal,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuditAfterSaleArgs {
    pub after_sale_id: i64,
    pub agree: bool,
    pub reject_reason: Option<String>,
    pub processor_id: i64,
    pub processor_name: String,
}

#[derive(Debug, Deserialize)]
pub struct AfterSaleIdArgs {
    pub after_sale_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct AfterSaleListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub consumer_id: Option<i64>,
    pub order_id: Option<i64>,
    pub status: Option<String>,
    pub r#type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CloseAfterSaleArgs {
    pub after_sale_id: i64,
    pub operator_type: String,
    pub operator_id: Option<i64>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRefundArgs {
    pub after_sale_id: i64,
    pub refund_channel: String,
    pub transaction_id: Option<String>,
    pub refund_amount: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct RefundCallbackArgs {
    pub refund_no: String,
    pub status: String,
    pub transaction_id: Option<String>,
    pub callback_data: Option<String>,
    pub fail_reason: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SubmitLogisticsArgs {
    pub after_sale_id: i64,
    #[validate(length(min = 1, max = 50, message = "物流公司名称1-50"))]
    pub logistics_company: String,
    #[validate(length(min = 1, max = 100, message = "物流单号1-100"))]
    pub tracking_no: String,
    pub sender_name: Option<String>,
    pub sender_phone: Option<String>,
    pub sender_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmReceiveArgs {
    pub after_sale_id: i64,
    pub operator_id: i64,
    pub operator_name: String,
}

#[derive(Debug, Serialize)]
pub struct AfterSaleListResp {
    pub list: Vec<AfterSaleModel>,
    pub total: u64,
    pub page_num: u32,
    pub total_pages: u64,
}

#[derive(Debug, Deserialize)]
pub struct RefundListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub status: Option<String>,
    pub refund_no: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RefundListResp {
    pub list: Vec<AfterSaleRefundModel>,
    pub total: u64,
    pub page_num: u32,
    pub total_pages: u64,
}

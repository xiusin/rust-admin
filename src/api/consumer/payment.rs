use axum::routing::{get, post};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::consumer::payment_service;
use crate::common::error::Error;
use crate::common::result::ApiResponse;
use crate::model::prelude::ListData;
use crate::domain::model::m_payment::*;
use axum::{extract::{Path, Query}, Json};
use rust_decimal::Decimal;
use validator::Validate;
use serde::{Deserialize, Serialize};

fn parse_payment_method(s: &str) -> PaymentMethod {
    match s.to_lowercase().as_str() {
        "wechat" => PaymentMethod::Wechat,
        "alipay" => PaymentMethod::Alipay,
        "yeepay" => PaymentMethod::Yeepay,
        _ => PaymentMethod::Wechat,
    }
}

fn parse_payment_type(s: &str) -> PaymentType {
    match s.to_lowercase().as_str() {
        "app" => PaymentType::App,
        "h5" => PaymentType::H5,
        "mini" => PaymentType::Mini,
        "qrcode" => PaymentType::Qrcode,
        _ => PaymentType::App,
    }
}

pub async fn create_payment(
    Json(args): Json<CreatePaymentArgs>,
) -> Result<Json<ApiResponse<PaymentOrderModel>>, Error> {
    args.validate()?;

    let result = payment_service::create_order(CreatePaymentParams {
        consumer_id: args.consumer_id,
        payment_method: parse_payment_method(&args.payment_method),
        payment_type: parse_payment_type(&args.payment_type),
        amount: Decimal::try_from(args.amount).unwrap_or(Decimal::ZERO),
        order_no: None,
    })
    .await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_payment(
    Path(order_no): Path<String>,
) -> Result<Json<ApiResponse<Option<PaymentOrderModel>>>, Error> {
    let result = payment_service::get_order(&order_no).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn list_payments(
    Query(params): Query<PaymentListArgs>,
) -> Result<Json<ApiResponse<ListData<PaymentOrderModel>>>, Error> {
    let page_size = params.page_size;
    let page_num = params.page_num;
    let (items, total) = payment_service::list_orders(
        PaymentOrderSearchParams {
            page_num: params.page_num,
            page_size: params.page_size,
            status: params.status,
            start_time: None,
            end_time: None,
            consumer_id: None,
            order_no: None,
            payment_method: None,
        },
    )
    .await?;

    Ok(Json(ApiResponse::success(ListData {
        list: items,
        total,
        total_pages: (total + page_size.unwrap_or(10) as u64 - 1) / page_size.unwrap_or(10) as u64,
        page_num: page_num.unwrap_or(1) as u64,
    })))
}

pub async fn refund(
    Json(args): Json<RefundArgs>,
) -> Result<Json<ApiResponse<PaymentOrderModel>>, Error> {
    args.validate()?;
    let params = RefundParams {
        order_no: args.order_no,
        refund_amount: Decimal::try_from(args.refund_amount).unwrap_or(Decimal::ZERO),
        reason: args.reason,
    };
    let result = payment_service::refund(params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn close(
    Path(order_no): Path<String>,
) -> Result<Json<ApiResponse<PaymentOrderModel>>, Error> {
    let result = payment_service::close_order(&order_no).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn statistics(
    Query(args): Query<StatisticsArgs>,
) -> Result<Json<ApiResponse<PaymentStatistics>>, Error> {
    let result = payment_service::get_payment_statistics(args.consumer_id).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub fn payment_api() -> WebPath {
    WebPath::new()
        .route("/create", WebPathType::Post, Some("创建支付订单"), post(create_payment))
        .route("/{order_no}", WebPathType::Get, Some("获取支付订单"), get(get_payment))
        .route("/list", WebPathType::Get, Some("支付订单列表"), get(list_payments))
        .route("/refund", WebPathType::Post, Some("退款"), post(refund))
        .route("/close/{order_no}", WebPathType::Post, Some("关闭订单"), post(close))
        .route("/statistics", WebPathType::Get, Some("支付统计"), get(statistics))
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreatePaymentArgs {
    pub consumer_id: i64,
    #[validate(length(min = 1, message = "支付方式不能为空"))]
    pub payment_method: String,
    #[validate(length(min = 1, message = "支付类型不能为空"))]
    pub payment_type: String,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RefundArgs {
    #[validate(length(min = 1, message = "订单号不能为空"))]
    pub order_no: String,
    pub refund_amount: f64,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsArgs {
    pub consumer_id: Option<i64>,
}

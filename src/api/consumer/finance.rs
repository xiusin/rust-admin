use axum::routing::{get, post};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::consumer::finance_service;
use crate::common::error::Error;
use crate::common::result::ApiResponse;
use crate::model::prelude::ListData;
use crate::domain::args::a_finance::*;
use crate::domain::model::m_finance::*;
use axum::{extract::{Path, Query}, Json};
use rust_decimal::Decimal;
use validator::Validate;

pub async fn get_account(
    Path(consumer_id): Path<i64>,
) -> Result<Json<ApiResponse<AccountInfoResp>>, Error> {
    let result = finance_service::get_account(consumer_id).await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn recharge(
    Path(consumer_id): Path<i64>,
    Json(args): Json<RechargeArgs>,
) -> Result<Json<ApiResponse<TransactionModel>>, Error> {
    args.validate()?;

    let amount = Decimal::try_from(args.amount * 100.0).unwrap_or(Decimal::ZERO);

    let result = finance_service::recharge(RechargeParams {
        consumer_id,
        amount,
        related_order_no: Some(args.payment_order_no),
    })
    .await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn withdraw(
    Path(consumer_id): Path<i64>,
    Json(args): Json<WithdrawArgs>,
) -> Result<Json<ApiResponse<TransactionModel>>, Error> {
    args.validate()?;

    let amount = Decimal::try_from(args.amount * 100.0).unwrap_or(Decimal::ZERO);

    let result = finance_service::withdraw(WithdrawParams {
        consumer_id,
        amount,
    })
    .await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn list_transactions(
    Query(params): Query<TransactionListArgs>,
) -> Result<Json<ApiResponse<ListData<TransactionModel>>>, Error> {
    let page_size = params.page_size;
    let page_num = params.page_num;
    let (items, total) = finance_service::list_transactions(
        TransactionListParams {
            page_num: params.page_num,
            page_size: params.page_size,
            transaction_type: params.transaction_type,
            start_time: params.start_time,
            end_time: params.end_time,
            consumer_id: params.consumer_id,
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

pub fn finance_api() -> WebPath {
    WebPath::new()
        .route("/account/{id}", WebPathType::Get, Some("获取账户信息"), get(get_account))
        .route("/recharge/{id}", WebPathType::Post, Some("充值"), post(recharge))
        .route("/withdraw/{id}", WebPathType::Post, Some("提现"), post(withdraw))
        .route("/transactions", WebPathType::Get, Some("交易流水列表"), get(list_transactions))
}
use axum::{extract::Query, response::IntoResponse, Json};
use serde::Deserialize;
use crate::application::product::s_p_stock as stock_service;
use crate::domain::args::a_stock::*;
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct StoreIdPath {
    pub store_id: i64,
}

pub async fn list(Query(args): Query<StockListArgs>) -> impl IntoResponse {
    let result = stock_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn log_list(Query(args): Query<StockLogListArgs>) -> impl IntoResponse {
    let result = stock_service::log_list(args).await;
    ApiResponse::from_result(result)
}

pub async fn adjust(Json(args): Json<StockAdjustArgs>) -> impl IntoResponse {
    let result = stock_service::adjust(args, 0, "").await;
    ApiResponse::from_result(result)
}

pub async fn alert_list(Query(args): Query<StockAlertListArgs>) -> impl IntoResponse {
    let result = stock_service::alert_list(args).await;
    ApiResponse::from_result(result)
}

pub async fn alert_config(Json(args): Json<StockAlertConfigArgs>) -> impl IntoResponse {
    let result = stock_service::alert_config(args, 0, "").await;
    ApiResponse::from_result(result)
}

pub async fn statistics() -> impl IntoResponse {
    let result = stock_service::statistics().await;
    ApiResponse::from_result(result)
}

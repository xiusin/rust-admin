use axum::{extract::{Path, Query}, response::IntoResponse, Json};
use serde::Deserialize;
use crate::application::product::s_p_store as store_service;
use crate::domain::args::a_store::*;
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct StoreIdPath {
    pub id: i64,
}

pub async fn list(Query(args): Query<StoreListArgs>) -> impl IntoResponse {
    let result = store_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<StoreIdPath>) -> impl IntoResponse {
    let result = store_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<StoreAddArgs>) -> impl IntoResponse {
    let result = store_service::add(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<StoreEditArgs>) -> impl IntoResponse {
    let result = store_service::edit(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Json(args): Json<StoreDeleteArgs>) -> impl IntoResponse {
    let result = store_service::delete(args).await;
    ApiResponse::from_result(result)
}

pub async fn stock_list(Query(args): Query<StoreStockListArgs>) -> impl IntoResponse {
    let result = store_service::stock_list(args).await;
    ApiResponse::from_result(result)
}

pub async fn stock_adjust(Json(args): Json<StoreStockAdjustArgs>) -> impl IntoResponse {
    let result = store_service::stock_adjust(args, 0, "").await;
    ApiResponse::from_result(result)
}

pub async fn simple_list() -> impl IntoResponse {
    let result = store_service::simple_list().await;
    ApiResponse::from_result(result)
}

pub async fn statistics() -> impl IntoResponse {
    let result = store_service::statistics().await;
    ApiResponse::from_result(result)
}

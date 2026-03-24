use axum::{extract::{Path, Query}, response::IntoResponse, Json};
use serde::Deserialize;
use crate::application::product::s_p_product as product_service;
use crate::domain::args::a_product::*;
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ProductIdPath {
    pub product_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct SkuIdPath {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct BatchDeleteQuery {
    pub ids: String,
}

pub async fn sku_list(Path(path): Path<ProductIdPath>) -> impl IntoResponse {
    let result = product_service::sku_list(path.product_id).await;
    ApiResponse::from_result(result)
}

pub async fn sku_detail(Path(path): Path<SkuIdPath>) -> impl IntoResponse {
    let result = product_service::sku_detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn sku_add(Json(args): Json<SkuAddArgs>) -> impl IntoResponse {
    let result = product_service::sku_add(args).await;
    ApiResponse::from_result(result)
}

pub async fn sku_edit(Json(args): Json<SkuEditArgs>) -> impl IntoResponse {
    let result = product_service::sku_edit(args).await;
    ApiResponse::from_result(result)
}

pub async fn sku_delete(Query(query): Query<BatchDeleteQuery>) -> impl IntoResponse {
    let ids: Vec<i64> = query.ids.split(',').filter_map(|s| s.parse().ok()).collect();
    let result = product_service::sku_delete(SkuDeleteArgs { ids }).await;
    ApiResponse::from_result(result)
}

pub async fn sku_generate(Json(args): Json<SkuGenerateArgs>) -> impl IntoResponse {
    let result = product_service::sku_generate(args).await;
    ApiResponse::from_result(result)
}

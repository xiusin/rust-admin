use axum::{extract::{Path, Query}, response::IntoResponse, Json};
use serde::Deserialize;
use crate::application::product::s_p_product as product_service;
use crate::domain::args::a_product::*;
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ProductIdPath {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct BatchDeleteQuery {
    pub ids: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductStatusQuery {
    pub id: i64,
    pub status: i32,
}

#[derive(Debug, Deserialize)]
pub struct ProductAuditQuery {
    pub id: i64,
    pub audit_status: i32,
    pub audit_remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BatchStatusQuery {
    pub ids: String,
    pub status: i32,
}

pub async fn list(
    Query(args): Query<ProductListArgs>,
) -> impl IntoResponse {
    let result = product_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<ProductIdPath>) -> impl IntoResponse {
    let result = product_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<ProductAddArgs>) -> impl IntoResponse {
    let result = product_service::add(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<ProductEditArgs>) -> impl IntoResponse {
    let result = product_service::edit(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Query(query): Query<BatchDeleteQuery>) -> impl IntoResponse {
    let ids: Vec<i64> = query.ids.split(',').filter_map(|s| s.parse().ok()).collect();
    let result = product_service::batch_delete(ProductBatchDeleteArgs { ids }).await;
    ApiResponse::from_result(result)
}

pub async fn update_status(Query(query): Query<ProductStatusQuery>) -> impl IntoResponse {
    let result = product_service::update_status(query.id, query.status).await;
    ApiResponse::from_result(result)
}

pub async fn audit(Query(query): Query<ProductAuditQuery>) -> impl IntoResponse {
    let result = product_service::audit(query.id, query.audit_status, query.audit_remark).await;
    ApiResponse::from_result(result)
}

pub async fn batch_update_status(Query(query): Query<BatchStatusQuery>) -> impl IntoResponse {
    let ids: Vec<i64> = query.ids.split(',').filter_map(|s| s.parse().ok()).collect();
    let result = product_service::batch_update_status(ids, query.status).await;
    ApiResponse::from_result(result)
}

pub async fn batch_delete(Query(query): Query<BatchDeleteQuery>) -> impl IntoResponse {
    let ids: Vec<i64> = query.ids.split(',').filter_map(|s| s.parse().ok()).collect();
    let result = product_service::batch_delete(ProductBatchDeleteArgs { ids }).await;
    ApiResponse::from_result(result)
}

pub async fn statistics() -> impl IntoResponse {
    let result = product_service::statistics().await;
    ApiResponse::from_result(result)
}

pub async fn simple_list(Query(args): Query<Option<ProductSimpleListArgs>>) -> impl IntoResponse {
    let result = product_service::simple_list(args).await;
    ApiResponse::from_result(result)
}

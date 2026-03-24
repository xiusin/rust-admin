use axum::{extract::{Path, Query}, response::IntoResponse, Json};
use serde::Deserialize;
use crate::application::product::s_p_category as category_service;
use crate::domain::args::a_product_category::*;
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct CategoryIdPath {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct CategoryDeleteQuery {
    pub ids: String,
}

#[derive(Debug, Deserialize)]
pub struct CategoryStatusJson {
    pub id: i64,
    pub status: String,
}

pub async fn tree(Query(args): Query<CategoryTreeArgs>) -> impl IntoResponse {
    let result = category_service::tree(args).await;
    ApiResponse::from_result(result)
}

pub async fn list(Query(args): Query<CategoryListArgs>) -> impl IntoResponse {
    let result = category_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<CategoryIdPath>) -> impl IntoResponse {
    let result = category_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<CategoryAddArgs>) -> impl IntoResponse {
    let result = category_service::add(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<CategoryEditArgs>) -> impl IntoResponse {
    let result = category_service::edit(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Query(query): Query<CategoryDeleteQuery>) -> impl IntoResponse {
    let ids: Vec<i64> = query.ids.split(',').filter_map(|s| s.parse().ok()).collect();
    let result = category_service::delete(CategoryDeleteArgs { ids }).await;
    ApiResponse::from_result(result)
}

pub async fn update_status(Json(args): Json<CategoryStatusJson>) -> impl IntoResponse {
    let result = category_service::update_status(args.id, args.status).await;
    ApiResponse::from_result(result)
}

use axum::{extract::{Path, Query}, response::IntoResponse, Json};
use serde::Deserialize;
use crate::application::product::s_p_brand as brand_service;
use crate::domain::args::a_product_brand::*;
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct BrandIdPath {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct BrandDeleteQuery {
    pub ids: String,
}

#[derive(Debug, Deserialize)]
pub struct BrandStatusJson {
    pub id: i64,
    pub status: String,
}

pub async fn list(Query(args): Query<BrandListArgs>) -> impl IntoResponse {
    let result = brand_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<BrandIdPath>) -> impl IntoResponse {
    let result = brand_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<BrandAddArgs>) -> impl IntoResponse {
    let result = brand_service::add(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<BrandEditArgs>) -> impl IntoResponse {
    let result = brand_service::edit(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Query(query): Query<BrandDeleteQuery>) -> impl IntoResponse {
    let ids: Vec<i64> = query.ids.split(',').filter_map(|s| s.parse().ok()).collect();
    let result = brand_service::delete(BrandDeleteArgs { ids }).await;
    ApiResponse::from_result(result)
}

pub async fn update_status(Json(args): Json<BrandStatusJson>) -> impl IntoResponse {
    let result = brand_service::update_status(args.id, args.status).await;
    ApiResponse::from_result(result)
}

pub async fn simple_list() -> impl IntoResponse {
    let result = brand_service::simple_list().await;
    ApiResponse::from_result(result)
}

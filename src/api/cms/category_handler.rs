use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::application::cms::cms_category_service as category_service;
use crate::domain::args::a_cms_category::{
    CmsCategoryAddReq, CmsCategoryEditReq, CmsCategoryListReq, CmsCategoryTreeReq, CategorySortReq,
};
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct CategoryIdPath {
    pub id: i64,
}

pub async fn list(Query(args): Query<CmsCategoryListReq>) -> impl IntoResponse {
    let result = category_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn tree(Query(args): Query<CmsCategoryTreeReq>) -> impl IntoResponse {
    let result = category_service::tree(args).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<CategoryIdPath>) -> impl IntoResponse {
    let result = category_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<CmsCategoryAddReq>) -> impl IntoResponse {
    let result = category_service::add(args).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<CmsCategoryEditReq>) -> impl IntoResponse {
    let result = category_service::edit(args).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Path(path): Path<CategoryIdPath>) -> impl IntoResponse {
    let result = category_service::delete(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn sort(Json(args): Json<CategorySortReq>) -> impl IntoResponse {
    let result = category_service::sort(args).await;
    ApiResponse::from_result(result)
}

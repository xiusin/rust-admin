use axum::{extract::{Path, Query}, response::IntoResponse, Json};
use serde::Deserialize;
use crate::application::product::s_p_product_group as product_group_service;
use crate::domain::args::a_product_group::*;
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct GroupIdPath {
    pub id: i64,
}

pub async fn list(Query(args): Query<ProductGroupListArgs>) -> impl IntoResponse {
    let result = product_group_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<GroupIdPath>) -> impl IntoResponse {
    let result = product_group_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<ProductGroupAddArgs>) -> impl IntoResponse {
    let result = product_group_service::add(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<ProductGroupEditArgs>) -> impl IntoResponse {
    let result = product_group_service::edit(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Json(args): Json<ProductGroupDeleteArgs>) -> impl IntoResponse {
    let result = product_group_service::delete(args).await;
    ApiResponse::from_result(result)
}

pub async fn simple_list() -> impl IntoResponse {
    let result = product_group_service::simple_list().await;
    ApiResponse::from_result(result)
}

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::application::cms::cms_tag_service as tag_service;
use crate::domain::args::a_cms_tag::{
    CmsTagAddReq, CmsTagBatchAddReq, CmsTagEditReq, CmsTagListReq,
};
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct TagIdPath {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct TagCloudQuery {
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    50
}

pub async fn list(Query(args): Query<CmsTagListReq>) -> impl IntoResponse {
    let result = tag_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<TagIdPath>) -> impl IntoResponse {
    let result = tag_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<CmsTagAddReq>) -> impl IntoResponse {
    let result = tag_service::add(args).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<CmsTagEditReq>) -> impl IntoResponse {
    let result = tag_service::edit(args).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Path(path): Path<TagIdPath>) -> impl IntoResponse {
    let result = tag_service::delete(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn batch_add(Json(args): Json<CmsTagBatchAddReq>) -> impl IntoResponse {
    let result = tag_service::batch_add(args).await;
    ApiResponse::from_result(result)
}

pub async fn cloud(Query(query): Query<TagCloudQuery>) -> impl IntoResponse {
    let result = tag_service::get_cloud(query.limit).await;
    ApiResponse::from_result(result)
}

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::application::cms::cms_model_service as model_service;
use crate::domain::args::a_cms_model::{CmsModelAddReq, CmsModelEditReq, CmsModelListReq};
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ModelIdPath {
    pub id: i64,
}

pub async fn list(Query(args): Query<CmsModelListReq>) -> impl IntoResponse {
    let result = model_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<ModelIdPath>) -> impl IntoResponse {
    let result = model_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<CmsModelAddReq>) -> impl IntoResponse {
    let result = model_service::add(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<CmsModelEditReq>) -> impl IntoResponse {
    let result = model_service::edit(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Path(path): Path<ModelIdPath>) -> impl IntoResponse {
    let result = model_service::delete(path.id, 0).await;
    ApiResponse::from_result(result)
}

pub async fn enable(Path(path): Path<ModelIdPath>) -> impl IntoResponse {
    let result = model_service::enable(path.id, 0).await;
    ApiResponse::from_result(result)
}

pub async fn disable(Path(path): Path<ModelIdPath>) -> impl IntoResponse {
    let result = model_service::disable(path.id, 0).await;
    ApiResponse::from_result(result)
}

pub async fn copy(Path(path): Path<ModelIdPath>) -> impl IntoResponse {
    let result = model_service::copy(path.id, 0).await;
    ApiResponse::from_result(result)
}

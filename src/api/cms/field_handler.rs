use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::application::cms::cms_field_service as field_service;
use crate::domain::args::a_cms_field::{CmsFieldAddReq, CmsFieldEditReq, CmsFieldSortReq};
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct FieldIdPath {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ModelIdPath {
    pub model_id: i64,
}

pub async fn list(Path(path): Path<ModelIdPath>) -> impl IntoResponse {
    let result = field_service::list(path.model_id).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<FieldIdPath>) -> impl IntoResponse {
    let result = field_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<CmsFieldAddReq>) -> impl IntoResponse {
    let result = field_service::add(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<CmsFieldEditReq>) -> impl IntoResponse {
    let result = field_service::edit(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Path(path): Path<FieldIdPath>) -> impl IntoResponse {
    let result = field_service::delete(path.id, 0).await;
    ApiResponse::from_result(result)
}

pub async fn sort(Json(args): Json<CmsFieldSortReq>) -> impl IntoResponse {
    let result = field_service::sort(args, 0).await;
    ApiResponse::from_result(result)
}

use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::application::cms::form_config_service as form_service;
use crate::application::cms::form_render_service;
use crate::domain::args::a_form_config::{FormConfigAddReq, FormConfigEditReq};
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct FormIdPath {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ModelIdPath {
    pub model_id: i64,
}

pub async fn list(Path(path): Path<ModelIdPath>) -> impl IntoResponse {
    let result = form_service::list(path.model_id).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<FormIdPath>) -> impl IntoResponse {
    let result = form_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<FormConfigAddReq>) -> impl IntoResponse {
    let result = form_service::add(args).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<FormConfigEditReq>) -> impl IntoResponse {
    let result = form_service::edit(args).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Path(path): Path<FormIdPath>) -> impl IntoResponse {
    let result = form_service::delete(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn preview(Path(path): Path<FormIdPath>) -> impl IntoResponse {
    let result = form_render_service::render_schema(path.id).await;
    ApiResponse::from_result(result)
}

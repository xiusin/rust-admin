use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::application::cms::table_config_service as table_service;
use crate::application::cms::table_render_service;
use crate::domain::args::a_table_config::{TableConfigAddReq, TableConfigEditReq};
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct TableIdPath {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ModelIdPath {
    pub model_id: i64,
}

pub async fn list(Path(path): Path<ModelIdPath>) -> impl IntoResponse {
    let result = table_service::list(path.model_id).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<TableIdPath>) -> impl IntoResponse {
    let result = table_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<TableConfigAddReq>) -> impl IntoResponse {
    let result = table_service::add(args).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<TableConfigEditReq>) -> impl IntoResponse {
    let result = table_service::edit(args).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Path(path): Path<TableIdPath>) -> impl IntoResponse {
    let result = table_service::delete(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn preview(Path(path): Path<TableIdPath>) -> impl IntoResponse {
    let result = table_render_service::render_schema(path.id).await;
    ApiResponse::from_result(result)
}

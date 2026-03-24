use axum::{extract::{Path, Query}, response::IntoResponse, Json};
use serde::Deserialize;
use crate::application::product::s_p_attribute_template as attribute_template_service;
use crate::domain::args::a_attribute_template::*;
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct TemplateIdPath {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct CategoryIdPath {
    pub category_id: i64,
}

pub async fn list(Query(args): Query<AttributeTemplateListArgs>) -> impl IntoResponse {
    let result = attribute_template_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<TemplateIdPath>) -> impl IntoResponse {
    let result = attribute_template_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<AttributeTemplateAddArgs>) -> impl IntoResponse {
    let result = attribute_template_service::add(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<AttributeTemplateEditArgs>) -> impl IntoResponse {
    let result = attribute_template_service::edit(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Json(args): Json<AttributeTemplateDeleteArgs>) -> impl IntoResponse {
    let result = attribute_template_service::delete(args).await;
    ApiResponse::from_result(result)
}

pub async fn by_category(Path(path): Path<CategoryIdPath>) -> impl IntoResponse {
    let result = attribute_template_service::by_category(path.category_id).await;
    ApiResponse::from_result(result)
}

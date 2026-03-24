use axum::{extract::{Path, Query}, response::IntoResponse, Json};
use serde::Deserialize;
use crate::application::product::s_p_shipping_template as shipping_template_service;
use crate::domain::args::a_shipping_template::*;
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct TemplateIdPath {
    pub id: i64,
}

pub async fn list(Query(args): Query<ShippingTemplateListArgs>) -> impl IntoResponse {
    let result = shipping_template_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<TemplateIdPath>) -> impl IntoResponse {
    let result = shipping_template_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<ShippingTemplateAddArgs>) -> impl IntoResponse {
    let result = shipping_template_service::add(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<ShippingTemplateEditArgs>) -> impl IntoResponse {
    let result = shipping_template_service::edit(args, 0).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Json(args): Json<ShippingTemplateDeleteArgs>) -> impl IntoResponse {
    let result = shipping_template_service::delete(args).await;
    ApiResponse::from_result(result)
}

pub async fn calculate(Json(args): Json<ShippingCalculateArgs>) -> impl IntoResponse {
    let result = shipping_template_service::calculate(args).await;
    ApiResponse::from_result(result)
}

pub async fn simple_list() -> impl IntoResponse {
    let result = shipping_template_service::simple_list().await;
    ApiResponse::from_result(result)
}

use axum::{extract::Query, response::IntoResponse, Json};
use serde::Deserialize;
use crate::application::product::s_p_verification as verification_service;
use crate::domain::args::a_verification::*;
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct VerificationQuery {
    pub code: String,
}

pub async fn list(Query(args): Query<VerificationCodeListArgs>) -> impl IntoResponse {
    let result = verification_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn verify(Json(args): Json<VerificationArgs>) -> impl IntoResponse {
    let result = verification_service::verify(args, 0, "").await;
    ApiResponse::from_result(result)
}

pub async fn query(Query(query): Query<VerificationQuery>) -> impl IntoResponse {
    let result = verification_service::query(VerificationQueryArgs { code: query.code }).await;
    ApiResponse::from_result(result)
}

pub async fn log_list(Query(args): Query<VerificationLogListArgs>) -> impl IntoResponse {
    let result = verification_service::log_list(args).await;
    ApiResponse::from_result(result)
}

pub async fn statistics() -> impl IntoResponse {
    let result = verification_service::statistics().await;
    ApiResponse::from_result(result)
}

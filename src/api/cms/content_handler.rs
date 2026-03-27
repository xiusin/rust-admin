use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::application::cms::{
    cms_content_service as content_service,
    content_publish_service::{self, ContentStatus},
};
use crate::domain::args::a_cms_content::{CmsContentAddReq, CmsContentEditReq, CmsContentListReq};
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ContentIdPath {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ContentAuditReq {
    pub id: i64,
    pub status: i32,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VersionRollbackReq {
    pub content_id: i64,
    pub version: i32,
}

pub async fn list(Query(args): Query<CmsContentListReq>) -> impl IntoResponse {
    let result = content_service::list(args).await;
    ApiResponse::from_result(result)
}

pub async fn detail(Path(path): Path<ContentIdPath>) -> impl IntoResponse {
    let result = content_service::detail(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn add(Json(args): Json<CmsContentAddReq>) -> impl IntoResponse {
    let result = content_service::add(args).await;
    ApiResponse::from_result(result)
}

pub async fn edit(Json(args): Json<CmsContentEditReq>) -> impl IntoResponse {
    let result = content_service::edit(args).await;
    ApiResponse::from_result(result)
}

pub async fn delete(Path(path): Path<ContentIdPath>) -> impl IntoResponse {
    let result = content_service::delete(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn restore(Path(path): Path<ContentIdPath>) -> impl IntoResponse {
    let result = content_service::restore(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn publish(Path(path): Path<ContentIdPath>) -> impl IntoResponse {
    let result = content_publish_service::publish(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn offline(Path(path): Path<ContentIdPath>) -> impl IntoResponse {
    let result = content_publish_service::offline(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn audit(Json(args): Json<ContentAuditReq>) -> impl IntoResponse {
    let status = match args.status {
        0 => ContentStatus::Draft,
        1 => ContentStatus::Pending,
        2 => ContentStatus::Published,
        3 => ContentStatus::Rejected,
        4 => ContentStatus::Offline,
        _ => ContentStatus::Draft,
    };
    let result = content_publish_service::audit(args.id, status, args.reason).await;
    ApiResponse::from_result(result)
}

pub async fn version(Path(path): Path<ContentIdPath>) -> impl IntoResponse {
    let result = content_publish_service::get_versions(path.id).await;
    ApiResponse::from_result(result)
}

pub async fn version_rollback(Json(args): Json<VersionRollbackReq>) -> impl IntoResponse {
    let result = content_publish_service::rollback(args.content_id, args.version).await;
    ApiResponse::from_result(result)
}

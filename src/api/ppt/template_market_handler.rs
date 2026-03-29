use crate::api::web_path::{WebPath, WebPathType};
use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use axum::routing::{delete, get, post, put};
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn template_market() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("模板列表"), get(list_templates))
        .route("/search", WebPathType::Get, Some("搜索模板"), get(search_templates))
        .route("/recommend", WebPathType::Get, Some("推荐模板"), get(recommend_templates))
        .route("/:id", WebPathType::Get, Some("模板详情"), get(template_detail))
        .route("/upload", WebPathType::Post, Some("上传模板"), post(upload_template))
        .route("/edit", WebPathType::Put, Some("编辑模板"), put(edit_template))
        .route("/del/:id", WebPathType::Delete, Some("删除模板"), delete(delete_template))
        .route("/apply", WebPathType::Post, Some("应用模板"), post(apply_template))
        .route("/rate", WebPathType::Post, Some("评价模板"), post(rate_template))
        .route("/collect", WebPathType::Post, Some("收藏模板"), post(collect_template))
        .route("/favorite/:id", WebPathType::Post, Some("收藏模板"), post(favorite_template))
        .route("/unfavorite/:id", WebPathType::Delete, Some("取消收藏"), delete(unfavorite_template))
        .route("/my", WebPathType::Get, Some("我的模板"), get(my_templates))
        .route("/collected", WebPathType::Get, Some("收藏的模板"), get(collected_templates))
        .route("/ratings/:id", WebPathType::Get, Some("模板评价列表"), get(template_ratings))
        .route("/status", WebPathType::Put, Some("更新模板状态"), put(update_template_status))
}

#[derive(Debug, Serialize)]
pub struct TemplateInfo {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub rating: f32,
    pub download_count: i64,
}

pub async fn list_templates() -> impl axum::response::IntoResponse {
    ApiResponse::success(Vec::<TemplateInfo>::new())
}

#[derive(Debug, Deserialize, Validate)]
pub struct SearchQuery {
    pub keyword: Option<String>,
    pub category: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn search_templates() -> impl axum::response::IntoResponse {
    ApiResponse::success(Vec::<TemplateInfo>::new())
}

pub async fn recommend_templates() -> impl axum::response::IntoResponse {
    ApiResponse::success(Vec::<TemplateInfo>::new())
}

pub async fn template_detail(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(TemplateInfo {
        id: 1,
        name: "示例模板".to_string(),
        description: Some("模板描述".to_string()),
        thumbnail: None,
        rating: 4.5,
        download_count: 100,
    })
}

#[derive(Debug, Deserialize, Validate)]
pub struct UploadTemplateRequest {
    pub name: String,
    pub description: Option<String>,
    pub file_path: String,
}

pub async fn upload_template(VJson(_req): VJson<UploadTemplateRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(1i64)
}

#[derive(Debug, Deserialize, Validate)]
pub struct EditTemplateRequest {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn edit_template(VJson(_req): VJson<EditTemplateRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(())
}

pub async fn delete_template(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct ApplyTemplateRequest {
    pub template_id: i64,
    pub project_id: i64,
}

pub async fn apply_template(VJson(_req): VJson<ApplyTemplateRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success("应用成功")
}

#[derive(Debug, Deserialize, Validate)]
pub struct RateTemplateRequest {
    pub template_id: i64,
    pub rating: i32,
    pub comment: Option<String>,
}

pub async fn rate_template(VJson(_req): VJson<RateTemplateRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success("评价成功")
}

#[derive(Debug, Deserialize, Validate)]
pub struct CollectTemplateRequest {
    pub template_id: i64,
}

pub async fn collect_template(VJson(_req): VJson<CollectTemplateRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success("收藏成功")
}

pub async fn favorite_template(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success("收藏成功")
}

pub async fn unfavorite_template(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success("取消收藏成功")
}

pub async fn my_templates() -> impl axum::response::IntoResponse {
    ApiResponse::success(Vec::<TemplateInfo>::new())
}

pub async fn collected_templates() -> impl axum::response::IntoResponse {
    ApiResponse::success(Vec::<TemplateInfo>::new())
}

#[derive(Debug, Serialize)]
pub struct RatingInfo {
    pub id: i64,
    pub user_id: i64,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: String,
}

pub async fn template_ratings(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(Vec::<RatingInfo>::new())
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateStatusRequest {
    pub id: i64,
    pub status: String,
}

pub async fn update_template_status(VJson(_req): VJson<UpdateStatusRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(())
}

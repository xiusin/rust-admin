use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use crate::api::web_path::{WebPath, WebPathType};
use axum::routing::{get, post};
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn generate_routes() -> WebPath {
    WebPath::new()
        .route("/create", WebPathType::Post, Some("创建PPT生成任务"), post(create))
        .route("/from-document", WebPathType::Post, Some("从文档生成"), post(generate_from_document))
        .route("/from-text", WebPathType::Post, Some("从文本生成"), post(generate_from_text))
        .route("/from-outline", WebPathType::Post, Some("从大纲生成"), post(generate_from_outline))
        .route("/from-chart", WebPathType::Post, Some("从图表生成"), post(generate_from_chart))
        .route("/regenerate/:id", WebPathType::Post, Some("重新生成"), post(regenerate))
        .route("/export/:id", WebPathType::Get, Some("导出PPT"), get(export_ppt))
        .route("/export/:id/pdf", WebPathType::Get, Some("导出PDF"), get(export_pdf))
        .route("/export/:id/images", WebPathType::Get, Some("导出图片"), get(export_images))
        .route("/industries", WebPathType::Get, Some("获取支持的行业"), get(get_industries))
        .route("/preview-style/:industry", WebPathType::Get, Some("预览风格"), get(preview_style))
        .route("/progress/:task_id", WebPathType::Get, Some("获取生成进度"), get(get_progress))
        .route("/cancel/:task_id", WebPathType::Post, Some("取消生成任务"), post(cancel))
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct GenerateFromDocumentRequest {
    pub file_path: String,
    pub file_type: String,
    pub options: Option<GenerationOptionsRequest>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct GenerateFromTextRequest {
    #[validate(length(min = 1, message = "文本不能为空"))]
    pub text: String,
    #[validate(length(min = 1, max = 200, message = "标题长度1-200"))]
    pub title: String,
    pub options: Option<GenerationOptionsRequest>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct GenerateFromOutlineRequest {
    pub outline: Vec<OutlineItemRequest>,
    #[validate(length(min = 1, max = 200, message = "标题长度1-200"))]
    pub title: String,
    pub style: Option<StyleParamsRequest>,
    pub options: Option<GenerationOptionsRequest>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct GenerateFromChartRequest {
    pub chart_data: serde_json::Value,
    pub title: String,
    pub style: Option<StyleParamsRequest>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegenerateRequest {
    pub project_id: i64,
    pub page_indices: Option<Vec<usize>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerationOptionsRequest {
    pub output_format: Option<String>,
    pub style_preset: Option<String>,
    pub max_pages: Option<usize>,
    pub include_toc: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StyleParamsRequest {
    pub color_scheme: Option<ColorSchemeRequest>,
    pub font_scheme: Option<FontSchemeRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorSchemeRequest {
    pub primary_color: String,
    pub secondary_color: Option<String>,
    pub background_color: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FontSchemeRequest {
    pub title_font: Option<String>,
    pub body_font: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutlineItemRequest {
    pub title: String,
    pub level: u8,
    pub children: Option<Vec<OutlineItemRequest>>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateRequest {
    #[validate(length(min = 1, max = 200, message = "标题长度1-200"))]
    pub title: String,
    pub description: Option<String>,
    pub source_type: String,
    pub topic: Option<String>,
    pub outline: Option<String>,
    pub industry: Option<String>,
    pub style_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    pub project_id: i64,
    pub task_id: String,
    pub status: String,
}

pub async fn create(VJson(_req): VJson<CreateRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(CreateResponse {
        project_id: 1,
        task_id: "task_1".to_string(),
        status: "pending".to_string(),
    })
}

pub async fn generate_from_document(VJson(_req): VJson<GenerateFromDocumentRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(serde_json::json!({"project_id": 1, "status": "processing"}))
}

pub async fn generate_from_text(VJson(_req): VJson<GenerateFromTextRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(serde_json::json!({"project_id": 1, "status": "processing"}))
}

pub async fn generate_from_outline(VJson(_req): VJson<GenerateFromOutlineRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(serde_json::json!({"project_id": 1, "status": "processing"}))
}

pub async fn generate_from_chart(VJson(_req): VJson<GenerateFromChartRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(serde_json::json!({"project_id": 1, "status": "processing"}))
}

pub async fn regenerate(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(serde_json::json!({"project_id": 1, "status": "processing"}))
}

pub async fn export_ppt(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(serde_json::json!({"download_url": "/download/ppt.pptx"}))
}

pub async fn export_pdf(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(serde_json::json!({"download_url": "/download/ppt.pdf"}))
}

pub async fn export_images(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(serde_json::json!({"download_url": "/download/images.zip"}))
}

pub async fn get_industries() -> impl axum::response::IntoResponse {
    ApiResponse::success(vec![
        "科技", "金融", "医疗", "教育", "零售", "制造", "房地产", "能源"
    ])
}

pub async fn preview_style(Path(_industry): Path<String>) -> impl axum::response::IntoResponse {
    ApiResponse::success(serde_json::json!({"colors": ["#0066FF", "#FF6B00"], "fonts": ["Microsoft YaHei", "Arial"]}))
}

#[derive(Debug, Serialize)]
pub struct ProgressResponse {
    pub task_id: String,
    pub status: String,
    pub progress: i32,
    pub message: Option<String>,
}

pub async fn get_progress(Path(task_id): Path<String>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ProgressResponse {
        task_id,
        status: "processing".to_string(),
        progress: 50,
        message: Some("正在生成中...".to_string()),
    })
}

pub async fn cancel(Path(_task_id): Path<String>) -> impl axum::response::IntoResponse {
    ApiResponse::success(())
}

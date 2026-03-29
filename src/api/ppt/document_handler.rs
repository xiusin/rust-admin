use crate::api::web_path::{WebPath, WebPathType};
use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use axum::routing::{get, post};
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn document_routes() -> WebPath {
    WebPath::new()
        .route("/parse", WebPathType::Post, Some("解析文档"), post(parse_document))
        .route("/extract-text/:id", WebPathType::Get, Some("提取文本"), get(extract_text))
        .route("/extract-images/:id", WebPathType::Get, Some("提取图片"), get(extract_images))
        .route("/extract-structure/:id", WebPathType::Get, Some("提取结构"), get(extract_structure))
        .route("/convert/:id", WebPathType::Post, Some("转换格式"), post(convert_format))
        .route("/preview/:id", WebPathType::Get, Some("预览文档"), get(preview_document))
}

#[derive(Debug, Deserialize, Validate)]
pub struct ParseDocumentRequest {
    pub file_path: String,
    pub file_type: Option<String>,
    pub options: Option<ParseOptions>,
}

#[derive(Debug, Deserialize)]
pub struct ParseOptions {
    pub extract_images: Option<bool>,
    pub extract_tables: Option<bool>,
    pub ocr_enabled: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ParseDocumentResponse {
    pub document_id: i64,
    pub title: Option<String>,
    pub content: String,
    pub pages: i32,
    pub metadata: serde_json::Value,
}

pub async fn parse_document(VJson(_req): VJson<ParseDocumentRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ParseDocumentResponse {
        document_id: 1,
        title: Some("文档标题".to_string()),
        content: "文档内容".to_string(),
        pages: 10,
        metadata: serde_json::json!({}),
    })
}

#[derive(Debug, Serialize)]
pub struct ExtractTextResponse {
    pub text: String,
    pub word_count: i32,
}

pub async fn extract_text(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ExtractTextResponse {
        text: "提取的文本内容".to_string(),
        word_count: 100,
    })
}

#[derive(Debug, Serialize)]
pub struct ExtractImagesResponse {
    pub images: Vec<ImageInfo>,
}

#[derive(Debug, Serialize)]
pub struct ImageInfo {
    pub id: i64,
    pub url: String,
    pub width: i32,
    pub height: i32,
}

pub async fn extract_images(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ExtractImagesResponse {
        images: vec![],
    })
}

#[derive(Debug, Serialize)]
pub struct ExtractStructureResponse {
    pub sections: Vec<SectionInfo>,
}

#[derive(Debug, Serialize)]
pub struct SectionInfo {
    pub level: i32,
    pub title: String,
    pub content: String,
}

pub async fn extract_structure(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ExtractStructureResponse {
        sections: vec![],
    })
}

#[derive(Debug, Deserialize, Validate)]
pub struct ConvertFormatRequest {
    pub target_format: String,
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ConvertFormatResponse {
    pub output_path: String,
    pub file_size: i64,
}

pub async fn convert_format(Path(_id): Path<i64>, VJson(_req): VJson<ConvertFormatRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ConvertFormatResponse {
        output_path: "/output/converted.pdf".to_string(),
        file_size: 1024,
    })
}

#[derive(Debug, Serialize)]
pub struct PreviewDocumentResponse {
    pub preview_url: String,
    pub pages: i32,
}

pub async fn preview_document(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(PreviewDocumentResponse {
        preview_url: "/preview/1".to_string(),
        pages: 10,
    })
}

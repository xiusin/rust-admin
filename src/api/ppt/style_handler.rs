use crate::api::web_path::{WebPath, WebPathType};
use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use axum::routing::{get, post};
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn style_routes() -> WebPath {
    WebPath::new()
        .route("/generate", WebPathType::Post, Some("生成风格"), post(generate_style))
        .route("/presets", WebPathType::Get, Some("预设风格列表"), get(list_presets))
        .route("/presets/:id", WebPathType::Get, Some("预设风格详情"), get(get_preset))
        .route("/custom", WebPathType::Post, Some("创建自定义风格"), post(create_custom_style))
        .route("/custom/:id", WebPathType::Get, Some("获取自定义风格"), get(get_custom_style))
        .route("/custom/:id", WebPathType::Put, Some("更新自定义风格"), post(update_custom_style))
        .route("/analyze", WebPathType::Post, Some("分析风格"), post(analyze_style))
        .route("/match", WebPathType::Post, Some("匹配风格"), post(match_style))
}

#[derive(Debug, Deserialize, Validate)]
pub struct GenerateStyleRequest {
    pub industry: Option<String>,
    pub mood: Option<String>,
    pub color_preferences: Option<Vec<String>>,
    pub reference_image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StyleResponse {
    pub style_id: i64,
    pub color_scheme: ColorScheme,
    pub font_scheme: FontScheme,
    pub layout_rules: LayoutRules,
}

#[derive(Debug, Serialize)]
pub struct ColorScheme {
    pub primary_color: String,
    pub secondary_color: String,
    pub background_color: String,
    pub text_color: String,
}

#[derive(Debug, Serialize)]
pub struct FontScheme {
    pub title_font: String,
    pub body_font: String,
    pub title_size: i32,
    pub body_size: i32,
}

#[derive(Debug, Serialize)]
pub struct LayoutRules {
    pub margin: i32,
    pub spacing: i32,
    pub alignment: String,
}

pub async fn generate_style(VJson(_req): VJson<GenerateStyleRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(StyleResponse {
        style_id: 1,
        color_scheme: ColorScheme {
            primary_color: "#0066FF".to_string(),
            secondary_color: "#FF6B00".to_string(),
            background_color: "#FFFFFF".to_string(),
            text_color: "#333333".to_string(),
        },
        font_scheme: FontScheme {
            title_font: "Microsoft YaHei".to_string(),
            body_font: "Microsoft YaHei".to_string(),
            title_size: 36,
            body_size: 18,
        },
        layout_rules: LayoutRules {
            margin: 40,
            spacing: 20,
            alignment: "left".to_string(),
        },
    })
}

#[derive(Debug, Serialize)]
pub struct PresetStyle {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub thumbnail: Option<String>,
}

pub async fn list_presets() -> impl axum::response::IntoResponse {
    ApiResponse::success(Vec::<PresetStyle>::new())
}

pub async fn get_preset(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(PresetStyle {
        id: 1,
        name: "商务风格".to_string(),
        description: "专业商务演示风格".to_string(),
        thumbnail: None,
    })
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCustomStyleRequest {
    pub name: String,
    pub color_scheme: ColorSchemeRequest,
    pub font_scheme: FontSchemeRequest,
}

#[derive(Debug, Deserialize)]
pub struct ColorSchemeRequest {
    pub primary_color: String,
    pub secondary_color: String,
    pub background_color: String,
    pub text_color: String,
}

#[derive(Debug, Deserialize)]
pub struct FontSchemeRequest {
    pub title_font: String,
    pub body_font: String,
}

pub async fn create_custom_style(VJson(_req): VJson<CreateCustomStyleRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(1i64)
}

pub async fn get_custom_style(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(serde_json::json!({"id": 1, "name": "自定义风格"}))
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCustomStyleRequest {
    pub id: i64,
    pub name: Option<String>,
    pub color_scheme: Option<ColorSchemeRequest>,
    pub font_scheme: Option<FontSchemeRequest>,
}

pub async fn update_custom_style(VJson(_req): VJson<UpdateCustomStyleRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct AnalyzeStyleRequest {
    pub image_url: Option<String>,
    pub ppt_id: Option<i64>,
}

pub async fn analyze_style(VJson(_req): VJson<AnalyzeStyleRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(serde_json::json!({"colors": ["#0066FF"], "fonts": ["Microsoft YaHei"]}))
}

#[derive(Debug, Deserialize, Validate)]
pub struct MatchStyleRequest {
    pub content: String,
    pub industry: Option<String>,
}

pub async fn match_style(VJson(_req): VJson<MatchStyleRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(serde_json::json!({"style_id": 1, "confidence": 0.85}))
}

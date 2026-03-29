use crate::api::web_path::{WebPath, WebPathType};
use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use axum::routing::{get, post};
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn prompt_routes() -> WebPath {
    WebPath::new()
        .route("/reverse", WebPathType::Post, Some("反推提示词"), post(reverse_prompt))
        .route("/apply", WebPathType::Post, Some("应用提示词"), post(apply_prompt))
        .route("/save", WebPathType::Post, Some("保存提示词"), post(save_prompt))
        .route("/list", WebPathType::Get, Some("提示词列表"), get(list_prompts))
        .route("/optimize", WebPathType::Post, Some("优化提示词"), post(optimize_prompt))
        .route("/test", WebPathType::Post, Some("测试提示词"), post(test_prompt))
        .route("/validate", WebPathType::Post, Some("验证提示词"), post(validate_prompt))
        .route("/suggest", WebPathType::Post, Some("改进建议"), post(suggest_improvements))
        .route("/templates", WebPathType::Get, Some("模板列表"), get(list_templates))
        .route("/templates/:id", WebPathType::Get, Some("模板详情"), get(get_template))
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReversePromptRequest {
    pub ppt_file: Option<String>,
    pub ppt_id: Option<i64>,
    pub mode: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReversePromptResponse {
    pub prompt: String,
    pub confidence: f32,
    pub suggestions: Vec<String>,
}

pub async fn reverse_prompt(VJson(_req): VJson<ReversePromptRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ReversePromptResponse {
        prompt: "生成的提示词".to_string(),
        confidence: 0.85,
        suggestions: vec!["建议1".to_string(), "建议2".to_string()],
    })
}

#[derive(Debug, Deserialize, Validate)]
pub struct ApplyPromptRequest {
    #[validate(length(min = 10, message = "提示词长度至少10个字符"))]
    pub prompt: String,
    pub content: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApplyPromptResponse {
    pub message: String,
    pub generated_pages: i32,
}

pub async fn apply_prompt(VJson(_req): VJson<ApplyPromptRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ApplyPromptResponse {
        message: "应用成功".to_string(),
        generated_pages: 10,
    })
}

#[derive(Debug, Deserialize, Validate)]
pub struct SavePromptRequest {
    #[validate(length(min = 1, message = "名称不能为空"))]
    pub name: String,
    #[validate(length(min = 10, message = "提示词长度至少10个字符"))]
    pub prompt: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct SavePromptResponse {
    pub id: i64,
    pub message: String,
}

pub async fn save_prompt(VJson(_req): VJson<SavePromptRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(SavePromptResponse {
        id: 1,
        message: "保存成功".to_string(),
    })
}

#[derive(Debug, Serialize)]
pub struct PromptListItem {
    pub id: i64,
    pub name: String,
    pub prompt_preview: String,
    pub tags: Vec<String>,
    pub created_at: String,
}

pub async fn list_prompts() -> impl axum::response::IntoResponse {
    ApiResponse::success(Vec::<PromptListItem>::new())
}

#[derive(Debug, Deserialize, Validate)]
pub struct OptimizePromptRequest {
    pub prompt: String,
    pub goals: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct OptimizePromptResponse {
    pub optimized_prompt: String,
    pub improvements: Vec<String>,
}

pub async fn optimize_prompt(VJson(_req): VJson<OptimizePromptRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(OptimizePromptResponse {
        optimized_prompt: "优化后的提示词".to_string(),
        improvements: vec!["改进1".to_string()],
    })
}

#[derive(Debug, Deserialize, Validate)]
pub struct TestPromptRequest {
    pub prompt: String,
    pub test_cases: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct TestPromptResponse {
    pub success: bool,
    pub score: f32,
    pub feedback: String,
}

pub async fn test_prompt(VJson(_req): VJson<TestPromptRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(TestPromptResponse {
        success: true,
        score: 0.9,
        feedback: "测试通过".to_string(),
    })
}

#[derive(Debug, Deserialize, Validate)]
pub struct ValidatePromptRequest {
    pub prompt: String,
}

#[derive(Debug, Serialize)]
pub struct ValidatePromptResponse {
    pub valid: bool,
    pub issues: Vec<String>,
}

pub async fn validate_prompt(VJson(_req): VJson<ValidatePromptRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ValidatePromptResponse {
        valid: true,
        issues: vec![],
    })
}

#[derive(Debug, Deserialize, Validate)]
pub struct SuggestRequest {
    pub prompt: String,
}

#[derive(Debug, Serialize)]
pub struct SuggestResponse {
    pub suggestions: Vec<String>,
}

pub async fn suggest_improvements(VJson(_req): VJson<SuggestRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(SuggestResponse {
        suggestions: vec!["建议1".to_string(), "建议2".to_string()],
    })
}

#[derive(Debug, Serialize)]
pub struct PromptTemplate {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub prompt: String,
}

pub async fn list_templates() -> impl axum::response::IntoResponse {
    ApiResponse::success(Vec::<PromptTemplate>::new())
}

pub async fn get_template(Path(_id): Path<i64>) -> impl axum::response::IntoResponse {
    ApiResponse::success(PromptTemplate {
        id: 1,
        name: "默认模板".to_string(),
        description: "默认提示词模板".to_string(),
        prompt: "模板内容".to_string(),
    })
}

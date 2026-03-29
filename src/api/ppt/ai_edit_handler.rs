use crate::api::web_path::{WebPath, WebPathType};
use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use axum::routing::post;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn ai_edit_routes() -> WebPath {
    WebPath::new()
        .route("/polish", WebPathType::Post, Some("润色"), post(polish))
        .route("/continue", WebPathType::Post, Some("续写"), post(continue_writing))
        .route("/summarize", WebPathType::Post, Some("总结"), post(summarize))
        .route("/translate", WebPathType::Post, Some("翻译"), post(translate))
        .route("/expand", WebPathType::Post, Some("扩展"), post(expand))
        .route("/simplify", WebPathType::Post, Some("简化"), post(simplify))
        .route("/rewrite", WebPathType::Post, Some("重写"), post(rewrite))
        .route("/suggest", WebPathType::Post, Some("建议"), post(suggest))
}

#[derive(Debug, Deserialize, Validate)]
pub struct ContentRequest {
    #[validate(length(min = 1, message = "内容不能为空"))]
    pub content: String,
    pub style: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ContentResponse {
    pub result: String,
    pub changes: Option<Vec<String>>,
}

pub async fn polish(VJson(_req): VJson<ContentRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ContentResponse {
        result: "润色后的内容".to_string(),
        changes: Some(vec!["语法修正".to_string(), "表达优化".to_string()]),
    })
}

pub async fn continue_writing(VJson(_req): VJson<ContentRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ContentResponse {
        result: "续写的内容".to_string(),
        changes: None,
    })
}

pub async fn summarize(VJson(_req): VJson<ContentRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ContentResponse {
        result: "总结内容".to_string(),
        changes: None,
    })
}

#[derive(Debug, Deserialize, Validate)]
pub struct TranslateRequest {
    pub content: String,
    pub target_language: String,
}

pub async fn translate(VJson(_req): VJson<TranslateRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ContentResponse {
        result: "翻译后的内容".to_string(),
        changes: None,
    })
}

pub async fn expand(VJson(_req): VJson<ContentRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ContentResponse {
        result: "扩展后的内容".to_string(),
        changes: None,
    })
}

pub async fn simplify(VJson(_req): VJson<ContentRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ContentResponse {
        result: "简化后的内容".to_string(),
        changes: None,
    })
}

#[derive(Debug, Deserialize, Validate)]
pub struct RewriteRequest {
    pub content: String,
    pub style: String,
}

pub async fn rewrite(VJson(_req): VJson<RewriteRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ContentResponse {
        result: "重写后的内容".to_string(),
        changes: None,
    })
}

pub async fn suggest(VJson(_req): VJson<ContentRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success(vec![
        "建议1: 添加更多细节".to_string(),
        "建议2: 使用更简洁的表达".to_string(),
    ])
}

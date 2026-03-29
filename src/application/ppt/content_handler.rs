use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use crate::domain::entity::page_element;
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct ContentDetail {
    pub id: i64,
    pub page_id: i64,
    pub element_type: String,
    pub content: Option<serde_json::Value>,
    pub style: Option<serde_json::Value>,
    pub position: Option<serde_json::Value>,
    pub size: Option<serde_json::Value>,
}

pub async fn get_content_detail(Path(id): Path<i64>) -> axum::response::Response {
    let db = DB().await;
    
    if let Ok(Some(e)) = page_element::Entity::find_by_id(id).one(db).await {
        let detail = ContentDetail {
            id: e.id,
            page_id: e.page_id,
            element_type: e.element_type,
            content: e.content,
            style: e.style,
            position: e.position,
            size: e.size,
        };
        return ApiResponse::success(detail).into_response();
    }
    
    ApiResponse::not_found("内容不存在")
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct EditContentRequest {
    pub id: i64,
    pub content: Option<serde_json::Value>,
    pub style: Option<serde_json::Value>,
    pub position: Option<serde_json::Value>,
    pub size: Option<serde_json::Value>,
}

pub async fn edit_content(VJson(req): VJson<EditContentRequest>) -> axum::response::Response {
    // Validate the request
    if let Err(e) = req.validate() {
        return ApiResponse::bad_request(format!("验证失败：{}", e));
    }
    
    let db = DB().await;
    
    let element = match page_element::Entity::find_by_id(req.id).one(db).await {
        Ok(Some(e)) => e,
        Ok(None) => return ApiResponse::not_found("内容不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let now = Local::now().naive_local();
    let mut element: page_element::ActiveModel = element.into();
    
    if let Some(content) = req.content {
        element.content = Set(Some(content));
    }
    if let Some(style) = req.style {
        element.style = Set(Some(style));
    }
    if let Some(position) = req.position {
        element.position = Set(Some(position));
    }
    if let Some(size) = req.size {
        element.size = Set(Some(size));
    }
    element.updated_at = Set(Some(now));
    
    if let Err(e) = element.update(db).await {
        return ApiResponse::bad_request(e.to_string());
    }
    
    ApiResponse::success(()).into_response()
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct BatchEditRequest {
    pub ids: Vec<i64>,
    pub style: Option<serde_json::Value>,
}

pub async fn batch_edit_content(VJson(req): VJson<BatchEditRequest>) -> impl axum::response::IntoResponse {
    let db = DB().await;
    let mut updated = 0;
    
    for id in req.ids {
        if let Some(element) = page_element::Entity::find_by_id(id)
            .one(db)
            .await
            .ok()
            .flatten()
        {
            let mut element: page_element::ActiveModel = element.into();
            
            if let Some(ref style) = req.style {
                element.style = Set(Some(style.clone()));
            }
            
            if element.update(db).await.is_ok() {
                updated += 1;
            }
        }
    }
    
    ApiResponse::success(updated)
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct AIEnhanceRequest {
    pub enhance_type: String,
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct AIEnhanceResponse {
    pub original: serde_json::Value,
    pub enhanced: serde_json::Value,
    pub suggestions: Vec<String>,
}

pub async fn ai_enhance_content(Path(_id): Path<i64>, VJson(_req): VJson<AIEnhanceRequest>) -> impl axum::response::IntoResponse {
    let response = AIEnhanceResponse {
        original: serde_json::Value::Null,
        enhanced: serde_json::json!({"text": "增强后的内容"}),
        suggestions: vec![
            "建议使用更简洁的表达".to_string(),
            "可以添加数据支撑".to_string(),
        ],
    };
    
    ApiResponse::success(response)
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct ReplaceContentRequest {
    pub search: String,
    pub replace: String,
    pub page_ids: Option<Vec<i64>>,
}

#[derive(Debug, Serialize)]
pub struct ReplaceResult {
    pub replaced_count: usize,
    pub affected_pages: Vec<i64>,
}

pub async fn replace_content(VJson(_req): VJson<ReplaceContentRequest>) -> impl axum::response::IntoResponse {
    let response = ReplaceResult {
        replaced_count: 0,
        affected_pages: vec![],
    };
    
    ApiResponse::success(response)
}

use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use crate::domain::entity::{ppt_page, page_element};
use axum::extract::{Path, Query};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct GetSlidesQuery {
    pub project_id: i64,
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct AddSlideRequest {
    pub project_id: i64,
    pub title: Option<String>,
    pub content: Option<serde_json::Value>,
    pub style: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct EditSlideRequest {
    pub id: i64,
    pub title: Option<String>,
    pub content: Option<serde_json::Value>,
    pub style: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct SortSlidesRequest {
    pub project_id: i64,
    pub slide_ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, validator::Validate)]
pub struct ExportRequest {
    pub project_id: i64,
    pub format: String,
}

#[derive(Debug, Serialize)]
pub struct SlideResponse {
    pub id: i64,
    pub project_id: i64,
    pub slide_index: i32,
    pub title: Option<String>,
    pub content: Option<serde_json::Value>,
    pub style: Option<serde_json::Value>,
    pub animation: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct ExportResponse {
    pub download_url: String,
}

pub async fn get_slides(Query(params): Query<GetSlidesQuery>) -> axum::response::Response {
    let db = DB().await;
    
    let pages = match ppt_page::Entity::find()
        .filter(ppt_page::Column::ProjectId.eq(params.project_id))
        .order_by_asc(ppt_page::Column::PageOrder)
        .all(db)
        .await
    {
        Ok(p) => p,
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let mut slides = Vec::new();
    for page in pages {
        slides.push(SlideResponse {
            id: page.id,
            project_id: page.project_id,
            slide_index: page.page_order as i32,
            title: page.title,
            content: page.layout_config,
            style: page.style_config,
            animation: None,
            created_at: page.created_at.map(|t| t.to_string()).unwrap_or_default(),
            updated_at: page.updated_at.map(|t| t.to_string()).unwrap_or_default(),
        });
    }
    
    ApiResponse::success(slides).into_response()
}

pub async fn add_slide(VJson(req): VJson<AddSlideRequest>) -> axum::response::Response {
    let db = DB().await;
    
    let max_order: i32 = ppt_page::Entity::find()
        .filter(ppt_page::Column::ProjectId.eq(req.project_id))
        .order_by_desc(ppt_page::Column::PageOrder)
        .one(db)
        .await
        .ok()
        .flatten()
        .map(|p| p.page_order)
        .unwrap_or(0) as i32;
    
    let id = GID().await;
    let now = Local::now().naive_local();
    
    let slide = ppt_page::ActiveModel {
        id: Set(id),
        project_id: Set(req.project_id),
        page_order: Set(max_order + 1),
        page_type: Set("content".to_string()),
        title: Set(req.title),
        layout_config: Set(req.content),
        style_config: Set(req.style),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };
    
    match slide.insert(db).await {
        Ok(_) => ApiResponse::success(id).into_response(),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn edit_slide(VJson(req): VJson<EditSlideRequest>) -> axum::response::Response {
    let db = DB().await;
    
    let slide = match ppt_page::Entity::find_by_id(req.id)
        .one(db)
        .await
    {
        Ok(Some(s)) => s,
        Ok(None) => return ApiResponse::bad_request("幻灯片不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let now = Local::now().naive_local();
    let mut slide: ppt_page::ActiveModel = slide.into();
    
    if let Some(title) = req.title {
        slide.title = Set(Some(title));
    }
    if let Some(content) = req.content {
        slide.layout_config = Set(Some(content));
    }
    if let Some(style) = req.style {
        slide.style_config = Set(Some(style));
    }
    slide.updated_at = Set(Some(now));
    
    match slide.update(db).await {
        Ok(_) => ApiResponse::success(()).into_response(),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn delete_slide(Path(id): Path<i64>) -> axum::response::Response {
    let db = DB().await;
    
    let _ = page_element::Entity::delete_many()
        .filter(page_element::Column::PageId.eq(id))
        .exec(db)
        .await;
    
    match ppt_page::Entity::delete_by_id(id)
        .exec(db)
        .await
    {
        Ok(_) => ApiResponse::success(()).into_response(),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn sort_slides(VJson(req): VJson<SortSlidesRequest>) -> impl axum::response::IntoResponse {
    let db = DB().await;
    
    for (index, slide_id) in req.slide_ids.iter().enumerate() {
        if let Some(slide) = ppt_page::Entity::find_by_id(*slide_id)
            .one(db)
            .await
            .ok()
            .flatten()
        {
            let mut slide: ppt_page::ActiveModel = slide.into();
            slide.page_order = Set(index as i32 + 1);
            let _ = slide.update(db).await;
        }
    }
    
    ApiResponse::success(())
}

pub async fn export_ppt(VJson(req): VJson<ExportRequest>) -> impl axum::response::IntoResponse {
    let download_url = format!("/api/ppt/download/{}.{}", req.project_id, req.format);
    ApiResponse::success(ExportResponse { download_url })
}

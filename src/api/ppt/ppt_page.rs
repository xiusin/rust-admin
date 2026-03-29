use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use crate::domain::entity::ppt_page;
use axum::extract::{Path, Query};
use serde::{Deserialize, Serialize};

pub fn ppt_page() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取PPT页面列表"), get(list_pages))
        .route("/detail/:id", WebPathType::Get, Some("获取PPT页面详情"), get(get_page_detail))
        .route("/add", WebPathType::Post, Some("添加PPT页面"), post(add_page))
        .route("/edit", WebPathType::Put, Some("编辑PPT页面"), put(edit_page))
        .route("/del", WebPathType::Delete, Some("删除 PPT 页面"), delete(delete_pages_handler))
        .route("/reorder", WebPathType::Put, Some("重新排序页面"), put(reorder_pages_handler))
}

#[derive(Debug, Deserialize)]
pub struct ListPagesQuery {
    pub project_id: i64,
}

#[derive(Debug, Serialize)]
pub struct PageListItem {
    pub id: i64,
    pub project_id: i64,
    pub page_order: i32,
    pub page_type: String,
    pub title: Option<String>,
}

pub async fn list_pages(Query(query): Query<ListPagesQuery>) -> impl axum::response::IntoResponse {
    let db = DB().await;
    let pages = ppt_page::Entity::find()
        .filter(ppt_page::Column::ProjectId.eq(query.project_id))
        .order_by_asc(ppt_page::Column::PageOrder)
        .all(db)
        .await
        .unwrap_or_default();
    
    let items: Vec<PageListItem> = pages.iter().map(|p| PageListItem {
        id: p.id,
        project_id: p.project_id,
        page_order: p.page_order,
        page_type: p.page_type.clone(),
        title: p.title.clone(),
    }).collect();
    
    ApiResponse::success(items)
}

pub async fn get_page_detail(Path(id): Path<i64>) -> axum::response::Response {
    let db = DB().await;
    match ppt_page::Entity::find_by_id(id).one(db).await {
        Ok(Some(page)) => ApiResponse::success(page).into_response(),
        Ok(None) => ApiResponse::not_found("页面不存在"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct AddPageRequest {
    pub project_id: i64,
    pub page_type: Option<String>,
    pub title: Option<String>,
    pub layout_config: Option<serde_json::Value>,
    pub style_config: Option<serde_json::Value>,
}

pub async fn add_page(VJson(req): VJson<AddPageRequest>) -> axum::response::Response {
    let db = DB().await;
    let id = GID().await;
    let now = Local::now().naive_local();
    
    let page = ppt_page::ActiveModel {
        id: Set(id),
        project_id: Set(req.project_id),
        page_order: Set(0),
        page_type: Set(req.page_type.unwrap_or_else(|| "content".to_string())),
        title: Set(req.title),
        layout_config: Set(req.layout_config),
        style_config: Set(req.style_config),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };
    
    match page.insert(db).await {
        Ok(_) => ApiResponse::success(id).into_response(),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct EditPageRequest {
    pub id: i64,
    pub title: Option<String>,
    pub layout_config: Option<serde_json::Value>,
    pub style_config: Option<serde_json::Value>,
}

pub async fn edit_page(VJson(req): VJson<EditPageRequest>) -> axum::response::Response {
    let db = DB().await;
    match ppt_page::Entity::find_by_id(req.id).one(db).await {
        Ok(Some(page)) => {
            let now = Local::now().naive_local();
            let mut page: ppt_page::ActiveModel = page.into();
            if let Some(title) = req.title { page.title = Set(Some(title)); }
            if let Some(layout) = req.layout_config { page.layout_config = Set(Some(layout)); }
            if let Some(style) = req.style_config { page.style_config = Set(Some(style)); }
            page.updated_at = Set(Some(now));
            match page.update(db).await {
                Ok(_) => ApiResponse::success(()).into_response(),
                Err(e) => ApiResponse::bad_request(e.to_string()),
            }
        }
        Ok(None) => ApiResponse::not_found("页面不存在"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct DeletePagesRequest {
    pub ids: Vec<i64>,
}

#[axum::debug_handler]
pub async fn delete_pages_handler(VJson(req): VJson<DeletePagesRequest>) -> impl axum::response::IntoResponse {
    let db = DB().await;
    for id in req.ids {
        let _ = ppt_page::Entity::delete_by_id(id).exec(db).await;
    }
    ApiResponse::success(())
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct ReorderPagesRequest {
    pub project_id: i64,
    pub page_ids: Vec<i64>,
}

#[axum::debug_handler]
pub async fn reorder_pages_handler(VJson(req): VJson<ReorderPagesRequest>) -> impl axum::response::IntoResponse {
    let db = DB().await;
    for (index, id) in req.page_ids.iter().enumerate() {
        if let Some(page) = ppt_page::Entity::find_by_id(*id).one(db).await.ok().flatten() {
            let mut page: ppt_page::ActiveModel = page.into();
            page.page_order = Set(index as i32 + 1);
            let _ = page.update(db).await;
        }
    }
    ApiResponse::success(())
}

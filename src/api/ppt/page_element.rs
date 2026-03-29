use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use crate::domain::entity::page_element;
use axum::extract::{Path, Query};
use serde::{Deserialize, Serialize};

pub fn page_element() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取页面元素列表"), get(list_elements))
        .route("/detail/:id", WebPathType::Get, Some("获取页面元素详情"), get(get_element_detail))
        .route("/add", WebPathType::Post, Some("添加页面元素"), post(add_element))
        .route("/edit", WebPathType::Put, Some("编辑页面元素"), put(edit_element))
        .route("/del", WebPathType::Delete, Some("删除页面元素"), delete(delete_elements_handler))
        .route("/batch_add", WebPathType::Post, Some("批量添加元素"), post(batch_add_elements_handler))
}

#[derive(Debug, Deserialize)]
pub struct ListElementsQuery {
    pub page_id: i64,
}

#[derive(Debug, Serialize)]
pub struct ElementListItem {
    pub id: i64,
    pub page_id: i64,
    pub element_type: String,
    pub z_index: i32,
}

pub async fn list_elements(Query(query): Query<ListElementsQuery>) -> impl axum::response::IntoResponse {
    let db = DB().await;
    let elements = page_element::Entity::find()
        .filter(page_element::Column::PageId.eq(query.page_id))
        .order_by_asc(page_element::Column::ZIndex)
        .all(db)
        .await
        .unwrap_or_default();
    
    let items: Vec<ElementListItem> = elements.iter().map(|e| ElementListItem {
        id: e.id,
        page_id: e.page_id,
        element_type: e.element_type.clone(),
        z_index: e.z_index,
    }).collect();
    
    ApiResponse::success(items)
}

pub async fn get_element_detail(Path(id): Path<i64>) -> axum::response::Response {
    let db = DB().await;
    match page_element::Entity::find_by_id(id).one(db).await {
        Ok(Some(element)) => ApiResponse::success(element).into_response(),
        Ok(None) => ApiResponse::not_found("元素不存在"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct AddElementRequest {
    pub page_id: i64,
    pub element_type: String,
    pub content: Option<serde_json::Value>,
    pub style: Option<serde_json::Value>,
    pub position: Option<serde_json::Value>,
    pub size: Option<serde_json::Value>,
    pub z_index: Option<i32>,
}

pub async fn add_element(VJson(req): VJson<AddElementRequest>) -> axum::response::Response {
    let db = DB().await;
    let id = GID().await;
    let now = Local::now().naive_local();
    
    let element = page_element::ActiveModel {
        id: Set(id),
        page_id: Set(req.page_id),
        element_type: Set(req.element_type),
        content: Set(req.content.map(|c| sea_orm::JsonValue::from(c))),
        style: Set(req.style.map(|s| sea_orm::JsonValue::from(s))),
        position: Set(req.position.map(|p| sea_orm::JsonValue::from(p))),
        size: Set(req.size.map(|s| sea_orm::JsonValue::from(s))),
        rotation: Set(None),
        z_index: Set(req.z_index.unwrap_or(0)),
        locked: Set(0),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };
    
    match element.insert(db).await {
        Ok(_) => ApiResponse::success(id).into_response(),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct EditElementRequest {
    pub id: i64,
    pub content: Option<serde_json::Value>,
    pub style: Option<serde_json::Value>,
    pub position: Option<serde_json::Value>,
    pub size: Option<serde_json::Value>,
    pub z_index: Option<i32>,
}

pub async fn edit_element(VJson(req): VJson<EditElementRequest>) -> axum::response::Response {
    let db = DB().await;
    match page_element::Entity::find_by_id(req.id).one(db).await {
        Ok(Some(element)) => {
            let now = Local::now().naive_local();
            let mut element: page_element::ActiveModel = element.into();
            if let Some(content) = req.content { element.content = Set(Some(content)); }
            if let Some(style) = req.style { element.style = Set(Some(style)); }
            if let Some(position) = req.position { element.position = Set(Some(position)); }
            if let Some(size) = req.size { element.size = Set(Some(size)); }
            if let Some(z_index) = req.z_index { element.z_index = Set(z_index); }
            element.updated_at = Set(Some(now));
            match element.update(db).await {
                Ok(_) => ApiResponse::success(()).into_response(),
                Err(e) => ApiResponse::bad_request(e.to_string()),
            }
        }
        Ok(None) => ApiResponse::not_found("元素不存在"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct DeleteElementsRequest {
    pub ids: Vec<i64>,
}

#[axum::debug_handler]
pub async fn delete_elements_handler(VJson(req): VJson<DeleteElementsRequest>) -> impl axum::response::IntoResponse {
    let db = DB().await;
    for id in req.ids {
        let _ = page_element::Entity::delete_by_id(id).exec(db).await;
    }
    ApiResponse::success(())
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct BatchAddElementsRequest {
    pub page_id: i64,
    pub elements: Vec<AddElementRequest>,
}

#[axum::debug_handler]
pub async fn batch_add_elements_handler(VJson(_req): VJson<BatchAddElementsRequest>) -> impl axum::response::IntoResponse {
    ApiResponse::success("批量添加成功")
}

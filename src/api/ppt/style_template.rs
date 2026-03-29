use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use crate::domain::entity::style_template;
use axum::extract::{Path, Query};
use serde::{Deserialize, Serialize};

pub fn style_template() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取样式模板列表"), get(list_templates))
        .route("/detail/:id", WebPathType::Get, Some("获取样式模板详情"), get(get_template_detail))
        .route("/add", WebPathType::Post, Some("添加样式模板"), post(add_template))
        .route("/edit", WebPathType::Put, Some("编辑样式模板"), put(edit_template))
        .route("/del", WebPathType::Delete, Some("删除样式模板"), delete(delete_templates_handler))
        .route("/by_industry", WebPathType::Get, Some("按行业获取模板"), get(get_by_industry))
}

#[derive(Debug, Deserialize)]
pub struct ListTemplatesQuery {
    pub industry: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TemplateListItem {
    pub id: i64,
    pub name: String,
    pub industry: Option<String>,
}

pub async fn list_templates(Query(query): Query<ListTemplatesQuery>) -> impl axum::response::IntoResponse {
    let db = DB().await;
    let mut q = style_template::Entity::find();
    if let Some(industry) = query.industry {
        q = q.filter(style_template::Column::Industry.eq(industry));
    }
    let templates = q.all(db).await.unwrap_or_default();
    
    let items: Vec<TemplateListItem> = templates.iter().map(|t| TemplateListItem {
        id: t.id,
        name: t.name.clone(),
        industry: t.industry.clone(),
    }).collect();
    
    ApiResponse::success(items)
}

pub async fn get_template_detail(Path(id): Path<i64>) -> axum::response::Response {
    let db = DB().await;
    match style_template::Entity::find_by_id(id).one(db).await {
        Ok(Some(template)) => ApiResponse::success(template).into_response(),
        Ok(None) => ApiResponse::not_found("模板不存在"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct AddTemplateRequest {
    pub name: String,
    pub industry: Option<String>,
    pub color_scheme: Option<serde_json::Value>,
    pub font_scheme: Option<serde_json::Value>,
    pub layout_rules: Option<serde_json::Value>,
    pub component_styles: Option<serde_json::Value>,
}

pub async fn add_template(VJson(req): VJson<AddTemplateRequest>) -> axum::response::Response {
    let db = DB().await;
    let id = GID().await;
    let now = Local::now().naive_local();
    
    let template = style_template::ActiveModel {
        id: Set(id),
        name: Set(req.name),
        industry: Set(req.industry),
        color_scheme: Set(req.color_scheme.map(|c| sea_orm::JsonValue::from(c))),
        font_scheme: Set(req.font_scheme.map(|f| sea_orm::JsonValue::from(f))),
        layout_rules: Set(req.layout_rules.map(|l| sea_orm::JsonValue::from(l))),
        component_styles: Set(req.component_styles.map(|c| sea_orm::JsonValue::from(c))),
        preview_url: Set(None),
        usage_count: Set(0),
        created_at: Set(Some(now)),
    };
    
    match template.insert(db).await {
        Ok(_) => ApiResponse::success(id).into_response(),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct EditTemplateRequest {
    pub id: i64,
    pub name: Option<String>,
    pub industry: Option<String>,
    pub color_scheme: Option<serde_json::Value>,
    pub font_scheme: Option<serde_json::Value>,
    pub layout_rules: Option<serde_json::Value>,
    pub component_styles: Option<serde_json::Value>,
}

pub async fn edit_template(VJson(req): VJson<EditTemplateRequest>) -> axum::response::Response {
    let db = DB().await;
    match style_template::Entity::find_by_id(req.id).one(db).await {
        Ok(Some(template)) => {
            let mut template: style_template::ActiveModel = template.into();
            if let Some(name) = req.name { template.name = Set(name); }
            if let Some(industry) = req.industry { template.industry = Set(Some(industry)); }
            if let Some(color) = req.color_scheme { template.color_scheme = Set(Some(sea_orm::JsonValue::from(color))); }
            if let Some(font) = req.font_scheme { template.font_scheme = Set(Some(sea_orm::JsonValue::from(font))); }
            if let Some(layout) = req.layout_rules { template.layout_rules = Set(Some(sea_orm::JsonValue::from(layout))); }
            if let Some(comp) = req.component_styles { template.component_styles = Set(Some(sea_orm::JsonValue::from(comp))); }
            match template.update(db).await {
                Ok(_) => ApiResponse::success(()).into_response(),
                Err(e) => ApiResponse::bad_request(e.to_string()),
            }
        }
        Ok(None) => ApiResponse::not_found("模板不存在"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct DeleteTemplatesRequest {
    pub ids: Vec<i64>,
}

#[axum::debug_handler]
pub async fn delete_templates_handler(VJson(req): VJson<DeleteTemplatesRequest>) -> impl axum::response::IntoResponse {
    let db = DB().await;
    for id in req.ids {
        let _ = style_template::Entity::delete_by_id(id).exec(db).await;
    }
    ApiResponse::success(())
}

#[derive(Debug, Deserialize)]
pub struct ByIndustryQuery {
    pub industry: String,
}

pub async fn get_by_industry(Query(query): Query<ByIndustryQuery>) -> impl axum::response::IntoResponse {
    let db = DB().await;
    let templates = style_template::Entity::find()
        .filter(style_template::Column::Industry.eq(query.industry))
        .all(db)
        .await
        .unwrap_or_default();
    ApiResponse::success(templates)
}

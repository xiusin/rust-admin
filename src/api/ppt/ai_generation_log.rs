use axum::routing::{delete, get};
use crate::api::web_path::{WebPath, WebPathType};
use crate::common::result::ApiResponse;
use crate::service::prelude::*;
use crate::domain::entity::ai_generation_log;
use axum::extract::{Path, Query};
use serde::{Deserialize, Serialize};

pub fn ai_generation_log() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取 AI 生成日志列表"), get(list_logs))
        .route("/detail/:id", WebPathType::Get, Some("获取 AI 生成日志详情"), get(get_log_detail))
        .route("/del", WebPathType::Delete, Some("删除 AI 生成日志"), delete(delete_logs_handler))
}

#[derive(Debug, Deserialize)]
pub struct ListLogsQuery {
    pub task_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LogListItem {
    pub id: i64,
    pub task_type: String,
    pub status: String,
    pub created_at: Option<String>,
}

pub async fn list_logs(Query(query): Query<ListLogsQuery>) -> impl axum::response::IntoResponse {
    let db = DB().await;
    let mut q = ai_generation_log::Entity::find();
    if let Some(task_type) = query.task_type {
        q = q.filter(ai_generation_log::Column::TaskType.eq(task_type));
    }
    let logs = q.all(db).await.unwrap_or_default();
    
    let items: Vec<LogListItem> = logs.iter().map(|l| LogListItem {
        id: l.id,
        task_type: l.task_type.clone(),
        status: l.status.clone(),
        created_at: l.created_at.map(|t| t.to_string()),
    }).collect();
    
    ApiResponse::success(items)
}

pub async fn get_log_detail(Path(id): Path<i64>) -> axum::response::Response {
    let db = DB().await;
    match ai_generation_log::Entity::find_by_id(id).one(db).await {
        Ok(Some(log)) => ApiResponse::success(log).into_response(),
        Ok(None) => ApiResponse::not_found("日志不存在"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct DeleteLogsRequest {
    pub ids: Vec<i64>,
}

#[axum::debug_handler]
pub async fn delete_logs_handler(VJson(req): VJson<DeleteLogsRequest>) -> impl axum::response::IntoResponse {
    let db = DB().await;
    for id in req.ids {
        let _ = ai_generation_log::Entity::delete_by_id(id).exec(db).await;
    }
    ApiResponse::success(())
}

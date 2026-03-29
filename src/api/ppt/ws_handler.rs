use crate::api::web_path::{WebPath, WebPathType};
use crate::application::ppt::ws_handler;
use axum::routing::get;

pub fn ws_routes() -> WebPath {
    WebPath::new()
        .route("/connect", WebPathType::Get, Some("WebSocket连接"), get(ws_handler::ws_handler))
        .route("/progress/:task_id", WebPathType::Get, Some("任务进度"), get(ws_handler::get_task_progress))
}

use crate::api::web_path::{WebPath, WebPathType};
use crate::application::ppt::content_handler;
use axum::routing::{get, post, put};

pub fn content_routes() -> WebPath {
    WebPath::new()
        .route("/:id", WebPathType::Get, Some("获取内容详情"), get(content_handler::get_content_detail))
        .route("/:id", WebPathType::Put, Some("编辑内容"), put(content_handler::edit_content))
        .route("/batch-edit", WebPathType::Put, Some("批量编辑"), put(content_handler::batch_edit_content))
        .route("/ai-enhance/:id", WebPathType::Post, Some("AI增强内容"), post(content_handler::ai_enhance_content))
        .route("/replace", WebPathType::Post, Some("内容替换"), post(content_handler::replace_content))
}

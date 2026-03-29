use crate::api::web_path::{WebPath, WebPathType};
use crate::application::ppt::analyze_handler;
use axum::routing::post;

pub fn analyze_routes() -> WebPath {
    WebPath::new()
        .route("/industry", WebPathType::Post, Some("行业识别"), post(analyze_handler::analyze_industry))
        .route("/keywords", WebPathType::Post, Some("关键词提取"), post(analyze_handler::extract_keywords))
        .route("/structure", WebPathType::Post, Some("内容结构分析"), post(analyze_handler::analyze_structure))
        .route("/sentiment", WebPathType::Post, Some("情感分析"), post(analyze_handler::analyze_sentiment))
}

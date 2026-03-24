use axum::routing::{get, post};
use crate::api::web_path::{WebPath, WebPathType};
use super::verification_handler as handler;

pub fn product_verification() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取核销码列表"), get(handler::list))
        .route("/verify", WebPathType::Post, Some("核销操作"), post(handler::verify))
        .route("/query", WebPathType::Get, Some("查询核销码"), get(handler::query))
        .route("/log", WebPathType::Get, Some("获取核销记录"), get(handler::log_list))
        .route("/statistics", WebPathType::Get, Some("核销统计"), get(handler::statistics))
}

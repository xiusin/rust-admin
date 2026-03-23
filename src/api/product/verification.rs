use axum::routing::{get, post};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::product::verification_service;

pub fn product_verification() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取核销码列表"), get(verification_service::list))
        .route("/verify", WebPathType::Post, Some("核销操作"), post(verification_service::verify))
        .route("/query", WebPathType::Get, Some("查询核销码"), get(verification_service::query))
        .route("/log", WebPathType::Get, Some("获取核销记录"), get(verification_service::log_list))
        .route("/statistics", WebPathType::Get, Some("核销统计"), get(verification_service::statistics))
}

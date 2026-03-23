use axum::routing::{get, post};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::product::stock_service;

pub fn product_stock() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取库存列表"), get(stock_service::list))
        .route("/log", WebPathType::Get, Some("获取库存日志"), get(stock_service::log_list))
        .route("/adjust", WebPathType::Post, Some("库存调整"), post(stock_service::adjust))
        .route("/alertList", WebPathType::Get, Some("获取库存预警列表"), get(stock_service::alert_list))
        .route("/alertConfig", WebPathType::Post, Some("配置库存预警"), post(stock_service::alert_config))
        .route("/statistics", WebPathType::Get, Some("库存统计"), get(stock_service::statistics))
}

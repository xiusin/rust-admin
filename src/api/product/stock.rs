use axum::routing::{get, post};
use crate::api::web_path::{WebPath, WebPathType};
use super::stock_handler as handler;

pub fn product_stock() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取库存列表"), get(handler::list))
        .route("/log", WebPathType::Get, Some("获取库存日志"), get(handler::log_list))
        .route("/adjust", WebPathType::Post, Some("调整库存"), post(handler::adjust))
        .route("/alertList", WebPathType::Get, Some("获取预警列表"), get(handler::alert_list))
        .route("/alertConfig", WebPathType::Post, Some("配置预警"), post(handler::alert_config))
        .route("/statistics", WebPathType::Get, Some("库存统计"), get(handler::statistics))
}

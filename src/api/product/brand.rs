use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use super::brand_handler as handler;

pub fn product_brand() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取品牌列表"), get(handler::list))
        .route("/:id", WebPathType::Get, Some("获取品牌详情"), get(handler::detail))
        .route("/add", WebPathType::Post, Some("新增品牌"), post(handler::add))
        .route("/edit", WebPathType::Put, Some("编辑品牌"), put(handler::edit))
        .route("/delete", WebPathType::Delete, Some("删除品牌"), delete(handler::delete))
        .route("/updateStatus", WebPathType::Put, Some("更新品牌状态"), put(handler::update_status))
        .route("/simpleList", WebPathType::Get, Some("获取品牌简单列表"), get(handler::simple_list))
}

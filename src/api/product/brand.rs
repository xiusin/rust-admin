use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::product::brand_service;

pub fn product_brand() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取品牌列表"), get(brand_service::list))
        .route("/:id", WebPathType::Get, Some("获取品牌详情"), get(brand_service::detail))
        .route("/add", WebPathType::Post, Some("新增品牌"), post(brand_service::add))
        .route("/edit", WebPathType::Put, Some("编辑品牌"), put(brand_service::edit))
        .route("/delete", WebPathType::Delete, Some("删除品牌"), delete(brand_service::delete))
        .route("/updateStatus", WebPathType::Put, Some("更新品牌状态"), put(brand_service::update_status))
        .route("/simpleList", WebPathType::Get, Some("获取品牌简单列表"), get(brand_service::simple_list))
}

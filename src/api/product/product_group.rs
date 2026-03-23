use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::product::product_group_service;

pub fn product_group() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取分组列表"), get(product_group_service::list))
        .route("/:id", WebPathType::Get, Some("获取分组详情"), get(product_group_service::detail))
        .route("/add", WebPathType::Post, Some("新增分组"), post(product_group_service::add))
        .route("/edit", WebPathType::Put, Some("编辑分组"), put(product_group_service::edit))
        .route("/delete", WebPathType::Delete, Some("删除分组"), delete(product_group_service::delete))
        .route("/simpleList", WebPathType::Get, Some("获取分组简单列表"), get(product_group_service::simple_list))
}

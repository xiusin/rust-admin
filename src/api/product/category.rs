use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::product::category_service;

pub fn product_category() -> WebPath {
    WebPath::new()
        .route("/tree", WebPathType::Get, Some("获取分类树"), get(category_service::tree))
        .route("/list", WebPathType::Get, Some("获取分类列表"), get(category_service::list))
        .route("/:id", WebPathType::Get, Some("获取分类详情"), get(category_service::detail))
        .route("/add", WebPathType::Post, Some("新增分类"), post(category_service::add))
        .route("/edit", WebPathType::Put, Some("编辑分类"), put(category_service::edit))
        .route("/delete", WebPathType::Delete, Some("删除分类"), delete(category_service::delete))
        .route("/updateStatus", WebPathType::Put, Some("更新分类状态"), put(category_service::update_status))
}

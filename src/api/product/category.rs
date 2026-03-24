use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use super::category_handler as handler;

pub fn product_category() -> WebPath {
    WebPath::new()
        .route("/tree", WebPathType::Get, Some("获取分类树"), get(handler::tree))
        .route("/list", WebPathType::Get, Some("获取分类列表"), get(handler::list))
        .route("/:id", WebPathType::Get, Some("获取分类详情"), get(handler::detail))
        .route("/add", WebPathType::Post, Some("新增分类"), post(handler::add))
        .route("/edit", WebPathType::Put, Some("编辑分类"), put(handler::edit))
        .route("/delete", WebPathType::Delete, Some("删除分类"), delete(handler::delete))
        .route("/updateStatus", WebPathType::Put, Some("更新分类状态"), put(handler::update_status))
}

use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use super::product_group_handler as handler;

pub fn product_group() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取分组列表"), get(handler::list))
        .route("/:id", WebPathType::Get, Some("获取分组详情"), get(handler::detail))
        .route("/add", WebPathType::Post, Some("新增分组"), post(handler::add))
        .route("/edit", WebPathType::Put, Some("编辑分组"), put(handler::edit))
        .route("/delete", WebPathType::Delete, Some("删除分组"), delete(handler::delete))
        .route("/simpleList", WebPathType::Get, Some("获取分组简单列表"), get(handler::simple_list))
}

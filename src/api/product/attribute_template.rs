use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use super::attribute_template_handler as handler;

pub fn product_attribute_template() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取属性模板列表"), get(handler::list))
        .route("/:id", WebPathType::Get, Some("获取属性模板详情"), get(handler::detail))
        .route("/add", WebPathType::Post, Some("新增属性模板"), post(handler::add))
        .route("/edit", WebPathType::Put, Some("编辑属性模板"), put(handler::edit))
        .route("/delete", WebPathType::Delete, Some("删除属性模板"), delete(handler::delete))
        .route("/byCategory/:categoryId", WebPathType::Get, Some("根据分类获取属性模板"), get(handler::by_category))
}

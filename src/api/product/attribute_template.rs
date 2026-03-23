use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::product::attribute_template_service;

pub fn product_attribute_template() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取属性模板列表"), get(attribute_template_service::list))
        .route("/:id", WebPathType::Get, Some("获取属性模板详情"), get(attribute_template_service::detail))
        .route("/add", WebPathType::Post, Some("新增属性模板"), post(attribute_template_service::add))
        .route("/edit", WebPathType::Put, Some("编辑属性模板"), put(attribute_template_service::edit))
        .route("/delete", WebPathType::Delete, Some("删除属性模板"), delete(attribute_template_service::delete))
        .route("/byCategory/:categoryId", WebPathType::Get, Some("根据分类获取属性模板"), get(attribute_template_service::by_category))
}

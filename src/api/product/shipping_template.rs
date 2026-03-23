use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::product::shipping_template_service;

pub fn product_shipping() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取运费模板列表"), get(shipping_template_service::list))
        .route("/:id", WebPathType::Get, Some("获取运费模板详情"), get(shipping_template_service::detail))
        .route("/add", WebPathType::Post, Some("新增运费模板"), post(shipping_template_service::add))
        .route("/edit", WebPathType::Put, Some("编辑运费模板"), put(shipping_template_service::edit))
        .route("/delete", WebPathType::Delete, Some("删除运费模板"), delete(shipping_template_service::delete))
        .route("/calculate", WebPathType::Post, Some("计算运费"), post(shipping_template_service::calculate))
        .route("/simpleList", WebPathType::Get, Some("获取运费模板简单列表"), get(shipping_template_service::simple_list))
}

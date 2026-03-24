use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use super::shipping_template_handler as handler;

pub fn product_shipping() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取运费模板列表"), get(handler::list))
        .route("/:id", WebPathType::Get, Some("获取运费模板详情"), get(handler::detail))
        .route("/add", WebPathType::Post, Some("新增运费模板"), post(handler::add))
        .route("/edit", WebPathType::Put, Some("编辑运费模板"), put(handler::edit))
        .route("/delete", WebPathType::Delete, Some("删除运费模板"), delete(handler::delete))
        .route("/calculate", WebPathType::Post, Some("计算运费"), post(handler::calculate))
        .route("/simpleList", WebPathType::Get, Some("获取运费模板简单列表"), get(handler::simple_list))
}

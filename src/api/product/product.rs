use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use super::product_handler as handler;

pub fn product_list() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取商品列表"), get(handler::list))
        .route("/:id", WebPathType::Get, Some("获取商品详情"), get(handler::detail))
        .route("/add", WebPathType::Post, Some("新增商品"), post(handler::add))
        .route("/edit", WebPathType::Put, Some("编辑商品"), put(handler::edit))
        .route("/delete", WebPathType::Delete, Some("删除商品"), delete(handler::delete))
        .route("/updateStatus", WebPathType::Put, Some("更新商品状态"), put(handler::update_status))
        .route("/audit", WebPathType::Put, Some("审核商品"), put(handler::audit))
        .route("/batchUpdateStatus", WebPathType::Put, Some("批量更新状态"), put(handler::batch_update_status))
        .route("/batchDelete", WebPathType::Delete, Some("批量删除"), delete(handler::batch_delete))
        .route("/statistics", WebPathType::Get, Some("商品统计"), get(handler::statistics))
        .route("/simpleList", WebPathType::Get, Some("商品简单列表"), get(handler::simple_list))
}

pub fn product_sku() -> WebPath {
    WebPath::new()
        .route("/list/:product_id", WebPathType::Get, Some("获取SKU列表"), get(handler::sku_list))
        .route("/:id", WebPathType::Get, Some("获取SKU详情"), get(handler::sku_detail))
        .route("/add", WebPathType::Post, Some("新增SKU"), post(handler::sku_add))
        .route("/edit", WebPathType::Put, Some("编辑SKU"), put(handler::sku_edit))
        .route("/delete", WebPathType::Delete, Some("删除SKU"), delete(handler::sku_delete))
        .route("/generate", WebPathType::Post, Some("生成SKU组合"), post(handler::sku_generate))
}

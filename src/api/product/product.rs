use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::product::product_service;

pub fn product() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取商品列表"), get(product_service::list))
        .route("/:id", WebPathType::Get, Some("获取商品详情"), get(product_service::detail))
        .route("/add", WebPathType::Post, Some("新增商品"), post(product_service::add))
        .route("/edit", WebPathType::Put, Some("编辑商品"), put(product_service::edit))
        .route("/delete", WebPathType::Delete, Some("删除商品"), delete(product_service::delete))
        .route("/updateStatus", WebPathType::Put, Some("更新商品状态"), put(product_service::update_status))
        .route("/audit", WebPathType::Put, Some("审核商品"), put(product_service::audit))
        .route("/batchUpdateStatus", WebPathType::Put, Some("批量更新状态"), put(product_service::batch_update_status))
        .route("/batchDelete", WebPathType::Delete, Some("批量删除"), delete(product_service::batch_delete))
        .route("/statistics", WebPathType::Get, Some("商品统计"), get(product_service::statistics))
        .route("/simpleList", WebPathType::Get, Some("商品简单列表"), get(product_service::simple_list))
}

pub fn product_sku() -> WebPath {
    WebPath::new()
        .route("/list/:product_id", WebPathType::Get, Some("获取SKU列表"), get(product_service::sku_list))
        .route("/:id", WebPathType::Get, Some("获取SKU详情"), get(product_service::sku_detail))
        .route("/add", WebPathType::Post, Some("新增SKU"), post(product_service::sku_add))
        .route("/edit", WebPathType::Put, Some("编辑SKU"), put(product_service::sku_edit))
        .route("/delete", WebPathType::Delete, Some("删除SKU"), delete(product_service::sku_delete))
        .route("/generate", WebPathType::Post, Some("生成SKU组合"), post(product_service::sku_generate))
}

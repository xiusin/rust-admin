use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::product::store_service;

pub fn product_store() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取门店列表"), get(store_service::list))
        .route("/:id", WebPathType::Get, Some("获取门店详情"), get(store_service::detail))
        .route("/add", WebPathType::Post, Some("新增门店"), post(store_service::add))
        .route("/edit", WebPathType::Put, Some("编辑门店"), put(store_service::edit))
        .route("/delete", WebPathType::Delete, Some("删除门店"), delete(store_service::delete))
        .route("/stock/:storeId", WebPathType::Get, Some("获取门店库存"), get(store_service::stock_list))
        .route("/stock/adjust", WebPathType::Post, Some("调整门店库存"), post(store_service::stock_adjust))
        .route("/simpleList", WebPathType::Get, Some("获取门店简单列表"), get(store_service::simple_list))
        .route("/statistics", WebPathType::Get, Some("门店统计"), get(store_service::statistics))
}

use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use super::store_handler as handler;

pub fn product_store() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取门店列表"), get(handler::list))
        .route("/:id", WebPathType::Get, Some("获取门店详情"), get(handler::detail))
        .route("/add", WebPathType::Post, Some("新增门店"), post(handler::add))
        .route("/edit", WebPathType::Put, Some("编辑门店"), put(handler::edit))
        .route("/delete", WebPathType::Delete, Some("删除门店"), delete(handler::delete))
        .route("/stock/:storeId", WebPathType::Get, Some("获取门店库存"), get(handler::stock_list))
        .route("/stock/adjust", WebPathType::Post, Some("调整门店库存"), post(handler::stock_adjust))
        .route("/simpleList", WebPathType::Get, Some("门店简单列表"), get(handler::simple_list))
        .route("/statistics", WebPathType::Get, Some("门店统计"), get(handler::statistics))
}

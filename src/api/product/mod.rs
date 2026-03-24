pub mod product_handler;
pub mod sku_handler;
pub mod stock_handler;
pub mod store_handler;
pub mod brand_handler;
pub mod category_handler;
pub mod shipping_template_handler;
pub mod product_group_handler;
pub mod attribute_template_handler;
pub mod verification_handler;

use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};

pub fn router_product() -> WebPath {
    WebPath::new().nest(
        "/product",
        WebPath::new()
            .route("/list", WebPathType::Get, Some("获取商品列表"), get(product_handler::list))
            .route("/:id", WebPathType::Get, Some("获取商品详情"), get(product_handler::detail))
            .route("/add", WebPathType::Post, Some("新增商品"), post(product_handler::add))
            .route("/edit", WebPathType::Put, Some("编辑商品"), put(product_handler::edit))
            .route("/delete", WebPathType::Delete, Some("删除商品"), delete(product_handler::delete))
            .route("/updateStatus", WebPathType::Put, Some("更新商品状态"), put(product_handler::update_status))
            .route("/audit", WebPathType::Put, Some("审核商品"), put(product_handler::audit))
            .route("/batchUpdateStatus", WebPathType::Put, Some("批量更新状态"), put(product_handler::batch_update_status))
            .route("/batchDelete", WebPathType::Delete, Some("批量删除"), delete(product_handler::batch_delete))
            .route("/statistics", WebPathType::Get, Some("商品统计"), get(product_handler::statistics))
            .route("/simpleList", WebPathType::Get, Some("商品简单列表"), get(product_handler::simple_list))
            .route("/sku/list/:product_id", WebPathType::Get, Some("获取SKU列表"), get(sku_handler::sku_list))
            .route("/sku/:id", WebPathType::Get, Some("获取SKU详情"), get(sku_handler::sku_detail))
            .route("/sku/add", WebPathType::Post, Some("新增SKU"), post(sku_handler::sku_add))
            .route("/sku/edit", WebPathType::Put, Some("编辑SKU"), put(sku_handler::sku_edit))
            .route("/sku/delete", WebPathType::Delete, Some("删除SKU"), delete(sku_handler::sku_delete))
            .route("/sku/generate", WebPathType::Post, Some("生成SKU组合"), post(sku_handler::sku_generate))
            .route("/stock/list", WebPathType::Get, Some("获取库存列表"), get(stock_handler::list))
            .route("/stock/log", WebPathType::Get, Some("获取库存日志"), get(stock_handler::log_list))
            .route("/stock/adjust", WebPathType::Post, Some("调整库存"), post(stock_handler::adjust))
            .route("/stock/alertList", WebPathType::Get, Some("获取预警列表"), get(stock_handler::alert_list))
            .route("/stock/alertConfig", WebPathType::Post, Some("配置预警"), post(stock_handler::alert_config))
            .route("/stock/statistics", WebPathType::Get, Some("库存统计"), get(stock_handler::statistics))
            .route("/store/list", WebPathType::Get, Some("获取门店列表"), get(store_handler::list))
            .route("/store/:id", WebPathType::Get, Some("获取门店详情"), get(store_handler::detail))
            .route("/store/add", WebPathType::Post, Some("新增门店"), post(store_handler::add))
            .route("/store/edit", WebPathType::Put, Some("编辑门店"), put(store_handler::edit))
            .route("/store/delete", WebPathType::Delete, Some("删除门店"), delete(store_handler::delete))
            .route("/store/stock/list", WebPathType::Get, Some("获取门店库存"), get(store_handler::stock_list))
            .route("/store/stock/adjust", WebPathType::Post, Some("调整门店库存"), post(store_handler::stock_adjust))
            .route("/store/simpleList", WebPathType::Get, Some("门店简单列表"), get(store_handler::simple_list))
            .route("/store/statistics", WebPathType::Get, Some("门店统计"), get(store_handler::statistics))
            .route("/brand/list", WebPathType::Get, Some("获取品牌列表"), get(brand_handler::list))
            .route("/brand/:id", WebPathType::Get, Some("获取品牌详情"), get(brand_handler::detail))
            .route("/brand/add", WebPathType::Post, Some("新增品牌"), post(brand_handler::add))
            .route("/brand/edit", WebPathType::Put, Some("编辑品牌"), put(brand_handler::edit))
            .route("/brand/delete", WebPathType::Delete, Some("删除品牌"), delete(brand_handler::delete))
            .route("/brand/updateStatus", WebPathType::Put, Some("更新品牌状态"), put(brand_handler::update_status))
            .route("/brand/simpleList", WebPathType::Get, Some("获取品牌简单列表"), get(brand_handler::simple_list))
            .route("/category/tree", WebPathType::Get, Some("获取分类树"), get(category_handler::tree))
            .route("/category/list", WebPathType::Get, Some("获取分类列表"), get(category_handler::list))
            .route("/category/:id", WebPathType::Get, Some("获取分类详情"), get(category_handler::detail))
            .route("/category/add", WebPathType::Post, Some("新增分类"), post(category_handler::add))
            .route("/category/edit", WebPathType::Put, Some("编辑分类"), put(category_handler::edit))
            .route("/category/delete", WebPathType::Delete, Some("删除分类"), delete(category_handler::delete))
            .route("/category/updateStatus", WebPathType::Put, Some("更新分类状态"), put(category_handler::update_status))
            .route("/shipping/list", WebPathType::Get, Some("获取运费模板列表"), get(shipping_template_handler::list))
            .route("/shipping/:id", WebPathType::Get, Some("获取运费模板详情"), get(shipping_template_handler::detail))
            .route("/shipping/add", WebPathType::Post, Some("新增运费模板"), post(shipping_template_handler::add))
            .route("/shipping/edit", WebPathType::Put, Some("编辑运费模板"), put(shipping_template_handler::edit))
            .route("/shipping/delete", WebPathType::Delete, Some("删除运费模板"), delete(shipping_template_handler::delete))
            .route("/shipping/calculate", WebPathType::Post, Some("计算运费"), post(shipping_template_handler::calculate))
            .route("/shipping/simpleList", WebPathType::Get, Some("获取运费模板简单列表"), get(shipping_template_handler::simple_list))
            .route("/group/list", WebPathType::Get, Some("获取分组列表"), get(product_group_handler::list))
            .route("/group/:id", WebPathType::Get, Some("获取分组详情"), get(product_group_handler::detail))
            .route("/group/add", WebPathType::Post, Some("新增分组"), post(product_group_handler::add))
            .route("/group/edit", WebPathType::Put, Some("编辑分组"), put(product_group_handler::edit))
            .route("/group/delete", WebPathType::Delete, Some("删除分组"), delete(product_group_handler::delete))
            .route("/group/simpleList", WebPathType::Get, Some("获取分组简单列表"), get(product_group_handler::simple_list))
            .route("/attribute/list", WebPathType::Get, Some("获取属性模板列表"), get(attribute_template_handler::list))
            .route("/attribute/:id", WebPathType::Get, Some("获取属性模板详情"), get(attribute_template_handler::detail))
            .route("/attribute/add", WebPathType::Post, Some("新增属性模板"), post(attribute_template_handler::add))
            .route("/attribute/edit", WebPathType::Put, Some("编辑属性模板"), put(attribute_template_handler::edit))
            .route("/attribute/delete", WebPathType::Delete, Some("删除属性模板"), delete(attribute_template_handler::delete))
            .route("/attribute/byCategory/:categoryId", WebPathType::Get, Some("根据分类获取属性模板"), get(attribute_template_handler::by_category))
            .route("/verification/list", WebPathType::Get, Some("获取核销码列表"), get(verification_handler::list))
            .route("/verification/verify", WebPathType::Post, Some("核销操作"), post(verification_handler::verify))
            .route("/verification/query", WebPathType::Get, Some("查询核销码"), get(verification_handler::query))
            .route("/verification/log", WebPathType::Get, Some("获取核销记录"), get(verification_handler::log_list))
            .route("/verification/statistics", WebPathType::Get, Some("核销统计"), get(verification_handler::statistics)),
    )
}

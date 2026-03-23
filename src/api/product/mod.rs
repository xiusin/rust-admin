pub mod category;
pub mod brand;
pub mod product;
pub mod product_group;
pub mod shipping_template;
pub mod stock;
pub mod store;
pub mod verification;
pub mod attribute_template;

use crate::api::web_path::WebPath;

pub fn router_product() -> WebPath {
    WebPath::new()
        .nest("/category", category::product_category())
        .nest("/brand", brand::product_brand())
        .nest("/list", product::product_list())
        .nest("/sku", product::product_sku())
        .nest("/group", product_group::product_group())
        .nest("/shipping", shipping_template::product_shipping())
        .nest("/stock", stock::product_stock())
        .nest("/store", store::product_store())
        .nest("/verification", verification::product_verification())
        .nest("/attribute", attribute_template::product_attribute_template())
}

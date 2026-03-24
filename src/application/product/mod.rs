pub mod category_service;
pub mod brand_service;
pub mod product_service;
pub mod product_group_service;
pub mod shipping_template_service;
pub mod stock_service;
pub mod store_service;
pub mod verification_service;
pub mod attribute_template_service;

pub use brand_service as s_p_brand;
pub use category_service as s_p_category;
pub use product_service as s_p_product;
pub use product_group_service as s_p_product_group;
pub use shipping_template_service as s_p_shipping_template;
pub use stock_service as s_p_stock;
pub use store_service as s_p_store;
pub use verification_service as s_p_verification;
pub use attribute_template_service as s_p_attribute_template;

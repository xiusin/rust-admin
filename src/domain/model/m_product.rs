use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProductListItem {
    pub id: i64,
    pub category_id: i64,
    pub category_name: Option<String>,
    pub brand_id: Option<i64>,
    pub brand_name: Option<String>,
    pub name: String,
    pub subtitle: Option<String>,
    pub cover_image: String,
    pub images: Option<Vec<String>>,
    pub product_type: i32,
    pub product_type_name: String,
    pub status: i32,
    pub status_name: String,
    pub audit_status: i32,
    pub audit_status_name: String,
    pub sale_status: i32,
    pub sale_status_name: String,
    pub line_price: f64,
    pub sale_price: f64,
    pub cost_price: f64,
    pub stock: i32,
    pub sales: i32,
    pub virtual_sales: i32,
    pub is_multi_spec: i32,
    pub is_hot: i32,
    pub is_new: i32,
    pub is_recommend: i32,
    pub sort: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub sku_count: i32,
    pub group_names: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProductDetail {
    pub id: i64,
    pub category_id: i64,
    pub category_name: Option<String>,
    pub brand_id: Option<i64>,
    pub brand_name: Option<String>,
    pub name: String,
    pub subtitle: Option<String>,
    pub cover_image: String,
    pub images: Option<Vec<String>>,
    pub video: Option<String>,
    pub detail: Option<String>,
    pub product_type: i32,
    pub status: i32,
    pub audit_status: i32,
    pub audit_remark: Option<String>,
    pub sale_status: i32,
    pub sale_time: Option<DateTime>,
    pub line_price: f64,
    pub sale_price: f64,
    pub cost_price: f64,
    pub stock: i32,
    pub sales: i32,
    pub virtual_sales: i32,
    pub limit_buy: i32,
    pub limit_type: i32,
    pub shipping_method: i32,
    pub shipping_template_id: Option<i64>,
    pub shipping_template_name: Option<String>,
    pub weight: f64,
    pub volume: f64,
    pub unit: String,
    pub sort: i32,
    pub is_multi_spec: i32,
    pub is_hot: i32,
    pub is_new: i32,
    pub is_recommend: i32,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub category_ids: Option<Vec<i64>>,
    pub group_ids: Option<Vec<i64>>,
    pub specs: Vec<SpecItem>,
    pub skus: Vec<SkuItem>,
    pub attributes: Vec<ProductAttributeItem>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SpecItem {
    pub id: i64,
    pub name: String,
    pub sort: i32,
    pub values: Vec<SpecValueItem>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SpecValueItem {
    pub id: i64,
    pub value: String,
    pub image: Option<String>,
    pub color_code: Option<String>,
    pub sort: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SkuItem {
    pub id: i64,
    pub sku_code: String,
    pub spec_value_ids: String,
    pub spec_text: String,
    pub image: Option<String>,
    pub sale_price: f64,
    pub line_price: f64,
    pub cost_price: f64,
    pub stock: i32,
    pub sales: i32,
    pub weight: f64,
    pub volume: f64,
    pub status: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProductAttributeItem {
    pub id: i64,
    pub attribute_id: i64,
    pub attribute_name: String,
    pub attribute_value: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProductSimple {
    pub id: i64,
    pub name: String,
    pub cover_image: String,
    pub sale_price: f64,
    pub stock: i32,
    pub status: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SkuSimple {
    pub id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub sku_code: String,
    pub spec_text: String,
    pub image: Option<String>,
    pub sale_price: f64,
    pub stock: i32,
    pub status: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProductStatistics {
    pub total_products: i64,
    pub on_sale_count: i64,
    pub off_sale_count: i64,
    pub out_of_stock_count: i64,
    pub pending_audit_count: i64,
}

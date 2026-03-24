use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ProductAddArgs {
    pub category_id: i64,
    pub brand_id: Option<i64>,
    #[validate(length(min = 1, max = 200, message = "商品名称长度1-200"))]
    pub name: String,
    pub subtitle: Option<String>,
    #[validate(length(min = 1, message = "封面图片不能为空"))]
    pub cover_image: String,
    pub images: Option<Vec<String>>,
    pub video: Option<String>,
    pub detail: Option<String>,
    pub product_type: Option<i32>,
    pub line_price: Option<f64>,
    pub sale_price: Option<f64>,
    pub cost_price: Option<f64>,
    pub stock: Option<i32>,
    pub virtual_sales: Option<i32>,
    pub limit_buy: Option<i32>,
    pub limit_type: Option<i32>,
    pub shipping_method: Option<i32>,
    pub shipping_template_id: Option<i64>,
    pub weight: Option<f64>,
    pub volume: Option<f64>,
    pub unit: Option<String>,
    pub sort: Option<i32>,
    pub is_multi_spec: Option<i32>,
    pub is_hot: Option<i32>,
    pub is_new: Option<i32>,
    pub is_recommend: Option<i32>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub group_ids: Option<Vec<i64>>,
    pub category_ids: Option<Vec<i64>>,
    pub specs: Option<Vec<SpecArgs>>,
    pub skus: Option<Vec<SkuAddArgs>>,
    pub attributes: Option<Vec<ProductAttributeArgs>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ProductEditArgs {
    pub id: i64,
    pub category_id: i64,
    pub brand_id: Option<i64>,
    #[validate(length(min = 1, max = 200, message = "商品名称长度1-200"))]
    pub name: String,
    pub subtitle: Option<String>,
    #[validate(length(min = 1, message = "封面图片不能为空"))]
    pub cover_image: String,
    pub images: Option<Vec<String>>,
    pub video: Option<String>,
    pub detail: Option<String>,
    pub product_type: Option<i32>,
    pub line_price: Option<f64>,
    pub sale_price: Option<f64>,
    pub cost_price: Option<f64>,
    pub stock: Option<i32>,
    pub virtual_sales: Option<i32>,
    pub limit_buy: Option<i32>,
    pub limit_type: Option<i32>,
    pub shipping_method: Option<i32>,
    pub shipping_template_id: Option<i64>,
    pub weight: Option<f64>,
    pub volume: Option<f64>,
    pub unit: Option<String>,
    pub sort: Option<i32>,
    pub is_hot: Option<i32>,
    pub is_new: Option<i32>,
    pub is_recommend: Option<i32>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub group_ids: Option<Vec<i64>>,
    pub category_ids: Option<Vec<i64>>,
    pub specs: Option<Vec<SpecArgs>>,
    pub skus: Option<Vec<SkuEditArgs>>,
    pub attributes: Option<Vec<ProductAttributeArgs>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductListArgs {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub name: Option<String>,
    pub category_id: Option<i64>,
    pub brand_id: Option<i64>,
    pub status: Option<i32>,
    pub audit_status: Option<i32>,
    pub product_type: Option<i32>,
    pub is_hot: Option<i32>,
    pub is_new: Option<i32>,
    pub is_recommend: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductDeleteArgs {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductStatusArgs {
    pub ids: Vec<i64>,
    pub status: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductBatchStatusArgs {
    pub ids: Vec<i64>,
    pub status: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductAuditArgs {
    pub id: i64,
    pub audit_status: i32,
    pub audit_remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductBatchCategoryArgs {
    pub ids: Vec<i64>,
    pub category_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SpecArgs {
    #[validate(length(min = 1, max = 50, message = "规格名长度1-50"))]
    pub name: String,
    pub sort: Option<i32>,
    pub values: Vec<SpecValueArgs>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SpecValueArgs {
    #[validate(length(min = 1, max = 100, message = "规格值长度1-100"))]
    pub value: String,
    pub image: Option<String>,
    pub color_code: Option<String>,
    pub sort: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SkuAddArgs {
    pub product_id: i64,
    pub sku_code: Option<String>,
    pub spec_value_ids: Option<String>,
    pub spec_text: Option<String>,
    pub image: Option<String>,
    #[validate(range(min = 0.01, message = "售价必须大于0"))]
    pub sale_price: f64,
    pub line_price: Option<f64>,
    pub cost_price: Option<f64>,
    pub stock: Option<i32>,
    pub weight: Option<f64>,
    pub volume: Option<f64>,
    pub status: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SkuEditArgs {
    pub id: i64,
    pub sku_code: Option<String>,
    pub spec_value_ids: Option<String>,
    pub spec_text: Option<String>,
    pub image: Option<String>,
    #[validate(range(min = 0.01, message = "售价必须大于0"))]
    pub sale_price: Option<f64>,
    pub line_price: Option<f64>,
    pub cost_price: Option<f64>,
    pub stock: Option<i32>,
    pub weight: Option<f64>,
    pub volume: Option<f64>,
    pub status: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductAttributeArgs {
    pub attribute_id: Option<i64>,
    pub attribute_name: String,
    pub attribute_value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductBatchDeleteArgs {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductSimpleListArgs {
    pub name: Option<String>,
    pub status: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SkuDeleteArgs {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct SkuGenerateArgs {
    pub product_id: i64,
    pub specs: Vec<SpecArgs>,
    pub stock: Option<i32>,
}

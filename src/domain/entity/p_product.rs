//! `SeaORM` Entity for p_product

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_product")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub category_id: i64,
    pub brand_id: Option<i64>,
    pub name: String,
    pub subtitle: Option<String>,
    pub cover_image: String,
    pub images: Option<Json>,
    pub video: Option<String>,
    pub detail: Option<String>,
    pub product_type: i32,
    pub status: i32,
    pub audit_status: i32,
    pub audit_remark: Option<String>,
    pub sale_status: i32,
    pub sale_time: Option<DateTime>,
    pub line_price: Decimal,
    pub sale_price: Decimal,
    pub cost_price: Decimal,
    pub stock: i32,
    pub sales: i32,
    pub virtual_sales: i32,
    pub limit_buy: i32,
    pub limit_type: i32,
    pub shipping_method: i32,
    pub shipping_template_id: Option<i64>,
    pub weight: Decimal,
    pub volume: Decimal,
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
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_category::Entity",
        from = "Column::CategoryId",
        to = "super::p_category::Column::Id"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::p_brand::Entity",
        from = "Column::BrandId",
        to = "super::p_brand::Column::Id"
    )]
    Brand,
    #[sea_orm(
        belongs_to = "super::p_shipping_template::Entity",
        from = "Column::ShippingTemplateId",
        to = "super::p_shipping_template::Column::Id"
    )]
    ShippingTemplate,
    #[sea_orm(has_many = "super::p_sku::Entity")]
    Sku,
    #[sea_orm(has_many = "super::p_spec::Entity")]
    Spec,
    #[sea_orm(has_many = "super::p_product_group_relation::Entity")]
    ProductGroupRelation,
    #[sea_orm(has_many = "super::p_product_category::Entity")]
    ProductCategory,
    #[sea_orm(has_many = "super::p_stock_log::Entity")]
    StockLog,
    #[sea_orm(has_many = "super::p_product_attribute::Entity")]
    ProductAttribute,
}

impl Related<super::p_sku::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sku.def()
    }
}

impl Related<super::p_spec::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Spec.def()
    }
}

impl Related<super::p_product_group::Entity> for Entity {
    fn to() -> RelationDef {
        super::p_product_group_relation::Relation::PProductGroup.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::p_product_group_relation::Relation::PProduct.def().rev())
    }
}

impl Related<super::p_category::Entity> for Entity {
    fn to() -> RelationDef {
        super::p_product_category::Relation::PCategory.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::p_product_category::Relation::PProduct.def().rev())
    }
}

impl Related<super::p_brand::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Brand.def()
    }
}

impl Related<super::p_shipping_template::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ShippingTemplate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

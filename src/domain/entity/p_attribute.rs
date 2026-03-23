//! `SeaORM` Entity for p_attribute

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_attribute")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub template_id: i64,
    pub name: String,
    pub attr_type: i32,
    pub is_required: i32,
    pub is_filter: i32,
    pub sort: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_attribute_template::Entity",
        from = "Column::TemplateId",
        to = "super::p_attribute_template::Column::Id"
    )]
    AttributeTemplate,
    #[sea_orm(has_many = "super::p_attribute_value::Entity")]
    AttributeValue,
    #[sea_orm(has_many = "super::p_product_attribute::Entity")]
    ProductAttribute,
}

impl Related<super::p_attribute_template::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AttributeTemplate.def()
    }
}

impl Related<super::p_attribute_value::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AttributeValue.def()
    }
}

impl Related<super::p_product_attribute::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductAttribute.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

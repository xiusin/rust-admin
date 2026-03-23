//! `SeaORM` Entity for p_shipping_template_region

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_shipping_template_region")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub template_id: i64,
    pub region_type: i32,
    pub region_ids: Option<String>,
    pub region_names: Option<String>,
    pub first_unit: i32,
    pub first_fee: Decimal,
    pub continue_unit: i32,
    pub continue_fee: Decimal,
    pub is_free: i32,
    pub free_condition_value: Decimal,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_shipping_template::Entity",
        from = "Column::TemplateId",
        to = "super::p_shipping_template::Column::Id"
    )]
    ShippingTemplate,
}

impl Related<super::p_shipping_template::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ShippingTemplate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

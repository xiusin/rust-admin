//! `SeaORM` Entity for p_spec_value

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_spec_value")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub spec_id: i64,
    pub product_id: i64,
    pub value: String,
    pub image: Option<String>,
    pub color_code: Option<String>,
    pub sort: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_spec::Entity",
        from = "Column::SpecId",
        to = "super::p_spec::Column::Id"
    )]
    Spec,
    #[sea_orm(
        belongs_to = "super::p_product::Entity",
        from = "Column::ProductId",
        to = "super::p_product::Column::Id"
    )]
    Product,
}

impl Related<super::p_spec::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Spec.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

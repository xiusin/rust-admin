//! `SeaORM` Entity for p_product_group_relation

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_product_group_relation")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub product_id: i64,
    pub group_id: i64,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_product::Entity",
        from = "Column::ProductId",
        to = "super::p_product::Column::Id"
    )]
    PProduct,
    #[sea_orm(
        belongs_to = "super::p_product_group::Entity",
        from = "Column::GroupId",
        to = "super::p_product_group::Column::Id"
    )]
    PProductGroup,
}

impl Related<super::p_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PProduct.def()
    }
}

impl Related<super::p_product_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PProductGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

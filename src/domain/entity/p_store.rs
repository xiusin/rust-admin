//! `SeaORM` Entity for p_store

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_store")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub logo: Option<String>,
    pub cover_image: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub address: Option<String>,
    pub longitude: Option<Decimal>,
    pub latitude: Option<Decimal>,
    pub business_hours: Option<String>,
    pub description: Option<String>,
    pub sort: i32,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::p_store_stock::Entity")]
    StoreStock,
}

impl Related<super::p_store_stock::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StoreStock.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_user_level_config")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub level: i32,
    pub level_name: String,
    pub min_exp: i32,
    pub max_exp: Option<i32>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub discount_rate: Decimal,
    pub privileges: Option<Json>,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_freight_template")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub calculation_type: String,
    pub first_weight: Option<Decimal>,
    pub first_price: Option<Decimal>,
    pub additional_weight: Option<Decimal>,
    pub additional_price: Option<Decimal>,
    pub region_rules: Json,
    pub free_shipping_rules: Json,
    pub is_active: bool,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
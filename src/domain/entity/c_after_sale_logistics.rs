use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_after_sale_logistics")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub after_sale_id: i64,
    pub logistics_type: String,
    pub logistics_company: Option<String>,
    pub tracking_no: Option<String>,
    pub sender_name: Option<String>,
    pub sender_phone: Option<String>,
    pub sender_address: Option<String>,
    pub receiver_name: Option<String>,
    pub receiver_phone: Option<String>,
    pub receiver_address: Option<String>,
    pub status: String,
    pub shipped_at: Option<DateTime>,
    pub received_at: Option<DateTime>,
    pub tracking_info: Option<Json>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::c_after_sale::Entity",
        from = "Column::AfterSaleId",
        to = "super::c_after_sale::Column::Id"
    )]
    AfterSale,
}

impl Related<super::c_after_sale::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AfterSale.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

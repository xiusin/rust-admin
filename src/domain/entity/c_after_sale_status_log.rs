use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_after_sale_status_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub after_sale_id: i64,
    pub old_status: Option<String>,
    pub new_status: String,
    pub operator_type: String,
    pub operator_id: Option<i64>,
    pub operator_name: Option<String>,
    pub remark: Option<String>,
    pub created_at: Option<DateTime>,
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

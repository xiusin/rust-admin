use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_user_ban")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub consumer_id: i64,
    pub ban_type: String,
    pub reason: String,
    pub start_at: DateTime,
    pub end_at: Option<DateTime>,
    pub banned_by: Option<i64>,
    pub unban_at: Option<DateTime>,
    pub unban_by: Option<i64>,
    pub unban_reason: Option<String>,
    pub status: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::c_consumer::Entity",
        from = "Column::ConsumerId",
        to = "super::c_consumer::Column::Id"
    )]
    Consumer,
}

impl Related<super::c_consumer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Consumer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

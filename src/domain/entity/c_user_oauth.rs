use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_user_oauth")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub consumer_id: i64,
    pub oauth_type: String,
    pub oauth_id: String,
    pub oauth_name: Option<String>,
    pub oauth_avatar: Option<String>,
    pub oauth_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<DateTime>,
    pub bind_at: DateTime,
    pub unbind_at: Option<DateTime>,
    pub is_primary: bool,
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

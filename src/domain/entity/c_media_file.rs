use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_media_file")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub consumer_id: i64,
    pub file_name: String,
    pub file_type: String,
    pub file_format: Option<String>,
    pub file_size: i64,
    pub file_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub oss_bucket: Option<String>,
    pub oss_key: Option<String>,
    pub is_deleted: bool,
    pub created_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::c_consumer::Entity",
        from = "Column::ConsumerId",
        to = "super::c_consumer::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    CConsumer,
}

impl Related<super::c_consumer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CConsumer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
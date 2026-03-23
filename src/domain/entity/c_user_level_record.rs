use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "c_user_level_record")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub consumer_id: i64,
    pub old_level: Option<i32>,
    pub new_level: i32,
    pub old_exp: Option<i32>,
    pub new_exp: i32,
    pub exp_change: i32,
    pub change_type: String,
    pub source: Option<String>,
    pub source_id: Option<String>,
    pub remark: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

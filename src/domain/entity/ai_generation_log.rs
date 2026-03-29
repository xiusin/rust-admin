//! `SeaORM` Entity for ai_generation_log

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ai_generation_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub project_id: Option<i64>,
    #[sea_orm(string_len = 50)]
    pub task_type: String,
    #[sea_orm(string_len = 50)]
    pub ai_provider: String,
    #[sea_orm(string_len = 100)]
    pub model: String,
    #[sea_orm(column_type = "Json", nullable)]
    pub input_data: Option<Json>,
    #[sea_orm(column_type = "Json", nullable)]
    pub output_data: Option<Json>,
    pub tokens_used: i32,
    pub cost: Option<Decimal>,
    pub duration_ms: i32,
    #[sea_orm(string_len = 20)]
    pub status: String,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ppt_project::Entity",
        from = "Column::ProjectId",
        to = "super::ppt_project::Column::Id"
    )]
    PptProject,
}

impl Related<super::ppt_project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PptProject.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

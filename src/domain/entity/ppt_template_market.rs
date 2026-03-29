//! `SeaORM` Entity for ppt_template_market

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ppt_template_market")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(string_len = 100)]
    pub name: String,
    #[sea_orm(string_len = 500, nullable)]
    pub description: Option<String>,
    #[sea_orm(string_len = 50, nullable)]
    pub industry: Option<String>,
    #[sea_orm(string_len = 50, nullable)]
    pub style: Option<String>,
    #[sea_orm(column_type = "Json", nullable)]
    pub style_params: Option<Json>,
    #[sea_orm(string_len = 500, nullable)]
    pub thumbnail_url: Option<String>,
    #[sea_orm(column_type = "Json", nullable)]
    pub preview_urls: Option<Json>,
    #[sea_orm(string_len = 500, nullable)]
    pub template_file: Option<String>,
    pub uploader_id: i64,
    #[sea_orm(string_len = 50)]
    pub uploader_name: String,
    pub downloads: i32,
    #[sea_orm(column_type = "Decimal(Some((3,2)))")]
    pub rating: Decimal,
    pub rating_count: i32,
    pub collection_count: i32,
    pub is_free: i8,
    #[sea_orm(column_type = "Decimal(Some((10,2)))", nullable)]
    pub price: Option<Decimal>,
    #[sea_orm(string_len = 1)]
    pub status: String,
    #[sea_orm(string_len = 500, nullable)]
    pub remark: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ppt_template_rating::Entity")]
    Ratings,
    #[sea_orm(has_many = "super::ppt_template_collection::Entity")]
    Collections,
}

impl Related<super::ppt_template_rating::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ratings.def()
    }
}

impl Related<super::ppt_template_collection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collections.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

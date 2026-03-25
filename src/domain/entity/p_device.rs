use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "p_device")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub user_id: i64,
    pub license_id: i64,
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
    pub last_active_time: Option<DateTime>,
    pub status: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::p_license::Entity",
        from = "Column::LicenseId",
        to = "super::p_license::Column::Id"
    )]
    License,
}

impl Related<super::p_license::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::License.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

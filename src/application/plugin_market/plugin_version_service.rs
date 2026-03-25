use crate::domain::entity::p_plugin_version::*;
use crate::domain::entity::p_plugin_version::Entity as VersionEntity;
use crate::domain::entity::p_plugin::Entity as PluginEntity;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use crate::domain::model::m_plugin::*;

#[derive(Debug, Serialize, Clone)]
pub struct VersionListItem {
    pub id: i64,
    pub plugin_id: i64,
    pub version: String,
    pub changelog: Option<String>,
    pub is_latest: i32,
    pub status: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
}

pub async fn list(plugin_id: i64) -> Result<Vec<VersionListItem>, Error> {
    let db = DB().await;

    let versions = VersionEntity::find()
        .filter(p_plugin_version::Column::PluginId.eq(plugin_id))
        .order_by_desc(p_plugin_version::Column::CreatedAt)
        .all(db)
        .await?;

    let items: Vec<VersionListItem> = versions
        .into_iter()
        .map(|v| VersionListItem {
            id: v.id,
            plugin_id: v.plugin_id,
            version: v.version.clone(),
            changelog: v.changelog.clone(),
            is_latest: v.is_latest,
            status: v.status,
            created_at: v.created_at,
        })
        .collect();

    Ok(items)
}

pub async fn detail(id: i64) -> Result<Option<Version>, Error> {
    let db = DB().await;
    let version = VersionEntity::find_by_id(id).one(db).await?;
    Ok(version)
}

pub async fn latest(plugin_id: i64) -> Result<Option<Version>, Error> {
    let db = DB().await;

    let version = VersionEntity::find()
        .filter(p_plugin_version::Column::PluginId.eq(plugin_id))
        .filter(p_plugin_version::Column::IsLatest.eq(1))
        .filter(p_plugin_version::Column::Status.eq(1))
        .one(db)
        .await?;

    Ok(version)
}

pub async fn publish(plugin_id: i64, params: PublishVersionParams) -> Result<i64, Error> {
    let db = DB().await;

    let max_id: Option<i64> = VersionEntity::find()
        .select_only()
        .column_as(p_plugin_version::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;

    let version = p_plugin_version::ActiveModel {
        id: Set(new_id),
        plugin_id: Set(plugin_id),
        version: Set(params.version.clone()),
        changelog: Set(params.changelog),
        download_url: Set(params.download_url),
        file_hash: Set(params.file_hash),
        file_size: Set(params.file_size),
        min_app_version: Set(params.min_app_version),
        is_latest: Set(1),
        status: Set(1),
        created_at: Set(Some(chrono::Utc::now().naive_utc())),
    };

    version.insert(db).await?;

    VersionEntity::update_many()
        .col_expr(p_plugin_version::Column::IsLatest, Value::Int(Some(0)))
        .filter(p_plugin_version::Column::PluginId.eq(plugin_id))
        .filter(p_plugin_version::Column::Version.ne(params.version.clone()))
        .exec(db)
        .await?;

    PluginEntity::update_many()
        .col_expr(p_plugin::Column::Version, Value::String(Some(params.version.clone().into())))
        .col_expr(p_plugin::Column::UpdatedAt, Value::Datetime(Some(chrono::Utc::now().naive_utc())))
        .filter(p_plugin::Column::Id.eq(plugin_id))
        .exec(db)
        .await?;

    Ok(new_id)
}

#[derive(Debug, Deserialize)]
pub struct PublishVersionParams {
    pub version: String,
    pub changelog: Option<String>,
    pub download_url: String,
    pub file_hash: String,
    pub file_size: i64,
    pub min_app_version: Option<String>,
}
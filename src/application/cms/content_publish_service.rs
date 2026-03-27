use crate::common::error::Error;
use crate::domain::entity::{cms_content, cms_content_version};
use crate::model::prelude::*;
use chrono::{Local, NaiveDateTime};
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ContentVersionItem {
    pub id: i64,
    pub content_id: i64,
    pub version: i32,
    pub change_log: Option<String>,
    pub operator_id: Option<i64>,
    pub created_at: Option<NaiveDateTime>,
}

pub async fn publish(id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_content::Entity::find_by_id(id)
        .filter(cms_content::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("内容不存在"))?;

    let mut model: cms_content::ActiveModel = item.into();
    model.status = Set("published".to_string());
    model.publish_time = Set(Some(now));
    model.updated_at = Set(Some(now));
    model.update(db).await?;

    Ok(())
}

pub async fn offline(id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_content::Entity::find_by_id(id)
        .filter(cms_content::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("内容不存在"))?;

    let mut model: cms_content::ActiveModel = item.into();
    model.status = Set("offline".to_string());
    model.updated_at = Set(Some(now));
    model.update(db).await?;

    Ok(())
}

pub async fn schedule_publish(id: i64, publish_time: NaiveDateTime) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_content::Entity::find_by_id(id)
        .filter(cms_content::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("内容不存在"))?;

    let mut model: cms_content::ActiveModel = item.into();
    model.status = Set("scheduled".to_string());
    model.publish_time = Set(Some(publish_time));
    model.updated_at = Set(Some(now));
    model.update(db).await?;

    Ok(())
}

pub async fn audit(id: i64, status: ContentStatus, reason: Option<String>) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_content::Entity::find_by_id(id)
        .filter(cms_content::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("内容不存在"))?;

    let status_str = match status {
        ContentStatus::Draft => "draft",
        ContentStatus::Pending => "pending",
        ContentStatus::Published => "published",
        ContentStatus::Rejected => "rejected",
        ContentStatus::Offline => "offline",
    };

    let mut model: cms_content::ActiveModel = item.into();
    model.status = Set(status_str.to_string());
    model.updated_at = Set(Some(now));

    if let Some(audit_reason) = reason {
        let extra_data = model.extra_data.clone().take();
        let mut extra: serde_json::Value = extra_data
            .as_ref()
            .and_then(|d| d.as_ref())
            .and_then(|d| serde_json::from_str(d).ok())
            .unwrap_or(serde_json::json!({}));
        extra["audit_reason"] = serde_json::json!(audit_reason);
        model.extra_data = Set(Some(extra.to_string()));
    }

    model.update(db).await?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum ContentStatus {
    Draft,
    Pending,
    Published,
    Rejected,
    Offline,
}

pub async fn create_version(
    content_id: i64,
    change_log: Option<String>,
    operator_id: i64,
) -> Result<i32> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let content = cms_content::Entity::find_by_id(content_id)
        .filter(cms_content::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("内容不存在"))?;

    let max_version = cms_content_version::Entity::find()
        .filter(cms_content_version::Column::ContentId.eq(content_id))
        .order_by_desc(cms_content_version::Column::Version)
        .one(db)
        .await?;

    let new_version = match max_version {
        Some(v) => v.version + 1,
        None => 1,
    };

    let id = GID().await;
    let data = serde_json::to_string(&content).ok();

    let version_model = cms_content_version::ActiveModel {
        id: Set(id),
        content_id: Set(content_id),
        version: Set(new_version),
        data: Set(data),
        change_log: Set(change_log),
        operator_id: Set(Some(operator_id)),
        created_at: Set(Some(now)),
    };

    version_model.insert(db).await?;

    Ok(new_version)
}

pub async fn get_versions(content_id: i64) -> Result<Vec<ContentVersionItem>> {
    let db = DB().await;

    let versions = cms_content_version::Entity::find()
        .filter(cms_content_version::Column::ContentId.eq(content_id))
        .order_by_desc(cms_content_version::Column::Version)
        .all(db)
        .await?;

    Ok(versions
        .into_iter()
        .map(|v| ContentVersionItem {
            id: v.id,
            content_id: v.content_id,
            version: v.version,
            change_log: v.change_log,
            operator_id: v.operator_id,
            created_at: v.created_at,
        })
        .collect())
}

pub async fn rollback(content_id: i64, version: i32) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let version_record = cms_content_version::Entity::find()
        .filter(cms_content_version::Column::ContentId.eq(content_id))
        .filter(cms_content_version::Column::Version.eq(version))
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("版本不存在"))?;

    let data = version_record
        .data
        .ok_or_else(|| Error::bad_request("版本数据为空"))?;

    let old_content: cms_content::Model =
        serde_json::from_str(&data).map_err(|_| Error::bad_request("版本数据解析失败"))?;

    let current = cms_content::Entity::find_by_id(content_id)
        .filter(cms_content::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("内容不存在"))?;

    let mut model: cms_content::ActiveModel = current.into();
    model.title = Set(old_content.title);
    model.slug = Set(old_content.slug);
    model.keywords = Set(old_content.keywords);
    model.description = Set(old_content.description);
    model.thumbnail = Set(old_content.thumbnail);
    model.images = Set(old_content.images);
    model.attachments = Set(old_content.attachments);
    model.content_type = Set(old_content.content_type);
    model.extra_data = Set(old_content.extra_data);
    model.template = Set(old_content.template);
    model.updated_at = Set(Some(now));
    model.update(db).await?;

    Ok(())
}

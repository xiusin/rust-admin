use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::error::{Error, Result},
    domain::{
        args::a_pay_channel::{PayChannelAdd, PayChannelConfig, PayChannelEdit, PayChannelSearch},
        entity::pay_channel,
    },
};

use super::super::entity::pay_channel::Entity as PayChannel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayChannelRes {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub channel_type: String,
    pub scenes: Vec<String>,
    pub config: Option<serde_json::Value>,
    pub sort: i32,
    pub is_active: bool,
    pub remark: Option<String>,
    pub created_at: Option<String>,
}

impl From<pay_channel::Model> for PayChannelRes {
    fn from(m: pay_channel::Model) -> Self {
        let scenes = m
            .scenes
            .and_then(|s| serde_json::from_value::<Vec<String>>(s).ok())
            .unwrap_or_default();
        Self {
            id: m.id,
            name: m.name,
            code: m.code,
            channel_type: m.channel_type,
            scenes,
            config: m.config,
            sort: m.sort,
            is_active: m.is_active == 1,
            remark: m.remark,
            created_at: m.created_at.map(|t| t.to_string()),
        }
    }
}

impl PayChannel {
    pub async fn list(db: &DatabaseConnection, search: PayChannelSearch) -> Result<Vec<PayChannelRes>> {
        let mut condition = Condition::all();

        if let Some(name) = &search.name {
            condition = condition.add(pay_channel::Column::Name.contains(name));
        }
        if let Some(channel_type) = &search.channel_type {
            condition = condition.add(pay_channel::Column::ChannelType.eq(channel_type));
        }
        if let Some(is_active) = search.is_active {
            condition = condition.add(pay_channel::Column::IsActive.eq(is_active));
        }

        let list = PayChannel::find()
            .filter(condition)
            .filter(pay_channel::Column::DeletedAt.is_null())
            .order_by_asc(pay_channel::Column::Sort)
            .all(db)
            .await?;

        Ok(list.into_iter().map(|m| m.into()).collect())
    }

    pub async fn add(db: &DatabaseConnection, arg: PayChannelAdd) -> Result<String> {
        let scenes_json = serde_json::to_value(&arg.scenes).unwrap_or(serde_json::json!([]));
        
        let active = pay_channel::ActiveModel {
            name: Set(arg.name),
            code: Set(arg.code),
            channel_type: Set(arg.channel_type),
            scenes: Set(Some(scenes_json)),
            config: Set(None),
            sort: Set(arg.sort.unwrap_or(0)),
            is_active: Set(arg.is_active.unwrap_or(1)),
            remark: Set(arg.remark),
            ..Default::default()
        };

        active.insert(db).await?;
        Ok("success".to_string())
    }

    pub async fn edit(db: &DatabaseConnection, arg: PayChannelEdit) -> Result<String> {
        let channel = PayChannel::find_by_id(arg.id)
            .one(db)
            .await?
            .ok_or_else(|| Error::not_found("渠道不存在"))?;

        let scenes_json = serde_json::to_value(&arg.scenes).unwrap_or(serde_json::json!([]));

        let mut active: pay_channel::ActiveModel = channel.into();
        active.name = Set(arg.name);
        active.code = Set(arg.code);
        active.channel_type = Set(arg.channel_type);
        active.scenes = Set(Some(scenes_json));
        active.sort = Set(arg.sort.unwrap_or(0));
        active.is_active = Set(arg.is_active.unwrap_or(1));
        active.remark = Set(arg.remark);

        active.update(db).await?;
        Ok("success".to_string())
    }

    pub async fn delete(db: &DatabaseConnection, id: i64) -> Result<String> {
        let channel = PayChannel::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::not_found("渠道不存在"))?;

        let mut active: pay_channel::ActiveModel = channel.into();
        active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));

        active.update(db).await?;
        Ok("success".to_string())
    }

    pub async fn update_config(db: &DatabaseConnection, arg: PayChannelConfig) -> Result<String> {
        let channel = PayChannel::find_by_id(arg.id)
            .one(db)
            .await?
            .ok_or_else(|| Error::not_found("渠道不存在"))?;

        let mut active: pay_channel::ActiveModel = channel.into();
        active.config = Set(Some(arg.config));

        active.update(db).await?;
        Ok("success".to_string())
    }

    pub async fn toggle_status(db: &DatabaseConnection, id: i64) -> Result<String> {
        let channel = PayChannel::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::not_found("渠道不存在"))?;

        let new_status = if channel.is_active == 1 { 0 } else { 1 };
        let mut active: pay_channel::ActiveModel = channel.into();
        active.is_active = Set(new_status);

        active.update(db).await?;
        Ok("success".to_string())
    }
}

use crate::common::error::Error;
use crate::domain::model::m_user_extension::*;
use crate::domain::entity::prelude::{CUserOauth, CConsumer};
use crate::domain::entity::{c_user_oauth};
use crate::model::prelude::*;
use sea_orm::*;
use chrono::Local;

fn generate_id() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub async fn bind_oauth(params: BindOauthParams) -> Result<UserOauthModel> {
    let db = DB_WRITE().await;

    let oauth_type_str = serde_json::to_string(&params.oauth_type).unwrap_or_default();

    let existing = CUserOauth::find()
        .filter(c_user_oauth::Column::OauthType.eq(&oauth_type_str))
        .filter(c_user_oauth::Column::OauthId.eq(&params.oauth_id))
        .filter(c_user_oauth::Column::Status.eq("active"))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if existing.is_some() {
        return Err(Error::bad_request("该第三方账号已被绑定"));
    }

    let _consumer = CConsumer::find_by_id(params.consumer_id)
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("用户不存在"))?;

    let now = Local::now().naive_local();

    let existing_binds = CUserOauth::find()
        .filter(c_user_oauth::Column::ConsumerId.eq(params.consumer_id))
        .filter(c_user_oauth::Column::Status.eq("active"))
        .count(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let is_primary = existing_binds == 0;

    let oauth = c_user_oauth::ActiveModel {
        id: Set(generate_id()),
        consumer_id: Set(params.consumer_id),
        oauth_type: Set(oauth_type_str.clone()),
        oauth_id: Set(params.oauth_id.clone()),
        oauth_name: Set(params.oauth_name.clone()),
        oauth_avatar: Set(params.oauth_avatar.clone()),
        oauth_token: Set(params.oauth_token),
        refresh_token: Set(params.refresh_token),
        token_expires_at: Set(params.token_expires_at),
        bind_at: Set(now),
        unbind_at: Set(None),
        is_primary: Set(is_primary),
        status: Set("active".to_string()),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };

    let inserted = CUserOauth::insert(oauth)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Insert error: {}", e)))?;

    Ok(UserOauthModel {
        id: inserted.last_insert_id,
        consumer_id: params.consumer_id,
        oauth_type: oauth_type_str,
        oauth_id: params.oauth_id,
        oauth_name: params.oauth_name,
        oauth_avatar: params.oauth_avatar,
        bind_at: now,
        unbind_at: None,
        is_primary,
        status: "active".to_string(),
        created_at: Some(now),
    })
}

pub async fn unbind_oauth(params: UnbindOauthParams) -> Result<()> {
    let db = DB_WRITE().await;

    let oauth_type_str = serde_json::to_string(&params.oauth_type).unwrap_or_default();

    let oauth = CUserOauth::find()
        .filter(c_user_oauth::Column::ConsumerId.eq(params.consumer_id))
        .filter(c_user_oauth::Column::OauthType.eq(&oauth_type_str))
        .filter(c_user_oauth::Column::Status.eq("active"))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::not_found("未找到绑定记录"))?;

    let active_binds = CUserOauth::find()
        .filter(c_user_oauth::Column::ConsumerId.eq(params.consumer_id))
        .filter(c_user_oauth::Column::Status.eq("active"))
        .count(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    if active_binds <= 1 {
        return Err(Error::bad_request("至少保留一个绑定账号"));
    }

    let now = Local::now().naive_local();
    let mut active_model: c_user_oauth::ActiveModel = oauth.into();
    active_model.status = Set("unbind".to_string());
    active_model.unbind_at = Set(Some(now));
    active_model.updated_at = Set(Some(now));

    CUserOauth::update(active_model)
        .exec(db)
        .await
        .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;

    Ok(())
}

pub async fn list_oauth(consumer_id: i64) -> Result<Vec<UserOauthModel>> {
    let db = DB_WRITE().await;

    let oauths = CUserOauth::find()
        .filter(c_user_oauth::Column::ConsumerId.eq(consumer_id))
        .filter(c_user_oauth::Column::Status.eq("active"))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(oauths
        .into_iter()
        .map(|o| UserOauthModel {
            id: o.id,
            consumer_id: o.consumer_id,
            oauth_type: o.oauth_type,
            oauth_id: o.oauth_id,
            oauth_name: o.oauth_name,
            oauth_avatar: o.oauth_avatar,
            bind_at: o.bind_at,
            unbind_at: o.unbind_at,
            is_primary: o.is_primary,
            status: o.status,
            created_at: o.created_at,
        })
        .collect())
}

pub async fn find_by_oauth(oauth_type: &OAuthType, oauth_id: &str) -> Result<Option<(i64, UserOauthModel)>> {
    let db = DB_WRITE().await;

    let oauth_type_str = serde_json::to_string(oauth_type).unwrap_or_default();

    let oauth = CUserOauth::find()
        .filter(c_user_oauth::Column::OauthType.eq(&oauth_type_str))
        .filter(c_user_oauth::Column::OauthId.eq(oauth_id))
        .filter(c_user_oauth::Column::Status.eq("active"))
        .one(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    Ok(oauth.map(|o| {
        (
            o.consumer_id,
            UserOauthModel {
                id: o.id,
                consumer_id: o.consumer_id,
                oauth_type: o.oauth_type,
                oauth_id: o.oauth_id,
                oauth_name: o.oauth_name,
                oauth_avatar: o.oauth_avatar,
                bind_at: o.bind_at,
                unbind_at: o.unbind_at,
                is_primary: o.is_primary,
                status: o.status,
                created_at: o.created_at,
            },
        )
    }))
}

pub async fn set_primary_bind(consumer_id: i64, target_oauth_id: i64) -> Result<()> {
    let db = DB_WRITE().await;

    let all_binds = CUserOauth::find()
        .filter(c_user_oauth::Column::ConsumerId.eq(consumer_id))
        .filter(c_user_oauth::Column::Status.eq("active"))
        .all(db)
        .await
        .map_err(|e| Error::internal_error(format!("Database error: {}", e)))?;

    let now = Local::now().naive_local();
    for bind in all_binds {
        let is_target = bind.id == target_oauth_id;
        let mut active_model: c_user_oauth::ActiveModel = bind.into();
        active_model.is_primary = Set(is_target);
        active_model.updated_at = Set(Some(now));

        CUserOauth::update(active_model)
            .exec(db)
            .await
            .map_err(|e| Error::internal_error(format!("Update error: {}", e)))?;
    }

    Ok(())
}

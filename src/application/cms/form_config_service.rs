use crate::domain::args::a_form_config::{FormConfigAddReq, FormConfigEditReq};
use crate::domain::entity::cms_form_config;
use crate::domain::model::m_form_config::{FormConfigDetail, FormConfigItem};
use crate::service::prelude::*;
use sea_orm::QueryOrder;

pub async fn list(model_id: i64) -> Result<Vec<FormConfigItem>> {
    let db = DB().await;
    let items = cms_form_config::Entity::find()
        .filter(cms_form_config::Column::ModelId.eq(model_id))
        .filter(cms_form_config::Column::Status.eq("0"))
        .order_by_asc(cms_form_config::Column::Id)
        .into_model::<FormConfigItem>()
        .all(db)
        .await?;
    Ok(items)
}

pub async fn detail(id: i64) -> Result<FormConfigDetail> {
    let db = DB().await;
    let config = cms_form_config::Entity::find_by_id(id)
        .into_model::<FormConfigDetail>()
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("表单配置不存在"))?;
    Ok(config)
}

pub async fn add(args: FormConfigAddReq) -> Result<i64> {
    let db = DB().await;
    let id = GID().await;
    let now = Local::now().naive_local();
    
    let config = cms_form_config::ActiveModel {
        id: Set(id),
        model_id: Set(args.model_id),
        name: Set(args.name),
        code: Set(args.code),
        form_type: Set(args.form_type),
        layout: Set(args.layout),
        groups: Set(args.groups),
        actions: Set(args.actions),
        rules: Set(args.rules),
        hooks: Set(args.hooks),
        is_default: Set(if args.is_default == 1 { "1" } else { "0" }.to_string()),
        status: Set(args.status),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };
    
    if args.is_default == 1 {
        clear_default(db, args.model_id).await?;
    }
    
    config.insert(db).await?;
    Ok(id)
}

pub async fn edit(args: FormConfigEditReq) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();
    
    let config = cms_form_config::Entity::find_by_id(args.id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("表单配置不存在"))?;
    
    let mut active: cms_form_config::ActiveModel = config.into();
    active.name = Set(args.name);
    active.layout = Set(args.layout);
    active.groups = Set(args.groups);
    active.actions = Set(args.actions);
    active.rules = Set(args.rules);
    active.hooks = Set(args.hooks);
    active.is_default = Set(if args.is_default == 1 { "1" } else { "0" }.to_string());
    active.status = Set(args.status);
    active.updated_at = Set(Some(now));
    
    if args.is_default == 1 {
        let model_id = active.model_id.clone().unwrap();
        clear_default(db, model_id).await?;
    }
    
    active.update(db).await?;
    Ok(())
}

pub async fn delete(id: i64) -> Result<()> {
    let db = DB().await;
    let config = cms_form_config::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("表单配置不存在"))?;
    
    config.delete(db).await?;
    Ok(())
}

pub async fn get_default(model_id: i64) -> Result<Option<FormConfigDetail>> {
    let db = DB().await;
    let config = cms_form_config::Entity::find()
        .filter(cms_form_config::Column::ModelId.eq(model_id))
        .filter(cms_form_config::Column::IsDefault.eq("1"))
        .filter(cms_form_config::Column::Status.eq("0"))
        .into_model::<FormConfigDetail>()
        .one(db)
        .await?;
    Ok(config)
}

pub async fn set_default(id: i64) -> Result<()> {
    let db = DB().await;
    let config = cms_form_config::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("表单配置不存在"))?;
    
    let model_id = config.model_id;
    clear_default(db, model_id).await?;
    
    let mut active: cms_form_config::ActiveModel = config.into();
    active.is_default = Set("1".to_string());
    active.updated_at = Set(Some(Local::now().naive_local()));
    active.update(db).await?;
    
    Ok(())
}

async fn clear_default(db: &DatabaseConnection, model_id: i64) -> Result<()> {
    cms_form_config::Entity::update_many()
        .filter(cms_form_config::Column::ModelId.eq(model_id))
        .filter(cms_form_config::Column::IsDefault.eq("1"))
        .set(cms_form_config::ActiveModel {
            is_default: Set("0".to_string()),
            updated_at: Set(Some(Local::now().naive_local())),
            ..Default::default()
        })
        .exec(db)
        .await?;
    Ok(())
}

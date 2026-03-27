use crate::domain::args::a_table_config::{TableConfigAddReq, TableConfigEditReq};
use crate::domain::entity::cms_table_config;
use crate::domain::model::m_table_config::{TableConfigDetail, TableConfigItem};
use crate::service::prelude::*;
use sea_orm::QueryOrder;

pub async fn list(model_id: i64) -> Result<Vec<TableConfigItem>> {
    let db = DB().await;
    let items = cms_table_config::Entity::find()
        .filter(cms_table_config::Column::ModelId.eq(model_id))
        .filter(cms_table_config::Column::Status.eq("0"))
        .order_by_asc(cms_table_config::Column::Id)
        .into_model::<TableConfigItem>()
        .all(db)
        .await?;
    Ok(items)
}

pub async fn detail(id: i64) -> Result<TableConfigDetail> {
    let db = DB().await;
    let config = cms_table_config::Entity::find_by_id(id)
        .into_model::<TableConfigDetail>()
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("表格配置不存在"))?;
    Ok(config)
}

pub async fn add(args: TableConfigAddReq) -> Result<i64> {
    let db = DB().await;
    let id = GID().await;
    let now = Local::now().naive_local();
    
    let config = cms_table_config::ActiveModel {
        id: Set(id),
        model_id: Set(args.model_id),
        name: Set(args.name),
        code: Set(args.code),
        columns: Set(args.columns),
        search: Set(args.search),
        filter: Set(args.filter),
        actions: Set(args.actions),
        batch_actions: Set(args.batch_actions),
        toolbar: Set(args.toolbar),
        pagination: Set(args.pagination),
        features: Set(args.features),
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

pub async fn edit(args: TableConfigEditReq) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();
    
    let config = cms_table_config::Entity::find_by_id(args.id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("表格配置不存在"))?;
    
    let mut active: cms_table_config::ActiveModel = config.into();
    active.name = Set(args.name);
    active.columns = Set(args.columns);
    active.search = Set(args.search);
    active.filter = Set(args.filter);
    active.actions = Set(args.actions);
    active.batch_actions = Set(args.batch_actions);
    active.toolbar = Set(args.toolbar);
    active.pagination = Set(args.pagination);
    active.features = Set(args.features);
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
    let config = cms_table_config::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("表格配置不存在"))?;
    
    config.delete(db).await?;
    Ok(())
}

pub async fn get_default(model_id: i64) -> Result<Option<TableConfigDetail>> {
    let db = DB().await;
    let config = cms_table_config::Entity::find()
        .filter(cms_table_config::Column::ModelId.eq(model_id))
        .filter(cms_table_config::Column::IsDefault.eq("1"))
        .filter(cms_table_config::Column::Status.eq("0"))
        .into_model::<TableConfigDetail>()
        .one(db)
        .await?;
    Ok(config)
}

pub async fn set_default(id: i64) -> Result<()> {
    let db = DB().await;
    let config = cms_table_config::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("表格配置不存在"))?;
    
    let model_id = config.model_id;
    clear_default(db, model_id).await?;
    
    let mut active: cms_table_config::ActiveModel = config.into();
    active.is_default = Set("1".to_string());
    active.updated_at = Set(Some(Local::now().naive_local()));
    active.update(db).await?;
    
    Ok(())
}

async fn clear_default(db: &DatabaseConnection, model_id: i64) -> Result<()> {
    cms_table_config::Entity::update_many()
        .filter(cms_table_config::Column::ModelId.eq(model_id))
        .filter(cms_table_config::Column::IsDefault.eq("1"))
        .set(cms_table_config::ActiveModel {
            is_default: Set("0".to_string()),
            updated_at: Set(Some(Local::now().naive_local())),
            ..Default::default()
        })
        .exec(db)
        .await?;
    Ok(())
}

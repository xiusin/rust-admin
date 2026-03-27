use crate::common::error::Error;
use crate::domain::args::a_cms_field::{CmsFieldAddReq, CmsFieldEditReq, CmsFieldSortReq};
use crate::domain::entity::cms_field;
use crate::domain::entity::cms_model;
use crate::domain::model::m_cms_field::{CmsFieldDetail, CmsFieldItem};
use crate::service::prelude::*;
use chrono::Local;
use sea_orm::*;

pub async fn list(model_id: i64) -> Result<Vec<CmsFieldItem>> {
    let db = DB().await;

    let fields = cms_field::Entity::find()
        .filter(cms_field::Column::ModelId.eq(model_id))
        .order_by_asc(cms_field::Column::Sort)
        .all(db)
        .await?;

    let items: Vec<CmsFieldItem> = fields
        .into_iter()
        .map(|f| CmsFieldItem {
            id: f.id,
            name: f.name,
            code: f.code,
            field_type: f.field_type,
            db_type: f.db_type,
            is_required: f.is_required.parse().unwrap_or(0),
            sort: f.sort,
        })
        .collect();

    Ok(items)
}

pub async fn detail(id: i64) -> Result<CmsFieldDetail> {
    let db = DB().await;

    let field = cms_field::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("字段不存在"))?;

    let form_config: Option<serde_json::Value> = field
        .form_config
        .as_ref()
        .and_then(|s| serde_json::from_str(s).ok());

    let placeholder = form_config
        .as_ref()
        .and_then(|c| c.get("placeholder"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let validation_rule = field
        .validation
        .as_ref()
        .and_then(|s| serde_json::from_str(s).ok());

    Ok(CmsFieldDetail {
        id: field.id,
        model_id: field.model_id,
        name: field.name,
        code: field.code,
        field_type: field.field_type,
        db_type: field.db_type,
        is_required: field.is_required.parse().unwrap_or(0),
        is_searchable: field.is_searchable.parse().unwrap_or(0),
        is_list_show: field.is_list_show.parse().unwrap_or(0),
        is_form_show: field.is_form_show.parse().unwrap_or(0),
        default_value: field.default_value,
        placeholder,
        validation_rule,
        sort: field.sort,
        created_at: field.created_at,
        updated_at: field.updated_at,
    })
}

pub async fn add(args: CmsFieldAddReq, _user_id: i64) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    let model = cms_model::Entity::find_by_id(args.model_id)
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;

    if model.is_enabled == "1" {
        return Err(Error::bad_request("模型已启用，无法添加字段"));
    }

    let existing = cms_field::Entity::find()
        .filter(cms_field::Column::ModelId.eq(args.model_id))
        .filter(cms_field::Column::Code.eq(&args.code))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(Error::bad_request("字段编码已存在"));
    }

    let max_sort = cms_field::Entity::find()
        .filter(cms_field::Column::ModelId.eq(args.model_id))
        .all(db)
        .await?
        .iter()
        .map(|f| f.sort)
        .max()
        .unwrap_or(0);

    let field = cms_field::ActiveModel {
        id: Set(id),
        model_id: Set(args.model_id),
        name: Set(args.name),
        code: Set(args.code),
        field_type: Set(args.field_type),
        db_type: Set(args.db_type),
        default_value: Set(args.default_value),
        is_required: Set(args.is_required.to_string()),
        is_unique: Set(args.is_unique.to_string()),
        is_searchable: Set(args.is_searchable.to_string()),
        is_sortable: Set(args.is_sortable.to_string()),
        is_filterable: Set(args.is_filterable.to_string()),
        is_list_show: Set(args.is_list_show.to_string()),
        is_form_show: Set(args.is_form_show.to_string()),
        is_detail_show: Set(args.is_detail_show.to_string()),
        form_config: Set(args.form_config),
        table_config: Set(args.table_config),
        validation: Set(args.validation),
        sort: Set(if args.sort > 0 { args.sort } else { max_sort + 1 }),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };

    field.insert(db).await?;

    Ok(id)
}

pub async fn edit(args: CmsFieldEditReq, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let field = cms_field::Entity::find_by_id(args.id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("字段不存在"))?;

    let model = cms_model::Entity::find_by_id(field.model_id)
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;

    if model.is_enabled == "1" {
        return Err(Error::bad_request("模型已启用，无法编辑字段"));
    }

    let mut model: cms_field::ActiveModel = field.into();
    model.name = Set(args.name);
    model.default_value = Set(args.default_value);
    model.is_required = Set(args.is_required.to_string());
    model.is_unique = Set(args.is_unique.to_string());
    model.is_searchable = Set(args.is_searchable.to_string());
    model.is_sortable = Set(args.is_sortable.to_string());
    model.is_filterable = Set(args.is_filterable.to_string());
    model.is_list_show = Set(args.is_list_show.to_string());
    model.is_form_show = Set(args.is_form_show.to_string());
    model.is_detail_show = Set(args.is_detail_show.to_string());
    model.form_config = Set(args.form_config);
    model.table_config = Set(args.table_config);
    model.validation = Set(args.validation);
    model.sort = Set(args.sort);
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    Ok(())
}

pub async fn delete(id: i64, _user_id: i64) -> Result<()> {
    let db = DB().await;

    let field = cms_field::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("字段不存在"))?;

    let model = cms_model::Entity::find_by_id(field.model_id)
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;

    if model.is_enabled == "1" {
        return Err(Error::bad_request("模型已启用，无法删除字段"));
    }

    cms_field::Entity::delete_by_id(id).exec(db).await?;

    Ok(())
}

pub async fn sort(args: CmsFieldSortReq, _user_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let model = cms_model::Entity::find_by_id(args.model_id)
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;

    if model.is_enabled == "1" {
        return Err(Error::bad_request("模型已启用，无法排序字段"));
    }

    let txn = db.begin().await?;

    for (index, field_id) in args.field_ids.iter().enumerate() {
        let field = cms_field::Entity::find_by_id(*field_id)
            .one(&txn)
            .await?
            .ok_or_else(|| Error::not_found("字段不存在"))?;

        if field.model_id != args.model_id {
            return Err(Error::bad_request("字段不属于该模型"));
        }

        let mut active_model: cms_field::ActiveModel = field.into();
        active_model.sort = Set(index as i32 + 1);
        active_model.updated_at = Set(Some(now));
        active_model.update(&txn).await?;
    }

    txn.commit().await?;

    Ok(())
}

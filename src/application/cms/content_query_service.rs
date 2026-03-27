use crate::common::error::Error;
use crate::domain::entity::{cms_content, cms_field, cms_model};
use crate::domain::model::m_cms_content::CmsContentList;
use crate::model::prelude::*;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentFilterReq {
    pub model_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_top: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recommend: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hot: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_filters: Option<Vec<FilterCondition>>,
    #[serde(default = "default_page_num")]
    pub page_num: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FilterCondition {
    pub field: String,
    pub operator: String,
    pub value: serde_json::Value,
}

fn default_page_num() -> u32 {
    1
}

fn default_page_size() -> u32 {
    10
}

pub async fn search(
    keyword: &str,
    model_id: Option<i64>,
    page_num: u32,
    page_size: u32,
) -> Result<ListData<CmsContentList>> {
    let db = DB().await;

    let mut conditions = Condition::all();
    conditions = conditions.add(cms_content::Column::DeletedAt.is_null());
    conditions = conditions.add(cms_content::Column::Status.eq("published"));

    if let Some(mid) = model_id {
        conditions = conditions.add(cms_content::Column::ModelId.eq(mid));
    }

    let search_condition = Condition::any()
        .add(cms_content::Column::Title.contains(keyword))
        .add(cms_content::Column::Keywords.contains(keyword))
        .add(cms_content::Column::Description.contains(keyword));
    conditions = conditions.add(search_condition);

    let total = cms_content::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = cms_content::Entity::find()
        .filter(conditions)
        .order_by_desc(cms_content::Column::Sort)
        .order_by_desc(cms_content::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let list: Vec<CmsContentList> = items
        .into_iter()
        .map(|item| CmsContentList {
            id: item.id,
            model_id: item.model_id,
            category_id: item.category_id,
            title: item.title,
            thumbnail: item.thumbnail,
            status: parse_status(&item.status),
            publish_time: item.publish_time,
            view_count: item.view_count,
            created_at: item.created_at,
        })
        .collect();

    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size as u64 - 1) / page_size as u64,
        page_num: page_num as u64,
    })
}

pub async fn advanced_filter(args: ContentFilterReq) -> Result<ListData<CmsContentList>> {
    let db = DB().await;
    let page_num = args.page_num;
    let page_size = args.page_size;

    let mut conditions = Condition::all();
    conditions = conditions.add(cms_content::Column::ModelId.eq(args.model_id));
    conditions = conditions.add(cms_content::Column::DeletedAt.is_null());

    if let Some(category_id) = args.category_id {
        conditions = conditions.add(cms_content::Column::CategoryId.eq(category_id));
    }
    if let Some(status) = &args.status {
        conditions = conditions.add(cms_content::Column::Status.eq(status));
    }
    if let Some(author_id) = args.author_id {
        conditions = conditions.add(cms_content::Column::AuthorId.eq(author_id));
    }
    if let Some(is_top) = args.is_top {
        conditions = conditions.add(cms_content::Column::IsTop.eq(is_top.to_string()));
    }
    if let Some(is_recommend) = args.is_recommend {
        conditions = conditions.add(cms_content::Column::IsRecommend.eq(is_recommend.to_string()));
    }
    if let Some(is_hot) = args.is_hot {
        conditions = conditions.add(cms_content::Column::IsHot.eq(is_hot.to_string()));
    }
    if let Some(start_time) = &args.start_time {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(start_time, "%Y-%m-%d %H:%M:%S") {
            conditions = conditions.add(cms_content::Column::CreatedAt.gte(dt));
        }
    }
    if let Some(end_time) = &args.end_time {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(end_time, "%Y-%m-%d %H:%M:%S") {
            conditions = conditions.add(cms_content::Column::CreatedAt.lte(dt));
        }
    }

    let total = cms_content::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = cms_content::Entity::find()
        .filter(conditions)
        .order_by_desc(cms_content::Column::Sort)
        .order_by_desc(cms_content::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let list: Vec<CmsContentList> = items
        .into_iter()
        .map(|item| CmsContentList {
            id: item.id,
            model_id: item.model_id,
            category_id: item.category_id,
            title: item.title,
            thumbnail: item.thumbnail,
            status: parse_status(&item.status),
            publish_time: item.publish_time,
            view_count: item.view_count,
            created_at: item.created_at,
        })
        .collect();

    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size as u64 - 1) / page_size as u64,
        page_num: page_num as u64,
    })
}

pub async fn get_dynamic_fields(content_id: i64) -> Result<serde_json::Value> {
    let db = DB().await;

    let content = cms_content::Entity::find_by_id(content_id)
        .filter(cms_content::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("内容不存在"))?;

    let fields = cms_field::Entity::find()
        .filter(cms_field::Column::ModelId.eq(content.model_id))
        .order_by_asc(cms_field::Column::Sort)
        .all(db)
        .await?;

    let extra_data: serde_json::Value = content
        .extra_data
        .and_then(|d| serde_json::from_str(&d).ok())
        .unwrap_or(serde_json::json!({}));

    let mut result = serde_json::Map::new();

    for field in fields {
        let field_value = extra_data.get(&field.code).cloned().unwrap_or_else(|| {
            field.default_value.clone().map(|v| serde_json::json!(v)).unwrap_or(serde_json::json!(null))
        });
        result.insert(field.code, field_value);
    }

    Ok(serde_json::Value::Object(result))
}

pub async fn query_dynamic_content(
    table_name: &str,
    filters: Vec<FilterCondition>,
    page_num: u32,
    page_size: u32,
) -> Result<ListData<serde_json::Value>> {
    let db = DB().await;

    let model = cms_model::Entity::find()
        .filter(cms_model::Column::TableName.eq(table_name))
        .filter(cms_model::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("模型不存在"))?;

    let mut conditions = Condition::all();
    conditions = conditions.add(cms_content::Column::ModelId.eq(model.id));
    conditions = conditions.add(cms_content::Column::DeletedAt.is_null());
    conditions = conditions.add(cms_content::Column::Status.eq("published"));

    let _total = cms_content::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = cms_content::Entity::find()
        .filter(conditions)
        .order_by_desc(cms_content::Column::Sort)
        .order_by_desc(cms_content::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let list: Vec<serde_json::Value> = items
        .into_iter()
        .filter_map(|item| {
            let extra_data: serde_json::Value = item
                .extra_data
                .and_then(|d| serde_json::from_str(&d).ok())
                .unwrap_or(serde_json::json!({}));

            let mut result = serde_json::Map::new();
            result.insert("id".to_string(), serde_json::json!(item.id));
            result.insert("title".to_string(), serde_json::json!(item.title));
            result.insert("status".to_string(), serde_json::json!(item.status));
            result.insert("created_at".to_string(), serde_json::json!(item.created_at));

            if let serde_json::Value::Object(extra_map) = extra_data {
                for (k, v) in extra_map {
                    result.insert(k, v);
                }
            }

            let item_value = serde_json::Value::Object(result);

            if apply_filters(&item_value, &filters) {
                Some(item_value)
            } else {
                None
            }
        })
        .collect();

    let filtered_total = list.len() as u64;

    Ok(ListData {
        list,
        total: filtered_total,
        total_pages: (filtered_total + page_size as u64 - 1) / page_size as u64,
        page_num: page_num as u64,
    })
}

fn apply_filters(item: &serde_json::Value, filters: &[FilterCondition]) -> bool {
    if filters.is_empty() {
        return true;
    }

    for filter in filters {
        let field_value = item.get(&filter.field);
        if let Some(value) = field_value {
            if !compare_values(value, &filter.operator, &filter.value) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn compare_values(left: &serde_json::Value, operator: &str, right: &serde_json::Value) -> bool {
    match operator {
        "eq" | "equals" => left == right,
        "ne" | "not_equals" => left != right,
        "gt" | "greater_than" => {
            if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
                l > r
            } else {
                false
            }
        }
        "gte" | "greater_than_or_equals" => {
            if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
                l >= r
            } else {
                false
            }
        }
        "lt" | "less_than" => {
            if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
                l < r
            } else {
                false
            }
        }
        "lte" | "less_than_or_equals" => {
            if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
                l <= r
            } else {
                false
            }
        }
        "contains" => {
            if let (Some(l), Some(r)) = (left.as_str(), right.as_str()) {
                l.contains(r)
            } else {
                false
            }
        }
        "starts_with" => {
            if let (Some(l), Some(r)) = (left.as_str(), right.as_str()) {
                l.starts_with(r)
            } else {
                false
            }
        }
        "ends_with" => {
            if let (Some(l), Some(r)) = (left.as_str(), right.as_str()) {
                l.ends_with(r)
            } else {
                false
            }
        }
        "in" => {
            if let Some(arr) = right.as_array() {
                arr.contains(left)
            } else {
                false
            }
        }
        "not_in" => {
            if let Some(arr) = right.as_array() {
                !arr.contains(left)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn parse_status(status: &str) -> i32 {
    match status {
        "draft" => 0,
        "pending" => 1,
        "published" => 2,
        "rejected" => 3,
        "offline" => 4,
        _ => 0,
    }
}

use crate::common::error::Error;
use crate::domain::args::a_cms_content::{CmsContentAddReq, CmsContentEditReq, CmsContentListReq};
use crate::domain::entity::{cms_content, cms_content_tag, cms_tag};
use crate::domain::model::m_cms_content::{CmsContentDetail, CmsContentList};
use crate::domain::model::m_cms_tag::CmsTagItem;
use crate::model::prelude::*;
use chrono::Local;
use sea_orm::*;

pub async fn list(args: CmsContentListReq) -> Result<ListData<CmsContentList>> {
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
    if let Some(keyword) = &args.keyword {
        conditions = conditions.add(
            Condition::any()
                .add(cms_content::Column::Title.contains(keyword))
                .add(cms_content::Column::Keywords.contains(keyword))
                .add(cms_content::Column::Description.contains(keyword)),
        );
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
        .order_by_desc(cms_content::Column::IsTop)
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

pub async fn detail(id: i64) -> Result<CmsContentDetail> {
    let db = DB().await;

    let item = cms_content::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("内容不存在"))?;

    let tags = get_content_tags(id, db).await?;

    let images: Option<Vec<String>> = item
        .images
        .as_ref()
        .and_then(|v| serde_json::from_str(v).ok());

    let source = item.source.clone();
    Ok(CmsContentDetail {
        id: item.id,
        model_id: item.model_id,
        category_id: item.category_id,
        title: item.title,
        subtitle: item.slug,
        thumbnail: item.thumbnail,
        images,
        keywords: item.keywords,
        description: item.description,
        content: item.extra_data,
        author: source.clone(),
        source,
        status: parse_status(&item.status),
        publish_time: item.publish_time,
        view_count: item.view_count,
        like_count: item.like_count,
        comment_count: item.comment_count,
        is_top: parse_int_flag(&item.is_top),
        is_recommend: parse_int_flag(&item.is_recommend),
        created_at: item.created_at,
        updated_at: item.updated_at,
        tags,
    })
}

async fn get_content_tags(content_id: i64, db: &DatabaseConnection) -> Result<Vec<CmsTagItem>> {
    let tags = cms_content_tag::Entity::find()
        .filter(cms_content_tag::Column::ContentId.eq(content_id))
        .all(db)
        .await?;

    let tag_ids: Vec<i64> = tags.iter().map(|t| t.tag_id).collect();

    if tag_ids.is_empty() {
        return Ok(vec![]);
    }

    let tag_items = cms_tag::Entity::find()
        .filter(cms_tag::Column::Id.is_in(tag_ids))
        .filter(cms_tag::Column::DeletedAt.is_null())
        .all(db)
        .await?;

    Ok(tag_items
        .into_iter()
        .map(|t| CmsTagItem {
            id: t.id,
            name: t.name,
            code: t.code,
            slug: t.slug,
            color: t.color,
            icon: t.icon,
            description: t.description,
            content_count: t.content_count,
            sort: t.sort,
            status: t.status,
            created_at: t.created_at,
        })
        .collect())
}

pub async fn add(args: CmsContentAddReq) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    let txn = db.begin().await?;

    let model = cms_content::ActiveModel {
        id: Set(id),
        model_id: Set(args.model_id),
        category_id: Set(args.category_id),
        title: Set(args.title),
        slug: Set(args.slug),
        keywords: Set(args.keywords),
        description: Set(args.description),
        author_id: Set(args.author_id),
        source: Set(args.source),
        thumbnail: Set(args.thumbnail),
        images: Set(args.images),
        attachments: Set(args.attachments),
        content_type: Set(args.content_type),
        status: Set(args.status),
        publish_time: Set(args.publish_time),
        expire_time: Set(args.expire_time),
        sort: Set(args.sort),
        view_count: Set(0),
        like_count: Set(0),
        comment_count: Set(0),
        is_top: Set(args.is_top.to_string()),
        is_recommend: Set(args.is_recommend.to_string()),
        is_hot: Set(args.is_hot.to_string()),
        allow_comment: Set(args.allow_comment.to_string()),
        password: Set(args.password),
        template: Set(args.template),
        extra_data: Set(args.extra_data),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };

    model.insert(&txn).await?;

    if let Some(tag_ids) = args.tag_ids {
        save_content_tags(id, &tag_ids, &txn).await?;
    }

    txn.commit().await?;

    Ok(id)
}

async fn save_content_tags(
    content_id: i64,
    tag_ids: &[i64],
    txn: &DatabaseTransaction,
) -> Result<()> {
    if tag_ids.is_empty() {
        return Ok(());
    }

    let mut models = Vec::new();
    for tag_id in tag_ids {
        let id = GID().await;
        models.push(cms_content_tag::ActiveModel {
            id: Set(id),
            content_id: Set(content_id),
            tag_id: Set(*tag_id),
            created_at: Set(Some(Local::now().naive_local())),
        });
    }
    
    cms_content_tag::Entity::insert_many(models).exec(txn).await?;
    
    Ok(())
}

pub async fn edit(args: CmsContentEditReq) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let txn = db.begin().await?;

    let item = cms_content::Entity::find_by_id(args.id)
        .one(&txn)
        .await?
        .ok_or_else(|| Error::not_found("内容不存在"))?;

    let mut model: cms_content::ActiveModel = item.into();
    model.category_id = Set(args.category_id);
    model.title = Set(args.title);
    model.slug = Set(args.slug);
    model.keywords = Set(args.keywords);
    model.description = Set(args.description);
    model.source = Set(args.source);
    model.thumbnail = Set(args.thumbnail);
    model.images = Set(args.images);
    model.attachments = Set(args.attachments);
    model.status = Set(args.status);
    model.publish_time = Set(args.publish_time);
    model.expire_time = Set(args.expire_time);
    model.sort = Set(args.sort);
    model.is_top = Set(args.is_top.to_string());
    model.is_recommend = Set(args.is_recommend.to_string());
    model.is_hot = Set(args.is_hot.to_string());
    model.allow_comment = Set(args.allow_comment.to_string());
    model.password = Set(args.password);
    model.template = Set(args.template);
    model.extra_data = Set(args.extra_data);
    model.updated_at = Set(Some(now));

    model.update(&txn).await?;

    if let Some(tag_ids) = args.tag_ids {
        cms_content_tag::Entity::delete_many()
            .filter(cms_content_tag::Column::ContentId.eq(args.id))
            .exec(&txn)
            .await?;
        save_content_tags(args.id, &tag_ids, &txn).await?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn delete(id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_content::Entity::find_by_id(id)
        .filter(cms_content::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("内容不存在"))?;

    let mut model: cms_content::ActiveModel = item.into();
    model.deleted_at = Set(Some(now));
    model.update(db).await?;

    Ok(())
}

pub async fn restore(id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_content::Entity::find_by_id(id)
        .filter(cms_content::Column::DeletedAt.is_not_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("内容不存在或未删除"))?;

    let mut model: cms_content::ActiveModel = item.into();
    model.deleted_at = Set(None);
    model.updated_at = Set(Some(now));
    model.update(db).await?;

    Ok(())
}

pub async fn get_by_model(
    model_id: i64,
    page_num: u32,
    page_size: u32,
) -> Result<ListData<CmsContentList>> {
    let db = DB().await;

    let conditions = Condition::all()
        .add(cms_content::Column::ModelId.eq(model_id))
        .add(cms_content::Column::DeletedAt.is_null())
        .add(cms_content::Column::Status.eq("published"));

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

pub async fn get_by_category(
    category_id: i64,
    page_num: u32,
    page_size: u32,
) -> Result<ListData<CmsContentList>> {
    let db = DB().await;

    let conditions = Condition::all()
        .add(cms_content::Column::CategoryId.eq(category_id))
        .add(cms_content::Column::DeletedAt.is_null())
        .add(cms_content::Column::Status.eq("published"));

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

fn parse_int_flag(flag: &str) -> i32 {
    match flag {
        "1" => 1,
        _ => 0,
    }
}

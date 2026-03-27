use crate::common::error::Error;
use crate::domain::args::a_cms_tag::*;
use crate::domain::entity::{cms_content_tag, cms_tag};
use crate::domain::model::m_cms_tag::*;
use crate::model::prelude::*;
use chrono::Local;
use sea_orm::*;

pub async fn list(args: CmsTagListReq) -> Result<ListData<CmsTagItem>> {
    let db = DB().await;
    let page_num = args.page_num;
    let page_size = args.page_size;

    let mut conditions = Condition::all();
    conditions = conditions.add(cms_tag::Column::DeletedAt.is_null());

    if let Some(name) = &args.name {
        conditions = conditions.add(cms_tag::Column::Name.contains(name));
    }
    if let Some(status) = &args.status {
        conditions = conditions.add(cms_tag::Column::Status.eq(status));
    }

    let total = cms_tag::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await?;

    let items = cms_tag::Entity::find()
        .filter(conditions)
        .order_by_asc(cms_tag::Column::Sort)
        .order_by_desc(cms_tag::Column::CreatedAt)
        .paginate(db, page_size as u64)
        .fetch_page((page_num - 1) as u64)
        .await?;

    let list: Vec<CmsTagItem> = items
        .into_iter()
        .map(|item| CmsTagItem {
            id: item.id,
            name: item.name,
            code: item.code,
            slug: item.slug,
            color: item.color,
            icon: item.icon,
            description: item.description,
            content_count: item.content_count,
            sort: item.sort,
            status: item.status,
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

pub async fn detail(id: i64) -> Result<CmsTagDetail> {
    let db = DB().await;

    let item = cms_tag::Entity::find_by_id(id)
        .filter(cms_tag::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("标签不存在"))?;

    Ok(CmsTagDetail {
        id: item.id,
        name: item.name,
        code: item.code,
        slug: item.slug,
        color: item.color,
        icon: item.icon,
        description: item.description,
        seo_title: item.seo_title,
        seo_keywords: item.seo_keywords,
        seo_description: item.seo_description,
        content_count: item.content_count,
        sort: item.sort,
        status: item.status,
        created_at: item.created_at,
        updated_at: item.updated_at,
    })
}

pub async fn add(args: CmsTagAddReq) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    let existing = cms_tag::Entity::find()
        .filter(cms_tag::Column::Code.eq(&args.code))
        .filter(cms_tag::Column::DeletedAt.is_null())
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(Error::bad_request("标签编码已存在"));
    }

    let model = cms_tag::ActiveModel {
        id: Set(id),
        name: Set(args.name),
        code: Set(Some(args.code)),
        slug: Set(args.slug),
        color: Set(args.color),
        icon: Set(args.icon),
        description: Set(args.description),
        seo_title: Set(args.seo_title),
        seo_keywords: Set(args.seo_keywords),
        seo_description: Set(args.seo_description),
        content_count: Set(0),
        sort: Set(args.sort),
        status: Set(args.status),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };

    model.insert(db).await?;

    Ok(id)
}

pub async fn edit(args: CmsTagEditReq) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_tag::Entity::find_by_id(args.id)
        .filter(cms_tag::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("标签不存在"))?;

    let mut model: cms_tag::ActiveModel = item.into();
    model.name = Set(args.name);
    model.slug = Set(args.slug);
    model.color = Set(args.color);
    model.icon = Set(args.icon);
    model.description = Set(args.description);
    model.seo_title = Set(args.seo_title);
    model.seo_keywords = Set(args.seo_keywords);
    model.seo_description = Set(args.seo_description);
    model.sort = Set(args.sort);
    model.status = Set(args.status);
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    Ok(())
}

pub async fn delete(id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_tag::Entity::find_by_id(id)
        .filter(cms_tag::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("标签不存在"))?;

    cms_content_tag::Entity::delete_many()
        .filter(cms_content_tag::Column::TagId.eq(id))
        .exec(db)
        .await?;

    let mut model: cms_tag::ActiveModel = item.into();
    model.deleted_at = Set(Some(now));
    model.update(db).await?;

    Ok(())
}

pub async fn batch_add(args: CmsTagBatchAddReq) -> Result<Vec<i64>> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let mut ids = Vec::new();

    for tag_name in args.tags {
        let existing = cms_tag::Entity::find()
            .filter(cms_tag::Column::Name.eq(&tag_name))
            .filter(cms_tag::Column::DeletedAt.is_null())
            .one(db)
            .await?;

        if let Some(existing) = existing {
            ids.push(existing.id);
            continue;
        }

        let id = GID().await;
        let code = generate_code_from_name(&tag_name);

        let model = cms_tag::ActiveModel {
            id: Set(id),
            name: Set(tag_name),
            code: Set(Some(code)),
            slug: Set(None),
            color: Set(None),
            icon: Set(None),
            description: Set(None),
            seo_title: Set(None),
            seo_keywords: Set(None),
            seo_description: Set(None),
            content_count: Set(0),
            sort: Set(0),
            status: Set("0".to_string()),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
            deleted_at: Set(None),
        };

        model.insert(db).await?;
        ids.push(id);
    }

    Ok(ids)
}

fn generate_code_from_name(name: &str) -> String {
    use std::collections::HashMap;
    let pinyin_map: HashMap<char, &str> = [
        ('的', "de"), ('是', "shi"), ('在', "zai"), ('有', "you"),
        ('和', "he"), ('了', "le"), ('不', "bu"), ('人', "ren"),
        ('都', "dou"), ('一', "yi"), ('个', "ge"), ('上', "shang"),
        ('也', "ye"), ('很', "hen"), ('到', "dao"), ('说', "shuo"),
        ('要', "yao"), ('去', "qu"), ('自', "zi"), ('能', "neng"),
    ].iter().cloned().collect();

    let mut code = String::new();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            code.push(c.to_ascii_lowercase());
        } else if let Some(&pinyin) = pinyin_map.get(&c) {
            code.push_str(pinyin);
        }
    }

    if code.is_empty() {
        format!("tag_{}", chrono::Utc::now().timestamp())
    } else {
        code
    }
}

pub async fn get_cloud(limit: u32) -> Result<Vec<CmsTagItem>> {
    let db = DB().await;

    let items = cms_tag::Entity::find()
        .filter(cms_tag::Column::DeletedAt.is_null())
        .filter(cms_tag::Column::Status.eq("0"))
        .filter(cms_tag::Column::ContentCount.gt(0))
        .order_by_desc(cms_tag::Column::ContentCount)
        .limit(limit as u64)
        .all(db)
        .await?;

    let list: Vec<CmsTagItem> = items
        .into_iter()
        .map(|item| CmsTagItem {
            id: item.id,
            name: item.name,
            code: item.code,
            slug: item.slug,
            color: item.color,
            icon: item.icon,
            description: item.description,
            content_count: item.content_count,
            sort: item.sort,
            status: item.status,
            created_at: item.created_at,
        })
        .collect();

    Ok(list)
}

pub async fn update_content_count(tag_id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let count = cms_content_tag::Entity::find()
        .filter(cms_content_tag::Column::TagId.eq(tag_id))
        .count(db)
        .await?;

    let item = cms_tag::Entity::find_by_id(tag_id)
        .filter(cms_tag::Column::DeletedAt.is_null())
        .one(db)
        .await?;

    if let Some(item) = item {
        let mut model: cms_tag::ActiveModel = item.into();
        model.content_count = Set(count as i64);
        model.updated_at = Set(Some(now));
        model.update(db).await?;
    }

    Ok(())
}

pub async fn get_by_content(content_id: i64) -> Result<Vec<CmsTagItem>> {
    let db = DB().await;

    let relations = cms_content_tag::Entity::find()
        .filter(cms_content_tag::Column::ContentId.eq(content_id))
        .all(db)
        .await?;

    let tag_ids: Vec<i64> = relations.iter().map(|r| r.tag_id).collect();

    if tag_ids.is_empty() {
        return Ok(vec![]);
    }

    let tags = cms_tag::Entity::find()
        .filter(cms_tag::Column::Id.is_in(tag_ids))
        .filter(cms_tag::Column::DeletedAt.is_null())
        .filter(cms_tag::Column::Status.eq("0"))
        .order_by_asc(cms_tag::Column::Sort)
        .all(db)
        .await?;

    let list: Vec<CmsTagItem> = tags
        .into_iter()
        .map(|item| CmsTagItem {
            id: item.id,
            name: item.name,
            code: item.code,
            slug: item.slug,
            color: item.color,
            icon: item.icon,
            description: item.description,
            content_count: item.content_count,
            sort: item.sort,
            status: item.status,
            created_at: item.created_at,
        })
        .collect();

    Ok(list)
}

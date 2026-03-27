use crate::common::error::Error;
use crate::domain::args::a_cms_category::*;
use crate::domain::entity::{cms_category, cms_content};
use crate::domain::model::m_cms_category::*;
use crate::model::prelude::*;
use chrono::Local;
use sea_orm::*;

pub async fn list(args: CmsCategoryListReq) -> Result<Vec<CmsCategoryItem>> {
    let db = DB().await;

    let mut conditions = Condition::all();
    conditions = conditions.add(cms_category::Column::DeletedAt.is_null());
    conditions = conditions.add(cms_category::Column::ModelId.eq(args.model_id));

    if let Some(status) = &args.status {
        conditions = conditions.add(cms_category::Column::Status.eq(status));
    }

    let items = cms_category::Entity::find()
        .filter(conditions)
        .order_by_asc(cms_category::Column::Sort)
        .order_by_desc(cms_category::Column::CreatedAt)
        .all(db)
        .await?;

    let list: Vec<CmsCategoryItem> = items
        .into_iter()
        .map(|item| CmsCategoryItem {
            id: item.id,
            parent_id: item.parent_id,
            model_id: item.model_id,
            name: item.name,
            code: item.code,
            slug: item.slug,
            icon: item.icon,
            image: item.image,
            description: item.description,
            sort: item.sort,
            status: item.status,
            page_size: item.page_size,
            created_at: item.created_at,
        })
        .collect();

    Ok(list)
}

pub async fn tree(args: CmsCategoryTreeReq) -> Result<Vec<CmsCategoryTree>> {
    let db = DB().await;

    let conditions = Condition::all()
        .add(cms_category::Column::DeletedAt.is_null())
        .add(cms_category::Column::ModelId.eq(args.model_id));

    let items = cms_category::Entity::find()
        .filter(conditions)
        .order_by_asc(cms_category::Column::Sort)
        .all(db)
        .await?;

    let tree = build_category_tree(&items, 0);

    Ok(tree)
}

fn build_category_tree(items: &[cms_category::Model], parent_id: i64) -> Vec<CmsCategoryTree> {
    items
        .iter()
        .filter(|item| item.parent_id == parent_id)
        .map(|item| CmsCategoryTree {
            id: item.id,
            parent_id: item.parent_id,
            model_id: item.model_id,
            name: item.name.clone(),
            code: item.code.clone(),
            slug: item.slug.clone(),
            icon: item.icon.clone(),
            image: item.image.clone(),
            description: item.description.clone(),
            sort: item.sort,
            status: item.status.clone(),
            page_size: item.page_size,
            children: build_category_tree(items, item.id),
        })
        .collect()
}

pub async fn detail(id: i64) -> Result<CmsCategoryDetail> {
    let db = DB().await;

    let item = cms_category::Entity::find_by_id(id)
        .filter(cms_category::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("分类不存在"))?;

    Ok(CmsCategoryDetail {
        id: item.id,
        parent_id: item.parent_id,
        model_id: item.model_id,
        name: item.name,
        code: item.code,
        slug: item.slug,
        icon: item.icon,
        image: item.image,
        description: item.description,
        keywords: item.keywords,
        template_list: item.template_list,
        template_detail: item.template_detail,
        page_size: item.page_size,
        sort: item.sort,
        status: item.status,
        seo_title: item.seo_title,
        seo_keywords: item.seo_keywords,
        seo_description: item.seo_description,
        created_at: item.created_at,
        updated_at: item.updated_at,
    })
}

pub async fn add(args: CmsCategoryAddReq) -> Result<i64> {
    let db = DB().await;
    let now = Local::now().naive_local();
    let id = GID().await;

    if args.parent_id > 0 {
        let parent = cms_category::Entity::find_by_id(args.parent_id)
            .filter(cms_category::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or_else(|| Error::bad_request("父分类不存在"))?;

        if parent.status != "0" {
            return Err(Error::bad_request("父分类已禁用"));
        }
    }

    let existing = cms_category::Entity::find()
        .filter(cms_category::Column::ModelId.eq(args.model_id))
        .filter(cms_category::Column::Code.eq(&args.code))
        .filter(cms_category::Column::DeletedAt.is_null())
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(Error::bad_request("分类编码已存在"));
    }

    let model = cms_category::ActiveModel {
        id: Set(id),
        parent_id: Set(args.parent_id),
        model_id: Set(Some(args.model_id)),
        name: Set(args.name),
        code: Set(Some(args.code)),
        slug: Set(args.slug),
        icon: Set(args.icon),
        image: Set(args.image),
        description: Set(args.description),
        keywords: Set(args.keywords),
        template_list: Set(args.template_list),
        template_detail: Set(args.template_detail),
        page_size: Set(args.page_size),
        sort: Set(args.sort),
        status: Set(args.status),
        seo_title: Set(args.seo_title),
        seo_keywords: Set(args.seo_keywords),
        seo_description: Set(args.seo_description),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };

    model.insert(db).await?;

    Ok(id)
}

pub async fn edit(args: CmsCategoryEditReq) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let item = cms_category::Entity::find_by_id(args.id)
        .filter(cms_category::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("分类不存在"))?;

    if args.parent_id > 0 && args.parent_id != item.parent_id {
        if args.parent_id == args.id {
            return Err(Error::bad_request("不能将分类设置为自己的子分类"));
        }

        let parent = cms_category::Entity::find_by_id(args.parent_id)
            .filter(cms_category::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or_else(|| Error::bad_request("父分类不存在"))?;

        if parent.status != "0" {
            return Err(Error::bad_request("父分类已禁用"));
        }

        if is_child_of(db, args.id, args.parent_id).await? {
            return Err(Error::bad_request("不能将分类移动到自己的子分类下"));
        }
    }

    let mut model: cms_category::ActiveModel = item.into();
    model.parent_id = Set(args.parent_id);
    model.name = Set(args.name);
    model.slug = Set(args.slug);
    model.icon = Set(args.icon);
    model.image = Set(args.image);
    model.description = Set(args.description);
    model.keywords = Set(args.keywords);
    model.template_list = Set(args.template_list);
    model.template_detail = Set(args.template_detail);
    model.page_size = Set(args.page_size);
    model.sort = Set(args.sort);
    model.status = Set(args.status);
    model.seo_title = Set(args.seo_title);
    model.seo_keywords = Set(args.seo_keywords);
    model.seo_description = Set(args.seo_description);
    model.updated_at = Set(Some(now));

    model.update(db).await?;

    Ok(())
}

fn is_child_of(db: &DatabaseConnection, parent_id: i64, child_id: i64) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<bool>> + Send + '_>> {
    Box::pin(async move {
        let children = cms_category::Entity::find()
            .filter(cms_category::Column::ParentId.eq(parent_id))
            .filter(cms_category::Column::DeletedAt.is_null())
            .all(db)
            .await?;

        for child in children {
            if child.id == child_id {
                return Ok(true);
            }
            if is_child_of(db, child.id, child_id).await? {
                return Ok(true);
            }
        }

        Ok(false)
    })
}

pub async fn delete(id: i64) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    let children = cms_category::Entity::find()
        .filter(cms_category::Column::ParentId.eq(id))
        .filter(cms_category::Column::DeletedAt.is_null())
        .count(db)
        .await?;

    if children > 0 {
        return Err(Error::bad_request("存在子分类，无法删除"));
    }

    let content_count = cms_content::Entity::find()
        .filter(cms_content::Column::CategoryId.eq(id))
        .filter(cms_content::Column::DeletedAt.is_null())
        .count(db)
        .await?;

    if content_count > 0 {
        return Err(Error::bad_request("分类下存在内容，无法删除"));
    }

    let item = cms_category::Entity::find_by_id(id)
        .filter(cms_category::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("分类不存在"))?;

    let mut model: cms_category::ActiveModel = item.into();
    model.deleted_at = Set(Some(now));
    model.update(db).await?;

    Ok(())
}

pub async fn sort(args: CategorySortReq) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_local();

    for item in args.items {
        let category = cms_category::Entity::find_by_id(item.id)
            .filter(cms_category::Column::DeletedAt.is_null())
            .one(db)
            .await?;

        if let Some(category) = category {
            let mut model: cms_category::ActiveModel = category.into();
            model.sort = Set(item.sort);
            model.updated_at = Set(Some(now));
            model.update(db).await?;
        }
    }

    Ok(())
}

pub async fn get_children(parent_id: i64) -> Result<Vec<CmsCategoryItem>> {
    let db = DB().await;

    let items = cms_category::Entity::find()
        .filter(cms_category::Column::ParentId.eq(parent_id))
        .filter(cms_category::Column::DeletedAt.is_null())
        .filter(cms_category::Column::Status.eq("0"))
        .order_by_asc(cms_category::Column::Sort)
        .all(db)
        .await?;

    let list: Vec<CmsCategoryItem> = items
        .into_iter()
        .map(|item| CmsCategoryItem {
            id: item.id,
            parent_id: item.parent_id,
            model_id: item.model_id,
            name: item.name,
            code: item.code,
            slug: item.slug,
            icon: item.icon,
            image: item.image,
            description: item.description,
            sort: item.sort,
            status: item.status,
            page_size: item.page_size,
            created_at: item.created_at,
        })
        .collect();

    Ok(list)
}

pub async fn get_path(id: i64) -> Result<Vec<CmsCategoryItem>> {
    let db = DB().await;

    let mut path = Vec::new();
    let mut current_id = Some(id);

    while let Some(cid) = current_id {
        let item = cms_category::Entity::find_by_id(cid)
            .filter(cms_category::Column::DeletedAt.is_null())
            .one(db)
            .await?;

        match item {
            Some(category) => {
                current_id = if category.parent_id > 0 {
                    Some(category.parent_id)
                } else {
                    None
                };

                path.push(CmsCategoryItem {
                    id: category.id,
                    parent_id: category.parent_id,
                    model_id: category.model_id,
                    name: category.name,
                    code: category.code,
                    slug: category.slug,
                    icon: category.icon,
                    image: category.image,
                    description: category.description,
                    sort: category.sort,
                    status: category.status,
                    page_size: category.page_size,
                    created_at: category.created_at,
                });
            }
            None => break,
        }
    }

    path.reverse();

    Ok(path)
}

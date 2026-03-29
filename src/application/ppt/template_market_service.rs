use crate::service::prelude::*;
use crate::domain::entity::{ppt_template_market, ppt_template_rating, ppt_template_collection, ppt_template_tag, ppt_template_tag_relation, ppt_project};
use crate::domain::model::m_template_market::*;
use crate::domain::args::a_template_market::*;
use crate::model::prelude::ListData;
use rust_decimal::Decimal;

pub async fn list(
    VQuery(page): VQuery<PageParams>,
    VQuery(search): VQuery<TemplateListArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let page_num = page.page_num.unwrap_or(1);
    let page_size = page.page_size.unwrap_or(10);
    
    let mut conditions = Condition::all()
        .add(ppt_template_market::Column::DeletedAt.is_null())
        .add(ppt_template_market::Column::Status.eq("1"));
    
    if let Some(ref name) = search.name {
        conditions = conditions.add(ppt_template_market::Column::Name.contains(name));
    }
    if let Some(ref industry) = search.industry {
        conditions = conditions.add(ppt_template_market::Column::Industry.eq(industry));
    }
    if let Some(ref style) = search.style {
        conditions = conditions.add(ppt_template_market::Column::Style.eq(style));
    }
    if let Some(is_free) = search.is_free {
        conditions = conditions.add(ppt_template_market::Column::IsFree.eq(is_free));
    }
    if let Some(uploader_id) = search.uploader_id {
        conditions = conditions.add(ppt_template_market::Column::UploaderId.eq(uploader_id));
    }
    
    let total = ppt_template_market::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await
        .unwrap_or(0);
    
    let order_by = match search.sort_by.as_deref() {
        Some("popular") => ppt_template_market::Column::CollectionCount,
        Some("downloads") => ppt_template_market::Column::Downloads,
        Some("rating") => ppt_template_market::Column::Rating,
        _ => ppt_template_market::Column::CreatedAt,
    };
    
    let items = ppt_template_market::Entity::find()
        .filter(conditions)
        .order_by_desc(order_by)
        .offset((page_num - 1) * page_size)
        .limit(page_size)
        .all(db)
        .await;
    
    match items {
        Ok(items) => {
            let list: Vec<TemplateMarketListItem> = items.into_iter().map(|t| {
                let _preview_urls: Vec<String> = t.preview_urls
                    .and_then(|v| serde_json::from_value(v).ok())
                    .unwrap_or_default();
                TemplateMarketListItem {
                    id: t.id,
                    name: t.name,
                    thumbnail_url: t.thumbnail_url,
                    industry: t.industry,
                    style: t.style,
                    downloads: t.downloads,
                    rating: t.rating,
                    rating_count: t.rating_count,
                    collection_count: t.collection_count,
                    is_free: t.is_free,
                    price: t.price,
                    uploader_name: t.uploader_name,
                    tags: vec![],
                    created_at: t.created_at,
                }
            }).collect();
            
            ApiResponse::ok(ListData {
                list,
                total,
                total_pages: (total + page_size as u64 - 1) / page_size as u64,
                page_num: page_num as u64,
            })
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn detail(Path(id): Path<i64>) -> impl IntoResponse {
    let db = DB().await;
    
    let template = match ppt_template_market::Entity::find()
        .filter(ppt_template_market::Column::Id.eq(id))
        .filter(ppt_template_market::Column::DeletedAt.is_null())
        .one(db)
        .await
    {
        Ok(Some(t)) => t,
        Ok(None) => return ApiResponse::not_found("模板不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let preview_urls: Vec<String> = template.preview_urls
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    
    let tags = get_template_tags(db, id).await.unwrap_or_default();
    
    ApiResponse::ok(TemplateMarketDetail {
        id: template.id,
        name: template.name,
        description: template.description,
        industry: template.industry,
        style: template.style,
        style_params: template.style_params,
        preview_urls,
        template_file: template.template_file,
        downloads: template.downloads,
        rating: template.rating,
        rating_count: template.rating_count,
        collection_count: template.collection_count,
        is_free: template.is_free,
        price: template.price,
        uploader: CreatorInfo {
            id: template.uploader_id,
            name: template.uploader_name,
        },
        tags,
        status: template.status,
        created_at: template.created_at,
        updated_at: template.updated_at,
    })
}

pub async fn upload(
    user: UserInfo,
    VJson(arg): VJson<TemplateUploadArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let id = GID().await;
    let now = Local::now().naive_local();
    
    let preview_urls_json = arg.preview_urls.clone()
        .map(|urls| serde_json::to_value(urls).unwrap_or(serde_json::Value::Null));
    
    let template = ppt_template_market::ActiveModel {
        id: Set(id),
        name: Set(arg.name),
        description: Set(arg.description),
        industry: Set(arg.industry),
        style: Set(arg.style),
        style_params: Set(arg.style_params),
        thumbnail_url: Set(arg.preview_urls.as_ref().and_then(|v| v.first().cloned())),
        preview_urls: Set(preview_urls_json),
        template_file: Set(arg.template_file),
        uploader_id: Set(user.uid),
        uploader_name: Set(user.username.clone()),
        downloads: Set(0),
        rating: Set(Decimal::ZERO),
        rating_count: Set(0),
        collection_count: Set(0),
        is_free: Set(arg.is_free),
        price: Set(arg.price),
        status: Set("0".to_string()),
        remark: Set(None),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };
    
    match template.insert(db).await {
        Ok(_) => {
            if let Some(tags) = arg.tags {
                let _ = save_template_tags(db, id, &tags).await;
            }
            ApiResponse::ok(id)
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn edit(
    user: UserInfo,
    VJson(arg): VJson<TemplateEditArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let template = match ppt_template_market::Entity::find_by_id(arg.id)
        .one(db)
        .await
    {
        Ok(Some(t)) => t,
        Ok(None) => return ApiResponse::not_found("模板不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    if template.uploader_id != user.uid {
        return ApiResponse::forbidden("无权限编辑此模板");
    }
    
    let mut template: ppt_template_market::ActiveModel = template.into();
    
    if let Some(name) = arg.name {
        template.name = Set(name);
    }
    template.description = Set(arg.description);
    template.industry = Set(arg.industry);
    template.style = Set(arg.style);
    template.style_params = Set(arg.style_params);
    
    if let Some(ref preview_urls) = arg.preview_urls {
        template.preview_urls = Set(Some(serde_json::to_value(preview_urls).unwrap_or(serde_json::Value::Null)));
        template.thumbnail_url = Set(preview_urls.first().cloned());
    }
    
    template.template_file = Set(arg.template_file);
    template.is_free = Set(arg.is_free.unwrap_or(template.is_free.unwrap()));
    template.price = Set(arg.price);
    template.updated_at = Set(Some(Local::now().naive_local()));
    
    match template.update(db).await {
        Ok(_) => {
            if let Some(tags) = arg.tags {
                let _ = update_template_tags(db, arg.id, &tags).await;
            }
            ApiResponse::ok("更新成功")
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn delete(
    user: UserInfo,
    VJson(arg): VJson<TemplateDeleteArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    let now = Local::now().naive_local();
    
    for id in arg.ids {
        if let Some(template) = ppt_template_market::Entity::find_by_id(id)
            .one(db)
            .await
            .unwrap_or(None)
        {
            if template.uploader_id != user.uid {
                continue;
            }
            
            let mut template: ppt_template_market::ActiveModel = template.into();
            template.deleted_at = Set(Some(now));
            template.updated_at = Set(Some(now));
            let _ = template.update(db).await;
        }
    }
    
    ApiResponse::ok("删除成功")
}

pub async fn search(
    VQuery(search): VQuery<TemplateSearchArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let page_num = search.page_num.unwrap_or(1);
    let page_size = search.page_size.unwrap_or(10);
    
    let mut conditions = Condition::all()
        .add(ppt_template_market::Column::DeletedAt.is_null())
        .add(ppt_template_market::Column::Status.eq("1"));
    
    if let Some(ref keyword) = search.keyword {
        conditions = conditions.add(
            Condition::any()
                .add(ppt_template_market::Column::Name.contains(keyword))
                .add(ppt_template_market::Column::Description.contains(keyword))
        );
    }
    if let Some(ref industry) = search.industry {
        conditions = conditions.add(ppt_template_market::Column::Industry.eq(industry));
    }
    if let Some(ref style) = search.style {
        conditions = conditions.add(ppt_template_market::Column::Style.eq(style));
    }
    if let Some(is_free) = search.is_free {
        conditions = conditions.add(ppt_template_market::Column::IsFree.eq(is_free));
    }
    
    let total = ppt_template_market::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await
        .unwrap_or(0);
    
    let items = ppt_template_market::Entity::find()
        .filter(conditions)
        .order_by_desc(ppt_template_market::Column::Downloads)
        .offset(((page_num - 1) * page_size) as u64)
        .limit(page_size as u64)
        .all(db)
        .await;
    
    match items {
        Ok(items) => {
            let list: Vec<TemplateMarketListItem> = items.into_iter().map(|t| {
                TemplateMarketListItem {
                    id: t.id,
                    name: t.name,
                    thumbnail_url: t.thumbnail_url,
                    industry: t.industry,
                    style: t.style,
                    downloads: t.downloads,
                    rating: t.rating,
                    rating_count: t.rating_count,
                    collection_count: t.collection_count,
                    is_free: t.is_free,
                    price: t.price,
                    uploader_name: t.uploader_name,
                    tags: vec![],
                    created_at: t.created_at,
                }
            }).collect();
            
            ApiResponse::ok(ListData {
                list,
                total,
                total_pages: (total + page_size as u64 - 1) / page_size as u64,
                page_num: page_num as u64,
            })
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn recommend(
    VQuery(arg): VQuery<TemplateRecommendArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    let limit = arg.limit.unwrap_or(10);
    
    let mut conditions = Condition::all()
        .add(ppt_template_market::Column::DeletedAt.is_null())
        .add(ppt_template_market::Column::Status.eq("1"));
    
    if let Some(ref industry) = arg.industry {
        conditions = conditions.add(ppt_template_market::Column::Industry.eq(industry));
    }
    
    let items = ppt_template_market::Entity::find()
        .filter(conditions)
        .order_by_desc(ppt_template_market::Column::Rating)
        .order_by_desc(ppt_template_market::Column::Downloads)
        .limit(limit as u64)
        .all(db)
        .await;
    
    match items {
        Ok(items) => {
            let list: Vec<TemplateMarketListItem> = items.into_iter().map(|t| {
                TemplateMarketListItem {
                    id: t.id,
                    name: t.name,
                    thumbnail_url: t.thumbnail_url,
                    industry: t.industry,
                    style: t.style,
                    downloads: t.downloads,
                    rating: t.rating,
                    rating_count: t.rating_count,
                    collection_count: t.collection_count,
                    is_free: t.is_free,
                    price: t.price,
                    uploader_name: t.uploader_name,
                    tags: vec![],
                    created_at: t.created_at,
                }
            }).collect();
            
            ApiResponse::ok(list)
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn apply_template(
    user: UserInfo,
    VJson(arg): VJson<TemplateApplyArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let template = match ppt_template_market::Entity::find_by_id(arg.template_id)
        .one(db)
        .await
    {
        Ok(Some(t)) if t.deleted_at.is_none() && t.status == "1" => t,
        Ok(_) => return ApiResponse::not_found("模板不存在或已下架"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let project = match ppt_project::Entity::find_by_id(arg.project_id)
        .one(db)
        .await
    {
        Ok(Some(p)) if p.user_id == user.uid => p,
        Ok(_) => return ApiResponse::not_found("项目不存在或无权限"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let mut project: ppt_project::ActiveModel = project.into();
    project.style_template_id = Set(Some(arg.template_id));
    project.updated_at = Set(Some(Local::now().naive_local()));
    
    match project.update(db).await {
        Ok(_) => {
            let mut template: ppt_template_market::ActiveModel = template.into();
            template.downloads = Set(template.downloads.unwrap() + 1);
            let _ = template.update(db).await;
            
            ApiResponse::ok("应用成功")
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn rate(
    user: UserInfo,
    VJson(arg): VJson<TemplateRatingArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let _template = match ppt_template_market::Entity::find_by_id(arg.template_id)
        .one(db)
        .await
    {
        Ok(Some(t)) if t.deleted_at.is_none() => t,
        Ok(_) => return ApiResponse::not_found("模板不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let existing = ppt_template_rating::Entity::find()
        .filter(ppt_template_rating::Column::TemplateId.eq(arg.template_id))
        .filter(ppt_template_rating::Column::UserId.eq(user.uid))
        .one(db)
        .await;
    
    match existing {
        Ok(Some(rating)) => {
            let mut rating: ppt_template_rating::ActiveModel = rating.into();
            rating.rating = Set(arg.rating);
            rating.comment = Set(arg.comment);
            rating.updated_at = Set(Some(Local::now().naive_local()));
            
            match rating.update(db).await {
                Ok(_) => ApiResponse::ok("评价已更新"),
                Err(e) => ApiResponse::bad_request(e.to_string()),
            }
        }
        Ok(None) => {
            let id = GID().await;
            let now = Local::now().naive_local();
            
            let rating = ppt_template_rating::ActiveModel {
                id: Set(id),
                template_id: Set(arg.template_id),
                user_id: Set(user.uid),
                user_name: Set(user.username.clone()),
                rating: Set(arg.rating),
                comment: Set(arg.comment),
                created_at: Set(Some(now)),
                updated_at: Set(Some(now)),
            };
            
            match rating.insert(db).await {
                Ok(_) => {
                    let _ = update_template_rating(db, arg.template_id).await;
                    ApiResponse::ok("评价成功")
                }
                Err(e) => ApiResponse::bad_request(e.to_string()),
            }
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn collect(
    user: UserInfo,
    VJson(arg): VJson<TemplateCollectionArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let template = match ppt_template_market::Entity::find_by_id(arg.template_id)
        .one(db)
        .await
    {
        Ok(Some(t)) if t.deleted_at.is_none() => t,
        Ok(_) => return ApiResponse::not_found("模板不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let existing = ppt_template_collection::Entity::find()
        .filter(ppt_template_collection::Column::TemplateId.eq(arg.template_id))
        .filter(ppt_template_collection::Column::UserId.eq(user.uid))
        .one(db)
        .await;
    
    match existing {
        Ok(Some(_)) => {
            let _ = ppt_template_collection::Entity::delete_many()
                .filter(ppt_template_collection::Column::TemplateId.eq(arg.template_id))
                .filter(ppt_template_collection::Column::UserId.eq(user.uid))
                .exec(db)
                .await;
            
            let mut template: ppt_template_market::ActiveModel = template.into();
            template.collection_count = Set((template.collection_count.unwrap() - 1).max(0));
            let _ = template.update(db).await;
            
            ApiResponse::ok("取消收藏成功")
        }
        Ok(None) => {
            let id = GID().await;
            let now = Local::now().naive_local();
            
            let collection = ppt_template_collection::ActiveModel {
                id: Set(id),
                template_id: Set(arg.template_id),
                user_id: Set(user.uid),
                created_at: Set(Some(now)),
            };
            
            match collection.insert(db).await {
                Ok(_) => {
                    let mut template: ppt_template_market::ActiveModel = template.into();
                    template.collection_count = Set(template.collection_count.unwrap() + 1);
                    let _ = template.update(db).await;
                    
                    ApiResponse::ok("收藏成功")
                }
                Err(e) => ApiResponse::bad_request(e.to_string()),
            }
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn get_my_templates(
    user: UserInfo,
    VQuery(page): VQuery<PageParams>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let page_num = page.page_num.unwrap_or(1);
    let page_size = page.page_size.unwrap_or(10);
    
    let conditions = Condition::all()
        .add(ppt_template_market::Column::UploaderId.eq(user.uid))
        .add(ppt_template_market::Column::DeletedAt.is_null());
    
    let total = ppt_template_market::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await
        .unwrap_or(0);
    
    let items = ppt_template_market::Entity::find()
        .filter(conditions)
        .order_by_desc(ppt_template_market::Column::CreatedAt)
        .offset(((page_num - 1) * page_size) as u64)
        .limit(page_size as u64)
        .all(db)
        .await;
    
    match items {
        Ok(items) => {
            let list: Vec<TemplateMarketListItem> = items.into_iter().map(|t| {
                TemplateMarketListItem {
                    id: t.id,
                    name: t.name,
                    thumbnail_url: t.thumbnail_url,
                    industry: t.industry,
                    style: t.style,
                    downloads: t.downloads,
                    rating: t.rating,
                    rating_count: t.rating_count,
                    collection_count: t.collection_count,
                    is_free: t.is_free,
                    price: t.price,
                    uploader_name: t.uploader_name,
                    tags: vec![],
                    created_at: t.created_at,
                }
            }).collect();
            
            ApiResponse::ok(ListData {
                list,
                total,
                total_pages: (total + page_size as u64 - 1) / page_size as u64,
                page_num: page_num as u64,
            })
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn get_collected_templates(
    user: UserInfo,
    VQuery(page): VQuery<PageParams>,
) -> impl IntoResponse {
    let db = DB().await;
    
    let page_num = page.page_num.unwrap_or(1);
    let page_size = page.page_size.unwrap_or(10);
    
    let collections = ppt_template_collection::Entity::find()
        .filter(ppt_template_collection::Column::UserId.eq(user.uid))
        .order_by_desc(ppt_template_collection::Column::CreatedAt)
        .offset(((page_num - 1) * page_size) as u64)
        .limit(page_size as u64)
        .all(db)
        .await;
    
    match collections {
        Ok(collections) => {
            let mut list = Vec::new();
            for c in collections {
                if let Some(template) = ppt_template_market::Entity::find_by_id(c.template_id)
                    .one(db)
                    .await
                    .unwrap_or(None)
                {
                    if template.deleted_at.is_none() {
                        list.push(TemplateCollectionItem {
                            id: c.id,
                            template_id: template.id,
                            template_name: template.name,
                            thumbnail_url: template.thumbnail_url,
                            created_at: c.created_at,
                        });
                    }
                }
            }
            
            let total = ppt_template_collection::Entity::find()
                .filter(ppt_template_collection::Column::UserId.eq(user.uid))
                .count(db)
                .await
                .unwrap_or(0);
            
            ApiResponse::ok(ListData {
                list,
                total,
                total_pages: (total + page_size as u64 - 1) / page_size as u64,
                page_num: page_num as u64,
            })
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn get_ratings(Path(template_id): Path<i64>) -> impl IntoResponse {
    let db = DB().await;
    
    let ratings = ppt_template_rating::Entity::find()
        .filter(ppt_template_rating::Column::TemplateId.eq(template_id))
        .order_by_desc(ppt_template_rating::Column::CreatedAt)
        .limit(20)
        .all(db)
        .await;
    
    match ratings {
        Ok(ratings) => {
            let list: Vec<TemplateRatingItem> = ratings.into_iter().map(|r| {
                TemplateRatingItem {
                    id: r.id,
                    user_name: r.user_name,
                    rating: r.rating,
                    comment: r.comment,
                    created_at: r.created_at,
                }
            }).collect();
            
            ApiResponse::ok(list)
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn update_status(
    VJson(arg): VJson<TemplateStatusArgs>,
) -> impl IntoResponse {
    let db = DB().await;
    
    for id in arg.ids {
        if let Some(template) = ppt_template_market::Entity::find_by_id(id)
            .one(db)
            .await
            .unwrap_or(None)
        {
            let mut template: ppt_template_market::ActiveModel = template.into();
            template.status = Set(arg.status.clone());
            template.updated_at = Set(Some(Local::now().naive_local()));
            let _ = template.update(db).await;
        }
    }
    
    ApiResponse::ok("状态更新成功")
}

async fn get_template_tags(db: &DatabaseConnection, template_id: i64) -> std::result::Result<Vec<String>, DbErr> {
    let relations = ppt_template_tag_relation::Entity::find()
        .filter(ppt_template_tag_relation::Column::TemplateId.eq(template_id))
        .all(db)
        .await?;
    
    let mut tags = Vec::new();
    for relation in relations {
        if let Some(tag) = ppt_template_tag::Entity::find_by_id(relation.tag_id)
            .one(db)
            .await?
        {
            tags.push(tag.name);
        }
    }
    
    Ok(tags)
}

async fn save_template_tags(db: &DatabaseConnection, template_id: i64, tags: &[String]) -> std::result::Result<(), DbErr> {
    for tag_name in tags {
        let tag = find_or_create_tag(db, tag_name).await?;
        
        let id = GID().await;
        let relation = ppt_template_tag_relation::ActiveModel {
            id: Set(id),
            template_id: Set(template_id),
            tag_id: Set(tag.id),
            created_at: Set(Some(Local::now().naive_local())),
        };
        
        relation.insert(db).await?;
    }
    
    Ok(())
}

async fn update_template_tags(db: &DatabaseConnection, template_id: i64, tags: &[String]) -> std::result::Result<(), DbErr> {
    let _ = ppt_template_tag_relation::Entity::delete_many()
        .filter(ppt_template_tag_relation::Column::TemplateId.eq(template_id))
        .exec(db)
        .await;
    
    save_template_tags(db, template_id, tags).await
}

async fn find_or_create_tag(db: &DatabaseConnection, name: &str) -> std::result::Result<ppt_template_tag::Model, DbErr> {
    if let Some(tag) = ppt_template_tag::Entity::find()
        .filter(ppt_template_tag::Column::Name.eq(name))
        .one(db)
        .await?
    {
        let mut tag: ppt_template_tag::ActiveModel = tag.into();
        tag.usage_count = Set(tag.usage_count.unwrap() + 1);
        tag.update(db).await
    } else {
        let id = GID().await;
        let tag = ppt_template_tag::ActiveModel {
            id: Set(id),
            name: Set(name.to_string()),
            description: Set(None),
            usage_count: Set(1),
            created_at: Set(Some(Local::now().naive_local())),
        };
        
        tag.insert(db).await
    }
}

async fn update_template_rating(db: &DatabaseConnection, template_id: i64) -> std::result::Result<(), DbErr> {
    let ratings = ppt_template_rating::Entity::find()
        .filter(ppt_template_rating::Column::TemplateId.eq(template_id))
        .all(db)
        .await?;
    
    let count = ratings.len() as i32;
    let total: i32 = ratings.iter().map(|r| r.rating).sum();
    let avg = if count > 0 { Decimal::from(total) / Decimal::from(count) } else { Decimal::ZERO };
    
    if let Some(template) = ppt_template_market::Entity::find_by_id(template_id)
        .one(db)
        .await?
    {
        let mut template: ppt_template_market::ActiveModel = template.into();
        template.rating = Set(avg);
        template.rating_count = Set(count);
        template.update(db).await?;
    }
    
    Ok(())
}

pub async fn delete_by_id(Path(id): Path<i64>) -> impl IntoResponse {
    let db = DB().await;
    
    match ppt_template_market::Entity::find_by_id(id)
        .one(db)
        .await
    {
        Ok(Some(t)) if t.deleted_at.is_none() => {
            let now = Local::now().naive_local();
            let mut template: ppt_template_market::ActiveModel = t.into();
            template.deleted_at = Set(Some(now));
            match template.update(db).await {
                Ok(_) => ApiResponse::ok("删除成功"),
                Err(e) => ApiResponse::bad_request(e.to_string()),
            }
        }
        Ok(_) => ApiResponse::not_found("模板不存在"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn favorite(Path(id): Path<i64>) -> impl IntoResponse {
    let db = DB().await;
    
    let template = match ppt_template_market::Entity::find_by_id(id)
        .one(db)
        .await
    {
        Ok(Some(t)) if t.deleted_at.is_none() => t,
        Ok(_) => return ApiResponse::not_found("模板不存在"),
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    
    let collection_id = GID().await;
    let now = Local::now().naive_local();
    
    let collection = ppt_template_collection::ActiveModel {
        id: Set(collection_id),
        template_id: Set(id),
        user_id: Set(0),
        created_at: Set(Some(now)),
    };
    
    match collection.insert(db).await {
        Ok(_) => {
            let mut template: ppt_template_market::ActiveModel = template.into();
            template.collection_count = Set(template.collection_count.unwrap() + 1);
            let _ = template.update(db).await;
            ApiResponse::ok("收藏成功")
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn unfavorite(Path(id): Path<i64>) -> impl IntoResponse {
    let db = DB().await;
    
    let _ = ppt_template_collection::Entity::delete_many()
        .filter(ppt_template_collection::Column::TemplateId.eq(id))
        .exec(db)
        .await;
    
    if let Some(template) = ppt_template_market::Entity::find_by_id(id)
        .one(db)
        .await
        .ok()
        .flatten()
    {
        let mut template: ppt_template_market::ActiveModel = template.into();
        template.collection_count = Set((template.collection_count.unwrap() - 1).max(0));
        let _ = template.update(db).await;
    }
    
    ApiResponse::ok("取消收藏成功")
}

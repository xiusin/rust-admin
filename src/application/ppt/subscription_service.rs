use crate::service::prelude::*;
use crate::domain::entity::{ppt_subscription_plan, ppt_user_subscription, ppt_project};
use crate::domain::model::m_ppt_subscription::*;
use crate::domain::args::a_ppt_subscription::*;
use crate::common::error::{Error, Result};

pub async fn list_plans(
    page: PageParams,
    search: SubscriptionPlanListArgs,
) -> Result<ListData<SubscriptionPlanListItem>> {
    let db = DB().await;
    
    let page_num = page.page_num.unwrap_or(1);
    let page_size = page.page_size.unwrap_or(10);
    
    let mut conditions = Condition::all()
        .add(ppt_subscription_plan::Column::DeletedAt.is_null());
    
    if let Some(ref name) = search.name {
        conditions = conditions.add(ppt_subscription_plan::Column::Name.contains(name));
    }
    if let Some(ref code) = search.code {
        conditions = conditions.add(ppt_subscription_plan::Column::Code.contains(code));
    }
    if let Some(is_active) = search.is_active {
        conditions = conditions.add(ppt_subscription_plan::Column::IsActive.eq(is_active));
    }
    
    let total = ppt_subscription_plan::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await
        .unwrap_or(0);
    
    let items = ppt_subscription_plan::Entity::find()
        .filter(conditions)
        .order_by_asc(ppt_subscription_plan::Column::SortOrder)
        .offset((page_num - 1) * page_size)
        .limit(page_size)
        .all(db)
        .await?;
    
    let list = items.into_iter().map(|item| SubscriptionPlanListItem {
        id: item.id,
        name: item.name,
        code: item.code,
        description: item.description,
        price: item.price,
        original_price: item.original_price,
        duration_days: item.duration_days,
        max_projects: item.max_projects,
        ai_credits: item.ai_credits,
        features: item.features,
        is_active: item.is_active,
        sort_order: item.sort_order,
        created_at: item.created_at,
    }).collect();
    
    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size - 1) / page_size,
        page_num,
    })
}

pub async fn get_plan_detail(plan_id: i64) -> Result<SubscriptionPlanDetail> {
    let db = DB().await;
    
    let plan = ppt_subscription_plan::Entity::find_by_id(plan_id)
        .filter(ppt_subscription_plan::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("套餐不存在"))?;
    
    Ok(SubscriptionPlanDetail {
        id: plan.id,
        name: plan.name,
        code: plan.code,
        description: plan.description,
        price: plan.price,
        original_price: plan.original_price,
        duration_days: plan.duration_days,
        max_projects: plan.max_projects,
        ai_credits: plan.ai_credits,
        features: plan.features,
        is_active: plan.is_active,
        sort_order: plan.sort_order,
        created_at: plan.created_at,
        updated_at: plan.updated_at,
    })
}

pub async fn get_active_plans() -> Result<Vec<SubscriptionPlanListItem>> {
    let db = DB().await;
    
    let items = ppt_subscription_plan::Entity::find()
        .filter(ppt_subscription_plan::Column::IsActive.eq(1))
        .filter(ppt_subscription_plan::Column::DeletedAt.is_null())
        .order_by_asc(ppt_subscription_plan::Column::SortOrder)
        .all(db)
        .await?;
    
    Ok(items.into_iter().map(|item| SubscriptionPlanListItem {
        id: item.id,
        name: item.name,
        code: item.code,
        description: item.description,
        price: item.price,
        original_price: item.original_price,
        duration_days: item.duration_days,
        max_projects: item.max_projects,
        ai_credits: item.ai_credits,
        features: item.features,
        is_active: item.is_active,
        sort_order: item.sort_order,
        created_at: item.created_at,
    }).collect())
}

pub async fn get_user_subscription(user_id: i64) -> Result<Option<UserSubscriptionInfo>> {
    let db = DB().await;
    
    let now = Local::now().naive_utc();
    
    let subscription = ppt_user_subscription::Entity::find()
        .filter(ppt_user_subscription::Column::UserId.eq(user_id))
        .filter(ppt_user_subscription::Column::DeletedAt.is_null())
        .filter(ppt_user_subscription::Column::Status.eq("active"))
        .filter(ppt_user_subscription::Column::ExpiresAt.gt(now))
        .one(db)
        .await?;
    
    match subscription {
        Some(sub) => {
            let plan = ppt_subscription_plan::Entity::find_by_id(sub.plan_id)
                .one(db)
                .await?
                .ok_or_else(|| Error::not_found("套餐不存在"))?;
            
            let days_remaining = (sub.expires_at - now).num_days().max(0);
            
            Ok(Some(UserSubscriptionInfo {
                id: sub.id,
                user_id: sub.user_id,
                plan_id: sub.plan_id,
                plan_name: plan.name,
                plan_code: plan.code,
                order_id: sub.order_id,
                started_at: sub.started_at,
                expires_at: sub.expires_at,
                ai_credits_total: sub.ai_credits_total,
                ai_credits_used: sub.ai_credits_used,
                ai_credits_remaining: sub.ai_credits_remaining,
                projects_count: sub.projects_count,
                max_projects: plan.max_projects,
                auto_renew: sub.auto_renew,
                status: sub.status,
                is_active: true,
                days_remaining,
                created_at: sub.created_at,
            }))
        }
        None => Ok(None),
    }
}

pub async fn check_feature_access(user_id: i64, feature: &str) -> Result<FeatureAccess> {
    let db = DB().await;
    
    if let Some(subscription) = get_user_subscription(user_id).await? {
        let plan = ppt_subscription_plan::Entity::find_by_id(subscription.plan_id)
            .one(db)
            .await?
            .ok_or_else(|| Error::not_found("套餐不存在"))?;
        
        if let Some(features) = plan.features {
            if let Some(obj) = features.as_object() {
                if obj.contains_key(feature) || obj.contains_key("all_features") {
                    return Ok(FeatureAccess {
                        feature: feature.to_string(),
                        has_access: true,
                        reason: None,
                    });
                }
            }
        }
        
        Ok(FeatureAccess {
            feature: feature.to_string(),
            has_access: false,
            reason: Some("当前套餐不包含此功能".to_string()),
        })
    } else {
        let free_plan = ppt_subscription_plan::Entity::find()
            .filter(ppt_subscription_plan::Column::Code.eq("free"))
            .filter(ppt_subscription_plan::Column::DeletedAt.is_null())
            .one(db)
            .await?;
        
        if let Some(plan) = free_plan {
            if let Some(features) = plan.features {
                if let Some(obj) = features.as_object() {
                    if obj.contains_key(feature) {
                        return Ok(FeatureAccess {
                            feature: feature.to_string(),
                            has_access: true,
                            reason: None,
                        });
                    }
                }
            }
        }
        
        Ok(FeatureAccess {
            feature: feature.to_string(),
            has_access: false,
            reason: Some("请先订阅套餐".to_string()),
        })
    }
}

pub async fn check_project_limit(user_id: i64) -> Result<bool> {
    let db = DB().await;
    
    if let Some(subscription) = get_user_subscription(user_id).await? {
        if subscription.max_projects == -1 {
            return Ok(true);
        }
        
        let project_count = ppt_project::Entity::find()
            .filter(ppt_project::Column::UserId.eq(user_id))
            .count(db)
            .await
            .unwrap_or(0) as i32;
        
        Ok(project_count < subscription.max_projects)
    } else {
        let project_count = ppt_project::Entity::find()
            .filter(ppt_project::Column::UserId.eq(user_id))
            .count(db)
            .await
            .unwrap_or(0) as i32;
        
        Ok(project_count < 3)
    }
}

pub async fn increment_project_count(user_id: i64) -> Result<()> {
    let db = DB().await;
    
    if let Some(subscription) = get_user_subscription(user_id).await? {
        let sub_model = ppt_user_subscription::Entity::find_by_id(subscription.id)
            .one(db)
            .await?
            .ok_or_else(|| Error::not_found("订阅不存在"))?;
        
        let mut active_model: ppt_user_subscription::ActiveModel = sub_model.into();
        active_model.projects_count = sea_orm::ActiveValue::Set(subscription.projects_count + 1);
        active_model.updated_at = sea_orm::ActiveValue::Set(Some(Local::now().naive_utc()));
        
        active_model.update(db).await?;
    }
    
    Ok(())
}

pub async fn add_plan(args: SubscriptionPlanAddArgs) -> Result<SubscriptionPlanDetail> {
    let db = DB().await;
    
    let existing = ppt_subscription_plan::Entity::find()
        .filter(ppt_subscription_plan::Column::Code.eq(&args.code))
        .filter(ppt_subscription_plan::Column::DeletedAt.is_null())
        .one(db)
        .await?;
    
    if existing.is_some() {
        return Err(Error::conflict("套餐编码已存在"));
    }
    
    let now = Local::now().naive_utc();
    let id = GID().await;
    
    let plan = ppt_subscription_plan::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
        name: sea_orm::ActiveValue::Set(args.name),
        code: sea_orm::ActiveValue::Set(args.code),
        description: sea_orm::ActiveValue::Set(args.description),
        price: sea_orm::ActiveValue::Set(args.price),
        original_price: sea_orm::ActiveValue::Set(args.original_price),
        duration_days: sea_orm::ActiveValue::Set(args.duration_days),
        max_projects: sea_orm::ActiveValue::Set(args.max_projects.unwrap_or(-1)),
        ai_credits: sea_orm::ActiveValue::Set(args.ai_credits),
        features: sea_orm::ActiveValue::Set(args.features),
        is_active: sea_orm::ActiveValue::Set(1),
        sort_order: sea_orm::ActiveValue::Set(args.sort_order.unwrap_or(0)),
        created_at: sea_orm::ActiveValue::Set(Some(now)),
        updated_at: sea_orm::ActiveValue::Set(Some(now)),
        deleted_at: sea_orm::ActiveValue::Set(None),
    };
    
    let plan = plan.insert(db).await?;
    
    Ok(SubscriptionPlanDetail {
        id: plan.id,
        name: plan.name,
        code: plan.code,
        description: plan.description,
        price: plan.price,
        original_price: plan.original_price,
        duration_days: plan.duration_days,
        max_projects: plan.max_projects,
        ai_credits: plan.ai_credits,
        features: plan.features,
        is_active: plan.is_active,
        sort_order: plan.sort_order,
        created_at: plan.created_at,
        updated_at: plan.updated_at,
    })
}

pub async fn edit_plan(args: SubscriptionPlanEditArgs) -> Result<SubscriptionPlanDetail> {
    let db = DB().await;
    
    let plan = ppt_subscription_plan::Entity::find_by_id(args.id)
        .filter(ppt_subscription_plan::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("套餐不存在"))?;
    
    let now = Local::now().naive_utc();
    let mut active_model: ppt_subscription_plan::ActiveModel = plan.into();
    
    if let Some(name) = args.name {
        active_model.name = sea_orm::ActiveValue::Set(name);
    }
    if let Some(description) = args.description {
        active_model.description = sea_orm::ActiveValue::Set(Some(description));
    }
    if let Some(price) = args.price {
        active_model.price = sea_orm::ActiveValue::Set(price);
    }
    if let Some(original_price) = args.original_price {
        active_model.original_price = sea_orm::ActiveValue::Set(Some(original_price));
    }
    if let Some(duration_days) = args.duration_days {
        active_model.duration_days = sea_orm::ActiveValue::Set(duration_days);
    }
    if let Some(max_projects) = args.max_projects {
        active_model.max_projects = sea_orm::ActiveValue::Set(max_projects);
    }
    if let Some(ai_credits) = args.ai_credits {
        active_model.ai_credits = sea_orm::ActiveValue::Set(ai_credits);
    }
    if let Some(features) = args.features {
        active_model.features = sea_orm::ActiveValue::Set(Some(features));
    }
    if let Some(is_active) = args.is_active {
        active_model.is_active = sea_orm::ActiveValue::Set(is_active);
    }
    if let Some(sort_order) = args.sort_order {
        active_model.sort_order = sea_orm::ActiveValue::Set(sort_order);
    }
    
    active_model.updated_at = sea_orm::ActiveValue::Set(Some(now));
    
    let plan = active_model.update(db).await?;
    
    Ok(SubscriptionPlanDetail {
        id: plan.id,
        name: plan.name,
        code: plan.code,
        description: plan.description,
        price: plan.price,
        original_price: plan.original_price,
        duration_days: plan.duration_days,
        max_projects: plan.max_projects,
        ai_credits: plan.ai_credits,
        features: plan.features,
        is_active: plan.is_active,
        sort_order: plan.sort_order,
        created_at: plan.created_at,
        updated_at: plan.updated_at,
    })
}

pub async fn delete_plans(ids: Vec<i64>) -> Result<()> {
    let db = DB().await;
    let now = Local::now().naive_utc();
    
    for id in ids {
        let plan = ppt_subscription_plan::Entity::find_by_id(id)
            .one(db)
            .await?;
        
        if let Some(plan) = plan {
            let mut active_model: ppt_subscription_plan::ActiveModel = plan.into();
            active_model.deleted_at = sea_orm::ActiveValue::Set(Some(now));
            active_model.updated_at = sea_orm::ActiveValue::Set(Some(now));
            active_model.update(db).await?;
        }
    }
    
    Ok(())
}

pub async fn create_subscription(args: crate::domain::args::a_ppt_subscription::SubscribeArgs) -> Result<UserSubscriptionInfo> {
    let db = DB().await;
    let id = GID().await;
    let now = Local::now().naive_utc();
    
    let plan = ppt_subscription_plan::Entity::find_by_id(args.plan_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("套餐不存在"))?;
    
    let end_time = now + chrono::Duration::days(plan.duration_days as i64);
    
    let subscription = ppt_user_subscription::ActiveModel {
        id: Set(id),
        user_id: Set(args.user_id),
        plan_id: Set(args.plan_id),
        order_id: Set(None),
        status: Set("active".to_string()),
        started_at: Set(now),
        expires_at: Set(end_time),
        ai_credits_total: Set(plan.ai_credits),
        ai_credits_used: Set(0),
        ai_credits_remaining: Set(plan.ai_credits),
        projects_count: Set(0),
        auto_renew: Set(0),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        deleted_at: Set(None),
    };
    
    subscription.insert(db).await?;
    
    let days_remaining = (end_time - now).num_days();
    
    Ok(UserSubscriptionInfo {
        id,
        user_id: args.user_id,
        plan_id: args.plan_id,
        plan_name: plan.name,
        plan_code: plan.code,
        order_id: None,
        started_at: now,
        expires_at: end_time,
        ai_credits_total: plan.ai_credits,
        ai_credits_used: 0,
        ai_credits_remaining: plan.ai_credits,
        projects_count: 0,
        max_projects: plan.max_projects,
        auto_renew: 0,
        status: "active".to_string(),
        is_active: true,
        days_remaining,
        created_at: Some(now),
    })
}

pub async fn get_user_subscription_api() -> axum::response::Response {
    match get_user_subscription(0).await {
        Ok(Some(sub)) => crate::common::result::ApiResponse::success(sub).into_response(),
        Ok(None) => crate::common::result::ApiResponse::not_found("未找到订阅信息"),
        Err(e) => crate::common::result::ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn cancel_subscription_api() -> impl axum::response::IntoResponse {
    crate::common::result::ApiResponse::success("取消成功")
}

pub async fn get_subscription_history_api() -> impl axum::response::IntoResponse {
    crate::common::result::ApiResponse::success(Vec::<UserSubscriptionInfo>::new())
}

pub async fn check_limit_api() -> impl axum::response::IntoResponse {
    crate::common::result::ApiResponse::success(true)
}

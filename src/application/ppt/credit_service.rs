use crate::service::prelude::*;
use crate::domain::entity::{ppt_user_subscription, ppt_credit_record, ppt_project};
use crate::domain::model::m_ppt_subscription::*;
use crate::domain::args::a_ppt_subscription::*;
use crate::common::error::{Error, Result};

pub async fn get_balance(user_id: i64) -> Result<CreditBalance> {
    let db = DB().await;
    
    let now = Local::now().naive_utc();
    
    let subscription = ppt_user_subscription::Entity::find()
        .filter(ppt_user_subscription::Column::UserId.eq(user_id))
        .filter(ppt_user_subscription::Column::Status.eq("active"))
        .filter(ppt_user_subscription::Column::DeletedAt.is_null())
        .filter(ppt_user_subscription::Column::ExpiresAt.gt(now))
        .one(db)
        .await?;
    
    match subscription {
        Some(sub) => Ok(CreditBalance {
            total: sub.ai_credits_total,
            used: sub.ai_credits_used,
            remaining: sub.ai_credits_remaining,
            reset_at: Some(sub.expires_at),
        }),
        None => Ok(CreditBalance {
            total: 10,
            used: 0,
            remaining: 10,
            reset_at: None,
        }),
    }
}

pub async fn consume(
    user_id: i64,
    task_type: &str,
    amount: i32,
    project_id: Option<i64>,
    description: Option<String>,
) -> Result<()> {
    let db = DB().await;
    
    let now = Local::now().naive_utc();
    
    let subscription = ppt_user_subscription::Entity::find()
        .filter(ppt_user_subscription::Column::UserId.eq(user_id))
        .filter(ppt_user_subscription::Column::Status.eq("active"))
        .filter(ppt_user_subscription::Column::DeletedAt.is_null())
        .filter(ppt_user_subscription::Column::ExpiresAt.gt(now))
        .one(db)
        .await?;
    
    let (subscription_id, balance_before, balance_after) = match subscription {
        Some(sub) => {
            if sub.ai_credits_remaining < amount {
                return Err(Error::bad_request("AI额度不足"));
            }
            
            let sub_id = sub.id;
            let balance_before = sub.ai_credits_remaining;
            let balance_after = balance_before - amount;
            let credits_used = sub.ai_credits_used;
            
            let mut sub_model: ppt_user_subscription::ActiveModel = sub.into();
            sub_model.ai_credits_used = sea_orm::ActiveValue::Set(
                credits_used + amount
            );
            sub_model.ai_credits_remaining = sea_orm::ActiveValue::Set(balance_after);
            sub_model.updated_at = sea_orm::ActiveValue::Set(Some(now));
            sub_model.update(db).await?;
            
            (Some(sub_id), balance_before, balance_after)
        }
        None => {
            let balance_before = get_free_credits(user_id).await?;
            if balance_before < amount {
                return Err(Error::bad_request("AI额度不足，请升级套餐"));
            }
            let balance_after = balance_before - amount;
            (None, balance_before, balance_after)
        }
    };
    
    let id = GID().await;
    
    let record = ppt_credit_record::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
        user_id: sea_orm::ActiveValue::Set(user_id),
        subscription_id: sea_orm::ActiveValue::Set(subscription_id),
        project_id: sea_orm::ActiveValue::Set(project_id),
        task_type: sea_orm::ActiveValue::Set(task_type.to_string()),
        amount: sea_orm::ActiveValue::Set(-amount),
        balance_before: sea_orm::ActiveValue::Set(balance_before),
        balance_after: sea_orm::ActiveValue::Set(balance_after),
        source: sea_orm::ActiveValue::Set("consume".to_string()),
        description: sea_orm::ActiveValue::Set(description),
        metadata: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::Set(Some(now)),
    };
    
    record.insert(db).await?;
    
    check_and_alert(user_id, balance_after).await?;
    
    Ok(())
}

pub async fn recharge(
    user_id: i64,
    amount: i32,
    source: &str,
    subscription_id: Option<i64>,
    description: Option<String>,
) -> Result<()> {
    let db = DB().await;
    
    let now = Local::now().naive_utc();
    
    let subscription = if let Some(sub_id) = subscription_id {
        ppt_user_subscription::Entity::find_by_id(sub_id)
            .filter(ppt_user_subscription::Column::UserId.eq(user_id))
            .filter(ppt_user_subscription::Column::DeletedAt.is_null())
            .one(db)
            .await?
    } else {
        ppt_user_subscription::Entity::find()
            .filter(ppt_user_subscription::Column::UserId.eq(user_id))
            .filter(ppt_user_subscription::Column::Status.eq("active"))
            .filter(ppt_user_subscription::Column::DeletedAt.is_null())
            .filter(ppt_user_subscription::Column::ExpiresAt.gt(now))
            .one(db)
            .await?
    };
    
    let (sub_id, balance_before, balance_after) = match subscription {
        Some(sub) => {
            let sub_id = sub.id;
            let balance_before = sub.ai_credits_remaining;
            let balance_after = balance_before + amount;
            let credits_total = sub.ai_credits_total;
            
            let mut sub_model: ppt_user_subscription::ActiveModel = sub.into();
            sub_model.ai_credits_total = sea_orm::ActiveValue::Set(
                credits_total + amount
            );
            sub_model.ai_credits_remaining = sea_orm::ActiveValue::Set(balance_after);
            sub_model.updated_at = sea_orm::ActiveValue::Set(Some(now));
            sub_model.update(db).await?;
            
            (Some(sub_id), balance_before, balance_after)
        }
        None => {
            let balance_before = get_free_credits(user_id).await?;
            let balance_after = balance_before + amount;
            (None, balance_before, balance_after)
        }
    };
    
    let id = GID().await;
    
    let record = ppt_credit_record::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
        user_id: sea_orm::ActiveValue::Set(user_id),
        subscription_id: sea_orm::ActiveValue::Set(sub_id),
        project_id: sea_orm::ActiveValue::Set(None),
        task_type: sea_orm::ActiveValue::Set("recharge".to_string()),
        amount: sea_orm::ActiveValue::Set(amount),
        balance_before: sea_orm::ActiveValue::Set(balance_before),
        balance_after: sea_orm::ActiveValue::Set(balance_after),
        source: sea_orm::ActiveValue::Set(source.to_string()),
        description: sea_orm::ActiveValue::Set(description),
        metadata: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::Set(Some(now)),
    };
    
    record.insert(db).await?;
    
    Ok(())
}

pub async fn get_usage_history(
    page: PageParams,
    args: CreditRecordListArgs,
) -> Result<ListData<CreditUsageRecord>> {
    let db = DB().await;
    
    let page_num = page.page_num.unwrap_or(1);
    let page_size = page.page_size.unwrap_or(10);
    
    let mut conditions = Condition::all()
        .add(ppt_credit_record::Column::UserId.eq(args.user_id));
    
    if let Some(ref task_type) = args.task_type {
        conditions = conditions.add(ppt_credit_record::Column::TaskType.eq(task_type));
    }
    if let Some(ref source) = args.source {
        conditions = conditions.add(ppt_credit_record::Column::Source.eq(source));
    }
    if let Some(project_id) = args.project_id {
        conditions = conditions.add(ppt_credit_record::Column::ProjectId.eq(project_id));
    }
    
    let total = ppt_credit_record::Entity::find()
        .filter(conditions.clone())
        .count(db)
        .await
        .unwrap_or(0);
    
    let items = ppt_credit_record::Entity::find()
        .filter(conditions)
        .order_by_desc(ppt_credit_record::Column::CreatedAt)
        .offset((page_num - 1) * page_size)
        .limit(page_size)
        .all(db)
        .await?;
    
    let mut list = Vec::new();
    for item in items {
        let project_title = if let Some(project_id) = item.project_id {
            ppt_project::Entity::find_by_id(project_id)
                .one(db)
                .await?
                .map(|p| p.title)
        } else {
            None
        };
        
        list.push(CreditUsageRecord {
            id: item.id,
            user_id: item.user_id,
            subscription_id: item.subscription_id,
            project_id: item.project_id,
            project_title,
            task_type: item.task_type,
            amount: item.amount,
            balance_before: item.balance_before,
            balance_after: item.balance_after,
            source: item.source,
            description: item.description,
            created_at: item.created_at,
        });
    }
    
    Ok(ListData {
        list,
        total,
        total_pages: (total + page_size - 1) / page_size,
        page_num,
    })
}

async fn get_free_credits(user_id: i64) -> Result<i32> {
    let db = DB().await;
    
    let total_used: i32 = ppt_credit_record::Entity::find()
        .filter(ppt_credit_record::Column::UserId.eq(user_id))
        .filter(ppt_credit_record::Column::Amount.lt(0))
        .column(ppt_credit_record::Column::Amount)
        .into_tuple::<i32>()
        .all(db)
        .await?
        .iter()
        .map(|v| v.abs())
        .sum();
    
    Ok(10 - total_used)
}

async fn check_and_alert(user_id: i64, remaining: i32) -> Result<()> {
    if remaining <= 5 && remaining > 0 {
        info!("用户 {} AI额度即将用尽，剩余: {}", user_id, remaining);
    } else if remaining == 0 {
        info!("用户 {} AI额度已用尽", user_id);
    }
    Ok(())
}

pub async fn check_credit_sufficient(user_id: i64, required: i32) -> Result<bool> {
    let balance = get_balance(user_id).await?;
    Ok(balance.remaining >= required)
}

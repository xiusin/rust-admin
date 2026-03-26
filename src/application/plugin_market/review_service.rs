use crate::domain::entity::p_plugin_review;
use crate::domain::entity::p_plugin_review::Entity as ReviewEntity;
use crate::domain::entity::p_plugin;
use crate::domain::entity::p_plugin::Entity as PluginEntity;
use crate::common::error::Error;
use crate::infrastructure::db::DB;
use sea_orm::*;
use sea_orm::prelude::Expr;
use serde::{Deserialize, Serialize};

pub async fn create(user_id: i64, params: CreateReviewParams) -> Result<i64, Error> {
    let db = DB().await;

    if params.rating < 1 || params.rating > 5 {
        return Err(Error::bad_request("评分必须在1-5之间"));
    }

    let existing = ReviewEntity::find()
        .filter(p_plugin_review::Column::UserId.eq(user_id))
        .filter(p_plugin_review::Column::PluginId.eq(params.plugin_id))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(Error::bad_request("已评价过该插件"));
    }

    let max_id: Option<i64> = ReviewEntity::find()
        .select_only()
        .column_as(p_plugin_review::Column::Id.max(), "max_id")
        .into_tuple()
        .one(db)
        .await?;

    let new_id = max_id.unwrap_or(0) + 1;

    let review = p_plugin_review::ActiveModel {
        id: Set(new_id),
        plugin_id: Set(params.plugin_id),
        user_id: Set(user_id),
        rating: Set(params.rating),
        content: Set(Some(params.content)),
        reply_content: Set(None),
        reply_time: Set(None),
        status: Set(1),
        created_at: Set(Some(chrono::Utc::now().naive_utc())),
    };

    review.insert(db).await?;

    update_plugin_rating(params.plugin_id).await?;

    Ok(new_id)
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewParams {
    pub plugin_id: i64,
    pub rating: i32,
    pub content: String,
}

pub async fn list(plugin_id: i64, page_num: u32, page_size: u32) -> Result<(Vec<ReviewItem>, i64), Error> {
    let db = DB().await;
    let offset = (page_num - 1) * page_size;

    let total = ReviewEntity::find()
        .filter(p_plugin_review::Column::PluginId.eq(plugin_id))
        .filter(p_plugin_review::Column::Status.eq(1))
        .count(db)
        .await?;

    let reviews = ReviewEntity::find()
        .filter(p_plugin_review::Column::PluginId.eq(plugin_id))
        .filter(p_plugin_review::Column::Status.eq(1))
        .order_by_desc(p_plugin_review::Column::CreatedAt)
        .offset(offset as u64)
        .limit(page_size as u64)
        .all(db)
        .await?;

    let items: Vec<ReviewItem> = reviews
        .into_iter()
        .map(|r| ReviewItem {
            id: r.id,
            plugin_id: r.plugin_id,
            user_id: r.user_id,
            user_name: None,
            user_avatar: None,
            rating: r.rating,
            content: r.content,
            reply_content: r.reply_content,
            reply_time: r.reply_time,
            status: r.status,
            created_at: r.created_at,
        })
        .collect();

    Ok((items, total as i64))
}

#[derive(Debug, Serialize, Clone)]
pub struct ReviewItem {
    pub id: i64,
    pub plugin_id: i64,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub user_avatar: Option<String>,
    pub rating: i32,
    pub content: Option<String>,
    pub reply_content: Option<String>,
    pub reply_time: Option<chrono::NaiveDateTime>,
    pub status: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
}

pub async fn reply(review_id: i64, reply_content: String) -> Result<(), Error> {
    let db = DB().await;

    let review = ReviewEntity::find_by_id(review_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::not_found("评论不存在"))?;

    let mut active_model: p_plugin_review::ActiveModel = review.into();
    active_model.reply_content = Set(Some(reply_content));
    active_model.reply_time = Set(Some(chrono::Utc::now().naive_utc()));
    active_model.update(db).await?;

    Ok(())
}

pub async fn get_statistics(plugin_id: i64) -> Result<ReviewStatistics, Error> {
    let db = DB().await;

    let total = ReviewEntity::find()
        .filter(p_plugin_review::Column::PluginId.eq(plugin_id))
        .filter(p_plugin_review::Column::Status.eq(1))
        .count(db)
        .await?;

    let reviews = ReviewEntity::find()
        .filter(p_plugin_review::Column::PluginId.eq(plugin_id))
        .filter(p_plugin_review::Column::Status.eq(1))
        .all(db)
        .await?;

    let mut total_rating: i32 = 0;
    let mut rating_dist = RatingDistribution {
        five_star: 0,
        four_star: 0,
        three_star: 0,
        two_star: 0,
        one_star: 0,
    };

    for review in reviews {
        total_rating += review.rating;
        match review.rating {
            5 => rating_dist.five_star += 1,
            4 => rating_dist.four_star += 1,
            3 => rating_dist.three_star += 1,
            2 => rating_dist.two_star += 1,
            1 => rating_dist.one_star += 1,
            _ => {}
        }
    }

    let average_rating = if total > 0 {
        total_rating as f64 / total as f64
    } else {
        5.0
    };

    Ok(ReviewStatistics {
        total: total as i64,
        average_rating,
        rating_distribution: rating_dist,
    })
}

#[derive(Debug, Serialize)]
pub struct ReviewStatistics {
    pub total: i64,
    pub average_rating: f64,
    pub rating_distribution: RatingDistribution,
}

#[derive(Debug, Serialize)]
pub struct RatingDistribution {
    pub five_star: i64,
    pub four_star: i64,
    pub three_star: i64,
    pub two_star: i64,
    pub one_star: i64,
}

async fn update_plugin_rating(plugin_id: i64) -> Result<(), Error> {
    let db = DB().await;

    let stats = get_statistics(plugin_id).await?;

    PluginEntity::update_many()
        .col_expr(
            p_plugin::Column::Rating,
            Expr::value(rust_decimal::Decimal::from_str_exact(
                &format!("{:.2}", stats.average_rating)
            ).unwrap_or_default())
        )
        .filter(p_plugin::Column::Id.eq(plugin_id))
        .exec(db)
        .await?;

    Ok(())
}
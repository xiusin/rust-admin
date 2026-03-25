use serde::{Deserialize, Serialize};
use crate::model::prelude::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ReviewItem {
    pub id: i64,
    pub plugin_id: i64,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub user_avatar: Option<String>,
    pub rating: i32,
    pub content: Option<String>,
    pub reply_content: Option<String>,
    pub reply_time: Option<DateTime>,
    pub status: i32,
    pub created_at: Option<DateTime>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ReviewStatistics {
    pub total: i64,
    pub average_rating: f64,
    pub rating_distribution: RatingDistribution,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct RatingDistribution {
    pub five_star: i64,
    pub four_star: i64,
    pub three_star: i64,
    pub two_star: i64,
    pub one_star: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewParams {
    pub plugin_id: i64,
    pub rating: i32,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ReplyReviewParams {
    pub review_id: i64,
    pub reply_content: String,
}
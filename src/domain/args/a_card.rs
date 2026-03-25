use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GenerateCardsParams {
    pub plugin_id: i64,
    pub plan_id: i64,
    pub count: i32,
    pub price: f64,
    pub expire_days: i32,
}

#[derive(Debug, Deserialize)]
pub struct RedeemCardParams {
    pub card_no: String,
    pub card_pwd: String,
}

#[derive(Debug, Deserialize)]
pub struct FreezeCardParams {
    pub card_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UnfreezeCardParams {
    pub card_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct CardSearchParams {
    pub batch_id: Option<i64>,
    pub plugin_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub status: Option<i32>,
    pub keyword: Option<String>,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct BatchSearchParams {
    pub plugin_id: Option<i64>,
    pub status: Option<i32>,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}
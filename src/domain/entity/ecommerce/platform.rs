use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcommercePlatform {
    pub id: i64,
    pub platform_type: String,
    pub name: String,
    pub app_key: String,
    pub app_secret: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl EcommercePlatform {
    pub fn new(
        platform_type: String,
        name: String,
        app_key: String,
        app_secret: String,
    ) -> Self {
        let now = chrono::Local::now().naive_local();
        Self {
            id: 0,
            platform_type,
            name,
            app_key,
            app_secret,
            access_token: None,
            refresh_token: None,
            status: 1,
            created_at: now,
            updated_at: now,
        }
    }
}

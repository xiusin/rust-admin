use serde::{Deserialize, Serialize};
use sea_orm::FromQueryResult;

#[derive(Debug, Deserialize, Serialize)]
pub struct PermissionCategoryAdd {
    pub name: String,
    pub code: String,
    pub sort: i32,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PermissionCategoryEdit {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub sort: i32,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PermissionCategorySearch {
    pub name: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromQueryResult)]
pub struct PermissionCategoryRes {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub sort: i32,
    pub remark: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

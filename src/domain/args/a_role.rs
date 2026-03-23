use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct RoleSearch {
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub status: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Validate, Serialize)]
pub struct RoleAddReq {
    pub role_name: String,
    pub role_key: String,
    pub order: i32,
    pub status: String,
    #[serde(default)]
    pub data_scope: Option<String>,
    pub remark: Option<String>,
}

//菜单
#[derive(Deserialize, Validate, Serialize, Debug)]
pub struct RoleEditReq {
    pub role_id: i64,
    pub role_name: String,
    pub role_key: String,
    pub order: i32,
    #[serde(default)]
    pub data_scope: Option<String>,
    pub status: String,
    pub remark: Option<String>,
    #[serde(default)]
    pub menu: Option<Vec<i64>>,

    #[serde(default)]
    pub data_depts: Option<Vec<i64>>,
}

#[derive(Deserialize, Validate, Serialize)]
pub struct RoleReq {
    pub role_id: i64,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct RoleMenuAssignReq {
    pub role_id: i64,
    pub menu_ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone, Default)]
pub struct RoleMenuResp {
    pub role_id: i64,
    pub role_name: String,
    pub role_key: String,
    pub order: i32,
    pub data_scope: String,
    pub status: String,
    #[sea_orm(skip)]
    pub mens: Vec<RoleMResp>,
}
#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone, Default)]
pub struct RoleMResp {
    pub id: i64,
    pub title: String,
}
#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone, Default)]
pub struct RoleResp {
    pub role_id: i64,
    pub role_name: String,
    pub role_key: String,
    pub order: i32,
    pub data_scope: String,
    pub status: String,
    pub remark: Option<String>,
    #[sea_orm(skip)]
    pub admin: Option<bool>,
    #[sea_orm(skip)]
    pub menus: Vec<RoleMenuInfo>,
    pub created_by: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_by: Option<String>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct RoleMenuInfo {
    pub menu_id: i64,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone, Default)]
pub struct RoleDeptResp {
    pub dept_id: i64
}

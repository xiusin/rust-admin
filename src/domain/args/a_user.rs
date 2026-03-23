use crate::model::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct UserSearch {
    #[serde(default)]
    pub dept_id: i64,
    pub user_name: Option<String>,
    pub phonenumber: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct LoginParams {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Clone, Deserialize, Validate, Debug)]
pub struct ResetPasswordParams {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, Clone, Deserialize, Validate, Debug)]
pub struct ChangePasswordParams {
    pub uid: i64,
    pub new_password: String,
}
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct SysUserRes {
    pub id: i64,
    pub dept_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    pub user_name: String,
    pub nick_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonenumber: Option<String>,
    pub sex: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    pub roles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
    pub admin: bool,
}
#[derive(Serialize, Clone, Deserialize, Debug, FromQueryResult)]
pub struct SysUserAndRole {
    pub dept_id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub user_type: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
    pub role_id: i64,
    pub role_name: String,
    pub role_key: String,
    pub order: i32,
    pub data_scope: Option<String>,
    pub menu_check_strictly: Option<String>,
    pub dept_check_strictly: Option<String>,
}

#[derive(Serialize, Clone, Deserialize, Debug, Validate)]
pub struct SysUserEdit {
    pub id: i64,
    pub dept_ids: Vec<i64>,
    pub dept_id: i64,
    pub role_ids: Vec<i64>,
    pub role_id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub user_type: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

#[derive(Serialize, Clone, Deserialize, Debug, Validate)]
pub struct SysUserAdd {
    pub dept_ids: Vec<i64>,
    pub dept_id: i64,
    pub role_ids: Vec<i64>,
    pub role_id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
    pub user_type: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

#[derive(Serialize, Clone, Deserialize)]

pub struct UserInfoDetail {
    pub username: String,
    pub nickname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub did: i64,
    pub rid: i64,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct UserInfoRes {
    pub username: String,
    pub nickname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub did: i64,
    pub rid: i64,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct DeptsAndRoles {
    pub depts: Vec<i64>,
    pub roles: Vec<i64>,
}

#[derive(Serialize, Clone, Deserialize, Debug, Validate)]
pub struct UserId {
    pub uid: i64,
}

#[derive(Serialize, Clone, Deserialize, Debug, Validate)]
pub struct UserIds {
    pub uids: Vec<i64>,
}

#[derive(Serialize, Clone, Deserialize, Debug, Validate)]
pub struct UserAvatarEdit {
    pub avatar: String,
}

#[derive(Serialize, Clone, Deserialize, Validate)]
pub struct DeptAndRole {
    pub rid: i64,
    pub did: i64,
}

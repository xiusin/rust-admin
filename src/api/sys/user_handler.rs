use qiluo_macros::handler;

use crate::common::ApiResponse;
use crate::common::error::Result;
use crate::domain::args::a_user::*;
use crate::model::prelude::{VQuery, PageParams, ListData};
use axum::extract::Path;
use axum::Json;

#[handler(
    method = "GET",
    path = "/api/sys/user/list",
    tag = "sys_user",
    summary = "获取用户列表",
    required = true,
    roles = ["admin"],
    operation = "查询用户列表"
)]
pub async fn list(
    VQuery(_page): VQuery<PageParams>,
    VQuery(_search): VQuery<UserSearch>,
) -> Result<ApiResponse<ListData<SysUserRes>>> {
    todo!()
}

#[handler(
    method = "GET",
    path = "/api/sys/user/{id}",
    tag = "sys_user",
    summary = "获取用户详情",
    required = true,
    permissions = ["user:read"],
    operation = "查看用户详情"
)]
pub async fn get_user(
    Path(_id): Path<i64>,
) -> Result<ApiResponse<SysUserRes>> {
    todo!()
}

#[handler(
    method = "POST",
    path = "/api/sys/user/add",
    tag = "sys_user",
    summary = "创建用户",
    required = true,
    roles = ["admin"],
    operation = "创建用户"
)]
pub async fn create(
    Json(_body): Json<SysUserAdd>,
) -> Result<ApiResponse<SysUserRes>> {
    todo!()
}

#[handler(
    method = "PUT",
    path = "/api/sys/user/edit",
    tag = "sys_user",
    summary = "更新用户",
    required = true,
    roles = ["admin"],
    operation = "更新用户"
)]
pub async fn update(
    Json(_body): Json<SysUserEdit>,
) -> Result<ApiResponse<SysUserRes>> {
    todo!()
}

#[handler(
    method = "DELETE",
    path = "/api/sys/user/del",
    tag = "sys_user",
    summary = "删除用户",
    required = true,
    roles = ["admin"],
    operation = "删除用户",
    requests = 10,
    period = "1m"
)]
pub async fn delete(
    Json(_body): Json<UserIds>,
) -> Result<ApiResponse<String>> {
    todo!()
}

pub fn init_user_handlers() {
    std::sync::LazyLock::force(&__REG_INIT_list);
    std::sync::LazyLock::force(&__REG_INIT_get_user);
    std::sync::LazyLock::force(&__REG_INIT_create);
    std::sync::LazyLock::force(&__REG_INIT_update);
    std::sync::LazyLock::force(&__REG_INIT_delete);
}
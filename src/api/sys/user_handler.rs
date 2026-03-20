use qiluo_macros::{route, auth, log, rate_limit};

use crate::common::ApiResponse;
use crate::common::error::Result;
use crate::core::{ROUTE_REGISTRY, RouteEntry};
use crate::domain::args::a_user::*;
use crate::model::prelude::{VQuery, PageParams, ListData};
use axum::extract::Path;
use axum::Json;

#[route(method = "GET", path = "/api/sys/user/list", tag = "sys_user", summary = "获取用户列表")]
#[auth(required = true, roles = ["admin"])]
#[log(operation = "查询用户列表")]
pub async fn list(
    VQuery(_page): VQuery<PageParams>,
    VQuery(_search): VQuery<UserSearch>,
) -> Result<ApiResponse<ListData<SysUserRes>>> {
    todo!()
}

#[route(method = "GET", path = "/api/sys/user/{id}", tag = "sys_user", summary = "获取用户详情")]
#[auth(required = true, permissions = ["user:read"])]
#[log(operation = "查看用户详情")]
pub async fn get_user(
    Path(_id): Path<i64>,
) -> Result<ApiResponse<SysUserRes>> {
    todo!()
}

#[route(method = "POST", path = "/api/sys/user/add", tag = "sys_user", summary = "创建用户")]
#[auth(required = true, roles = ["admin"])]
#[log(operation = "创建用户")]
pub async fn create(
    Json(_body): Json<SysUserAdd>,
) -> Result<ApiResponse<SysUserRes>> {
    todo!()
}

#[route(method = "PUT", path = "/api/sys/user/edit", tag = "sys_user", summary = "更新用户")]
#[auth(required = true, roles = ["admin"])]
#[log(operation = "更新用户")]
pub async fn update(
    Json(_body): Json<SysUserEdit>,
) -> Result<ApiResponse<SysUserRes>> {
    todo!()
}

#[route(method = "DELETE", path = "/api/sys/user/del", tag = "sys_user", summary = "删除用户")]
#[auth(required = true, roles = ["admin"])]
#[log(operation = "删除用户")]
#[rate_limit(requests = 10, period = "1m")]
pub async fn delete(
    Json(_body): Json<UserIds>,
) -> Result<ApiResponse<String>> {
    todo!()
}

pub fn register_routes() {
    ROUTE_REGISTRY.register("list", RouteEntry {
        info: __route_meta_list(),
        auth: Some(__auth_meta_list()),
        log: Some(__log_meta_list()),
        rate_limit: None,
    });

    ROUTE_REGISTRY.register("get_user", RouteEntry {
        info: __route_meta_get_user(),
        auth: Some(__auth_meta_get_user()),
        log: Some(__log_meta_get_user()),
        rate_limit: None,
    });

    ROUTE_REGISTRY.register("create", RouteEntry {
        info: __route_meta_create(),
        auth: Some(__auth_meta_create()),
        log: Some(__log_meta_create()),
        rate_limit: None,
    });

    ROUTE_REGISTRY.register("update", RouteEntry {
        info: __route_meta_update(),
        auth: Some(__auth_meta_update()),
        log: Some(__log_meta_update()),
        rate_limit: None,
    });

    ROUTE_REGISTRY.register("delete", RouteEntry {
        info: __route_meta_delete(),
        auth: Some(__auth_meta_delete()),
        log: Some(__log_meta_delete()),
        rate_limit: Some(__rate_limit_meta_delete()),
    });
}
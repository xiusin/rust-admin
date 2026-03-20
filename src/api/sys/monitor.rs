use axum::routing::{delete, get, post};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::sys::*;

pub fn sys_login_info() -> WebPath {
    WebPath::new().route(
        "/list",
        WebPathType::Get,
        Some("获取登录日志列表"),
        get(s_sys_login_info::list),
    )
}

pub fn sys_user_online() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取在线用户列表"),
            get(s_sys_white_jwt::list),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("退出在线用户"),
            delete(s_sys_white_jwt::delete),
        )
}

pub fn sys_operation_log() -> WebPath {
    WebPath::new().route(
        "/list",
        WebPathType::Get,
        Some("获取操作日志列表"),
        get(s_sys_operation_log::list),
    )
}

pub fn sys_cache() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取缓存列表"),
            get(s_sys_cache::list),
        )
        .route(
            "/clear",
            WebPathType::Post,
            Some("清空缓存"),
            post(s_sys_cache::clear),
        )
}
use axum::routing::{get, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::sys::*;

pub fn sys_api_permission() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取api权限列表"),
            get(s_sys_api_permission::list),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑api权限"),
            put(s_sys_api_permission::edit),
        )
}
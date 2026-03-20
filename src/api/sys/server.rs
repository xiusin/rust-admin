use axum::routing::get;
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::sys::*;

pub fn sys_server_info() -> WebPath {
    WebPath::new().route(
        "/server_update",
        WebPathType::Get,
        Some("更新服务器信息"),
        get(s_sys_server_info::server_event),
    )
}
use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::sys::*;

pub fn menu() -> WebPath {
    WebPath::new()
        .route(
            "/all_router",
            WebPathType::Get,
            Some("全部路由"),
            get(s_sys_menu::all_router),
        )
        .route(
            "/list",
            WebPathType::Get,
            Some("菜单列表"),
            get(s_sys_menu::list),
        )
        .route(
            "/tree",
            WebPathType::Get,
            Some("获取菜单树"),
            get(s_sys_menu::tree),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑菜单"),
            put(s_sys_menu::edit),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加菜单"),
            post(s_sys_menu::add),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除菜单"),
            delete(s_sys_menu::delete),
        )
}
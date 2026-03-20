use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::sys::*;

pub fn sys_dept() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取部门列表"),
            get(s_sys_dept::list_tree),
        )
        .route(
            "/dept_tree",
            WebPathType::Get,
            Some("获取部门树"),
            get(s_sys_dept::dept_tree),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑部门"),
            put(s_sys_dept::edit),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加部门"),
            post(s_sys_dept::add),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除部门"),
            delete(s_sys_dept::delete),
        )
        .route(
            "/user_dept_name_list",
            WebPathType::Get,
            Some("获取用户拥有的部门名称列表"),
            get(s_sys_user_dept::user_dept_name_list),
        )
}
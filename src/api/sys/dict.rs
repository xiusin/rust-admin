use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::sys::*;

pub fn sys_dict_data() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取字典数据列表"),
            get(s_sys_dict_data::list),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加字典数据"),
            post(s_sys_dict_data::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑字典数据"),
            put(s_sys_dict_data::edit),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除字典数据"),
            delete(s_sys_dict_data::delete),
        )
        .route(
            "/get_by_type",
            WebPathType::Get,
            Some("根据类型获取字典数据"),
            get(s_sys_dict_data::get_by_type),
        )
}

pub fn sys_dict_type() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取字典类型列表"),
            get(s_sys_dict_type::list),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加字典类型"),
            post(s_sys_dict_type::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑字典类型"),
            put(s_sys_dict_type::edit),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除字典类型"),
            delete(s_sys_dict_type::delete),
        )
}
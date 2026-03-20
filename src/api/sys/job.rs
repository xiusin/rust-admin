use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::sys::*;

pub fn sys_job() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取定时任务列表"),
            get(s_sys_job::list),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加定时任务"),
            post(s_sys_job::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑定时任务"),
            put(s_sys_job::edit),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除定时任务"),
            delete(s_sys_job::delete),
        )
        .route(
            "/validate_cron",
            WebPathType::Post,
            Some("验证cron表达式"),
            post(s_sys_job::validate_cron),
        )
        .route(
            "/hand_execute_job",
            WebPathType::Post,
            Some("执行定时任务"),
            post(s_sys_job::hand_execute_job),
        )
}

pub fn sys_job_info() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取定时任务日志列表"),
            get(s_sys_job_log::list),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加定时任务日志"),
            post(s_sys_job_log::add),
        )
}
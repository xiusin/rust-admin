use axum::routing::{delete, get, post, put};
use crate::api::web_path::{WebPath, WebPathType};
use crate::application::ppt::{ppt_project_service, s_ppt_project};
use crate::service::prelude::*;

pub fn ppt_project() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取 PPT 项目列表"), get(list_handler))
        .route("/:id", WebPathType::Get, Some("获取 PPT 项目详情"), get(detail_handler))
        .route("/add", WebPathType::Post, Some("添加 PPT 项目"), post(add_handler))
        .route("/edit", WebPathType::Put, Some("编辑 PPT 项目"), put(edit_handler))
        .route("/del/:id", WebPathType::Delete, Some("删除 PPT 项目"), delete(s_ppt_project::delete_by_id))
        .route("/copy/:id", WebPathType::Post, Some("复制 PPT 项目"), post(s_ppt_project::copy))
        .route("/status", WebPathType::Put, Some("更新 PPT 项目状态"), put(update_status_handler))
        .route("/statistics", WebPathType::Get, Some("获取 PPT 项目统计"), get(s_ppt_project::statistics))
}

#[axum::debug_handler]
async fn list_handler(
    VQuery(page): VQuery<PageParams>,
    VQuery(search): VQuery<crate::domain::args::a_ppt::PptProjectListArgs>,
) -> impl axum::response::IntoResponse {
    ppt_project_service::list(VQuery(page), VQuery(search)).await
}

#[axum::debug_handler]
async fn detail_handler(Path(id): Path<i64>) -> impl axum::response::IntoResponse {
    ppt_project_service::detail(Path(id)).await
}

#[axum::debug_handler]
async fn add_handler(VJson(arg): VJson<crate::domain::args::a_ppt::PptProjectAddArgs>) -> impl axum::response::IntoResponse {
    ppt_project_service::add(VJson(arg)).await
}

#[axum::debug_handler]
async fn edit_handler(VJson(arg): VJson<crate::domain::args::a_ppt::PptProjectEditArgs>) -> impl axum::response::IntoResponse {
    ppt_project_service::edit(VJson(arg)).await
}

#[axum::debug_handler]
async fn update_status_handler(VJson(arg): VJson<crate::domain::args::a_ppt::PptProjectStatusArgs>) -> impl axum::response::IntoResponse {
    ppt_project_service::update_status(VJson(arg)).await
}

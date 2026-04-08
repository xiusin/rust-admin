use crate::model::sys::model::msys_permission_category::{PermissionCategoryAdd, PermissionCategoryEdit, PermissionCategorySearch, SysPermissionCategoryModel};
use crate::service::prelude::*;
use axum::Json;
pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    Query(search): Query<PermissionCategorySearch>,
) -> impl IntoResponse {
    let rlist = SysPermissionCategoryModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

pub async fn add(Json(arg): Json<PermissionCategoryAdd>) -> impl IntoResponse {
    let r = SysPermissionCategoryModel::add(arg).await;
    ApiResponse::from_result(r)
}

pub async fn edit(Json(arg): Json<PermissionCategoryEdit>) -> impl IntoResponse {
    let r = SysPermissionCategoryModel::edit(arg).await;
    ApiResponse::from_result(r)
}

pub async fn delete(path: axum::extract::Path<i64>) -> impl IntoResponse {
    let id = path.0;
    let r = SysPermissionCategoryModel::delete(id).await;
    ApiResponse::from_result(r)
}

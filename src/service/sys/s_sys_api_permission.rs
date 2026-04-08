use crate::api::web_path::WebPath;
use crate::model::sys::model::msys_api_permission::{ApiPermissionAdd, ApiPermissionEdit, ApiPermissionSearch, SysApiPermissionModel};
use crate::model::sys::entity::sys_api_permission;
use crate::service::prelude::*;
pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<ApiPermissionSearch>,
) -> impl IntoResponse {
    let rlist = SysApiPermissionModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

pub async fn add(VJson(arg): VJson<ApiPermissionAdd>) -> impl IntoResponse {
    let db = DB().await.begin().await;
    match db {
        Ok(mut tx) => {
            let result = SysApiPermissionModel::add_or_update(arg, &mut tx).await;
            match result {
                Ok(api) => {
                    tx.commit().await.unwrap();
                    ApiResponse::ok(api)
                }
                Err(e) => {
                    tx.rollback().await.unwrap();
                    ApiResponse::bad_request(e.to_string())
                }
            }
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn edit(VJson(arg): VJson<ApiPermissionEdit>) -> impl IntoResponse {
    let r = SysApiPermissionModel::edit(arg).await;
    ApiResponse::from_result(r)
}

pub async fn delete(path: axum::extract::Path<i64>) -> impl IntoResponse {
    let id = path.0;
    let db = DB().await;
    let result = sys_api_permission::Entity::delete_by_id(id).exec(db).await;
    match result {
        Ok(_) => ApiResponse::ok("Success"),
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn update_all() -> impl IntoResponse {
    let web_paths = crate::api::web_path::get_all_web_paths();
    let webstr = serde_json::to_string(&web_paths).unwrap();
    let result = update_all_api(webstr).await;
    ApiResponse::from_result(result)
}

pub async fn update_all_api(webstr: String) -> Result<String> {
    info!("update_all_api begin");
    let args: Vec<WebPath> = serde_json::from_str(&webstr).unwrap();
    let db = DB().await.begin().await?;

    for (sort, webpath) in args.into_iter().enumerate() {
        let apipath = ApiPermissionAdd {
            api: webpath.final_path.clone(),
            method: webpath.webmethod.as_str().to_string(),
            apiname: webpath.apiname.unwrap_or("None".to_owned()),
            sort: (sort + 1) as i32,
        };
        let apimodel = SysApiPermissionModel::add_or_update(apipath, &db).await?;
        let cache = CacheManager::instance().await;
        let _ = cache
            .set_value(&format!("api:{}", apimodel.api), &apimodel)
            .await;
    }
    info!("update_all_api ok");
    db.commit().await?;
    Ok("update_all_api ok".to_string())
}

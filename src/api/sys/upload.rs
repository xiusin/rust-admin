use axum::routing::post;
use crate::application::sys::s_sys_upload;
use crate::api::web_path::{WebPath, WebPathType};

pub fn sys_upload() -> WebPath {
    WebPath::new()
        .route(
            "",
            WebPathType::Post,
            Some("上传文件"),
            post(s_sys_upload::upload_file_api),
        )
}
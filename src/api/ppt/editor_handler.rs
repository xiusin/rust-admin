use crate::api::web_path::{WebPath, WebPathType};
use crate::application::ppt::editor_handler;
use axum::routing::{delete, get, post, put};

pub fn editor_routes() -> WebPath {
    WebPath::new()
        .route("/slides", WebPathType::Get, Some("获取幻灯片列表"), get(editor_handler::get_slides))
        .route("/slide/add", WebPathType::Post, Some("添加幻灯片"), post(editor_handler::add_slide))
        .route("/slide/edit", WebPathType::Put, Some("编辑幻灯片"), put(editor_handler::edit_slide))
        .route("/slide/del/:id", WebPathType::Delete, Some("删除幻灯片"), delete(editor_handler::delete_slide))
        .route("/slides/sort", WebPathType::Put, Some("排序幻灯片"), put(editor_handler::sort_slides))
        .route("/export", WebPathType::Post, Some("导出PPT"), post(editor_handler::export_ppt))
}

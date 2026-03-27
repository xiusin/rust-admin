use axum::{
    body::Body,
    extract::Path,
    http::{header, Response},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::application::cms::code_pack_service;
use crate::model::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ModelIdPath {
    pub model_id: i64,
}

pub async fn preview(Path(path): Path<ModelIdPath>) -> impl IntoResponse {
    let result = code_pack_service::preview_all(path.model_id).await;
    ApiResponse::from_result(result)
}

pub async fn download(Path(path): Path<ModelIdPath>) -> Response<Body> {
    match code_pack_service::download(path.model_id).await {
        Ok(zip_data) => {
            let filename = format!("model_{}_code.zip", path.model_id);
            Response::builder()
                .status(200)
                .header(
                    header::CONTENT_TYPE,
                    "application/zip",
                )
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                )
                .body(Body::from(zip_data))
                .unwrap()
        }
        Err(e) => {
            let error_msg = format!("{{\"message\": \"{}\"}}", e);
            Response::builder()
                .status(500)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(error_msg))
                .unwrap()
        }
    }
}

pub async fn file_tree(Path(path): Path<ModelIdPath>) -> impl IntoResponse {
    let result = code_pack_service::get_file_tree(path.model_id).await;
    ApiResponse::from_result(result)
}

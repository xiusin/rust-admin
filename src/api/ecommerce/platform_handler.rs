use axum::{extract::{Path, Json, State}, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use crate::application::ecommerce::services::platform_service::PlatformService;
use crate::domain::entity::ecommerce::platform::EcommercePlatform;

#[derive(Debug, Deserialize)]
pub struct CreatePlatformRequest {
    pub platform_type: String,
    pub name: String,
    pub app_key: String,
    pub app_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlatformRequest {
    pub name: Option<String>,
    pub app_key: Option<String>,
    pub app_secret: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub status: Option<i32>,
}

pub async fn get_platforms(State(service): State<PlatformService>) -> impl IntoResponse {
    match service.get_platforms() {
        Ok(platforms) => (StatusCode::OK, axum::Json(platforms)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(e.to_string())),
    }
}

pub async fn get_platform(Path(platform_id): Path<i64>, State(service): State<PlatformService>) -> impl IntoResponse {
    match service.get_platform(platform_id) {
        Ok(platform) => (StatusCode::OK, axum::Json(platform)),
        Err(e) => (StatusCode::NOT_FOUND, axum::Json(e.to_string())),
    }
}

pub async fn create_platform(Json(req): Json<CreatePlatformRequest>, State(service): State<PlatformService>) -> impl IntoResponse {
    let platform = EcommercePlatform::new(
        req.platform_type.clone(),
        req.name.clone(),
        req.app_key.clone(),
        req.app_secret.clone(),
    );
    match service.create_platform(platform) {
        Ok(platform) => (StatusCode::CREATED, axum::Json(platform)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(e.to_string())),
    }
}

pub async fn update_platform(Path(platform_id): Path<i64>, Json(req): Json<UpdatePlatformRequest>, State(service): State<PlatformService>) -> impl IntoResponse {
    // 这里需要先获取现有平台，然后更新字段
    match service.get_platform(platform_id) {
        Ok(mut platform) => {
            if let Some(name) = &req.name {
                platform.name = name.clone();
            }
            if let Some(app_key) = &req.app_key {
                platform.app_key = app_key.clone();
            }
            if let Some(app_secret) = &req.app_secret {
                platform.app_secret = app_secret.clone();
            }
            if let Some(access_token) = &req.access_token {
                platform.access_token = Some(access_token.clone());
            }
            if let Some(refresh_token) = &req.refresh_token {
                platform.refresh_token = Some(refresh_token.clone());
            }
            if let Some(status) = req.status {
                platform.status = status;
            }
            match service.update_platform(platform_id, platform) {
                Ok(platform) => (StatusCode::OK, axum::Json(platform)),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(e.to_string())),
            }
        }
        Err(e) => (StatusCode::NOT_FOUND, axum::Json(e.to_string())),
    }
}

pub async fn delete_platform(Path(platform_id): Path<i64>, State(service): State<PlatformService>) -> impl IntoResponse {
    match service.delete_platform(platform_id) {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(e.to_string())),
    }
}

pub async fn test_connection(Path(platform_id): Path<i64>, State(service): State<PlatformService>) -> impl IntoResponse {
    match service.test_connection(platform_id) {
        Ok(_) => (StatusCode::OK, axum::Json(serde_json::json!("Connection successful"))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(e.to_string())),
    }
}

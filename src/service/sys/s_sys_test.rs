use crate::common::ser::i64_to_string;
use crate::{common::ApiResponse, model::prelude::VJson};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Deserialize, Debug, Validate)]
pub struct UserId {
    #[serde(with = "i64_to_string")]
    pub uid: i64,
}

#[derive(Clone, Deserialize, Serialize, Debug, Validate)]
pub struct UserRId {
    #[serde(with = "i64_to_string")]
    pub ruid: i64,
}

#[allow(dead_code)]
pub async fn test(VJson(_arg): VJson<UserId>) -> impl IntoResponse {
    let st = UserRId { ruid: 1555 };
    ApiResponse::ok(st)
}

#[allow(dead_code)]
pub async fn list() -> impl IntoResponse {}

#[allow(dead_code)]
pub async fn edit() -> impl IntoResponse {}

#[allow(dead_code)]
pub async fn add() -> impl IntoResponse {}

#[allow(dead_code)]
pub async fn delete() -> impl IntoResponse {}

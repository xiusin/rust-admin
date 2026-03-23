use super::error::{Error, Result};
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct RespDataString(pub String);

// 空数据类型，用于没有业务数据的响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmptyData {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new(code: u16, message: String, data: T) -> Self {
        Self { code, message, data }
    }

    pub fn success(data: T) -> Self {
        Self::new(200, "success".to_string(), data)
    }

    pub fn ok(data: T) -> Response {
        (StatusCode::OK, Self::new(200, "操作成功".to_string(), data)).into_response()
    }

    pub fn ok_with_msg(data: T, message: impl Into<String>) -> Response {
        (StatusCode::OK, Self::new(200, message.into(), data)).into_response()
    }

    pub fn created(data: T) -> Response {
        (StatusCode::CREATED, Self::new(201, "创建成功".to_string(), data)).into_response()
    }

    pub fn created_with_msg(data: T, message: impl Into<String>) -> Response {
        (StatusCode::CREATED, Self::new(201, message.into(), data)).into_response()
    }

    // 从 Result 转换
    pub fn from_result(result: Result<T>) -> Response
    where
        T: Serialize,
    {
        match result {
            Ok(data) => Self::ok(data),
            Err(err) => {
                let (status, message) = match &err {
                    Error::Message(msg) => {
                        let status = if msg.contains("not found") {
                            StatusCode::NOT_FOUND
                        } else if msg.contains("unauthorized") {
                            StatusCode::UNAUTHORIZED
                        } else if msg.contains("forbidden") {
                            StatusCode::FORBIDDEN
                        } else if msg.contains("validation") {
                            StatusCode::UNPROCESSABLE_ENTITY
                        } else {
                            StatusCode::INTERNAL_SERVER_ERROR
                        };
                        (status, msg.clone())
                    }
                    Error::WithStatus(status, msg) => (*status, msg.clone()),
                };

                let code = status.as_u16();
                Self::error_response(status, message, code)
            }
        }
    }

    fn error_response(status: StatusCode, message: String, code: u16) -> Response {
        let api_response = ApiResponse::<EmptyData> {
            code,
            message,
            data: EmptyData {},
        };
        (status, api_response).into_response()
    }
}
impl ApiResponse<EmptyData> {
    pub fn custom(status: StatusCode, message: String) -> Response {
        let api_response = ApiResponse::<EmptyData> {
            code: status.as_u16(),
            message,
            data: EmptyData {},
        };
        (status, api_response).into_response()
    }

    pub fn unauthorized(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::UNAUTHORIZED, message.into(), 401)
    }

    pub fn forbidden(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::FORBIDDEN, message.into(), 403)
    }

    pub fn not_found(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::NOT_FOUND, message.into(), 404)
    }

    pub fn conflict(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::CONFLICT, message.into(), 409)
    }

    pub fn unprocessable_entity(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::UNPROCESSABLE_ENTITY, message.into(), 422)
    }

    pub fn internal_server_error(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::INTERNAL_SERVER_ERROR, message.into(), 500)
    }

    pub fn service_unavailable(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::SERVICE_UNAVAILABLE, message.into(), 503)
    }

    pub fn success_no_data(message: impl Into<String>) -> Response {
        let api_response = ApiResponse::<EmptyData> {
            code: 200,
            message: message.into(),
            data: EmptyData {},
        };
        (StatusCode::OK, api_response).into_response()
    }

    pub fn deleted(message: impl Into<String>) -> Response {
        let api_response = ApiResponse::<EmptyData> {
            code: 200,
            message: message.into(),
            data: EmptyData {},
        };
        (StatusCode::OK, api_response).into_response()
    }

    pub fn bad_request(message: impl Into<String>) -> Response {
        Self::error_response(StatusCode::BAD_REQUEST, message.into(), 400)
    }
}
impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match serde_json::to_string(&self) {
            Ok(json) => (
                [
                    ("Content-Type", "application/json;charset=UTF-8"),
                    ("Access-Control-Allow-Origin", "*"),
                    ("Cache-Control", "no-cache"),
                ],
                json,
            )
                .into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("Content-Type", "application/json;charset=UTF-8")],
                r#"{"code":500,"message":"序列化响应失败","data":{}}"#,
            )
                .into_response(),
        }
    }
}
#[allow(unused_macros)]
// 宏定义保持不变
macro_rules! api_ok {
    ($data:expr) => {
        ApiResponse::ok($data)
    };
    ($data:expr, $msg:expr) => {
        ApiResponse::ok_with_msg($data, $msg)
    };
}
#[allow(unused_macros)]
macro_rules! api_error {
    (400, $msg:expr) => {
        ApiResponse::bad_request($msg)
    };
    (401, $msg:expr) => {
        ApiResponse::unauthorized($msg)
    };
    (403, $msg:expr) => {
        ApiResponse::forbidden($msg)
    };
    (404, $msg:expr) => {
        ApiResponse::not_found($msg)
    };
    (409, $msg:expr) => {
        ApiResponse::conflict($msg)
    };
    (422, $msg:expr) => {
        ApiResponse::unprocessable_entity($msg)
    };
    (500, $msg:expr) => {
        ApiResponse::internal_server_error($msg)
    };
    (503, $msg:expr) => {
        ApiResponse::service_unavailable($msg)
    };
}
#[allow(unused_macros)]
macro_rules! api_created {
    ($data:expr) => {
        ApiResponse::created($data)
    };
    ($data:expr, $msg:expr) => {
        ApiResponse::created_with_msg($data, $msg)
    };
}
#[allow(unused_macros)]
macro_rules! api_deleted {
    ($msg:expr) => {
        ApiResponse::deleted($msg)
    };
}

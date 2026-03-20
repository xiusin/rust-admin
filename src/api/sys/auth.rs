use axum::{
    routing::{get, post},
    Router,
};
use crate::application::sys::*;

pub fn auth() -> Router {
    Router::new()
        .route("/login", post(s_sys_user::login))
        .route("/get_captcha", get(s_sys_captcha::get_captcha))
}

pub fn sys_test() -> Router {
    Router::new().route("/test1", get(s_sys_test::test))
}

pub fn white_sys() -> Router {
    Router::new().nest(
        "/sys",
        Router::new()
            .nest("/test", sys_test())
            .nest("/auth", auth()),
    )
}
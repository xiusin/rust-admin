pub mod ws;

use axum::{routing::get, Router};

pub fn white_monitor() -> Router {
    Router::new().nest(
        "/monitor",
        Router::new().route("/ws", get(ws::ws_handler))
    )
}

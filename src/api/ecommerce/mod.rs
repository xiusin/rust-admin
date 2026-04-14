pub mod platform_handler;

use axum::{Router, routing};

pub fn router_ecommerce() -> Router {
    Router::new()
        .route("/api/ecommerce/platforms", routing::get(platform_handler::get_platforms))
        .route("/api/ecommerce/platforms/:id", routing::get(platform_handler::get_platform))
        .route("/api/ecommerce/platforms", routing::post(platform_handler::create_platform))
        .route("/api/ecommerce/platforms/:id", routing::put(platform_handler::update_platform))
        .route("/api/ecommerce/platforms/:id", routing::delete(platform_handler::delete_platform))
        .route("/api/ecommerce/platforms/:id/test-connection", routing::post(platform_handler::test_connection))
}

pub mod health_check;
mod responses;

use axum::{Router, routing::get};
use crate::health_check::health_check_handler;

pub fn app() -> Router {
    Router::new()
        .route("/hello", get(|| async { "Hello, World!"}))
        .route("/api/health_check", get(health_check_handler))
}
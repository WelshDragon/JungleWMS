pub mod health_check;
mod responses;
mod state;
pub mod util;

use axum::{Router, routing::get, routing::post};
use crate::health_check::health_check_handler;
use std::sync::Arc;
use state::SharedState;


pub fn app() -> Router {
    let shared_state = SharedState::default();

    Router::new()
        .route("/hello", get(|| async { "Hello, World!"}))
        .route("/api/health_check", get(health_check_handler))
        .route("/api/authenticate", post(health_check_handler))
        .with_state(Arc::clone(&shared_state))
}
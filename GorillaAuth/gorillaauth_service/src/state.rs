
use std::collections::HashMap;
use std::sync::Arc;
use axum::body::Bytes;

#[derive(Default)]
pub struct AppState {
    db: HashMap<String, Bytes>,
}

pub type SharedState = Arc<tokio::sync::RwLock<AppState>>;

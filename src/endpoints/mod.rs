use axum::Router;

use crate::utils::state::AppState;

pub mod auth;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/auth", auth::router())
}

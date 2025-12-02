use axum::Router;

use crate::utils::state::ArcAppState;

pub mod auth;

pub fn create_router() -> Router<ArcAppState> {
    Router::new().nest("/auth", auth::router())
}

use axum::Router;

use crate::utils::state::ArcAppState;

pub mod auth;
pub mod users;

pub fn create_router() -> Router<ArcAppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
}

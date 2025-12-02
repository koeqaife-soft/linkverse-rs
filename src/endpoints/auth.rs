use axum::Router;

use crate::utils::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
}

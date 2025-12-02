use axum::Router;

use crate::utils::state::ArcAppState;

pub fn router() -> Router<ArcAppState> {
    Router::new()
}

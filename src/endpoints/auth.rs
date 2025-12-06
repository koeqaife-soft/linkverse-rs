use axum::{Router, extract::State, http::StatusCode, routing::post};
use serde::Deserialize;
use validator::Validate;

use crate::{
    database::{
        auth::{Tokens, create_tokens, get_user_by_email},
        conn::LazyConn,
    },
    utils::{
        response::{ApiResponse, AppError, log_and_convert, ok},
        security::check_password,
        state::ArcAppState,
        validate::ValidatedJson,
    },
};

#[derive(Debug, Deserialize, Validate)]
struct LoginPayload {
    #[validate(length(min = 8))]
    password: String,

    #[validate(email)]
    email: String,
}

async fn login(
    State(state): State<ArcAppState>,
    ValidatedJson(payload): ValidatedJson<LoginPayload>,
) -> Result<ApiResponse<Tokens>, AppError> {
    let conn_unlocked = LazyConn::new(state.db_pool.clone());
    let mut conn = conn_unlocked.lock().await;

    // Getting user
    let user_result = get_user_by_email(&payload.email, &mut conn)
        .await
        .map_err(log_and_convert)?;
    if user_result.is_none() {
        return Err(AppError::NotFound("USER_NOT_FOUND".to_string()));
    }

    // Checking password
    let user = user_result.unwrap();
    let correct = check_password(&user.password_hash, &payload.password);
    if !correct {
        return Err(AppError::Unauthorized("INCORRECT_PASSWORD".to_string()));
    }

    // Generating tokens
    let mut tx = conn.transaction().await.map_err(log_and_convert)?;
    let tokens = create_tokens(user.user_id, &mut tx, state)
        .await
        .map_err(log_and_convert)?;
    tx.commit().await.map_err(log_and_convert)?;

    return Ok(ok(tokens, StatusCode::OK));
}

pub fn router() -> Router<ArcAppState> {
    Router::new().route("/login", post(login))
}

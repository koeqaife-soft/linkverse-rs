use axum::{Router, extract::State, http::StatusCode, routing::post};
use serde::Deserialize;
use validator::Validate;

use crate::{
    database::{
        auth::{Tokens, create_tokens, get_user_by_email},
        conn::LazyConn,
    },
    utils::{
        response::{ApiResponse, AppError, FuncError, response},
        security::check_password,
        state::ArcAppState,
        validate::{ValidatedJson, validate_username},
    },
};

/// Login endpoint
mod login {
    use super::*;

    #[derive(Debug, Deserialize, Validate)]
    pub struct Payload {
        #[validate(length(min = 8))]
        password: String,

        #[validate(email)]
        email: String,
    }

    pub async fn handler(
        State(state): State<ArcAppState>,
        ValidatedJson(payload): ValidatedJson<Payload>,
    ) -> Result<ApiResponse<Tokens>, AppError> {
        let mut conn = LazyConn::new(state.db_pool.clone());

        // Getting user
        let user = get_user_by_email(&payload.email, &mut conn)
            .await?
            .ok_or(FuncError::UserNotFound)?;

        // Checking password
        let correct = check_password(&user.password_hash, &payload.password);
        if !correct {
            return Err(FuncError::IncorrectPassword.into());
        }

        // Generating tokens
        let mut tx = conn.transaction().await?;
        let tokens = create_tokens(user.user_id, &mut tx, state).await?;
        tx.commit().await?;

        return Ok(response(tokens, StatusCode::OK));
    }
}

/// Register endpoint
mod register {
    use crate::database::auth::{create_user, email_exists, username_exists};

    use super::*;

    #[derive(Debug, Deserialize, Validate)]
    pub struct Payload {
        #[validate(length(min = 8))]
        password: String,

        #[validate(email)]
        email: String,

        #[validate(custom(function = "validate_username"))]
        username: String,
    }

    pub async fn handler(
        State(state): State<ArcAppState>,
        ValidatedJson(payload): ValidatedJson<Payload>,
    ) -> Result<ApiResponse<Tokens>, AppError> {
        let mut conn = LazyConn::new(state.db_pool.clone());

        // Check existence of email
        if email_exists(&payload.email, &mut conn).await? {
            return Err(FuncError::UserAlreadyExists.into());
        }

        // Check existence of username
        if username_exists(&payload.username, &mut conn).await? {
            return Err(FuncError::UsernameExists.into());
        }

        // Creating new user and tokens
        let mut tx = conn.transaction().await?;
        let user_id = create_user(
            &payload.username,
            &payload.email,
            &payload.password,
            &mut tx,
        )
        .await?;
        let tokens = create_tokens(user_id, &mut tx, state).await?;
        tx.commit().await?;

        Ok(response(tokens, StatusCode::OK))
    }
}

pub fn router() -> Router<ArcAppState> {
    Router::new()
        .route("/login", post(login::handler))
        .route("/register", post(register::handler))
}

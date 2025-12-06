use crate::{
    database::lazy::{ArcLazyConn, LazyConn, MutexLazyConn, ResultError},
    entities::user::AuthUser,
    utils::{
        security::{generate_key, generate_token},
        state::ArcAppState,
    },
};
use deadpool_postgres::Transaction;
use serde::Serialize;
use tokio_postgres::Row;

#[derive(Debug, Serialize)]
pub struct Tokens {
    refresh: String,
    access: String,
}

async fn get_user_by(
    conn: &mut LazyConn,
    query_param: &(dyn tokio_postgres::types::ToSql + Sync),
    where_clause: &str,
) -> Result<Option<AuthUser>, ResultError> {
    let db = conn.get_client().await.map_err(ResultError::PoolError)?;
    let sql = format!(
        "
        SELECT username, user_id, email, password_hash,
               email_verified, pending_email,
               EXTRACT(EPOCH FROM pending_email_until)::BIGINT AS pending_email_until
        FROM users
        WHERE {}
        ",
        where_clause
    );

    let row = db
        .query_opt(&sql, &[query_param])
        .await
        .map_err(ResultError::QueryError)?;

    Ok(row.map(|row| row_to_auth_user(&row)))
}

fn row_to_auth_user(row: &Row) -> AuthUser {
    AuthUser {
        username: row.get("username"),
        user_id: row.get("user_id"),
        email: row.get("email"),
        password_hash: row.get("password_hash"),
        email_verified: row.get("email_verified"),
        pending_email: row.get("pending_email"),
        pending_email_until: row.get("pending_email_until"),
    }
}

pub async fn get_auth_user(
    user_id: &String,
    conn: &mut LazyConn,
) -> Result<Option<AuthUser>, ResultError> {
    get_user_by(conn, user_id, "user_id = $1").await
}

pub async fn get_user_by_email(
    email: &String,
    conn: &mut LazyConn,
) -> Result<Option<AuthUser>, ResultError> {
    get_user_by(conn, email, "email = $1").await
}

pub async fn create_tokens(
    user_id: String,
    tx: &mut Transaction<'_>,
    state: ArcAppState,
) -> Result<Tokens, ResultError> {
    let new_secret = generate_key(16);
    let new_session_id = state.snowflake.generate().await.to_string();

    let refresh = generate_token(
        &user_id,
        "refresh",
        true,
        &new_secret,
        &new_session_id,
        &state.config.signature_key,
    )
    .await
    .map_err(ResultError::AnyhowError)?;

    let access = generate_token(
        &user_id,
        "access",
        false,
        &new_secret,
        &new_session_id,
        &state.config.signature_key,
    )
    .await
    .map_err(ResultError::AnyhowError)?;

    tx.execute(
        "
        INSERT INTO auth_keys (user_id, token_secret, session_id)
        VALUES ($1, $2, $3)
        ",
        &[&user_id, &new_secret, &new_session_id],
    )
    .await
    .map_err(ResultError::QueryError)?;

    Ok(Tokens { refresh, access })
}

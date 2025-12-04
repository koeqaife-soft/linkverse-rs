use crate::{
    database::lazy::{ArcLazyConn, ResultError},
    entities::user::AuthUser,
};
use tokio_postgres::Row;

async fn get_user_by(
    conn: ArcLazyConn<'_>,
    query_param: &(dyn tokio_postgres::types::ToSql + Sync),
    where_clause: &str,
) -> Result<AuthUser, ResultError> {
    let mut locked_conn = conn.lock().await;
    let db = locked_conn
        .get_client()
        .await
        .map_err(ResultError::PoolError)?;
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
        .query_one(&sql, &[query_param])
        .await
        .map_err(ResultError::QueryError)?;

    Ok(row_to_auth_user(&row))
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
    user_id: String,
    conn: ArcLazyConn<'_>,
) -> Result<AuthUser, ResultError> {
    get_user_by(conn, &user_id, "user_id = $1").await
}

pub async fn get_user_by_email(
    email: String,
    conn: ArcLazyConn<'_>,
) -> Result<AuthUser, ResultError> {
    get_user_by(conn, &email, "email = $1").await
}

pub async fn create_auth_keys(user_id: String, conn: ArcLazyConn<'_>) {}

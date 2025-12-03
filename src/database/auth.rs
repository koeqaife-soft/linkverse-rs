use crate::{
    database::lazy::{ArcLazyConn, ResultError},
    entities::user::AuthUser,
};

pub async fn get_auth_user(user_id: String, conn: ArcLazyConn) -> Result<AuthUser, ResultError> {
    let mut locked_conn = conn.lock().await;
    let db = locked_conn
        .get_client()
        .await
        .map_err(ResultError::PoolError)?;
    let result = db
        .query_one(
            "
                SELECT username, user_id, email, password_hash,
                email_verified, pending_email,
                EXTRACT(EPOCH FROM pending_email_until)::BIGINT AS pending_email_until
                FROM users
                WHERE user_id = $1
                ",
            &[&user_id],
        )
        .await
        .map_err(ResultError::QueryError);

    match result {
        Ok(row) => {
            let user = AuthUser {
                username: row.get("username"),
                user_id: row.get("user_id"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                email_verified: row.get("email_verified"),
                pending_email: row.get("pending_email"),
                pending_email_until: row.get("pending_email_until"),
            };
            Ok(user)
        }
        Err(r) => Err(r),
    }
}

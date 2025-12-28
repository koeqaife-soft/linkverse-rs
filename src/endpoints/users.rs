use axum::{Router, extract::State, http::StatusCode, routing::get};
use serde::Serialize;

use crate::{
    database::conn::LazyConn,
    entities::user::User,
    extractors::auth::AuthSession,
    get_conn,
    utils::{
        response::{ApiResponse, AppError, FuncError, response},
        state::ArcAppState,
    },
};

mod me {
    use crate::{
        database::users::get_user,
        utils::perms::{permissions_to_list, role_permissions},
    };

    use super::*;

    #[derive(Debug, Serialize)]
    pub struct Returns {
        #[serde(flatten)]
        pub user: User,
        pub created_at: f64,
        pub permissions: Vec<&'static str>,
    }

    pub async fn handler(
        session: AuthSession,
        State(state): State<ArcAppState>,
    ) -> Result<ApiResponse<Returns>, AppError> {
        let mut conn = get_conn!(state);
        let user = get_user(&session.user_id, &mut conn)
            .await?
            .ok_or(FuncError::UserNotFound)?;

        let perms = role_permissions(&user.role_id);
        let permissions = permissions_to_list(perms);

        Ok(response(
            Returns {
                created_at: user.created_at(),
                user,
                permissions,
            },
            StatusCode::OK,
        ))
    }
}

pub fn router() -> Router<ArcAppState> {
    Router::new().route("/me", get(me::handler))
}

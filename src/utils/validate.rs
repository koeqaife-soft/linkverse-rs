use super::response::ApiResponse;
use axum::http::StatusCode;
use axum::{
    Json,
    extract::{FromRequest, Request},
};
use regex::Regex;
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<B, T> FromRequest<B> for ValidatedJson<T>
where
    B: Send + Sync + 'static,
    T: DeserializeOwned + Validate,
{
    type Rejection = (StatusCode, ApiResponse<()>);

    async fn from_request(req: Request, state: &B) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<T>::from_request(req, &state).await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                ApiResponse::<()>::err("INCORRECT_DATA"),
            )
        })?;

        payload.validate().map_err(|_| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiResponse::<()>::err("INCORRECT_DATA"),
            )
        })?;

        Ok(ValidatedJson(payload))
    }
}

fn validate_username(nickname: &str) -> bool {
    let re = Regex::new(r"^[A-Za-z0-9._]+$").unwrap();
    if !re.is_match(nickname) {
        return false;
    }

    if nickname.contains("..") || nickname.contains("__") {
        return false;
    }

    true
}

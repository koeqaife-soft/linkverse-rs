use axum::http::StatusCode;
use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<B, T> FromRequest<B> for ValidatedJson<T>
where
    B: Send + Sync + 'static,
    T: DeserializeOwned + Validate,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &B) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<T>::from_request(req, &state)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        payload
            .validate()
            .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;

        Ok(ValidatedJson(payload))
    }
}

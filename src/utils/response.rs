use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::error;

#[derive(Serialize, Debug)]
pub struct ApiResponseData<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Debug)]
pub struct ApiResponse<T> {
    data: ApiResponseData<T>,
    status: StatusCode,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T, status: StatusCode) -> Self {
        let data = ApiResponseData {
            success: true,
            data: Some(data),
            error: None,
        };
        Self { data, status }
    }

    pub fn err(msg: &str, status: StatusCode) -> Self {
        let data = ApiResponseData {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        };
        Self { data, status }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let json = axum::Json(self.data);
        (self.status, json).into_response()
    }
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Unauthorized(String),
    BadRequest(String),
    Internal(String),
    Forbidden(String),
    Conflict(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
        };

        let body = Json(ApiResponseData::<()> {
            success: false,
            data: None,
            error: Some(error_message),
        });

        (status, body).into_response()
    }
}

#[derive(Debug)]
pub enum FuncError {
    UserNotFound,
    IncorrectPassword,
    IncorrectData,
    UserAlreadyExists,
    UsernameExists,
}

impl From<FuncError> for AppError {
    fn from(err: FuncError) -> Self {
        match err {
            FuncError::UserNotFound => AppError::NotFound("USER_NOT_FOUND".into()),
            FuncError::IncorrectPassword => AppError::Unauthorized("INCORRECT_PASSWORD".into()),
            FuncError::IncorrectData => AppError::BadRequest("IncorrectData".into()),
            FuncError::UserAlreadyExists => AppError::Conflict("USER_ALREADY_EXISTS".into()),
            FuncError::UsernameExists => AppError::Conflict("USERNAME_EXISTS".into()),
        }
    }
}

pub fn ok<T>(data: T, status: StatusCode) -> ApiResponse<T> {
    ApiResponse::<T>::ok(data, status)
}

pub fn err<T>(msg: &str, status: StatusCode) -> ApiResponse<T> {
    ApiResponse::<T>::err(msg, status)
}

use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

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

pub fn ok<T>(data: T, status: StatusCode) -> ApiResponse<T> {
    ApiResponse::<T>::ok(data, status)
}

pub fn err<T>(msg: &str, status: StatusCode) -> ApiResponse<T> {
    ApiResponse::<T>::err(msg, status)
}

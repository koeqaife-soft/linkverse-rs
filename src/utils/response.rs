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

impl<T> ApiResponseData<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct ApiResponse<T> {
    data: ApiResponseData<T>,
    status: StatusCode,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T, status: StatusCode) -> Self {
        let data = ApiResponseData::ok(data);
        Self { data, status }
    }

    pub fn err(msg: &str, status: StatusCode) -> Self {
        let data = ApiResponseData::err(msg);
        Self { data, status }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let json = axum::Json(self.data);
        (self.status, json).into_response()
    }
}

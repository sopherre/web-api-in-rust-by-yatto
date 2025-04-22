use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("External API error")]
    ExternalApiError(#[from] reqwest::Error),

    #[error("User not found")]
    NotFound,

    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ExternalApiError(_) | AppError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
        }
    }
}

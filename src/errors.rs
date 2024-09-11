use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::IoError(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::RequestError(ref e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::BadRequest(ref e) => (StatusCode::BAD_REQUEST, e.to_string()),
        };

        (status, error_message).into_response()
    }
}

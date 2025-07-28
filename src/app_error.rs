use anyhow;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error(transparent)]
    JsonRejection(#[from] axum::extract::rejection::JsonRejection),
    #[error("Query Rejection")]
    QueryRejection,
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
    #[error("Bad Request")]
    BadRequest,
    #[error("Conflict")]
    Conflict,
    #[error("Not Found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::JsonRejection(e) => (StatusCode::BAD_REQUEST, e).into_response(),
            AppError::QueryRejection => {
                (StatusCode::BAD_REQUEST, "Query Rejection").into_response()
            }
            AppError::Validation(_) => {
                (StatusCode::BAD_REQUEST, "Validation error").into_response()
            }
            AppError::InternalError(_e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
            AppError::Conflict => (StatusCode::CONFLICT, "Conflict").into_response(),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            AppError::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request").into_response(),
        }
        .into_response()
    }
}

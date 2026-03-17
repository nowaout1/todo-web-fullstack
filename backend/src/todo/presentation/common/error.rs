use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

// TODO: add text! macro

#[derive(Error, Debug)]
pub enum AppError {
    #[error("not found")]
    NotFound,

    #[error("unauthorized")]
    Unauthorized,

    #[error("validation error")]
    Validation(Json<serde_json::Value>),

    #[error("unexpected internal error: {0}")]
    Internal(#[from] eyre::Error),
}

impl AppError {
    pub fn text(msg: &str) -> Self {
        Self::Validation(Json(json!({ "msg": msg })))
    }

    pub fn internal<E>(error: E) -> Self
    where
        E: Into<eyre::ErrReport>,
    {
        Self::Internal(eyre::eyre!(error))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, Json(json!("not found"))),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, Json(json!("unauthorized"))),
            Self::Validation(body) => (StatusCode::BAD_REQUEST, body),
            Self::Internal(error) => {
                tracing::error!("unexpected error: {error}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!("internal server error")),
                )
            }
        };

        (status, body).into_response()
    }
}

use axum::{extract::rejection::JsonRejection, response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;
use thiserror::Error;

pub(crate) type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),

    #[error("JsonResponse: {code} {json}")]
    JsonResponse {
        code: StatusCode,
        json: serde_json::Value,
    },

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error("NotFound")]
    NotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::JsonResponse { code, json } => (code, Json(json)).into_response(),
            Error::NotFound => StatusCode::NOT_FOUND.into_response(),
            _ => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": format!("{:?}", self) })),
            )
                .into_response(),
        }
    }
}

// todo minidump?

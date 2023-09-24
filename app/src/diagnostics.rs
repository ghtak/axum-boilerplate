use axum::{extract::rejection::{JsonRejection, PathRejection}, response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;
use thiserror::Error;

pub(crate) type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error(transparent)]
    PathRejection(#[from] PathRejection),


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
            Error::JsonRejection(err)  => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": format!("{:?}", err) })),
            )
                .into_response(),
            Error::PathRejection(err) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "message": format!("{:?}", err) })),
                )
                    .into_response(),
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

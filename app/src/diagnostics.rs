use axum::{
    extract::{
        multipart::MultipartError,
        rejection::{JsonRejection, PathRejection},
    },
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use serde_json::json;
use thiserror::Error;

use crate::define;

pub(crate) type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error(transparent)]
    PathRejection(#[from] PathRejection),

    #[error(transparent)]
    MultipartError(#[from] MultipartError),

    #[error(transparent)]
    SqlXError(sqlx::Error),

    #[error("RowNotFound")]
    RowNotFound,

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error("NotFound")]
    NotFound,

    #[error("Message {0}")]
    Message(String),

    #[error("JsonResponse: {code} {json}")]
    JsonResponse {
        code: StatusCode,
        json: serde_json::Value,
    },

    #[error("Not Implemented")]
    NotImplemented,
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Error::RowNotFound,
            _ => Error::SqlXError(value),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let mut res = match self {
            Error::JsonResponse { code, json } => (code, Json(json)).into_response(),
            Error::JsonRejection(err) => (
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
        };
        res.headers_mut().insert(
            define::CUSTOM_HEADER_IS_DIAGNOSTICS_ERROR,
            "true".parse().unwrap(),
        );
        res
    }
}

// todo minidump?

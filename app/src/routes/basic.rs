use axum::{extract::State, Router};
use hyper::StatusCode;

use crate::application_context::ApplicationContext;
use crate::diagnostics::{Result, Error};

async fn index() -> &'static str {
    tracing::debug!("hello_axum");
    "Hello Axum"
}

async fn error() -> Result<()> {
    //error::AppError::IoError(Error::from(ErrorKind::UnexpectedEof))
    Err(Error::JsonResponse {
        code: StatusCode::BAD_GATEWAY,
        json: serde_json::json!({
                "message": "json_message"
            }
        ),
    })
}

async fn state(State(_ctx): State<ApplicationContext>) -> Result<&'static str> {
    Ok("")
}

pub(crate) fn router(application_context: ApplicationContext) -> Router {
    Router::new()
        .route("/", axum::routing::get(index))
        .route("/error", axum::routing::get(error))
        .route("/state", axum::routing::get(state))
        .with_state(application_context)
}

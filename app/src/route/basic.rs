use axum::response::IntoResponse;
use axum::{extract::State, Router};
use axum::{Json, TypedHeader};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::app_state::AppState;
use crate::diagnostics::{Error, Result};

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

async fn state(State(_ctx): State<AppState>) -> Result<&'static str> {
    Ok("")
}

async fn cookie_typed_header(
    TypedHeader(cookies): TypedHeader<axum::headers::Cookie>,
) -> &'static str {
    tracing::debug!("{cookies:?}");
    "ok"
}

#[derive(Deserialize, Serialize, Debug)]
struct CookieValue {
    pub name: String,
    pub value: String,
}

//async fn cookie(jar: CookieJar) -> (StatusCode, CookieJar, Json<serde_json::Value>) {
    async fn cookie(jar: CookieJar) -> impl IntoResponse {
    let value = jar
        .get("session_id")
        .map(|v| v.value().to_owned())
        .unwrap_or("0".to_owned())
        .parse::<i32>()
        .unwrap_or(0);

    let jar = jar.add(Cookie::new("session_id", (value + 1).to_string()));

    let values = jar
        .iter()
        .map(|x| CookieValue {
            name: x.name().to_owned(),
            value: x.value().to_owned(),
        })
        .collect::<Vec<CookieValue>>();

    (StatusCode::ACCEPTED, jar, Json(json!({ "values": values })))
}

// let jar = jar.add(Cookie::new("session_id", "session_id"));
// let mesage = jar
//     .iter()
//     .map(|x| format!("{} {}", x.name(), x.value()))
//     .collect::<Vec<String>>()
//     .join("\n");
// (StatusCode::ACCEPTED, jar, Json(json!({
//     "message" : mesage
// })))

pub(crate) fn router(application_context: AppState) -> Router {
    Router::new()
        .route("/", axum::routing::get(index))
        .route("/error", axum::routing::get(error))
        .route("/state", axum::routing::get(state))
        .route("/cookie", axum::routing::get(cookie))
        .with_state(application_context)
}

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::define;

pub(crate) async fn response_map(res: Response) -> Response {
    if res.status().is_success() {
        return res;
    }

    match res.headers().get(define::CUSTOM_HEADER_IS_DIAGNOSTICS_ERROR) {
        Some(_) => return res,
        _ => (),
    }

    let status = res.status();
    match hyper::body::to_bytes(res.into_body()).await {
        Ok(bytes) => {
            if let Ok(message) = std::str::from_utf8(&bytes) {
                (status, Json(json!({ "error": message }))).into_response()
            } else {
                status.into_response()
            }
        }
        Err(_) => return status.into_response(),
    }
}

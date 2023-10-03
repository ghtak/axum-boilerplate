use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::define;

pub(crate) async fn response_map(res: Response) -> Response {
    if res.status().is_client_error() || res.status().is_server_error() {
        if let Some(_) = res
            .headers()
            .get(define::CUSTOM_HEADER_IS_DIAGNOSTICS_ERROR)
        {
            return res;
        }

        let status = res.status();
        if let Ok(bytes) = hyper::body::to_bytes(res.into_body()).await {
            if let Ok(message) = std::str::from_utf8(&bytes) {
                return (status, Json(json!({ "error": message }))).into_response();
            }
        }
        status.into_response()
    } else {
        res
    }
}

// pub async fn csrf_check<B>(
//     State(ctx): State<AppContext>,
//     cookies: Cookies,
//     request: Request<B>,
//     next: Next<B>,
// ) -> Response {
//     let (mut parts, body) = request.into_parts();
//     // ReadableSession life time 제어를 위해 from_request_parts 로 얻어와 drop 시킨다
//     let s = ReadableSession::from_request_parts(&mut parts, &ctx).await.unwrap();
//     let token = s.get::<String>(defines::CSRF_TOKEN_NAME);
//     let cookie = cookies.get(defines::CSRF_TOKEN_NAME);
//     if let (Some(token), Some(cookie)) = (token, cookie) {
//         debug!("{token} {}", cookie.value());
//     }
//     drop(s);
//     let response = next.run(Request::from_parts(parts, body)).await;

//     // do something with `response`...

//     response
// }

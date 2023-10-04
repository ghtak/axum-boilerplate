use std::collections::HashMap;

use axum::extract::{Multipart, Path, Query};
use axum::response::{Html, IntoResponse};
use axum::{extract::State, Router};
use axum::{Json, TypedHeader};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::{CookieJar, WithRejection};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::app_state::AppState;
use crate::diagnostics::{self, Error, Result};

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
    tracing::debug!("{:?}", _ctx);
    Result::Ok("xxx")
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

#[derive(Deserialize, Debug)]
struct JsonValue {
    id: i64,
    name: String,
}

async fn json_value(
    WithRejection(Json(v), _): WithRejection<Json<JsonValue>, diagnostics::Error>,
) -> Result<String> {
    Ok(format!("{:?}", v).to_owned())
}

async fn path_fn(
    WithRejection(Path(p), _): WithRejection<Path<i32>, diagnostics::Error>,
) -> impl IntoResponse {
    format!("{:?}", p).to_owned()
}

async fn path_v2(
    WithRejection(Path((p1, p2)), _): WithRejection<Path<(i32, i32)>, diagnostics::Error>,
) -> impl IntoResponse {
    format!("{:?} {:?}", p1, p2).to_owned()
}

#[derive(Deserialize, Debug)]
struct PathParam {
    a: i32,
    b: String,
}

async fn path_v3(
    WithRejection(Path(param), _): WithRejection<Path<PathParam>, diagnostics::Error>,
) -> impl IntoResponse {
    format!("{:?} {:?}", param.a, param.b).to_owned()
}

async fn query(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    format!("{:?}", params)
}

async fn multipart_get() -> Html<&'static str> {
    r#"
        <!DOCTYPE html>
        <html lang="ko">
        <head>
            <meta charset="UTF-8">
            <title>Title</title>
        </head>
        <body>
            <input id="video-file" type="file" name="file"/>
            <button onclick="sendfile()">업로드</button>
            <div id="result"></div>
        </body>
        <script>
            const sendfile = () => {
                const file = document.getElementById("video-file").files[0];
                const resultElement = document.getElementById("result");
                const formData = new FormData();
                formData.append("/a/b/c", file);
                fetch("/basic/multipart", {
                        method: "POST",
                        body: formData,
                        // headers: headers,
                    }).then(resp => {
                        resp.text().then(data => resultElement.textContent = data);
                    }).catch(err => {
                        console.error("Error uploading video chunk");
                    });
            }
        </script>
        </html>
        "#
    .into()
}

async fn multipart_post(mut multipart: Multipart) -> diagnostics::Result<&'static str> {
    while let Some(mut field) = multipart.next_field().await? {
        while let Some(chunk) = field.chunk().await.map_err(|_err| Error::NotFound)? {
            tracing::debug!(
                "Length of `{:?}` '{:?}'is {} bytes",
                field.name(),
                field.file_name(),
                chunk.len()
            );
        }
    }
    Ok("Done")
}

async fn tree(
    WithRejection(Path(path), _): WithRejection<Path<String>, diagnostics::Error>,
) -> impl IntoResponse {
    format!("Path : {}", path)
}

pub(crate) fn router_(app_state: AppState) -> Router {
    Router::new()
        .route("/", axum::routing::get(index))
        .route("/error", axum::routing::get(error))
        .route("/state", axum::routing::get(state))
        .route("/cookie", axum::routing::get(cookie))
        .route("/json_value", axum::routing::post(json_value))
        .route("/path/:id", axum::routing::get(path_fn))
        //.route("/path/:a/:b", axum::routing::get(path_v2))
        .route("/path/:a/:b", axum::routing::get(path_v3))
        .route("/query", axum::routing::get(query))
        .route(
            "/multipart",
            axum::routing::get(multipart_get).post(multipart_post),
        )
        .route("/tree/*path", axum::routing::get(tree))
        .with_state(app_state)
}

pub(crate) fn router(path: &'_ str) -> Router<AppState> {
    Router::new()
        .route(path, axum::routing::get(index))
        .route(
            [path, "/error"].join("").as_str(),
            axum::routing::get(error),
        )
        .route(
            [path, "/state"].join("").as_str(),
            axum::routing::get(state),
        )
        .route(
            [path, "/cookie"].join("").as_str(),
            axum::routing::get(cookie),
        )
        .route(
            [path, "/json_value"].join("").as_str(),
            axum::routing::post(json_value),
        )
        .route(
            [path, "/path/:id"].join("").as_str(),
            axum::routing::get(path_fn),
        )
        .route(
            [path, "/path/:a/:b"].join("").as_str(),
            axum::routing::get(path_v3),
        )
        .route(
            [path, "/query"].join("").as_str(),
            axum::routing::get(query),
        )
        .route(
            [path, "/multipart"].join("").as_str(),
            axum::routing::get(multipart_get).post(multipart_post),
        )
        .route(
            [path, "/tree/*path"].join("").as_str(),
            axum::routing::get(tree),
        )
}

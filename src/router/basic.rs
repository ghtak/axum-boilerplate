use std::collections::HashMap;

use async_session::{Session, SessionStore};
use axum::extract::{Multipart, Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{extract::State, Router};
use axum::{Json, TypedHeader};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::{CookieJar, WithRejection};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::app_state::{AppState, RedisConnection, SessionStoreImpl};
use crate::define;
use crate::diagnostics::{self, Error, Result};
use crate::entity::User;

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

async fn redis_ping(RedisConnection(mut conn): RedisConnection) -> impl IntoResponse {
    let replay: String = bb8_redis::redis::cmd("PING")
        .query_async(&mut *conn)
        .await
        .unwrap();
    replay
}

#[derive(Deserialize, Debug)]
struct RedisKV {
    key: String,
    value: String,
}

async fn redis_set(
    RedisConnection(mut conn): RedisConnection,
    WithRejection(Path(param), _): WithRejection<Path<RedisKV>, diagnostics::Error>,
) -> diagnostics::Result<()> {
    bb8_redis::redis::cmd("SET")
        .arg(param.key)
        .arg(param.value)
        .query_async(&mut *conn)
        .await
        .map_err(|e| diagnostics::Error::BB8Error(e.to_string()))?;
    Ok(())
}

async fn redis_get(
    RedisConnection(mut conn): RedisConnection,
    WithRejection(Path(key), _): WithRejection<Path<String>, diagnostics::Error>,
) -> diagnostics::Result<String> {
    let value: String = bb8_redis::redis::cmd("GET")
        .arg(key)
        .query_async(&mut *conn)
        .await
        .map_err(|e| diagnostics::Error::BB8Error(e.to_string()))?;
    Ok(value)
}

async fn session_option(user: Option<User>) -> impl IntoResponse {
    if let Some(u) = user {
        return u.name;
    }
    "None".to_owned()
}

async fn session_required(user: User) -> impl IntoResponse {
    user.name
}

async fn session_set(
    State(session_store): State<SessionStoreImpl>,
    WithRejection(Path(name), _): WithRejection<Path<String>, diagnostics::Error>,
    jar: CookieJar,
) -> diagnostics::Result<impl IntoResponse> {
    let user = User::with_name(name);

    let mut session = Session::new();
    session
        .insert("user", &user)
        .map_err(|e| diagnostics::Error::Message(e.to_string()))?;

    let cookie = session_store
        .store_session(session)
        .await
        .map_err(|e| diagnostics::Error::Message(e.to_string()))?
        .unwrap();

    let mut cookie = Cookie::new(define::SESSION_COOKIE, cookie);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");

    let jar = jar.add(cookie);

    Ok((StatusCode::OK, jar))
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/api/basic", get(index))
        .route("/api/basic/error", get(error))
        .route("/api/basic/state", get(state))
        .route("/api/basic/cookie", get(cookie))
        .route("/api/basic/json_value", post(json_value))
        .route("/api/basic/path/:id", get(path_fn))
        .route("/api/basic/path/:a/:b", get(path_v3))
        .route("/api/basic/query", get(query))
        .route(
            "/api/basic/multipart",
            get(multipart_get).post(multipart_post),
        )
        .route("/api/basic/tree/*path", get(tree))
        .route("/api/basic/redis/ping", get(redis_ping))
        .route("/api/basic/redis/:key/:value", get(redis_set))
        .route("/api/basic/redis/:key", get(redis_get))
        .route("/api/basic/session/option", get(session_option))
        .route("/api/basic/session/required", get(session_required))
        .route("/api/basic/session/:name", get(session_set))
}

pub(crate) fn router_(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/error", get(error))
        .route("/state", get(state))
        .route("/cookie", get(cookie))
        .route("/json_value", post(json_value))
        .route("/path/:id", get(path_fn))
        //.route("/path/:a/:b", get(path_v2))
        .route("/path/:a/:b", get(path_v3))
        .route("/query", get(query))
        .route("/multipart", get(multipart_get).post(multipart_post))
        .route("/tree/*path", get(tree))
        .with_state(app_state)
}

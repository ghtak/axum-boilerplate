pub(crate) mod basic;
mod v1;

use std::time::Duration;

use axum::{extract::DefaultBodyLimit, handler::HandlerWithoutStateExt, Router};
use hyper::{header, Method, Uri};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    //    trace::{DefaultMakeSpan, TraceLayer},
};

use crate::{
    app_state::AppState,
    diagnostics::Error,
    util::{config::HttpConfig, middleware},
};

pub(crate) fn init_router(app_state: AppState, config: &HttpConfig) -> Router {
    let static_serv_service = {
        ServeDir::new(config.static_directory.as_str())
            .not_found_service((|_uri: Uri| async move { Error::NotFound }).into_service())
    };

    let router = Router::new()
        .merge(basic::router())
        .merge(v1::sample_router::router());

    #[cfg(feature = "enable_websocket_pubsub_sample")]
    let router = router.merge(crate::ws::pubsub::router());

    router
        .layer(cors())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .layer(axum::middleware::map_response(middleware::response_map))
        // .layer( TraceLayer::new_for_http() .make_span_with(DefaultMakeSpan::default().include_headers(true)))
        .fallback_service(static_serv_service)
        .with_state(app_state)
}

// #todo from config
pub(crate) fn cors() -> CorsLayer {
    CorsLayer::new()
        //.allow_credentials(true)
        .allow_headers(vec![
            header::ACCEPT,
            header::ACCEPT_LANGUAGE,
            header::AUTHORIZATION,
            header::CONTENT_LANGUAGE,
            header::CONTENT_TYPE,
        ])
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
            Method::CONNECT,
            Method::PATCH,
            Method::TRACE,
        ])
        // .allow_origin(AllowOrigin::exact(
        //     "http://localhost:5173".parse().unwrap(), // Make sure this matches your frontend url
        // ))
        .allow_origin(Any)
        .max_age(Duration::from_secs(60 * 60))
}

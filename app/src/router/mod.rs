pub(crate) mod basic;

use axum::{handler::HandlerWithoutStateExt, Router};
use hyper::Uri;
use tower_http::services::ServeDir;

use crate::{app_state::AppState, diagnostics::Error, utils::config::HttpConfig};

pub(crate) fn init_router(app_state: AppState, config: &HttpConfig) -> Router {
    let static_serv_service = {
        ServeDir::new(config.static_directory.as_str())
            .not_found_service((|_uri: Uri| async move { Error::NotFound }).into_service())
    };

    Router::new()
        //.merge(basic::router(app_state.clone()))
        .nest("/basic", basic::router(app_state.clone()))
        .fallback_service(static_serv_service)
}

pub(crate) mod basic;

use axum::{handler::HandlerWithoutStateExt, Router};
use hyper::Uri;
use tower_http::services::ServeDir;

use crate::{diagnostics::Error, utils::config::HttpConfig, app_state::AppState};

// pub(crate) fn router<T>() -> Router<T>
// where
//     T: Sync + Send + Clone + 'static,
// {
//     Router::<T>::new().merge(basics::router::<T>())
// }

pub(crate) fn init_router(
    application_context: AppState,
    config: &HttpConfig) -> Router
{
    let static_serv_service = {
        ServeDir::new(config.static_directory.as_str())
            .not_found_service((|_uri: Uri| async move { Error::NotFound }).into_service())
    };

    Router::new()
        //.merge(basic::router(application_context.clone()))
        .nest("/basic", basic::router(application_context.clone()))
        .fallback_service(static_serv_service)
}
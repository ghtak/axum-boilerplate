#![allow(dead_code)]

use app_state::AppState;
use tokio;

mod app_state;
mod diagnostics;
mod route;
mod utils;

#[tokio::main]
async fn main() {
    let config = utils::config::TomlConfig::from_file("config.toml").unwrap();
    let _guard = utils::tracing::init(&config.tracing).unwrap();
    tracing::trace!("{config:?}");
    let application_context = AppState {};
    let router = route::init_router(application_context, &config.http);
    let address = config.http.socket_addr().unwrap();
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap()
}

// fn main() {
//     let rt = tokio::runtime::Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .unwrap();
//     let _ = rt.block_on(async_main());
// }

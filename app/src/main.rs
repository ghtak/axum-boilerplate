#![allow(dead_code)]

use std::env;

use app_state::AppState;
use tokio;

mod app_state;
mod define;
mod diagnostics;
mod dto;
mod entity;
mod repository;
mod router;
mod usecase;
mod util;

#[cfg(feature = "enable_websocket_pubsub_sample")]
mod ws;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    println!("{:?}", env::current_dir().unwrap());
    let config = util::config::TomlConfig::from_file("config.toml").unwrap();
    let _guard = util::tracing::init(&config.tracing).unwrap();
    tracing::trace!("{config:?}");
    let app_state = AppState::new(&config).await;
    let _ = app_state.create_tables().await;
    let router = router::init_router(app_state, &config.http);
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

#![allow(dead_code)]

use axum::{routing::get, Router};
use tokio;
mod utils;

// fn main() {
//     let rt = tokio::runtime::Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .unwrap();
//     let _ = rt.block_on(async_main());
// }

#[tokio::main]
async fn main() {
    let config = utils::config::TomlConfig::from_file("config.toml").unwrap();
    let _guard = utils::tracing::init_with_rolling_file(utils::tracing::Config::new(
        &config.log.directory,
        &config.log.file_name_prefix,
    ))
    .unwrap();

    tracing::trace!("{:?}", config);
    
    let address = format!("{}:{}", config.http.host, config.http.port);
    let router = Router::new().route("/", get(hello_axum));
    axum::Server::bind(&address.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}

async fn hello_axum() -> &'static str {
    tracing::debug!("hello_axum");
    "Hello Axum"
}

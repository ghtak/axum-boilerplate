#![allow(dead_code)]

use axum::{Router, routing::get};
use tokio;
mod utils;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _ = rt.block_on(async_main());
}

async fn async_main() {
    let _guard = utils::tracing::init_with_rolling_file(
        utils::tracing::Config::new("logs", "log")
    ).unwrap();

    let router = Router::new()
        .route("/", get(hello_axum));
    let address = format!("0.0.0.0:8089");
    tracing::trace!(address);
    axum::Server::bind(&address.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}

async fn hello_axum() -> &'static str {
    tracing::debug!("hello_axum");
    "Hello Axum"
}
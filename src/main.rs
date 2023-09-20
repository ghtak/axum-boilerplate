#![allow(dead_code)]

use tokio;
use tracing::{span, Level, event};
mod utils;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _ = rt.block_on(async_main());
}

async fn async_main() {
    let _guard = utils::tracing::init_with_rolling_file("logs", "log");
    tracing::debug!("X");
    use std::{thread, time};

    let ten_millis = time::Duration::from_millis(10);

    thread::sleep(ten_millis);
    // let span = span!(Level::TRACE, "my span");
    // span.enter();
    // event!(parent: &span, Level::INFO, "inside my_function!");

    // utils::log::init_file("log4rs.yaml").expect(
    //     "log::init_file failed"
    // );
    // log::debug!("Log Debug")
    // let router = Router::new();
    // let address = format!("0.0.0.0:8089");
    // axum::Server::bind(&address.parse().unwrap())
    //     .serve(router.into_make_service())
    //     .await
    //     .unwrap()
}



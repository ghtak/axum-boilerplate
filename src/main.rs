use tokio;
use axum::Router;

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
        "./logs".to_owned(),
        "axum-boilerplate".to_owned()
    );
    let router = Router::new();
    let address = format!("0.0.0.0:8089");
    axum::Server::bind(&address.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}

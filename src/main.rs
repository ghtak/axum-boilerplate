use tokio;
use axum::Router;


fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _ = rt.block_on(async_main());
}

async fn async_main() {
    let router = Router::new();

    let address = format!("0.0.0.0:8089");
    axum::Server::bind(&address.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}

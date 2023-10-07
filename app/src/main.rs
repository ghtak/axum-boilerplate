#![allow(dead_code)]

use std::env;

use app_state::AppState;
use tokio;

use crate::proto::{voting::VotingService, MultiplexService};

mod app_state;
mod define;
mod diagnostics;
mod dto;
mod entity;
mod repository;
mod router;
mod usecase;
mod util;
mod proto;

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
    // let rest = router::init_router(app_state, &config.http);
    // let address = config.http.socket_addr().unwrap();
    // axum::Server::bind(&address)
    //     .serve(rest.into_make_service())
    //     .await
    //     .unwrap()

    let rest = router::init_router(app_state, &config.http);
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let grpc = tonic::transport::Server::builder()
        .add_service(reflection_service)
        .add_service(proto::voting::voting_server::VotingServer::new(VotingService::default()))
        .into_service();

    let service = MultiplexService::new(rest, grpc);
    let address = config.http.socket_addr().unwrap();
    tracing::debug!("listening on {address}");
    hyper::Server::bind(&address)
        .serve(tower::make::Shared::new(service))
        .await
        .unwrap();
}

// fn main() {
//     let rt = tokio::runtime::Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .unwrap();
//     let _ = rt.block_on(async_main());
// }

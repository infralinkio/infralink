use axum::{Extension, Router};
use bollard::Docker;
use std::net::SocketAddr;

pub mod models;
pub mod routes;
pub mod state;

use crate::routes::init_routes;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "debug");

    pretty_env_logger::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let docker = Docker::connect_with_socket_defaults().unwrap();
    let state = state::State { docker };

    let app = Router::new()
        .nest("/", init_routes())
        .layer(Extension(state));

    log::info!("Starting infra api at {}", addr.to_string());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

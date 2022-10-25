use axum::Router;
use routes::init_routes;
use std::net::SocketAddr;

mod routes;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "debug");

    pretty_env_logger::init();

    let app = Router::new().nest("/", init_routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 42690));

    log::info!("Starting infra api at {}", addr.to_string());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

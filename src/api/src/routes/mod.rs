mod master;

use axum::{routing::get, Router};
use master::get_master_config;

async fn root() -> &'static str {
    "Hello World"
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/master", get(get_master_config))
}

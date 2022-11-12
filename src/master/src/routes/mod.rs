use axum::{
    routing::{get, post},
    Router,
};

mod health_check;
mod new;

use health_check::health_check;
use new::new;

async fn root() -> &'static str {
    "Hello World"
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/new", post(new))
}

use crate::handler::hello_world_handler;
use axum::routing::{get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(hello_world_handler))
}

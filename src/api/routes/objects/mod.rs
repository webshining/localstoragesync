use crate::app::AppState;
use axum::{Router, routing::post};

pub mod create;

fn objects_routes() -> Router<AppState> {
    Router::new().route("/", post(create::create_handler))
}

pub fn routes() -> Router<AppState> {
    Router::new().nest("/objects", objects_routes())
}

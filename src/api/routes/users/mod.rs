use crate::app::AppState;
use axum::{Router, routing::post};

pub mod login;
pub mod register;

fn users_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login::login_handler))
        .route("/register", post(register::register_handler))
}

pub fn routes() -> Router<AppState> {
    Router::new().nest("/users", users_routes())
}

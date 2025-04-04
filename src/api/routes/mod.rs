use crate::app::AppState;
use axum::Router;

pub mod objects;
pub mod users;

fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(objects::routes())
        .merge(users::routes())
}

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .nest("/api", api_routes())
        .with_state(app_state)
}

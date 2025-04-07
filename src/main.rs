use std::{net::SocketAddr, str::FromStr};

use axum::http::Method;
use localstorage::{api::routes, app::AppState, db, settings};
use tokio::net::TcpListener;
use tower_http::cors::{self, CorsLayer};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Load settings
    let settings = settings::Settings::new()
        .map_err(|err| {
            eprintln!("Error loading settings: {err}");
        })
        .unwrap();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize database connection
    let db = db::connect_surrealdb(settings.database.url.as_str()).await;
    tracing::info!("Connected to database");

    // Initialize the application state
    let app_state = AppState { db: db };

    // Initialize the router with the application state
    let app =
        routes::routes(app_state).layer(CorsLayer::new().allow_origin(cors::Any).allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ]));

    // Start the server
    let socket_addr = SocketAddr::from_str(settings.server.addr.as_str()).unwrap();
    let tcp_listener = TcpListener::bind(socket_addr).await.unwrap();
    tracing::info!("Listening on http://{}", socket_addr);
    axum::serve(tcp_listener, app).await.unwrap();
}

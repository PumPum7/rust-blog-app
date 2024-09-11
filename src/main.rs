mod errors;
mod handlers;
mod models;
mod state;

use axum::{routing::get, Router};
use state::AppState;
use std::sync::Arc;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState::new().await.expect("Failed to initialize app state"));

    let app = Router::new()
        .route("/", get(handlers::redirect_home))
        .route("/home", get(handlers::home))
        .route("/submit", axum::routing::post(handlers::submit_post))
        .route("/posts", get(handlers::get_posts))
        .nest_service("/images", ServeDir::new("images"))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
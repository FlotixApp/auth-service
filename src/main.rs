mod config;
mod session;
mod providers;
mod routes;

use axum::{routing::get, Router};
use crate::routes::auth::*;

use crate::config::Config;
use crate::session::session_layer;
use routes::auth::AppState;

#[tokio::main]
async fn main() {
    let cfg = Config::load();
    let state = AppState { cfg };

    let app = Router::new()
        .route("/auth/google", get(google_login))
        .route("/auth/google/callback", get(google_callback))
        .route("/auth/github", get(github_login))
        .route("/auth/github/callback", get(github_callback))
        .route("/auth/apple", get(apple_login))
        // .route("/auth/apple/callback", ...)
        .with_state(state)
        .layer(session_layer());

    println!("Auth service running on http://localhost:8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}


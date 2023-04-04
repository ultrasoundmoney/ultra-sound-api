use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
mod contract_leaderboard;
use contract_leaderboard::get_contracts;
mod state;
use state::AppState;
mod db;
mod env;

#[derive(Clone, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let db_pool = db::get_db_pool().await;

    let shared_state = Arc::new(AppState { db_pool });
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/contracts", get(get_contracts))
        .with_state(shared_state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use axum::{
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod contract_leaderboard;
use contract_leaderboard::get_contracts;


#[derive(Clone, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/contracts", get(get_contracts));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


}

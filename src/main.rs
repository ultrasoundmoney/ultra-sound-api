use std::net::SocketAddr;
mod app;
mod contract_leaderboard;
mod db;
mod env;
mod state;

#[tokio::main]
async fn main() {
    let app = app::get_app().await;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

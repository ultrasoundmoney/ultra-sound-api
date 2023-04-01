use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let mut users: HashMap<u32, User> = HashMap::new();
    users.insert(0, User {
        name: "John Doe".to_string(),
        email: "john@doe.com".to_string(),
    });
    let users_arc = Arc::new(RwLock::new(users));

    let get_users_arc = users_arc.clone();
    let get_users = || async move {
        let users = get_users_arc.read().unwrap().clone();
        Json(users)
    };

    let create_user_arc = users_arc.clone();
    let create_user = |Json(payload): Json<User>| async move {
        let mut users = create_user_arc.write().unwrap();
        let id = users.len() as u32;
        users.insert(id, payload);
        (StatusCode::CREATED, Json(id))
    };

    async fn index() -> &'static str {
        "Hello, World!"
    }

    let tracing_subscriber = tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(index))
        .route("/users", get(get_users))
        .route("/users", post(create_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


}

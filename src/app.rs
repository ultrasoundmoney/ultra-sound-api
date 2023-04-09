use crate::contract_leaderboard::get_contracts;
use crate::db::get_db_pool;
use crate::state::AppState;
use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub async fn get_app() -> Router {
    let db_pool = get_db_pool().await;
    get_app_with_db_pool(db_pool)
}

fn get_app_with_db_pool(db_pool: PgPool) -> Router {
    let shared_state = Arc::new(AppState { db_pool });
    Router::new()
        .route("v1/top-contracts", get(get_contracts))
        .with_state(shared_state)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::contract_leaderboard::ContractEntry;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_contracts_against_dev_db() {
        let app = get_app().await;
        let response = app
            .oneshot(
                Request::builder()
                    .uri("v1/top-contracts")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("Request Failed");
        assert_eq!(response.status(), 200);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let contracts: Vec<ContractEntry> = serde_json::from_slice(&body).unwrap();
        assert_eq!(contracts.len(), 100);

        for contract in contracts {
            assert!(contract.address.len() > 0);
            assert_eq!(&contract.address[..2], "0x");
            assert!(contract.base_fees > 0.0);
        }
    }
}
